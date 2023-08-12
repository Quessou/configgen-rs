use std::path::PathBuf;

pub fn read_configuration(config_file_path: &PathBuf) -> String {
    let file_to_read = std::fs::File::open(config_file_path).unwrap();
    let mut reader = std::io::BufReader::new(file_to_read);
    let mut buf = String::new();
    let r = std::io::Read::read_to_string(&mut reader, &mut buf);
    assert!(r.is_ok());
    buf
}
