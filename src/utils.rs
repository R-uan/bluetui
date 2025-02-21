pub fn extract_value(input: &str) -> &str {
    let value: Vec<&str> = input.splitn(2, ": ").collect();
    return value[1];
}
