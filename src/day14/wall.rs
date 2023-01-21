use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct VerticalWall {
    x: i64,
    y_top: i64,
    y_bottom: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct HorizontalWall {
    pub(super) y: i64,
    pub(super) x_left: i64,
    pub(super) x_right: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) enum Wall {
    Horizontal(HorizontalWall),
    Vertical(VerticalWall),
}

impl Wall {
    pub(super) fn new(start: (i64, i64), end: (i64, i64)) -> Self {
        // X positions are the same - this is vertical
        if start.0 == end.0 {
            // Smaller y values are higher
            let x = start.0;
            let (mut y_bottom, mut y_top) = (start.1, end.1);
            // TODO is this faster than an if statement?
            if start.1 < end.1 {
                std::mem::swap(&mut y_bottom, &mut y_top);
            }
            Self::Vertical(VerticalWall { x, y_bottom, y_top })
        } else {
            // Better be horizontal
            // Smaller x values are left
            let y = start.1;
            let (mut x_left, mut x_right) = (end.0, start.0);
            if start.0 < end.0 {
                std::mem::swap(&mut x_left, &mut x_right);
            }
            Self::Horizontal(HorizontalWall { y, x_left, x_right })
        }
    }

    fn top(&self) -> i64 {
        match self {
            Self::Vertical(v_wall) => v_wall.y_top,
            Self::Horizontal(h_wall) => h_wall.y,
        }
    }

    pub(super) fn bottom(&self) -> i64 {
        match self {
            Self::Vertical(v_wall) => v_wall.y_bottom,
            Self::Horizontal(h_wall) => h_wall.y,
        }
    }

    fn contains(&self, (x, y): (i64, i64)) -> bool {
        match self {
            Self::Vertical(v_wall) => v_wall.x == x && y >= v_wall.y_top && y <= v_wall.y_bottom,
            Self::Horizontal(h_wall) => h_wall.y == y && x >= h_wall.x_left && x <= h_wall.x_right,
        }
    }
}

enum SegmentKind {
    Start,
    End,
}
struct SegmentEvent {
    x: i64,
    kind: SegmentKind,
    idx: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct DisjointXRange {
    left: i64,
    right: i64,
}

impl Ord for DisjointXRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.left.cmp(&other.left)
    }
}

impl PartialOrd for DisjointXRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub(super) struct Walls {
    // `BTreeMap` was slightly faster than `HashMap` for me. We could bring in a dependency and use
    // a faster hash function but I'm trying to avoid those. This could also be a `Vec` and, unless
    // the X spread is huge (which, for my data, it isn't), it'll be much faster.
    v_walls: BTreeMap<i64, Vec<usize>>, // If long, could be BTreeSet
    h_walls: Vec<(DisjointXRange, Vec<usize>)>, // Ditto
    walls: Vec<Wall>,
}

impl Walls {
    pub(super) fn new(walls: Vec<Wall>) -> Self {
        // My input has lots of duplicate walls but, in general, this isn't an optimization.
        let mut unique_walls = HashSet::new();

        let mut events = Vec::new();
        let mut v_walls: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
        for (i, wall) in walls.iter().enumerate() {
            let is_new = unique_walls.insert(wall);
            if !is_new {
                continue;
            }
            match wall {
                Wall::Vertical(vertical_wall) => {
                    v_walls.entry(vertical_wall.x).or_default().push(i);
                }
                Wall::Horizontal(horizontal_wall) => {
                    events.extend([
                        SegmentEvent {
                            x: horizontal_wall.x_left,
                            kind: SegmentKind::Start,
                            idx: i,
                        },
                        SegmentEvent {
                            // Ranges are exclusive on the right
                            x: horizontal_wall.x_right + 1,
                            kind: SegmentKind::End,
                            idx: i,
                        },
                    ]);
                }
            }
        }
        events.sort_by_key(|segment_event| segment_event.x);

        let mut h_walls = Vec::new();
        let mut events = events.into_iter();

        if let Some(first_event) = events.next() {
            let mut left = first_event.x;
            let mut segments = vec![first_event.idx];

            for event in events {
                // Close off the current range
                let range = DisjointXRange {
                    left,
                    right: event.x,
                };
                h_walls.push((range, segments.clone()));

                // Begin a new range
                left = event.x;
                match event.kind {
                    SegmentKind::Start => {
                        segments.push(event.idx);
                    }
                    SegmentKind::End => {
                        let pos = segments
                            .iter()
                            .position(|&segment_idx| segment_idx == event.idx)
                            .expect("We're closing an event but we never added the segment!");
                        // We don't care about the order of elements inside this list ... yet.
                        segments.swap_remove(pos);
                    }
                }
            }
        }
        Self {
            v_walls,
            h_walls,
            walls,
        }
    }

    pub(super) fn intersection_for_vertical_ray(&self, (x, y): (i64, i64)) -> Option<usize> {
        let mut min_y = i64::MAX;
        let mut min_idx = None;

        let vertical_matches = self.vertical_matches(x);
        if let Some(vertical_matches) = vertical_matches {
            for &idx in vertical_matches {
                let top = self.top(idx);
                if top > y && top < min_y {
                    min_y = top;
                    min_idx = Some(idx);
                }
            }
        }

        let horizontal_matches = self.horizontal_matches(x);
        if let Some(horizontal_matches) = horizontal_matches {
            for &idx in horizontal_matches {
                let top = self.top(idx);
                if top > y && top < min_y {
                    min_y = top;
                    min_idx = Some(idx);
                }
            }
        }
        min_idx
    }

    pub(super) fn top(&self, wall_idx: usize) -> i64 {
        self.walls[wall_idx].top()
    }

    pub(super) fn contains(&self, (x, y): (i64, i64)) -> bool {
        let vertical_match_contains = self.vertical_matches(x).map_or(false, |idxs| {
            idxs.iter().any(|&idx| self.walls[idx].contains((x, y)))
        });
        if vertical_match_contains {
            return true;
        }

        let horizontal_match_contains = self.horizontal_matches(x).map_or(false, |idxs| {
            idxs.iter().any(|&idx| self.walls[idx].contains((x, y)))
        });
        if horizontal_match_contains {
            return true;
        }

        false
    }

    fn vertical_matches(&self, x: i64) -> Option<&Vec<usize>> {
        self.v_walls.get(&x)
    }

    fn horizontal_matches(&self, x: i64) -> Option<&Vec<usize>> {
        self.h_walls
            .binary_search_by(|(range, _)| {
                // Range is inclusive on left and exclusive on right.
                if x < range.left {
                    Ordering::Greater
                } else if x >= range.right {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .map(|idx| &self.h_walls[idx].1)
            .ok()
    }

    pub(super) fn add_sand_to(&mut self, wall_idx: usize, (x, y): (i64, i64)) {
        match &mut self.walls[wall_idx] {
            Wall::Vertical(v_wall) => {
                v_wall.y_top -= 1;
            }
            _ => {
                let len = self.walls.len();
                self.walls.push(Wall::Vertical(VerticalWall {
                    x,
                    y_top: y,
                    y_bottom: y,
                }));
                self.v_walls.entry(x).or_default().push(len);
            }
        }
    }
}
