fn find_joltage_part1(batteries: String) -> u32 {
    let mut best = 0;
    let batteries_int: Vec<u32> = batteries.chars().map(|x| x.to_digit(10).unwrap()).collect();
    for i in 0..batteries_int.len() - 1 {
        for j in i + 1..batteries_int.len() {
            let candidate = batteries_int[i] * 10 + batteries_int[j];
            if candidate > best {
                best = candidate
            }
        }
    }
    best
}

fn find_max_subnumber(batteries: String, k: usize) -> Vec<char> {
    let mut digit = Vec::<char>::new();
    for (i, ch) in batteries.char_indices() {
        while batteries.len() - i > k - digit.len()
            && digit
                .pop_if(|last_char| ch.to_digit(10) > last_char.to_digit(10))
                .is_some()
        {}

        if digit.len() < k {
            digit.push(ch);
        }
    }

    digit
}

// 17493
// 173685428989126
fn main() {
    let input = include_str!("./input.txt");
    let part1: u32 = input
        .lines()
        .map(|line| find_joltage_part1(line.to_string()))
        .sum();
    println!("{part1}");
    let part2: i64 = input
        .lines()
        .map(|line| {
            find_max_subnumber(line.to_string(), 12)
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        })
        .sum();
    println!("{part2}");
}

#[test]
fn part1() {
    let input = "987654321111111";
    let input = "811111111111119";
    let input = "234234234234278";
    let input = "818181911112111";
    let jolt = find_joltage_part1(input.to_string());
    assert_eq!(jolt, 92)
}

#[test]
fn part2() {
    let input = "987654321111111";
    let input = "811111111111119";
    // let input = "234234234234278";
    // let input = "818181911112111";
    // let input = "34234234234278";
    // let input = "12989191111211";
    // let input = "165878981214187";
    let jolt = find_max_subnumber(input.to_string(), 12)
        .iter()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    // assert_eq!(jolt, 878981214187)
    assert_eq!(jolt, 811111111119)
}
