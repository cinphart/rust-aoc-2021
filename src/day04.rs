use std::fs;

fn part1(_name: &str) -> u32 {
    0u32
}

fn part2(name: &str) -> u32 {
    let log = fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .map(|s| std::string::String::from(s))
        .filter(|s| s.len() != 0)
        .collect::<Vec<String>>();
    log.len() as u32
}

fn main() {
    println!("Part 1: {}", part1("data/Day04.txt"));
    println!("Part 2: {}", part2("data/Day04.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod day04 {
        use super::*;

        #[test]
        fn part1_works() {
            assert_eq!(part1("data/Day04_test.txt"), 4512);
        }

        #[test]
        fn part2_works() {
            assert_eq!(part2("data/Day04_test.txt"), 1924);
        }
    }
}
