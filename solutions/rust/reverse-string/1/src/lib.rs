pub fn reverse(input: &str) -> String {
    println!("Input message = {}", input);
    if input.is_empty() {
        return String::new();
    }
    let mut z = String :: new();
    let chars: Vec<char> = input.chars().collect();
    let mut x = (chars.len() as isize) - 1;
    while x >= 0 {
        z.push(chars[x as usize]);
        x -= 1;
    }

    z
}
