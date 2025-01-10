use std::fs::File;
use std::io::{self, Read};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_day10() {
        assert_eq!(1, 1);
    }

    fn create_file_input() -> File {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "12345").unwrap();
        temp_file.into_temp_path()
    }
}
