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

### 参考

https://learn.microsoft.com/ja-jp/windows/dev-environment/rust/setup