use git2::{Repository, Sort};
use std::env;
use std::collections::HashMap;

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

    let mut commit_to_tag = HashMap::new();
    for tag_name in tags.iter() {
        if let Some(tag_name) = tag_name {
            let tag = repo.revparse_single(tag_name).unwrap();
            commit_to_tag.insert(tag.id(), tag_name);
            println!("Tag name: {}", tag_name);
        }
    }

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

    // Add HEAD reference to revwalk
    if let Err(e) = revwalk.push_head() {
        eprintln!("Error pushing HEAD to revwalk: {}", e);
        std::process::exit(1);
    }

    for commit_id_result in revwalk {
        match commit_id_result {
            Ok(commit_id) => {
                if let Some(tag_name) = commit_to_tag.get(&commit_id) {
                    println!("Tag: {} Commit: {}", tag_name, commit_id);
                }
            }
            Err(e) => {
                eprintln!("Error processing commit: {}", e);
                std::process::exit(1);
            }
        }
    }
}
