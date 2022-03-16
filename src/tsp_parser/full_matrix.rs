use std::str::Lines;

use super::Tsp;

pub struct FullMatrixTspParser;

impl FullMatrixTspParser {
    pub fn parse(file_lines: &mut Lines, dimension: usize) -> Option<Tsp> {
        file_lines.next();

        let edges = FullMatrixTspParser::parse_lines_into_edges(file_lines, dimension)?;

        Some(Tsp { edges })
    }

    fn parse_lines_into_edges(file_lines: &mut Lines, dimension: usize) -> Option<Vec<Vec<u32>>> {
        let mut edges = Vec::new();

        for _ in 0..dimension {
            let mut curr_edges = Vec::new();

            let mut edge_counter = 0;

            let mut line = file_lines.next()?;

            let mut split_line = line.split_whitespace();

            while edge_counter < dimension {
                match split_line.next() {
                    Some(edge) => {
                        let mut edge = edge.parse().ok()?;

                        if edge == 9999 {
                            edge = 0;
                        }

                        curr_edges.push(edge);
                        edge_counter += 1;
                    }
                    None => {
                        line = file_lines.next()?;
                        split_line = line.split_whitespace();
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
