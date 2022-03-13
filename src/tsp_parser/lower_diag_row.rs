use std::str::Lines;

use super::Tsp;

pub struct LowerDiagRowTspParser;

impl LowerDiagRowTspParser {
    pub fn parse(file_lines: &mut Lines) -> Option<Tsp> {
        file_lines.next();

        let data_lines = file_lines.filter(|line| !(line == &"EOF"));

        let edges_collector = EdgesCollector::new();

        let edges = edges_collector.collect_edges(data_lines)?;

        Some(Tsp { edges })
    }
}

struct EdgesCollector {
    edges: Vec<Vec<u32>>,
    curr_edge: Vec<u32>,
    edge_index: usize,
}

impl EdgesCollector {
    fn new() -> Self {
        EdgesCollector {
            edges: Vec::new(),
            curr_edge: Vec::new(),
            edge_index: 0,
        }
    }

    // TODO: refactor this into smaller pieces
    fn collect_edges<'a, I>(mut self, data_lines: I) -> Option<Vec<Vec<u32>>>
    where
        I: Iterator<Item = &'a str>,
    {
        for line in data_lines {
            let line_edges = line.split_whitespace();

            for line_edge in line_edges {
                let line_edge = line_edge.parse().ok()?;

                self.curr_edge.push(line_edge);

                if line_edge != 0 {
                    self.edges[self.edge_index].push(line_edge);
                    self.edge_index += 1;
                } else {
                    self.edges.push(self.curr_edge);
                    self.curr_edge = Vec::new();
                    self.edge_index = 0;
                }
            }
        }

        Some(self.edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: change test to take string instead of open file.
    #[test]
    fn lower_diag_row_parser_works() {
        let data = "EDGE_WEIGHT_SECTION
 0 2 0 3
 3 0
EOF";

        let mut data_lines = data.lines();

        let tsp = LowerDiagRowTspParser::parse(&mut data_lines).expect("error while parsing data");
        assert_eq!(vec![vec![0, 2, 3], vec![2, 0, 3], vec![3, 3, 0]], tsp.edges);
    }
}
