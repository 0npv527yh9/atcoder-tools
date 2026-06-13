# TODO

このファイルはリファクタリング作業リストです。未完了の作業は `- [ ]` で残し、完了した作業は `Done` セクションへ移動します。優先度の高い順に並べています。

## P0: すぐ直したい

- [ ] `login` の `file_handler::save` 結果を捨てている箇所を修正する。
- [ ] ログイン時は保存済み CSRF token に頼らず、毎回新規取得する方針を明確にしてテストする。
- [ ] `run` 系関数から `unwrap_or_exit` / `process::exit` を外し、`main` だけで終了コードを決める。
- [ ] `file_handler::load_config` の `panic!` を通常のエラーにする。
- [ ] `file_handler::load_config` の `set_current_dir` を見直し、グローバルなカレントディレクトリ変更に依存しない構造にする。

## P1: テストしやすくする

- [ ] `app::fetch_test_suite` から IO・表示・ユースケースロジックを分離する。
- [ ] `app::login` から端末入力・表示・ログイン判定・保存処理を分離する。
- [ ] ignored な実 HTTP テストとは別に、fake/mock ベースの app テストを追加する。
- [ ] service / app / cli の境界ごとに単体テストを追加する。
- [ ] `command_handler` の実プロセス起動に依存しないテスト境界を作る。
- [ ] `terminal_handler::print_diffs` と diff 生成を分け、端末サイズ取得なしでテストできる範囲を増やす。

## P2: 依存方向を整理する

- [ ] `domain::path` から `handler::file_handler` 依存をなくす。
- [ ] `handler` という名前が実装詳細に寄りすぎているため、将来的に `infra` や `adapter` への整理を検討する。
- [ ] `dao` が HTTP と HTML parsing の両方を直接知っているため、責務を再確認する。
- [ ] `dto` に domain に近い型と設定用の型が混在しているため、必要になった時点で分割する。
- [ ] CLI 分岐とインスタンス生成を分け、必要なら application モジュールとして整理する。
- [ ] `http_handler::with_cookies` を外部 DI で置き換えられるか検討する。

## P3: 品質改善

- [ ] `cargo test` の警告を減らす。
- [ ] ignored テストの目的をコメントで明確にする。
- [ ] AtCoder HTML fixture の更新手順を文書化する。
- [ ] HTML parse の不正状態を単にスキップせず、検知できるエラーまたは警告にする。
- [ ] エラーメッセージを CLI 利用者向けに整える。
- [ ] ログインしていない状態とセッションファイルが存在しない状態を区別して扱う。
- [ ] 相対パスの基準ディレクトリを整理し、設定・テストケース読み込みで一貫させる。
- [ ] Windows 以外でも動くテストに寄せる。

## Done

まだありません。
