use git2::{Repository, Sort};
use std::env;

fn main() -> Result<(), git2::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_git_repository>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL)?;
    let tag_names = repo.tag_names(None)?;
    let mut tags: Vec<git2::Oid> = Vec::new();
    for tag in tag_names.iter().flatten() {
        let oid = repo.revparse_single(&format!("refs/tags/{}", tag)).unwrap().id();
        tags.push(oid);
    }
    tags.sort();
    tags.reverse();
    for oid in tags {
        revwalk.push(oid)?;
    }

    let id_vec: Result<Vec<_>, git2::Error> = revwalk
        .into_iter()
        .map(|id| id.and_then(|oid| repo.find_commit(oid)))
        .collect();
    for id in id_vec {
        let oid = id?;
        let commit = repo.find_commit(oid)?;
        println!("{} {}", oid, commit.summary().unwrap_or_default());
    }
    Ok(())
}

