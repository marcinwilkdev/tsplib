use std::str::Lines;

use super::Tsp;

pub struct FullMatrixTspParser;

impl FullMatrixTspParser {
    pub fn parse(file_lines: &mut Lines, dimension: u32) -> Option<Tsp> {
        file_lines.next();

        let edges: Vec<Vec<u32>> = file_lines
            .filter(|line| line.trim() != "")
            .filter_map(|line| FullMatrixTspParser::parse_full_matrix_line(line))
            .collect();

        Some(Tsp { edges })
    }

    fn parse_full_matrix_line(line: &str) -> Option<Vec<u32>> {
        line.split_whitespace()
            .map(|weight| weight.parse())
            .map(|weight| if let Ok(9999) = weight { Ok(0) } else { weight })
            .collect::<Result<Vec<u32>, _>>()
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_matrix_parser_working() {
        let data = "EDGE_WEIGHT_SECTION
 9999    2    3
    2 9999    3
    3    3 9999
EOF";

        let mut data_lines = data.lines();

        let tsp = FullMatrixTspParser::parse(&mut data_lines, 3).expect("error while parsing data");
        assert_eq!(vec![vec![0, 2, 3], vec![2, 0, 3], vec![3, 3, 0]], tsp.edges);
    }
}
