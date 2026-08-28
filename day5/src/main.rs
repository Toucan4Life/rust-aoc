fn main() {
    let input = include_str!("./input.txt").to_string();
    let sol1 = part1fix(input);
    println!("Solution 1 : {sol1}");
}

fn part1fix(input:String)-> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut temp = lines.splitn(2,|l| l.len()==0);
    let fresh_ingredients = temp.next().unwrap();
    let ingredients = temp.next().unwrap();
    let mut all_fresh_ingredients : Vec<(i64,i64)>= Vec::new();
    for fresh_ingredient in fresh_ingredients {
        let vec: Vec<i64> = fresh_ingredient.split('-').map(|x| x.parse().unwrap()).collect();
        let fresh_a = vec[0];
        let fresh_b = vec[1];
        all_fresh_ingredients.push((fresh_a,fresh_b));
    }
    ingredients.iter().filter(|&&x|all_fresh_ingredients.iter().any(|(start,end)| start <= &x.parse::<i64>().unwrap() && &x.parse::<i64>().unwrap() <= end )).count()
}

fn part1(input:String)-> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut temp = lines.splitn(2,|l| l.len()==0);
    let fresh_ingredients = temp.next().unwrap();
    let ingredients = temp.next().unwrap();
    let mut all_fresh_ingredients : Vec<i64>= Vec::new();
    for fresh_ingredient in fresh_ingredients {
        let vec: Vec<i64> = fresh_ingredient.split('-').map(|x| x.parse().unwrap()).collect();
        let fresh_a = vec[0];
        let fresh_b = vec[1];
        for i in fresh_a..fresh_b+1{
            all_fresh_ingredients.push(i);
        }
        all_fresh_ingredients.dedup();
    }
    ingredients.iter().filter(|&&x|all_fresh_ingredients.contains(&x.parse::<i64>().unwrap())).count()
}

#[test]
fn test_part_1() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    assert_eq!(3, part1fix(input.to_string()))
}