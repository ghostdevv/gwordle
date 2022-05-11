use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(short, long)]
    matcher: String,

    #[clap(short, long, default_value = "")]
    has: String,

    #[clap(short, long, default_value = "")]
    exclude: String
}


fn main() {
    let args = Args::parse();

    let word_list = fs::read_to_string("words.txt").expect("Failed to read words");
    let words: Vec<&str> = word_list.split("\n").collect();

    let regex = regex::Regex::new(&args.matcher).unwrap();

    println!("Matches:");

    for word in words {
        if regex.is_match(word) {
            if !args.has.is_empty() {
                let has_regex = regex::Regex::new(&format!("[{}]", args.has)).unwrap();

                if !has_regex.is_match(word) {
                    continue;
                }
            }
            
            if !args.exclude.is_empty() {
                let exclude_regex = regex::Regex::new(&format!("[{}]", args.exclude)).unwrap();

                if exclude_regex.is_match(word) {
                    continue;
                }
            }
            
            println!("{}", word);
        }
    }
}
