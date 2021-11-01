use std::convert::TryInto;
use std::io;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Could not read input");
    let mut setup = line.split(" ");
    let n: u32 = setup
        .nth(0)
        .unwrap()
        .trim()
        .parse()
        .expect("Could not parse for number of participants");
    let k: u32 = setup
        .nth(0)
        .unwrap()
        .trim()
        .parse()
        .expect("Could not parse for kth participant")
        - 1;
    io::stdin()
        .read_line(&mut line)
        .expect("Could not read input");
    let mut scores = line.split(" ");
    let mut advances = 0;
    let mut kth_value = 0;
    for (i, score_str) in scores.enumerate() {
        let score: u32 = score_str
            .trim()
            .parse()
            .expect("Could not parse score of participant");
        println!("{}", score);
        if i > k.try_into().unwrap() {
            if score > 0 {
                advances += 1;
            }
        } else {
            if k == i.try_into().unwrap() {
                kth_value = score;
            }
            if score >= kth_value {
                advances += 1
            } else {
                break;
            }
        }
    }
    println!("{}", advances);
}
