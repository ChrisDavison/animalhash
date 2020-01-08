use rand::random;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "animalhash", about = "Generate docker-esque keyword")]
struct Opts {
    /// Use semi-titlecase (i.e. capitalise on word boundaries)
    #[structopt(short, long)]
    titlecase: bool,

    /// Don't include a colour in the keyword
    #[structopt(long)]
    no_colour: bool,

    /// Don't include an adjective in the keyword
    #[structopt(long)]
    no_adjective: bool,

    /// Don't include an animal in the keyword
    #[structopt(long)]
    no_animal: bool,
}

fn title_case(word: &str) -> String {
    let lower = word.to_lowercase();
    let first = lower.chars().take(1).collect::<String>().to_uppercase();
    first + &lower[1..]
}

fn rand_line_from_string(string: &str) -> String {
    let lines: Vec<&str> = string.split('\n').collect();
    lines[random::<usize>() % lines.len()].trim().into()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
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

    if opts.titlecase {
        for elem in outparts.iter_mut().skip(1) {
            *elem = title_case(elem);
        }
    }
    let outstr = outparts.join("");

    println!("{}", outstr);
    Ok(())
}
