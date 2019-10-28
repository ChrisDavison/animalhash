use rand::random;
use std::env::args;
use std::process::exit;

const USAGE: &'static str = r##"animalhash

Generate a docker-esque adjective-colour-animal keyword.

Usage:
    animalhash [OPTIONS]

Options:
    -l --lowercase   Don't semi-titlecase the output
    --no-colour      Don't use a colour
    --no-adjective   Don't use an adjective
    --no-animal      Don't use an animal
"##;

struct Opts {
    use_titlecase: bool,
    no_colour: bool,
    no_adjective: bool,
    no_animal: bool,
}

impl Opts {
    fn from_args() -> Result<Opts, Box<dyn std::error::Error>> {
        let use_titlecase = args()
            .filter(|x| x == "-l" || x == "--lowercase")
            .collect::<String>().is_empty();
        let no_colour = !args()
            .filter(|x| x == "--no-colour")
            .collect::<String>().is_empty();
        let no_adjective = !args()
            .filter(|x| x == "--no-adjective")
            .collect::<String>().is_empty();
        let no_animal = !args()
            .filter(|x| x == "--no-animal")
            .collect::<String>().is_empty();
        let help = !args()
            .filter(|x| x == "-h" || x == "--help")
            .collect::<String>().is_empty();
        if help {
            println!("{}", USAGE);
            exit(1);
        }
        if no_colour && no_adjective && no_animal {
            return Err("Must have at least one of: colour, adjective, animal".into());
        }
        Ok(Opts{use_titlecase, no_colour, no_adjective, no_animal})
    }
}

fn title_case(word: &str) -> String {
    let lower = word.to_lowercase();
    let first = lower.chars().take(1).collect::<String>().to_uppercase();
    first + &lower[1..]
}

fn rand_line_from_string(string: &str) -> String {
    let lines: Vec<&str> = string.split('\n').collect();
    lines[random::<usize>() % lines.len()].into()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args()?;
    let animals = include_str!("../words/animals.txt");
    let adjectives = include_str!("../words/adjectives.txt");
    let colours = include_str!("../words/colours.txt");

    let mut outparts = Vec::new();
    if !opts.no_adjective {
        outparts.push(rand_line_from_string(adjectives));
    }

    if !opts.no_colour {
        outparts.push(rand_line_from_string(colours));
    }

    if !opts.no_animal {
        outparts.push(rand_line_from_string(animals));
    }

    if opts.use_titlecase {
        for (i, elem) in outparts.iter_mut().enumerate() {
            if i > 0 {
                *elem = title_case(elem);
            }
        }
    }

    let outstr = outparts.join("");

    println!("{}", outstr);
    Ok(())
}
