use std::str::Lines;

mod euc2d;
mod full_matrix;
mod lower_diag_row;

use euc2d::Euc2dTspParser;
use full_matrix::FullMatrixTspParser;
use lower_diag_row::LowerDiagRowTspParser;

pub struct Tsp {
    edges: Vec<Vec<u32>>,
}

#[derive(Debug, PartialEq)]
enum TspFileType {
    LowerDiagRow,
    FullMatrix,
    Euc2d,
}

impl Tsp {
    pub fn from_file(filename: &str) -> Option<Tsp> {
        let file_content = std::fs::read_to_string(filename).ok()?;
        let mut file_lines = file_content.lines();

        match Tsp::check_file_type(&mut file_lines)? {
            TspFileType::LowerDiagRow => LowerDiagRowTspParser::parse(&mut file_lines),
            TspFileType::FullMatrix => FullMatrixTspParser::parse(&mut file_lines),
            TspFileType::Euc2d => Euc2dTspParser::parse(&mut file_lines),
        }
    }

    pub fn get_edges(self) -> Vec<Vec<u32>> {
        self.edges
    }

    fn check_file_type(file_lines: &mut Lines) -> Option<TspFileType> {
        let edge_weight_type = file_lines.nth(4)?;

        if edge_weight_type.contains("EUC_2D") {
            Some(TspFileType::Euc2d)
        } else if edge_weight_type.contains("EXPLICIT") {
            Tsp::check_explicit_file_type(file_lines)
        } else {
            None
        }
    }

    fn check_explicit_file_type(file_lines: &mut Lines) -> Option<TspFileType> {
        let edge_weight_format = file_lines.next()?;

        if edge_weight_format.contains("FULL_MATRIX") {
            Some(TspFileType::FullMatrix)
        } else if edge_weight_format.contains("LOWER_DIAG_ROW") {
            Some(TspFileType::LowerDiagRow)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;

    #[test]
    fn check_test_files_exist() -> std::io::Result<()> {
        File::open("euc_2d")?;
        File::open("lower_diag_row")?;
        File::open("full_matrix")?;

        Ok(())
    }

    fn check_file_type_works(filename: &str, file_type: TspFileType) {
        let file_content = std::fs::read_to_string(filename).expect("file doesn't exist");
        let mut lines = file_content.lines();

        let tsp_type = Tsp::check_file_type(&mut lines).expect("file couldn't be parsed");
        assert_eq!(file_type, tsp_type);
    }

    #[test]
    fn check_euc_2d_file_type_works() {
        check_file_type_works("euc_2d", TspFileType::Euc2d);
    }

    #[test]
    fn check_full_matrix_file_type_works() {
        check_file_type_works("full_matrix", TspFileType::FullMatrix);
    }

    #[test]
    fn check_lower_diag_row_file_type_works() {
        check_file_type_works("lower_diag_row", TspFileType::LowerDiagRow);
    }

    #[test]
    fn lower_diag_row_works() {
        let tsp = Tsp::from_file("lower_diag_row").expect("Couldn't parse file");
        assert_eq!(vec![vec![0, 2, 3], vec![2, 0, 3], vec![3, 3, 0]], tsp.edges);
    }

    #[test]
    fn full_matrix_works() {
        let tsp = Tsp::from_file("full_matrix").expect("Couldn't parse file");
        assert_eq!(vec![vec![0, 2, 3], vec![2, 0, 3], vec![3, 3, 0]], tsp.edges);
    }

    #[test]
    fn euc_2d_works() {
        let tsp = Tsp::from_file("euc_2d").expect("Couldn't parse file");
        assert_eq!(
            vec![vec![0, 10, 7], vec![10, 0, 7], vec![7, 7, 0]],
            tsp.edges
        );
    }

    #[test]
    fn read_coords() {
        let filename = "brd14051.tsp";

        let tsp = Tsp::from_file(filename).expect("Couldn't parse file");
        let edges = tsp.get_edges();

        assert_eq!(14051, edges.len());
        assert_eq!(14051, edges[0].len());
    }
}
