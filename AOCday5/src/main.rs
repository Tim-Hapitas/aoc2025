use std::fs;
use aoc_common::idrange::IDRange;

fn main() {
    let contents = fs::read_to_string("src/ingredients.txt").expect("failed to read input from ingredients.txt");
    let (id_ranges, ingredient_ids) = parse_input(&contents);

    println!("Part one answer: {}", count_fresh_available(&id_ranges, &ingredient_ids));
    println!("Part two answer: {}", count_fresh_ingredients(id_ranges));
}

pub fn parse_input(contents: &str) -> (Vec<IDRange>, Vec<usize>) {
    let result: Vec<&str> = contents.split("\r\n\r\n").collect();
    assert_eq!(result.len(), 2, "whitespace split did not work as expected!");

    let id_ranges: Vec<IDRange> = result[0].lines().map(|line| line.parse::<IDRange>().expect("Failed on parsing id range from current line")).collect();
    let ingredient_ids: Vec<usize> = result[1].lines().map(|line| line.parse::<usize>().expect("Failed on parsing ingredient ids to usize")).collect();

    (id_ranges, ingredient_ids)
}

pub fn count_fresh_available(id_ranges: &[IDRange], ingredient_ids: &[usize]) -> usize {
    let mut num_fresh: usize = 0;

    for &ingredient in ingredient_ids {
        for range in id_ranges {
            if range.is_in_range(ingredient) {
                num_fresh += 1;
                break;
            }
        }
    }

    num_fresh
}

pub fn count_fresh_ingredients(id_ranges: Vec<IDRange>) -> usize {
    IDRange::consolidate(id_ranges).iter().map(|range| range.size()).sum()
}
