pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            // println!("{}", line);
            writeln!(writer, "{}", line);
        }
    }
}

pub fn answer() -> i32 {
    return 42;
}

