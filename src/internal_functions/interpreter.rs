pub fn parseline(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in input.match_indices(|c: char| c.is_whitespace() || c == '|' || c == '>')
    {
        if last != index {
            result.push(&input[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < input.len() {
        result.push(&input[last..]);
    }
    result.retain(|s| !s.trim().is_empty());
    result
}
