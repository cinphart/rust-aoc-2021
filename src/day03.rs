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
fn difference_at_pos(pos: usize, data: &Vec<Vec<char>>) -> i32 {
    data.iter().fold(0i32, |a, v| match v[pos] {
        '0' => a - 1,
        '1' => a + 1,
        _ => a,
    })
}

fn matching_only(pos: usize, ch: char, data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    data.iter()
        .filter(|s| s[pos] == ch)
        .map(|s| s.clone())
        .collect()
}

fn part2(name: &str) -> u32 {
    let log: Vec<Vec<char>> = fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .filter(|s| s.len() != 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let maxlen = log.iter().map(|s| s.len()).max().unwrap();

    let mut ogen = log.clone();
    let mut scrub = log.clone();

    for s in 0..maxlen {
        let ogen_pick = match difference_at_pos(s, &ogen) {
            x if x < 0 => '0',
            _ => '1',
        };
        let scrub_pick = match difference_at_pos(s, &scrub) {
            x if x < 0 => '1',
            _ => '0',
        };
        ogen = if ogen.len() > 1 {
            matching_only(s, ogen_pick, &ogen)
        } else {
            ogen
        };
        scrub = if scrub.len() > 1 {
            matching_only(s, scrub_pick, &scrub)
        } else {
            scrub
        };
    }

    let ogen_result = ogen[0]
        .iter()
        .fold(0u32, |a, s| return 2 * a + if *s == '1' { 1 } else { 0 });
    let scrub_result = scrub[0]
        .iter()
        .fold(0u32, |a, s| return 2 * a + if *s == '1' { 1 } else { 0 });

    ogen_result * scrub_result
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
    fn difference_at_pos_on_empty_works() {
        assert_eq!(difference_at_pos(0, &Vec::new()), 0)
    }

    fn inp() -> Vec<Vec<char>> {
        ["0101", "0001", "0101", "1000"]
            .iter()
            .map(|s| s.chars().collect())
            .collect()
    }

    #[test]
    fn difference_at_pos_more_ones_works() {
        assert_eq!(difference_at_pos(3, &inp()), 2);
    }

    #[test]
    fn difference_at_pos_more_zeros_works() {
        assert_eq!(difference_at_pos(0, &inp()), -2);
        assert_eq!(difference_at_pos(2, &inp()), -4);
    }

    #[test]
    fn difference_at_pos_same_works() {
        assert_eq!(difference_at_pos(1, &inp()), 0);
    }

    #[test]
    fn matching_only_works() {
        let original = inp();
        assert_eq!(
            matching_only(0, '0', &original),
            [0usize, 1usize, 2usize]
                .iter()
                .map(|s| original[*s].clone())
                .collect::<Vec<Vec<char>>>()
        );
        assert_eq!(
            matching_only(0, '1', &original),
            [3usize]
                .iter()
                .map(|s| original[*s].clone())
                .collect::<Vec<Vec<char>>>()
        );
        assert_eq!(
            matching_only(1, '0', &original),
            [1usize, 3usize]
                .iter()
                .map(|s| original[*s].clone())
                .collect::<Vec<Vec<char>>>()
        );
        assert_eq!(
            matching_only(1, '1', &original),
            [0usize, 2usize]
                .iter()
                .map(|s| original[*s].clone())
                .collect::<Vec<Vec<char>>>()
        );
    }
}
