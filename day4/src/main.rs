fn main() {
    let input = include_str!("./input.txt");
    let paper_number = part1(input);
    println!("Part 1: {}", paper_number)
}

fn part1(input: &str) -> i32 {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    lines.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().enumerate()
                .filter(|&(j, &c)| c == '@' && get_neighbor_count(&lines, i, j) < 5)
                .count()
        })
        .sum::<usize>() as i32
}

fn get_neighbor_count(lines: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    [-1, 0, 1].iter()
        .flat_map(|x_offset| {
            [-1, 0, 1].iter().filter_map(|y_offset| {
                if let Some(line) = i.checked_add_signed(*y_offset).and_then(|i_index| lines.get(i_index))
                    && let Some('@') = j.checked_add_signed(*x_offset).and_then(|j_index| line.get(j_index))
                {
                    Some(())
                } else {
                    None
                }
            })
        })
        .count()
}

#[test]
fn test_part_1() {
    let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    let paper_number = part1(input);
    assert_eq!(13, paper_number)
}
