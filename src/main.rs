use rand::random;

const USAGE: &str = "usage: animalhash [options]

Options:
    --no-adjective     Don't include adjective
    --no-animal        Don't include animal
    --no-colour        Don't include colour
    --camelcase        Use camelcase instead of kebabcase
    --pascalcase       Use pascalcase instead of kebabcase (capitalise every word)

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
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains("--help") || pargs.contains("-h") {
        println!("{}", USAGE);
        std::process::exit(1);
    }

    let animals = include_str!("../words/animals.txt");
    let adjectives = include_str!("../words/adjectives.txt");
    let colours = include_str!("../words/colours.txt");

    let mut outparts: Vec<String> = Vec::new();
    if !pargs.contains("--no-adjective") {
        outparts.push(rand_line_from_string(adjectives));
    }

    if !pargs.contains("--no-colour") {
        outparts.push(rand_line_from_string(colours));
    }

    if !pargs.contains("--no-animal") {
        outparts.push(rand_line_from_string(animals));
    }
    let mut joiner = "-";

    let pascal = pargs.contains("--pascalcase") || pargs.contains("-p");
    let camel = pargs.contains("--camelcase") || pargs.contains("-c");
    if camel || pascal {
        for elem in outparts.iter_mut().skip(if pascal { 0 } else { 1 }) {
            *elem = title_case(elem);
        }
        joiner = "";
    }
    let outstr = outparts.join(joiner);

    println!("{}", outstr);
}
