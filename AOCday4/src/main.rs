use std::fs;

const NEIGHBOUR_COORDS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1), 
    (0, -1),           (0, 1), 
    (1, -1),  (1, 0),  (1, 1)
];

fn main() {
    let contents = fs::read_to_string("src/paperrolls.txt").expect("failed to read data from input text.");
    let mut roll_matrix = parse_matrix(&contents);

    let part_one_answer = solve_part_one(&roll_matrix);
    let part_two_answer = solve_part_two(&mut roll_matrix);
    println!("{}", part_one_answer);
    println!("{}", part_two_answer);
}

pub fn solve_part_two(matrix: &mut Vec<Vec<u32>>) -> u32 {
    let mut total_rolls_removed = 0;
    
    loop {
        let mut accessible_roll_indices: Vec<(usize, usize)> = Vec::new();

        for (i, row) in matrix.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if matrix[i][j] == 0 {
                    continue;
                }

                if neighbour_sum(&matrix, (i, j)) == 1 {
                    accessible_roll_indices.push((i, j));
                }
            }
        }

        if accessible_roll_indices.len() == 0 {
            break;
        }

        for (i, j) in accessible_roll_indices.iter() {
            matrix[*i][*j] = 0;
        }

        total_rolls_removed += accessible_roll_indices.len() as u32;
    }

    total_rolls_removed
}

pub fn solve_part_one(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut num_rolls = 0u32;
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if matrix[i][j] == 0 {
                continue;
            }

            num_rolls += neighbour_sum(&matrix, (i, j))
        }
    }

    num_rolls
}

pub fn neighbour_sum(matrix: &Vec<Vec<u32>>, indices: (usize, usize)) -> u32 {
    let mut sum = 0;
    for (rel_i, rel_j) in NEIGHBOUR_COORDS {
        let neigh_i = indices.0 as i32 + rel_i;
        let neigh_j = indices.1 as i32 + rel_j;

        sum += matrix[neigh_i as usize][neigh_j as usize];
    }

    if sum < 4 {
        return 1
    } else {
        return 0
    }
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
         
