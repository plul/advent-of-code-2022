//! Day 7: No space left on device
//!
//! https://adventofcode.com/2022/day/7

use file_system::FileSystem;
use file_system::FileSystemNode;
use std::path::Path;
use std::path::PathBuf;

pub fn part_1(input: &str) -> u64 {
    let commands = parser::parse(input);
    let file_system = build_file_system(commands);

    file_system
        .dirs()
        .map(|(path, _)| path)
        .map(|path| file_system.size_recursive(path))
        .filter(|dir_size| *dir_size <= 100_000)
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let commands = parser::parse(input);
    let file_system = build_file_system(commands);

    let total_space = 70000000;
    let unused_space_required = 30000000;
    let used_space = file_system.size_recursive(Path::new(""));
    let free_space = total_space - used_space;
    debug_assert!(unused_space_required > free_space);
    let need_to_free = unused_space_required - free_space;

    file_system
        .dirs()
        .map(|(path, _)| path)
        .map(|path| file_system.size_recursive(path))
        .filter(|dir_size| *dir_size >= need_to_free)
        .min()
        .unwrap()
}

fn build_file_system(commands: impl IntoIterator<Item = Command>) -> FileSystem {
    let mut file_system = FileSystem::default();
    let mut cwd = PathBuf::new();

    for command in commands {
        match command {
            Command::Cd(cd) => match cd {
                Cd::Root => {
                    cwd.clear();
                }
                Cd::In(dir_name) => {
                    cwd.push(dir_name);
                }
                Cd::Out => {
                    assert!(cwd.pop());
                }
            },
            Command::Ls(ls) => {
                let entries = ls
                    .entries
                    .iter()
                    .map(LsEntry::name)
                    .map(ToOwned::to_owned)
                    .collect();
                file_system.insert(
                    cwd.clone(),
                    FileSystemNode::Directory {
                        entries: Some(entries),
                    },
                );

                for entry in ls.entries {
                    match entry {
                        LsEntry::Directory(d) => {
                            let p = cwd.join(d.name);
                            file_system.insert(p, FileSystemNode::Directory { entries: None });
                        }
                        LsEntry::File(f) => {
                            let p = cwd.join(f.name);
                            file_system.insert(p, FileSystemNode::File { size: f.size });
                        }
                    }
                }
            }
        }
    }

    file_system
}

enum Command {
    Cd(Cd),
    Ls(Ls),
}

enum Cd {
    Root,
    In(String),
    Out,
}

struct Ls {
    entries: Vec<LsEntry>,
}

#[derive(Debug)]
enum LsEntry {
    Directory(LsEntryDirectory),
    File(LsEntryFile),
}
impl LsEntry {
    fn name(&self) -> &str {
        match self {
            LsEntry::Directory(d) => &d.name,
            LsEntry::File(f) => &f.name,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LsEntryDirectory {
    name: String,
}

#[derive(Debug)]
struct LsEntryFile {
    name: String,
    size: u64,
}

mod file_system {
    use std::collections::BTreeMap;
    use std::path::Path;
    use std::path::PathBuf;

    #[derive(Default, Debug)]
    pub(super) struct FileSystem {
        nodes: BTreeMap<PathBuf, FileSystemNode>,
    }
    impl FileSystem {
        /// Return iterator over directories found in file system.
        pub fn dirs(&self) -> impl Iterator<Item = (&PathBuf, &FileSystemNode)> {
            self.nodes.iter().filter(|(_, node)| node.is_dir())
        }

        /// Insert path into file system.
        pub fn insert(&mut self, path: PathBuf, node: FileSystemNode) -> Option<FileSystemNode> {
            self.nodes.insert(path, node)
        }

        /// Size of file if node is a file, size of all files in subtree if node is a dir.
        ///
        /// # Panics
        ///
        /// Will panic if any path during traversal is not found in filesystem.
        pub fn size_recursive(&self, path: &Path) -> u64 {
            let node = self
                .nodes
                .get(path)
                .unwrap_or_else(|| panic!("Path {path:?} not found in file system."));
            let mut size = 0;

            match node {
                FileSystemNode::Directory { entries } => {
                    let entries = entries
                        .as_ref()
                        .unwrap_or_else(|| panic!("Path {path:?} has no information on its entries"));
                    for entry in entries {
                        let p = path.join(entry);
                        size += self.size_recursive(&p);
                    }
                }
                FileSystemNode::File { size: filesize } => {
                    size += filesize;
                }
            }
            size
        }
    }

    #[derive(Debug)]
    pub(super) enum FileSystemNode {
        Directory {
            /// Entries is None until we have entered with cd and listed the contents with ls
            entries: Option<Vec<String>>,
        },
        File {
            size: u64,
        },
    }
    impl FileSystemNode {
        pub fn is_dir(&self) -> bool {
            matches!(self, FileSystemNode::Directory { .. })
        }

        #[allow(dead_code)]
        pub fn is_file(&self) -> bool {
            matches!(self, FileSystemNode::File { .. })
        }
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Command> {
        all_consuming(many0(parse_command))(s).unwrap().1
    }

    fn parse_command(s: &str) -> IResult<&str, Command> {
        let parse_cd = map(parse_cd, Command::Cd);
        let parse_ls = map(parse_ls, Command::Ls);
        alt((parse_cd, parse_ls))(s)
    }

    fn parse_cd(s: &str) -> IResult<&str, Cd> {
        let (s, _) = tag("$ cd ")(s)?;

        let root = map(tag("/"), |_| Cd::Root);
        let cd_in = map(alphanumeric1, |p: &str| Cd::In(p.into()));
        let cd_out = map(tag(".."), |_| Cd::Out);

        let (s, cd) = terminated(alt((root, cd_in, cd_out)), line_ending)(s)?;
        Ok((s, cd))
    }

    fn parse_ls(s: &str) -> IResult<&str, Ls> {
        let (s, _) = terminated(tag("$ ls"), line_ending)(s)?;

        let parse_dir = map(parse_ls_dir, LsEntry::Directory);
        let parse_file = map(parse_ls_file, LsEntry::File);

        let (s, entries) = many0(alt((parse_dir, parse_file)))(s)?;

        Ok((s, Ls { entries }))
    }

    fn parse_ls_dir(s: &str) -> IResult<&str, LsEntryDirectory> {
        let (s, _) = tag("dir ")(s)?;
        let (s, name) = terminated(take_till_whitespace1, line_ending)(s)?;
        Ok((
            s,
            LsEntryDirectory {
                name: name.to_owned(),
            },
        ))
    }
    #[test]
    fn test_parse_ls_dir() {
        assert_eq!(
            parse_ls_dir("dir d\n").unwrap().1,
            LsEntryDirectory {
                name: String::from("d")
            }
        );
    }

    fn parse_ls_file(s: &str) -> IResult<&str, LsEntryFile> {
        let (s, size) = u64(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, name) = terminated(take_till_whitespace1, line_ending)(s)?;
        let file = LsEntryFile {
            name: name.to_owned(),
            size,
        };
        Ok((s, file))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 95437);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 24933642);
}
