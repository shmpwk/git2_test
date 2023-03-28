# git2_test


## Usage 
 Specify the path of the specified repository as an argument

- outputs all the tag.
```
cargo run --bin git-tag-logger <path_to_git_repository>
```

- outputs the commit IDs and summaries in topological order.
```
cargo run --bin git-commit-topo-sort <path_to_git_repository>
```

- outputs the tag IDs and summaries in topological order.
```
cargo run --bin git-tag-topo-sort <path_to_git_repository>
```


## How to create this repository

1. `cargo new <package_name> --bin`

2. Add git2 crate to Cargo.toml
  ```
  [dependencies]
  git2 = "0.13"
  ```

3. Place main.rs in src directory

4. `cargo build`
