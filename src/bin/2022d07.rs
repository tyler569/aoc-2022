use aoc_rs::get_input;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 7)?;
    let tree = parse(&input);

    println!("part1: {}", part1(&tree));
    println!("part2: {}", part2(&tree));

    Ok(())
}

#[derive(Debug)]
enum DirectoryEntry {
    Directory(DirectoryTree),
    File(usize),
}

type DirectoryTree = HashMap<String, DirectoryEntry>;

impl DirectoryEntry {
    fn size(&self) -> usize {
        match self {
            Self::File(size) => *size,
            Self::Directory(dir) => {
                dir.iter().fold(0, |acc, (_, v)| acc + v.size())
            },
        }
    }

    fn iter(&self) -> impl Iterator<Item = (&String, &DirectoryEntry)> + '_ {
        if let Self::Directory(dir) = self {
            dir.iter().filter(|(_, v)| matches!(v, &DirectoryEntry::Directory(_)))
        } else {
            panic!("You can't iterate a File")
        }
    }

    #[allow(unused)]
    fn prettyprint(&self, level: usize) {
        let empty = "";
        match self {
            Self::File(size) => println!("{size}"),
            Self::Directory(dir) => {
                println!();
                for (k, v) in dir {
                    print!("{empty:>level$}{k}: ");
                    v.prettyprint(level + 2);
                }
            }
        }
    }
}

fn parse(input: &str) -> DirectoryEntry {
    let mut iterator = input.trim().split("\n").collect::<Vec<_>>();
    let tree = parse_inner(&mut iterator);
    // tree.prettyprint(0);
    tree
}

fn parse_inner<'a>(input: &mut Vec<&str>) -> DirectoryEntry {
    let mut tree = DirectoryTree::new();

    while !input.is_empty() {
        let line = input.remove(0);

        let words = line.split(" ").collect_vec();
        match words.as_slice() {
            &["$", "cd", ".."] => break,
            &["$", "ls"] => continue,
            &["$", "cd", "/"] => continue,
            &["$", "cd", name] => { tree.insert(name.to_string(), parse_inner(input)); },
            &["dir", _] => {},
            &[size, name] => { tree.insert(name.to_string(), DirectoryEntry::File(size.parse().unwrap())); },
            l @ _ => panic!("unknown line format {l:?}"),
        }
    }

    DirectoryEntry::Directory(tree)
}

fn walk(tree: &DirectoryEntry, func: &mut impl FnMut(&String, &DirectoryEntry)) {
    tree.iter().for_each(|(name, entry)| {
        func(name, entry);
        walk(entry, func);
    })
}

fn part1(tree: &DirectoryEntry) -> usize {
    let mut subtree_sizes = HashMap::new();
    let mut number = 0;

    walk(tree, &mut |_, entry| {
        subtree_sizes.insert(format!("{number}"), entry.size());
        number += 1;
    });

    // println!("subtrees: {subtree_sizes:?}");
    // let mut subtrees_under_100k = subtree_sizes.iter().filter(|(_, &v)| v < 100_000).collect::<Vec<_>>();
    // subtrees_under_100k.sort();
    // println!("subtrees <100k: {subtrees_under_100k:?}");

    subtree_sizes.values().filter(|&&v| v < 100_000).sum()
}

fn part2(tree: &DirectoryEntry) -> usize {
    let mut subtree_sizes = HashMap::new();
    let mut number = 0;

    walk(tree, &mut |_, entry| {
        subtree_sizes.insert(format!("{number}"), entry.size());
        number += 1;
    });

    let total_size = tree.size();
    let free_space = 70_000_000 - total_size;
    let need_to_free = 30_000_000 - free_space;

    *subtree_sizes.values().filter(|&&v| v > need_to_free).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str =
"$ cd /
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
    fn test_part1() {
        let tree = parse(&SAMPLE);
        assert_eq!(part1(&tree), 95437);
    }

    #[test]
    fn test_part2() {
        let tree = parse(&SAMPLE);
        assert_eq!(part2(&tree), 24933642);
    }
}
