use std::collections::HashSet;
use regex::Regex;

fn main() {
    let args = std::env::args();
    let binding = args.collect::<Vec<String>>();
    let path = binding.get(1).expect("Should input file path to count!");
    let text = std::fs::read_to_string(path).expect("No such file!");
    let set: HashSet<&str> = HashSet::from_iter(Regex::new(r"\W+")
        .unwrap()
        .split(&text)
        .filter(|s| s.len() > 0)
    );
    let count = set.len();
    if count == 0 {
        println!("File {} is empty.", path);
    } else {
        println!("File {} has {} different {}.", path, count, match count {
            1 => "word",
            _ => "words"
        });
    }
}
