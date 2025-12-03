use std::fs;
use std::cmp::max;

fn main() {
    let battery_banks = fs::read_to_string("src/batterybanks.txt")
        .expect("Failed to parse input text.")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).expect("failed to parse chars in this line to digits") as u32).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    // println!("{:?}", battery_banks);

    let part_one_answer: u32 = battery_banks.iter().map(|bank| {
        find_max_joltage(bank)
    }).sum();

    println!("Part one answer: {}", part_one_answer);
}

pub fn find_max_joltage_part_one(bank: &Vec<u32>) -> u32 {
    let bank_matrix = BankMatrix::build_from_bank(bank);

    let mut max_joltage = 0;
    for i in 0..bank_matrix.size {
        for j in (i + 1)..bank_matrix.size {
            max_joltage = max_joltage.max(bank_matrix.elements[i][j])
        }
    }

    max_joltage
}

pub fn find_max_joltage_part_two(bank: &Vec<u32>, pair_indices: &Vec<usize>) -> Vec<(usize, usize)>{
    let mut bank_owned = bank.to_vec();
    let mut battery_indices: Vec<(usize, usize)> = Vec::new();
    let num_pairs: usize = 6;

    for pair in (0..num_pairs) {
        let bank_matrix = BankMatrix::build_from_bank(bank);
        let mut max_joltage = 0;
        let mut max_indices = (0, 0);

        for i in 0..bank_matrix.size {
            for j in (i + 1)..bank_matrix.size {
                max_joltage = max_joltage.max(bank_matrix.elements[i][j]);
                max_indices = (i, j);
            }
        }

        bank_owned.remove(max_indices.0);
        bank_owned.remove(max_indices.1);

        battery_indices.push(max_indices);
    }

    battery_indices
}

pub struct BankMatrix {
    elements: Vec<Vec<u32>>,
    size: usize,
}

impl BankMatrix {
    pub fn build_from_bank(bank: &Vec<u32>) -> Self {
        let bank = bank.to_vec();
        let size = bank.len();

        let elements: Vec<Vec<u32>> = (0..size).map(|i| {
            (0..size).map(|j| bank[i] * 10 + bank[j])
                .collect()
        })
        .collect();

        BankMatrix { elements, size }
    }
}


