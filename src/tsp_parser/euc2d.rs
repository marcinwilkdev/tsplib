use std::str::Lines;

use super::Tsp;

pub struct Euc2dTspParser;

impl Euc2dTspParser {
    pub fn parse(file_lines: &mut Lines) -> Option<Tsp> {
        file_lines.next();

        let coords: Option<Vec<(i32, i32)>> = file_lines
            .filter(|line| !(line == &"EOF"))
            .map(|line| Euc2dTspParser::parse_line_into_coords(&line))
            .collect();

        let coords = coords?;

        let edges = Euc2dTspParser::parse_distances(&coords);

        Some(Tsp { edges })
    }

    fn parse_line_into_coords(line: &str) -> Option<(i32, i32)> {
        let mut line = line.split_whitespace();

        let (x, y) = (line.nth(1)?, line.next()?);
        let (x, y): (f64, f64) = (x.parse().ok()?, y.parse().ok()?);

        Some((x as i32, y as i32))
    }

    fn parse_distances(coords: &[(i32, i32)]) -> Vec<Vec<u32>> {
        coords
            .iter()
            .map(|p1| Euc2dTspParser::calculate_distances_to_other_points(*p1, coords))
            .collect()
    }

    fn calculate_distances_to_other_points(p1: (i32, i32), coords: &[(i32, i32)]) -> Vec<u32> {
        coords
            .iter()
            .map(|p2| Euc2dTspParser::calculate_distance(p1, *p2))
            .collect()
    }

    fn calculate_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32 {
        (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64)
            .sqrt()
            .round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: change test to take string instead of open file.
    #[test]
    fn euc2d_parser_working() {
        let data = "NODE_COORD_SECTION
1 0.0 10.0
2 0.0 0.0
3 5.0 5.0
EOF";

        let mut data_lines = data.lines();

        let tsp = Euc2dTspParser::parse(&mut data_lines).expect("error while parsing data");

        assert_eq!(
            vec![vec![0, 10, 7], vec![10, 0, 7], vec![7, 7, 0]],
            tsp.edges
        );
    }
}
