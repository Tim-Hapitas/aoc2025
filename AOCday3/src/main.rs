use std::fs;

fn main() {
    let battery_banks = fs::read_to_string("src/batterybanks.txt")
        .expect("Failed to parse input text.")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).expect("failed to parse chars in this line to digits") as u32).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    // println!("{:?}", battery_banks);

    let part_one_answer: u64 = battery_banks.iter().map(|bank| {
        find_max_joltage(bank, 2)
    }).sum();

    let part_two_answer: u64 = battery_banks.iter().map(|bank| {
        find_max_joltage(bank, 12)
    }).sum();

    println!("Part one answer: {}", part_one_answer);
    println!("Part two answer: {}", part_two_answer);
}

pub fn find_max_joltage(bank: &Vec<u32>, num_batteries: usize) -> u64 {
    let bank_length = bank.len();
    let mut max_joltage = 0u64;
    let mut start: usize = 0;

    for i in 0..num_batteries {
        let mut max_dig = 0;
        let mut best_pos = start;

        let batteries_left = num_batteries - (i + 1);
        let end = bank_length - batteries_left;

        for pos in start..end {
            if bank[pos] > max_dig {
                max_dig = bank[pos];
                best_pos = pos;
            }
        }

        max_joltage = max_joltage * 10 + max_dig as u64;
        start = best_pos + 1;
    }

    max_joltage
}


