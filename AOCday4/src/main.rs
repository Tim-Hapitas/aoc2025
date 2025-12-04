use std::fs;

const NEIGHBOUR_COORDS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1), 
    (0, -1),           (0, 1), 
    (1, -1),  (1, 0),  (1, 1)
];

fn main() {
    let contents = fs::read_to_string("src/paperrolls.txt").expect("failed to read data from input text.");
    let roll_matrix = parse_matrix(&contents);

    println!("{}", solve_part_one(&roll_matrix));
    println!("{}", solve_part_two(roll_matrix));
}

pub fn solve_part_two(mut matrix: Vec<Vec<u32>>) -> u32 {
    std::iter::from_fn(|| {
        let accessible_roll_indices = find_accessable_rolls(&matrix);

        if accessible_roll_indices.is_empty() {
            None
        } else {
            for &idx in accessible_roll_indices.iter() {
                matrix[idx.0][idx.1] = 0;
            }
            Some(accessible_roll_indices.len() as u32)
        }
    }).sum()
}

pub fn solve_part_one(matrix: &Vec<Vec<u32>>) -> usize {
    find_accessable_rolls(matrix).len()
}

fn find_accessable_rolls(matrix: &[Vec<u32>]) -> Vec<(usize, usize)> {
    (0..matrix.len()).flat_map(|i| (0..matrix[0].len()).map(move |j|(i, j)))
    .filter(|&(i, j)| matrix[i][j] == 1 && count_neighbors(matrix, i, j) < 4)
    .collect()
}

pub fn count_neighbors(matrix: &[Vec<u32>], row: usize, col: usize) -> u32 {
    NEIGHBOUR_COORDS.iter().map(|&(rel_i, rel_j)| {
        let neigh_i = (row as i32 + rel_i) as usize;
        let neigh_j: usize = (col as i32 + rel_j) as usize;
        matrix[neigh_i][neigh_j]
    }).sum()
}

pub fn parse_matrix(roll_data: &str) -> Vec<Vec<u32>> {
    let raw_matrix: Vec<Vec<u32>> = roll_data.lines().map(|line| {
        line.chars().map(|c| match c {
            '@' => 1 as u32,
            '.' => 0 as u32,
            _ => panic!("found unexpected character in original roll data!"),
        }).collect()
    }).collect();

    pad_matrix(raw_matrix)
}

// our matrix index notation is i for rows, j for columns (follows linear algebra conventions)
pub fn pad_matrix(matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let n_row = matrix.len();
    let n_col = matrix[0].len();

    let mut padded_matrix = Vec::with_capacity(n_row + 2);
    
    padded_matrix.push(vec![0; n_col + 2]);
    for row in matrix {
        let mut extended_row = Vec::with_capacity(n_col + 2);
        extended_row.push(0);
        extended_row.extend(row);
        extended_row.push(0);
        padded_matrix.push(extended_row);
    }
    padded_matrix.push(vec![0; n_col + 2]);

    padded_matrix
}
         
