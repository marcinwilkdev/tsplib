#[cfg(test)]
mod tests {
    use crate::*;
    use std::os::unix::ffi::OsStrExt;

    #[test]
    fn test_all_files() {
        let filenames = std::fs::read_dir("test_files").unwrap();

        for filename in filenames {
            let filename = filename.unwrap().file_name();
            let str_filename = String::from_utf8_lossy(filename.as_bytes());

            let tsp = Tsp::from_file(&format!("test_files/{}", str_filename));

            assert!(tsp.is_some(), "{}", str_filename);
        }
    }
}
