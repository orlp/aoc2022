use anyhow::{Ok, Result};

/*
    We use the fact that each line (including the newline) consists of 4 bytes.
    We add to the part1, part2 scores based on this lookup table:

    Input     I+\n as u32  p1   p2
        A X    173547585,   4,   3
        A Y    173613121,   8,   4
        A Z    173678657,   3,   8
        B X    173547586,   1,   1
        B Y    173613122,   5,   5
        B Z    173678658,   9,   9
        C X    173547587,   7,   2
        C Y    173613123,   2,   6
        C Z    173678659,   6,   7

    u.wrapping_mul(1887065750_u32) >> 27 gives [15, 26, 4, 29, 8, 18, 11, 22, 0]
    Look up p1, p2 using this, encoded as p - (u == 173678658) - 1 to make all
    entries [0, 8) and thus fitting in 3 bits each packed inside a single u32.
*/

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day02.txt")?;
    let start = std::time::Instant::now();

    let ints: &[u32] = bytemuck::cast_slice(input.as_bytes());
    let mut part1 = ints.len() as u32;
    let mut part2 = ints.len() as u32;
    for u in ints {
        let o = u.wrapping_mul(1887065750_u32) >> 27;
        part1 += (*u == 173678658) as u32 + ((475903013 >> o) & 7);
        part2 += (*u == 173678658) as u32 + ((224201846 >> o) & 7);
    }

    let time = start.elapsed();
    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", time);
    Ok(())
}
