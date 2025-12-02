use std::{fs, str::FromStr};

fn main() {
    // parsing logic to grab ID ranges from the input text file
    let filepath = "src/idranges.txt";
    let tokens = read_input(filepath).expect("Failed to parse input text");
    let id_ranges = tokens
        .iter().map(|t| t.parse::<IDRange>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to build id ranges from tokens");


    // Part 1
    let part_one_answer = solve(&id_ranges, is_invalid_part_one);
    println!("Part one answer: {}", part_one_answer);
    
    // Part 2
    let part_two_answer = solve(&id_ranges, is_invalid_part_two);
    println!("Part two answer: {}", part_two_answer);
}
// Since the detials of part one and part two's solutions don't change all that much, I took a generic approach.
// The trick is after we build all ID values from the provided ID ranges, we only need to retain those IDs that are invalid
// given the problem's conditions. This means for both parts, we can just flag IDs as invalid, filter all the IDs by invalid, and then
// sum the IDs that are left over. the function solve builds all IDs from a slice of IDranges and is passed a generic function that
// does the filtering.
pub fn solve<F>(id_ranges: &[IDRange], is_invalid: F) -> usize 
where
    F: Fn(usize) -> bool,
{
    id_ranges.iter().flat_map(|range: &IDRange| range.first..=range.second).filter(|id| is_invalid(*id)).sum()
}

pub fn is_invalid_part_one(id: usize) -> bool {
    // an ID is invalid if a sequence of numbers within is repeated EXACTLY twice.
    // This means IDs with an even number of digits are suspect, and we only need to check if the first half of the ID equals
    // the second half.
    let s = id.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    
    let mid = s.len() / 2;
    &s[..mid] == &s[mid..]
}

pub fn is_invalid_part_two(id: usize) -> bool {
    let s = id.to_string();
    let id_len = s.len();

    // A bit more complicated. Now an ID is invalid if a sequence repeates an integer number of times.
    // We now need to fragment our ID into segments running from length 1 to length id_len / 2 (since the latter case represents 
    // a repetition occuring at least twice). The key here is we should only be checking fragmentation lengths that are integer multiples
    // of the id's length.
    for fragmentation_len in 1..=(id_len/2) {
        // skip over non integer multiple fragmentation lengths
        if id_len % fragmentation_len == 0 {
            let sub_range = &s[..fragmentation_len];
            if sub_range.repeat(id_len / fragmentation_len) == s {
                return true;
            }
        }
    }

    false
}

// returns the tokens between each comma in the input text
pub fn read_input(filename: &str) ->  std::io::Result<Vec<String>>{
    let contents = fs::read_to_string(filename)?;
    Ok(contents.split(",").map(|token| token.to_string()).collect())
}

// For represnting our ID ranges, probably overkill but conceptually this makes sense in my brain
pub struct IDRange{
    first: usize,
    second: usize,
}

#[derive(Debug)]
pub enum IDParseError{
    InvalidIDFormat,
    EmptyString,
    InvalidNumber,
}

impl FromStr for IDRange {
    type Err = IDParseError;

    // same approach as day one for parsing since im using a struct to represent an ID range
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(IDParseError::EmptyString)
        }

        let id_range = s.split("-")
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>();

        let id_range = id_range.map_err(|_| IDParseError::InvalidNumber)?;
        
        let [first, second] = id_range.as_slice() else {
            return Err(IDParseError::InvalidIDFormat)
        };
        
        Ok(IDRange { first: *first, second: *second })
    }
}
