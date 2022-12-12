use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let fs = build_file_system(input);

    dir_sizes(&fs)
        .into_iter()
        .filter(|&size| size <= 100_000)
        .sum()
}

// TODO write cool, stack based recursive iterator
fn dir_sizes(fs: &FileSystem) -> Vec<usize> {
    fn helper(fs: &FileSystem, sizes: &mut Vec<usize>) {
        for directory in fs.values() {
            sizes.push(directory.size);
            helper(&directory.contents, sizes);
        }
    }

    let mut sizes = Vec::new();
    helper(fs, &mut sizes);
    sizes
}

type FileSystem<'a> = HashMap<&'a str, Directory<'a>>;

#[derive(Debug, Default)]
struct Directory<'a> {
    size: usize,
    contents: FileSystem<'a>,
}

fn build_file_system(input: &str) -> FileSystem {
    let mut path = Vec::new();
    let mut fs = FileSystem::new();
    let mut seen_paths = HashSet::new();
    let mut skipping_ls = false;
    for line in input.lines() {
        if let Some(cd_target) = line.strip_prefix("$ cd ") {
            skipping_ls = false;
            match cd_target {
                "/" => path.clear(),
                ".." => {
                    path.pop();
                }
                dir => path.push(dir),
            }
        } else if line.starts_with('$') {
            // If we're `ls`ing the same path again, set a flag to skip all the file contents so we
            // don't mess up any sizes.
            //
            // NOTE: In my input, this isn't needed at all. But it seems like a good idea.
            let current_path = path.join("");
            skipping_ls = !seen_paths.insert(current_path);
        } else {
            if skipping_ls {
                continue;
            }

            let size: usize = match line.split(' ').next() {
                // We can ignore directories listed here. We'll presumably `cd` into them later.
                Some("dir") => continue,
                Some(size) => size.parse().expect("File size should be valid"),
                _ => panic!("Invalid result from `ls` command"),
            };

            let mut entry = fs.entry("/").or_default();
            for dir in &path {
                entry.size += size;
                entry = entry.contents.entry(dir).or_default();
            }
            entry.size += size;
        }
    }

    fs
}

pub fn part2(input: &str) -> usize {
    let fs = build_file_system(input);

    let disk_space = 70_000_000;
    let total_needed = 30_000_000;
    let total_used = fs.get("/").map_or(0, |dir| dir.size);
    let total_free = disk_space - total_used;
    let needed = total_needed - total_free;

    dir_sizes(&fs)
        .into_iter()
        .filter(|&size| size >= needed)
        .min()
        .expect("File system empty")
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let file = "$ cd /\n\
                        $ ls\n\
                        dir a\n\
                        14848514 b.txt\n\
                        8504156 c.dat\n\
                        dir d\n\
                        $ cd a\n\
                        $ ls\n\
                        dir e\n\
                        29116 f\n\
                        2557 g\n\
                        62596 h.lst\n\
                        $ cd e\n\
                        $ ls\n\
                        584 i\n\
                        $ cd ..\n\
                        $ cd ..\n\
                        $ cd d\n\
                        $ ls\n\
                        4060174 j\n\
                        8033020 d.log\n\
                        5626152 d.ext\n\
                        7214296 k";

            assert_eq!(part1(file), 95437);
        }

        #[test]
        fn repeated_ls() {
            let file = "$ cd /\n\
                        $ ls\n\
                        1 a.txt\n\
                        $ ls\n\
                        1 a.txt";

            assert_eq!(part1(file), 1);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part1(file), 1_086_293);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let file = "$ cd /\n\
                        $ ls\n\
                        dir a\n\
                        14848514 b.txt\n\
                        8504156 c.dat\n\
                        dir d\n\
                        $ cd a\n\
                        $ ls\n\
                        dir e\n\
                        29116 f\n\
                        2557 g\n\
                        62596 h.lst\n\
                        $ cd e\n\
                        $ ls\n\
                        584 i\n\
                        $ cd ..\n\
                        $ cd ..\n\
                        $ cd d\n\
                        $ ls\n\
                        4060174 j\n\
                        8033020 d.log\n\
                        5626152 d.ext\n\
                        7214296 k";

            assert_eq!(part2(file), 24_933_642);
        }

        #[test]
        fn my_input() {
            let file = include_str!("./input.txt");
            assert_eq!(part2(file), 366_028);
        }
    }
}
