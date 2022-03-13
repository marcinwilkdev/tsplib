use std::str::Lines;

struct Tsp {
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
        // change this
        let file_content = match std::fs::read_to_string(filename) {
            Ok(file_content) => file_content,
            Err(_) => return None,
        };

        let mut file_lines = file_content.lines();

        match Tsp::check_file_type(&mut file_lines)? {
            TspFileType::LowerDiagRow => Some(Tsp::from_lower_diag_row(&mut file_lines)),
            TspFileType::FullMatrix => Some(Tsp::from_lower_diag_row(&mut file_lines)),
            TspFileType::Euc2d => Some(Tsp::from_lower_diag_row(&mut file_lines)),
        }
    }

    fn check_file_type(file_lines: &mut Lines) -> Option<TspFileType> {
        for _ in 0..4 {
            file_lines.next();
        }

        let edge_weight_type = file_lines.next()?;
        let edge_weight_type = edge_weight_type.split_whitespace().nth(1)?;

        match edge_weight_type {
            "EUC_2D" => Some(TspFileType::Euc2d),
            "EXPLICIT" => Tsp::check_explicit_file_type(file_lines),
            _ => None,
        }
    }

    fn check_explicit_file_type(file_lines: &mut Lines) -> Option<TspFileType> {
        let edge_weight_format = file_lines.next()?;
        let edge_weight_format = edge_weight_format.split_whitespace().nth(1)?;

        match edge_weight_format {
            "FULL_MATRIX" => Some(TspFileType::FullMatrix),
            "LOWER_DIAG_ROW" => Some(TspFileType::LowerDiagRow),
            _ => None,
        }
    }

    fn from_lower_diag_row(file_lines: &mut Lines) -> Tsp {
        Tsp { edges: vec![] }
    }

    fn from_full_matrix(data: &str) -> Tsp {
        Tsp { edges: vec![] }
    }

    fn from_euc_2d(data: &str) -> Tsp {
        Tsp { edges: vec![] }
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
}
