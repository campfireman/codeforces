use std::io;

fn main() {
    const MIN_LEN: usize = 11;
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let n: u32 = input_line.trim().parse().expect("Not an integer");

    for _i in 0..n {
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let word: String = input_line.trim().parse().expect("Expected string.");
        if word.len() < MIN_LEN {
            println!("{}", word);
            continue;
        }
        let first = match word.chars().nth(0) {
            Some(c) => c,
            None => break,
        };
        let last = match word.chars().nth(word.len() - 1) {
            Some(c) => c,
            None => break,
        };
        let size = word.len() - 2;
        println!("{}{}{}", first, size, last)
    }
}
