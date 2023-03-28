use git2::{Repository, Sort};
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

    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(e) => {
            eprintln!("Error creating revwalk: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = revwalk.set_sorting(Sort::TOPOLOGICAL) {
        eprintln!("Error setting sorting: {}", e);
        std::process::exit(1);
    }

    for commit_id_result in revwalk {
        match commit_id_result {
            Ok(commit_id) => {
                let commit = repo.find_commit(commit_id).unwrap();
                println!("Commit: {} {}", commit.id(), commit.summary().unwrap_or("<no summary>"));
            }
            Err(e) => {
                eprintln!("Error processing commit: {}", e);
                std::process::exit(1);
            }
        }
    }
}

