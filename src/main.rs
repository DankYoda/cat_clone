use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &Path;
    if args.len() < 2 {
        path = Path::new("potato.txt");
    }
    else {
        path = Path::new(args[1].as_str());
    }
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            println!("Error opening file: {}. {}", path.display(), why);
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
