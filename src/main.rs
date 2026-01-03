use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &Path;
    if args.len() < 2 {
        path = Path::new("potato.txt");
    }
    else {
        path = Path::new(args[1].as_str());
    }
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => print!("{} contains:\n{}", path.display(), s),
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
    }
}
