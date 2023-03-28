# git2_test

- Specify the path of the specified repository as an argument
- outputs the commit IDs and summaries in topological order.

## How to create this repository

1. `cargo new git2_test --bin`

2. Add git2 crate to Cargo.toml
  ```
  [dependencies]
  git2 = "0.13"
  ```

3. Place main.rs in src directory

4. `cargo run <path_to_git_repository>`
