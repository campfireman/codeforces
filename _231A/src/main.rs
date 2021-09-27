use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let n: u32 = input_line.trim().parse().expect("Not an integer");

    let mut solvable = 0;
    let mut count = 0;
    for _i in 0..n {
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let word: String = input_line.trim().parse().expect("Expected string.");
        let answers = word.split(" ");

        for a in answers {
            if a == "1" {
                count += 1;
            }
        }
        if count > 1 {
            solvable += 1;
        }
        count = 0;
    }
    println!("{}", solvable);
}
