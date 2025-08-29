# FileAction

`file-action` は Rust 向けのシンプルなファイル操作ユーティリティです。テキスト、CSV、JSON の読み書きを型安全に行える API を提供します。

## インストール

Cargo.toml に以下を追加してください（公開済みのバージョンを指定します）：

```toml
[dependencies]
file-action = "0.1.0"
```

## クイックスタート

ライブラリとして使う簡単な例です。ファイルパスは呼び出し元の環境に合わせて指定してください。

```rust
use file_action::FileAction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person { name: String, age: u32 }

fn main() -> std::io::Result<()> {
    // テキストを書き込み（上書き）
    FileAction::write_new("example.txt", "Hello from FileAction!\n")?;

    // テキストを読み込み
    let text = FileAction::read("example.txt")?;
    println!("read: {}", text);

    // JSON の書き込み・読み込み
    let p = Person { name: "Alice".into(), age: 30 };
    FileAction::write_json("person.json", &p, None)?;
    let loaded: Person = FileAction::read_json("person.json")?;
    println!("loaded: {:?}", loaded);

    Ok(())
}
```

## API（概要）

- `FileAction::read(path: &str) -> Result<String>` — テキストファイルの読み込み。
- `FileAction::read_csv(path: &str) -> Result<Vec<Vec<String>>>` — CSV を行・列の 2 次元配列で返す。
- `FileAction::read_json<T: DeserializeOwned>(path: &str) -> Result<T>` — JSON を型指定で読み込む。
- `FileAction::write_new(path: &str, s: &str) -> Result<()>` — ファイルを上書きして書き込む。
- `FileAction::write_add(path: &str, s: &str) -> Result<()>` — 追記で書き込む。
- `FileAction::write_json<T: Serialize>(path: &str, obj: &T, space: Option<usize>) -> Result<()>` — JSON を書き込む（整形指定可能）。
- `FileAction::clear(path: &str) -> Result<()>` — ファイル内容を空にする。

詳細はコードのドキュメントコメント（`src/lib.rs`）をご参照ください。

## 開発中の例

このリポジトリには `src/main.rs` に簡単な動作例が含まれます。開発や試用の際は `cargo run` で実行できます。

## 貢献・問題報告

バグ報告や改善提案は GitHub リポジトリの Issue を使ってください（`repository` フィールドに記載の URL を参照）。

## ライセンス

MIT
