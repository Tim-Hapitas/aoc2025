use std::str::FromStr;

pub struct IDRange{
    start: usize,
    end: usize,
}

impl IDRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self {start, end}
    }

    //merges overlapping and adjacent IDranges into a minimal set of non-overlapping IDRanges.
    pub fn consolidate(mut id_ranges: Vec<IDRange>) -> Vec<IDRange> {
        id_ranges.sort_by_key(|range| range.start());

        let mut consolidated_ranges = Vec::new();
        let (mut local_range_start, mut local_range_end) = (id_ranges[0].start(), id_ranges[0].end());

        for range in id_ranges.iter().skip(1) {
            if range.start() <= local_range_end {
                local_range_end = local_range_end.max(range.end());
            } else {
                consolidated_ranges.push(IDRange::new(local_range_start, local_range_end));
                local_range_start = range.start();
                local_range_end = range.end();
            }
        }

        consolidated_ranges.push(IDRange::new(local_range_start, local_range_end));
        consolidated_ranges
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn is_in_range(&self, val: usize) -> bool {
        val >= self.start && val <= self.end
    }

    pub fn size(&self) -> usize {
        (self.end - self.start) + 1
    }
}

#[derive(Debug)]
pub enum IDParseError{
    InvalidIDFormat,
    EmptyString,
    InvalidNumber,
}

impl FromStr for IDRange {
    type Err = IDParseError;

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

        Ok(IDRange { start: *first, end: *second })
    }
}
