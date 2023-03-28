use git2::{Repository, Sort};
use std::env;

fn main() -> Result<(), git2::Error> {
    // リポジトリを開く
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_git_repository>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    let repo = Repository::open(path)?;

    // Revwalk を作成し、ソートオプションを指定する
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL)?;

    // タグ一覧を取得
    let tag_names = repo.tag_names(None)?;

    // タグのコミット ID を取得
    let mut commit_ids: Vec<git2::Oid> = Vec::new();
    for tag in tag_names.iter().flatten() {
        let oid = repo.revparse_single(&format!("refs/tags/{}", tag)).unwrap().id();
        commit_ids.push(oid);
    }

    // タグのコミット ID を Revwalk に追加
    for commit_id in &commit_ids {
        revwalk.push(*commit_id)?;
    }

    // ソートされたコミットをイテレートし、タグ名を取得して表示
    for commit_id in revwalk {
        let commit_id = commit_id?;
        // let commit = repo.find_commit(commit_id)?;

        let tag_name = commit_ids
            .iter()
            .find_map(|&id| {
                if id == commit_id {
                    repo.find_tag(id).ok().map(|tag| tag.name().unwrap_or("").to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "".to_string());

        if tag_name != "" {
            println!("{}", tag_name);
        }
    }

    Ok(())
}
