# AtCoder-Tools

AtCoder 用の CLI ツールです。Rust で実装しています。

このリポジトリはまだ作りかけです。現在は、AtCoder へのログイン、サンプルテストの取得、ローカルでのテスト実行を主な対象にしています。将来的には提出まで扱う予定です。

## 現在使える機能

- `cookie import`: ブラウザからコピーした AtCoder の Request Cookie ヘッダーを保存する。
- `cookie check`: 保存済み cookie でログイン状態を確認する。
- `fetch-test` / `f`: コンテストページまたはタスクページからサンプルテストを取得する。
- `test` / `t`: 設定済みコマンドでローカル実行し、サンプル出力と比較する。

`submit` / `s` は CLI にはありますが、まだ未実装です。

## セットアップ

プロジェクトのルートに `.atcoder` ディレクトリを作り、ユーザー設定を `.atcoder/config.toml` に置きます。

アプリ側の既定パスは `Config.toml` で管理されています。

- セッション情報: `.atcoder/session_data.json`
- タスク情報: `.atcoder/tasks_info.json`
- サンプルテスト: `test`
- ユーザー設定: `.atcoder/config.toml`

ユーザー設定には、言語ごとのコンパイルコマンドと実行コマンドを定義します。

```toml
[[language]]
name = "rust"
id = "5054"
src_path = "src/main.rs"

[language.compile]
command = "cargo"
args = ["build", "--release"]
working_dir = "."

[language.execute]
command = "target/release/main"
args = []
working_dir = "."
```

## 基本的な使い方

ブラウザで AtCoder にログインしたあと、DevTools などから Request Cookie ヘッダーをコピーして取り込みます。

```sh
cargo run -- cookie import
```

保存済み cookie でログイン状態を確認します。

```sh
cargo run -- cookie check
```

コンテストページから全タスクのサンプルを取得します。

```sh
cargo run -- fetch-test https://atcoder.jp/contests/abc388
```

タスクページから単一タスクのサンプルを取得します。

```sh
cargo run -- fetch-test https://atcoder.jp/contests/abc388/tasks/abc388_a
```

取得済みサンプルでテストします。

```sh
cargo run -- test rust A
```

一部のテストケースだけ実行します。

```sh
cargo run -- test rust A --test-cases 1 3
```

差分を省略せず表示します。

```sh
cargo run -- test rust A --verbose
```

## 現在の制限

- 実 HTTP に依存する処理があり、AtCoder の HTML 構造変更に弱いです。
- HTML parse に失敗した場合の扱いはまだ粗く、不正な状態を単にスキップしている箇所があります。
- 実 HTTP や実端末入力を使うテストは `#[ignore]` が多く、ユースケース単位の自動テストはまだ薄いです。
- 設定やテストケースの相対パス解決は、今後整理する予定です。
- 一部のテストは Windows 前提の実装を含みます。
- `submit` コマンドは未実装です。

## 設計メモ

- CLI から username/password で AtCoder へログインする処理は持ちません。ブラウザでログイン済みの cookie を `cookie import` で取り込み、保存済み cookie の確認は `cookie check` で行います。
- CSRF token は cookie 由来の session 情報として保存します。`cookie import` は AtCoder へ GET/POST せず、login のために fresh token を取得しません。
- `Url<PageType>` や `Html<PageType>` のような phantom marker type による型安全性は、このツールの重要な設計要素として残します。
- 開発初期は実装速度を優先してもよいですが、エラーは最終的に CLI 利用者が判断しやすい enum 型へ寄せます。

## 開発メモ

リファクタリング予定は `TODO.md`、今後追加したい機能は `FEATURES.md` にまとめています。AI エージェントや開発者向けの作業方針は `AGENTS.md` を参照してください。
