pub fn format_single_line(enumerate: (usize, String)) -> String {
    format!("Case #{}: {}", enumerate.0 + 1, enumerate.1)
}

pub fn format_multi_line(enumerate: (usize, String)) -> String {
    format!("Case #{}:\n{}", enumerate.0 + 1, enumerate.1)
}