use std::{fs, str::FromStr};

fn main() {
    let mut lock = Lock::default();

    let filename = "src/rotations.txt";
    let lines: Vec<String> = read_input(filename).expect("Failed to read input file");

    let rotation_sequence = build_rotation_sequence(lines).expect("Failed to parse rotations sequence");

    let lock_positions: Vec<usize> = rotation_sequence.iter().map(|r| {
        lock.update_lock_state(r);
        lock.read_lock_value()
    }).collect();

    let part_one_solution = lock_positions.iter().filter(|value| **value == 0).count();
    // I ended up just throwing on extra functionality into my lock struct to get the answer for part 2
    // hence the weird code structure
    let part_two_solution = lock.num_zero_ticks;

    println!("Part one solution {}", part_one_solution);
    println!("Part two solution {}", part_two_solution);
}

// Struct for representing physical rotations on the lock
#[derive(Debug, PartialEq)]
pub struct Rotation{
    direction: String,
    distance: usize,
}

#[derive(Debug)]
pub enum RotationParseError{
    EmptyString,
    InvalidDistance,
}

impl FromStr for Rotation {
    type Err = RotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().next().ok_or(RotationParseError::EmptyString)?.to_string();
        let distance = s[1..].parse::<usize>().map_err(|_| RotationParseError::InvalidDistance)?;

        Ok(Rotation{direction, distance})
    }
}

// Struct to represent our lock!
#[derive(Debug)]
pub struct Lock{
    dial_vals: Vec<usize>,
    current_pos: usize, // this is relative to the 0 value measured in a clockwise sense
    num_zero_ticks: usize, // state for zero crossing tracking
}

impl Lock {
    // we update the internal state of the lock as we follow the rotation sequence
    pub fn update_lock_state(&mut self, rotation: &Rotation) {
        let distance = if rotation.direction == "L" {
            self.dial_vals.len() - (rotation.distance % self.dial_vals.len())
        } else {
            rotation.distance
        };

        let new_pos = (self.current_pos + distance) % self.dial_vals.len();

        let distance_to_zero = if self.current_pos == 0 {
            self.dial_vals.len()
        } else {
            if rotation.direction == "L" {
                self.current_pos
            } else {
                self.dial_vals.len() - self.current_pos
            }
        };

        let zero_tick_count = if rotation.distance < distance_to_zero {
            0
        } else {
            1 + (rotation.distance - distance_to_zero) / self.dial_vals.len()
        };

        self.current_pos = new_pos;
        self.num_zero_ticks += zero_tick_count;
    }

    // retrieve the value that the dial is currently pointing at
    pub fn read_lock_value(&self) -> usize {
        self.dial_vals[self.current_pos]
    }
}

impl Default for Lock {
    fn default() -> Self {
        Lock { dial_vals: (0..100).collect(), current_pos: 50, num_zero_ticks: 0}
    }
}


// Helper functions
pub fn read_input(filename: &str) -> std::io::Result<Vec<String>> {
    let contents = fs::read_to_string(filename)?;

    Ok(contents.lines().map(|s| s.to_string()).collect())
}

pub fn build_rotation_sequence(sequence: Vec<String>) -> Result<Vec<Rotation>, RotationParseError> {
    // create a vector of rotations with instances of Rotation built from FromStr implementation (should be a one liner with iterators)
    let rotation_sequence: Vec<Rotation> = sequence
        .iter()
        .map(|r| r.parse::<Rotation>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rotation_sequence)
}



