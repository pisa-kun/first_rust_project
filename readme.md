# VSCodeで Hello, World!チュートリアル
MSのチュートリアルをそのままなぞっただけです。

### make project & debug

> cargo new first_rust_project

プロジェクトが自動で作成される。

> cd first_rust_project

> code ./

ワークスペースを開く

> code src/main.ts

```rs
// src/main.rs
fn main() {
    println!("Hello, world!");
}
```
2行目にブレークポイントをはってF5実行
ポップアップが表示されるのでyesを選択後にデバッグできるようになる

### コードをコンパイルして実行
> cargo run

### 簡易コマンド集
- プロジェクト作成
> cargo new project_name

- 実行
> cargo run

- バイナリ生成
> rustc src/main.rs

## LambdaをRustで実行(container経由)
**/hello_container 参照**

### 参考

https://learn.microsoft.com/ja-jp/windows/dev-environment/rust/setup

### Rust ByExample
https://doc.rust-jp.rs/rust-by-example-ja/