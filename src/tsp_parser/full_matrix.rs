use std::num::ParseIntError;
use std::str::Lines;

use super::Tsp;

pub struct FullMatrixTspParser;

impl FullMatrixTspParser {
    pub fn parse(file_lines: &mut Lines) -> Option<Tsp> {
        file_lines.next();

        let edges: Result<Vec<Vec<u32>>, ParseIntError> = file_lines
            .filter(|line| !(line == &"EOF"))
            .map(|line| FullMatrixTspParser::parse_full_matrix_line(line))
            .collect();

        let edges = edges.ok()?;

        Some(Tsp { edges })
    }

    fn parse_full_matrix_line(line: &str) -> Result<Vec<u32>, ParseIntError> {
        line.split_whitespace()
            .map(|weight| weight.parse())
            .map(|weight| if let Ok(9999) = weight { Ok(0) } else { weight })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: change test to take string instead of open file.
    #[test]
    fn full_matrix_parser_working() {
        let data = "EDGE_WEIGHT_SECTION
 9999    2    3
    2 9999    3
    3    3 9999
EOF";

        let mut data_lines = data.lines();

        let tsp = FullMatrixTspParser::parse(&mut data_lines).expect("error while parsing data");
        assert_eq!(vec![vec![0, 2, 3], vec![2, 0, 3], vec![3, 3, 0]], tsp.edges);
    }
}
