pub fn is_protected(query: &str) -> bool {
    query == "|" || query == ">" || query == "&" || io_seperator(query)
}

pub fn io_seperator(query: &str) -> bool {
    query == ";" || query == "&&"
}
