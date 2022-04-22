use std::fs;

fn part1(name: &str) -> u32 {
    let log: Vec<Vec<char>> = fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .filter(|s| s.len() != 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let maxlen = log.iter().map(|s| s.len()).max().unwrap();
    let counts = (0..maxlen)
        .map(|i| {
            log.iter().fold(0, |a, b| match b[i] {
                '0' => a - 1,
                '1' => a + 1,
                _ => a,
            })
        })
        .collect::<Vec<i32>>();
    let gamma: u32 = counts.iter().fold(0, |a, b| match *b {
        x if x < 0 => a * 2,
        x if x > 0 => a * 2 + 1,
        _ => a * 2 + 1,
    });
    let epsilon: u32 = counts.iter().fold(0, |a, b| match *b {
        x if x < 0 => a * 2 + 1,
        x if x > 0 => a * 2,
        _ => a * 2,
    });
    gamma * epsilon
}

// Returns +1 if 1 is most common, -1 if 0 is most common, 0 otherwise.
fn cmp(pos: usize, data: &Vec<Vec<char>>) -> i32 {
    data.iter().fold(0i32, |a, v| match v[pos] {
        '0' => a - 1,
        '1' => a + 1,
        _ => a,
    })
}

fn part2(name: &str) -> u32 {
    let log: Vec<Vec<char>> = fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .filter(|s| s.len() != 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let maxlen = log.iter().map(|s| s.len()).max().unwrap();
    (log.len() + maxlen) as u32
}

fn main() {
    println!("Part 1: {}", part1("data/Day03.txt"));
    println!("Part 2: {}", part2("data/Day03.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("data/Day03_test.txt"), 198);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("data/Day03_test.txt"), 230);
    }

    #[test]
    fn cmp_on_empty_works() {
        assert_eq!(cmp(0, &Vec::new()), 0)
    }

    fn inp() -> Vec<Vec<char>> {
        ["0101", "0001", "0101", "1000"]
            .iter()
            .map(|s| s.chars().collect())
            .collect()
    }

    #[test]
    fn cmp_more_ones_works() {
        assert_eq!(cmp(3, &inp()), 2);
    }

    #[test]
    fn cmp_more_zeros_works() {
        assert_eq!(cmp(0, &inp()), -2);
        assert_eq!(cmp(2, &inp()), -4);
    }

    #[test]
    fn cmp_same_works() {
        assert_eq!(cmp(1, &inp()), 0);
    }
}
