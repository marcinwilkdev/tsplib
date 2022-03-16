use std::str::Lines;

use super::Tsp;

pub struct FullMatrixTspParser;

impl FullMatrixTspParser {
    pub fn parse(file_lines: &mut Lines, dimension: usize) -> Option<Tsp> {
        let edges = FullMatrixTspParser::parse_lines_into_edges(file_lines, dimension)?;

        Some(Tsp { edges })
    }

    // TODO: Refactor this into smaller pieces.
    fn parse_lines_into_edges(file_lines: &mut Lines, dimension: usize) -> Option<Vec<Vec<u32>>> {
        let mut edges = Vec::new();

        let mut curr_line = file_lines.next()?;

        let mut line_weights = curr_line.split_whitespace();

        for _ in 0..dimension {
            let mut curr_edges = Vec::new();

            let mut edge_counter = 0;

            while edge_counter < dimension {
                match line_weights.next() {
                    Some(edge) => {
                        let edge = edge.parse().ok()?;

                        curr_edges.push(edge);
                        edge_counter += 1;
                    }
                    None => {
                        curr_line = file_lines.next()?;
                        line_weights = curr_line.split_whitespace();
                    }
                }
            }

            edges.push(curr_edges);
        }

        Some(edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_matrix_parser_working() {
        let data = "9999    2    3
    2 9999    3
    3    3 9999";

        let mut data_lines = data.lines();

        let tsp = FullMatrixTspParser::parse(&mut data_lines, 3).expect("error while parsing data");
        assert_eq!(vec![vec![9999, 2, 3], vec![2, 9999, 3], vec![3, 3, 9999]], tsp.edges);
    }
}
