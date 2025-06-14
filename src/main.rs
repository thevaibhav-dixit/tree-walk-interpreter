fn main() {
    while let Some(input) = std::io::stdin().lines().next() {
        match input {
            Ok(line) => {
                println!("{}", line);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
