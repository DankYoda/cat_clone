use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}
fn main() {
    let args = Args::parse();
    let file_path = Path::new(&args.file_path);
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(why) => {
            println!("Error opening file: {}. {}", file_path.display(), why);
            exit(2);
        },
    };

    let reader = BufReader::new(file);
    let mut line_count = 0;
    for line_result in reader.lines() {
        line_count += 1;
        let line = match line_result {
            Ok(line) => line,
            Err(why) => panic!("couldn't read line: {}. {}", line_count, why),
        };
        println!("{}", line);
        if line_count == 10 {
            break
        }
    }
}
