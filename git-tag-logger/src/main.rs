use git2::{Repository};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_git_repository>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Error opening repository at '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    let tags = match repo.tag_names(None) {
        Ok(tags) => tags,
        Err(e) => {
            eprintln!("Error getting tags: {}", e);
            std::process::exit(1);
        }
    };

    for tag_name in tags.iter() {
        println!("Tag name: {:?}", tag_name);
    }
}
