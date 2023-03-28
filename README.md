# git2_test

指定されたリポジトリ内のコミットをトポロジカル順序で表示するテスト。
指定されたリポジトリのパスを引数として受け取り、コミットIDとサマリーを出力する。

## How to create this repository

1. `cargo new git2_test --bin`

2. Cargo.tomlにgit2クレートを追加
  ```
  [dependencies]
  git2 = "0.13"
  ```

3. main.rsをsrcディレクトリに配置

4. `cargo run`
