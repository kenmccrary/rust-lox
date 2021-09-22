pub fn error(line: usize, message: &str) {
    report(line, "".to_string(), message);
}

pub fn report(line: usize, which: String, message: &str) {
    println!("[line: {}] Error: {} : {}", line, which, message);
}