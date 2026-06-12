fn main() {
    let input = include_str!("./input.txt").lines().map(|l| l.chars().collect()).collect();
    let paper_number = part1(input);
    println!("Part 1: {paper_number}");
}

fn part1(input: Vec<Vec<char>>) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, &c)| c == '@' && get_neighbor_count(&input, i, j) < 4)
                .count()
        })
        .sum::<usize>()
}

fn get_neighbor_count(lines: &[Vec<char>], i: usize, j: usize) -> usize {
    [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ]
    .iter()
    .filter_map(|(di, dj)| {
        let ni = i.checked_add_signed(*di)?;
        let nj = j.checked_add_signed(*dj)?;
        Some(lines.get(ni)?.get(nj)?)
    })
    .filter(|&&c| c == '@')
    .count()
}

#[test]
fn test_part_1() {
    let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    let paper_number = part1(input.lines().map(|l| l.chars().collect()).collect());
    assert_eq!(13, paper_number)
}
