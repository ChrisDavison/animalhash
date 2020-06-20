use rand::random;
use std::env;

const USAGE: &str = "usage: animalhash [options]

Options:
    --no-adjective     Don't include adjective
    --no-animal        Don't include animal
    --no-colour        Don't include colour
    -t --titlecase     Titlecase after the first word

    -h --help          Show this message";

fn title_case(word: &str) -> String {
    let lower = word.to_lowercase();
    let first = lower.chars().take(1).collect::<String>().to_uppercase();
    first + &lower[1..]
}

fn rand_line_from_string(string: &str) -> String {
    let lines: Vec<&str> = string.split('\n').collect();
    lines[random::<usize>() % lines.len()].trim().into()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
        let help = args.contains(&"--help".to_string()) || args.contains(&"-h".to_string());
    if help {
        println!("{}", USAGE);
        std::process::exit(1);
    }

    let animals = include_str!("../words/animals.txt");
    let adjectives = include_str!("../words/adjectives.txt");
    let colours = include_str!("../words/colours.txt");

    let mut outparts: Vec<String> = Vec::new();
    if !args.contains(&"--no-adjective".to_string()) {
        outparts.push(rand_line_from_string(adjectives));
    }

    if !args.contains(&"--no-colour".to_string()) {
        outparts.push(rand_line_from_string(colours));
    }


    if !args.contains(&"--no-animal".to_string()) {
        outparts.push(rand_line_from_string(animals));
    }

    if args.contains(&"--titlecase".to_string()) || args.contains(&"-t".to_string()) {
        for elem in outparts.iter_mut().skip(1) {
            *elem = title_case(elem);
        }
    }
    let outstr = outparts.join("");

    println!("{}", outstr);
}
