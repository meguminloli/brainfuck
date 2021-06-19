pub fn clear_string(s: String) -> String {
    let mut n = String::new();
    for i in s.chars() {
        match i {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => n.push(i),
            _ => ()
        }
    }
    n
}