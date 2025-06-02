pub fn temp_file() -> String {
    let temp_dir = std::env::temp_dir();
    let temp_file = format!("afg-{}.swift",std::process::id());
    let temp_file_full = temp_dir.join(temp_file);
    temp_file_full.to_str().unwrap().to_string()
}
