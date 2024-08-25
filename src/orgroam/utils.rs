pub fn clean_id(id: String) -> String {
    let quote: char = "\"".chars().next().unwrap();
    id.trim_matches(|c: char| c.eq(&quote)).to_string()
}