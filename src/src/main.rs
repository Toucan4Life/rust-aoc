fn find_joltage_part1(batteries: String) -> u32 {
    let mut best = 0;
    let batteries_int:Vec<u32>=batteries.chars().map(|x| x.to_digit(10).unwrap()).collect();
    for i in 0..batteries_int.iter().count()-1{
        for j in i+1..batteries_int.iter().count(){
            let candidate = batteries_int[i]*10+batteries_int[j];
            if candidate>best{
                best = candidate
            }
        }
    }
    best.try_into().unwrap()
}
fn find_ordered_permutation(batteries: String, k: i32){
    if k==1 {
        
    }
}
fn main() {
    let input = include_str!("./input.txt");
    let part1 :u32= input.lines().map(|line| find_joltage_part1(line.to_string())).sum();
    println!("{part1}");
}

#[test]
fn part1(){
    let input = "987654321111111";
    let input = "811111111111119";
    let input = "234234234234278";
    let input = "818181911112111";
    let jolt = find_joltage_part1(input.to_string());
    assert_eq!(jolt, 89)
}

#[test]
fn part2(){
    let input = "987654321111111";
    let input = "811111111111119";
    let input = "234234234234278";
    let input = "818181911112111";
    let jolt = find_joltage_part1(input.to_string());
    assert_eq!(jolt, 89)
}
