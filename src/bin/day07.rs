use anyhow::{bail, Context, Ok, Result};
use hashbrown::HashMap;
use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! { struct DirKey; }
type DirMap<'a> = SlotMap<DirKey, Directory<'a>>;
type DirSizeMap<'a> = SecondaryMap<DirKey, u64>;

#[derive(Clone, Debug, Default)]
struct Directory<'a> {
    children: HashMap<&'a str, DirKey>,
    files: Vec<(&'a str, u64)>,
}

fn update_size(k: DirKey, sm: &DirMap<'_>, sizes: &mut DirSizeMap) -> u64 {
    let child_keys = sm[k].children.values();
    let subtree_size: u64 = child_keys.map(|c| update_size(*c, sm, sizes)).sum();
    let files_size: u64 = sm[k].files.iter().map(|(_, sz)| sz).sum::<u64>();
    sizes.insert(k, subtree_size + files_size);
    subtree_size + files_size
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day07.txt")?;
    let start = std::time::Instant::now();

    let mut dirs = DirMap::with_key();
    let root = dirs.insert(Directory::default());
    let mut path = Vec::new();
    let mut command = Vec::with_capacity(3);
    for line in input.lines() {
        let cwd = *path.last().unwrap_or(&root);
        command.splice(.., line.trim().split_ascii_whitespace());
        match &command[..] {
            &["$", "cd", "/"] => path.clear(),
            &["$", "cd", ".."] => drop(path.pop()),
            &["$", "cd", dir] => path.push(*dirs[cwd].children.get(dir).context("no such dir")?),
            &["$", "ls"] => {},
            &["dir", dir] => {
                let child = dirs.insert(Directory::default());
                dirs[cwd].children.insert(dir, child);
            },
            &[num, dir] => dirs[cwd].files.push((dir, num.parse()?)),
            _ => bail!("unexpected command {command:?}"),
        }
    }

    let mut sizemap = DirSizeMap::new();
    update_size(root, &dirs, &mut sizemap);
    let to_clean_up = sizemap[root].saturating_sub(40_000_000);
    let part1: u64 = sizemap.values().filter(|sz| **sz <= 100_000).sum();
    let part2 = sizemap.values().filter(|sz| **sz >= to_clean_up).min();

    println!("part1: {part1}");
    println!("part2: {}", part2.context("no part2 solution")?);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
