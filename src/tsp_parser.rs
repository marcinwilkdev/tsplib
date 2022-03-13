use std::io::Result;
use std::str::Lines;

struct Tsp {
    edges: Vec<Vec<u32>>,
}

enum TspFileType {
    LowerDiagRow,
    FullMatrix,
    Euc2d,
}

impl Tsp {
    pub fn from_file(filename: &str) -> Result<Tsp> {
        let file_content = std::fs::read_to_string(filename)?;
        let mut file_lines = file_content.lines();

        match Tsp::check_file_type(&mut file_lines)? {
            TspFileType::LowerDiagRow => Ok(Tsp::from_lower_diag_row(&mut file_lines)),
            TspFileType::FullMatrix => Ok(Tsp::from_lower_diag_row(&mut file_lines)),
            TspFileType::Euc2d => Ok(Tsp::from_lower_diag_row(&mut file_lines)),
        }
    }

    fn check_file_type(file_lines: &mut Lines) -> Result<TspFileType> {
        Ok(TspFileType::LowerDiagRow)
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
        assert_eq!(vec![vec![0, 10, 7], vec![10, 0, 7], vec![7, 7, 0]], tsp.edges);
    }
}
