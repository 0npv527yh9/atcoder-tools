# TODO

このファイルはリファクタリング作業リストです。未完了の作業は `- [ ]` で残し、完了した作業は `Done` セクションへ移動します。優先度の高い順に並べています。

## P0: すぐ直したい

- [ ] 2. ログイン時は保存済み CSRF token に頼らず、毎回新規取得する方針を明確にしてテストする。

## P1: テストしやすくする

- [ ] 6. `usecase::fetch_test_suite` から IO・CLI 出力・ユースケースロジックを分離する。
- [ ] 7. `usecase::login` から端末入力・CLI 出力・ログイン判定・保存処理を分離する。
- [ ] 8. ignored な実 HTTP テストとは別に、fake/mock ベースの usecase / runner テストを追加する。
- [ ] 9. usecase / runner / cli の境界ごとに単体テストを追加する。
- [ ] 10. `command_handler` の実プロセス起動に依存しないテスト境界を作る。
- [ ] 11. `terminal_handler::print_diffs` と diff 生成を分け、端末サイズ取得なしでテストできる範囲を増やす。
- [ ] 32. `usecase::test` から `AC` 表示などの CLI 出力を分離し、テスト結果を runner 側で表示できる形にする。

## P2: 依存方向を整理する

- [ ] 14. `dao` が HTTP と HTML parsing の両方を直接知っているため、責務を再確認する。
- [ ] 15. `dto` に設定・保存形式・usecase 内部モデルが混在しているため、`config` / `model` / `infra` 境界へ分解する。
- [ ] 16. CLI 分岐とインスタンス生成を分け、必要なら application モジュールとして整理する。
- [ ] 17. `http_handler::with_cookies` を外部 DI で置き換えられるか検討する。
- [ ] 28. ビルド影響範囲と責務境界を見直し、`infra::atcoder` や file system adapter を workspace crate として分離する必要があるか判断する。
- [ ] 29. `runner` の DAO/session 組み立て helper 名を見直し、`setup` ではなく「何を元に何を作るか」が分かる名前にする。
- [ ] 30. usecase の戻り値から `Dao` を外し、session 保存 I/F を `SessionData` または意味のある結果型に整理する。
- [ ] 31. `infra::atcoder::url::Url<PageType>` の生成を `From<String>` ではなく `TryFrom` / `FromStr` ベースにし、不正な URL を型として作れないようにする。
- [ ] 33. `cli` が `infra::atcoder::url::FetchTaskUrl` に直接依存しているため、fetch-test の CLI 引数と usecase 入力モデルの境界を整理する。
  - `cli` は URL 文字列を受け取るだけにし、AtCoder URL の解析・派生 URL 構築には直接依存しない。
  - `usecase::fetch_test_suite` には `Contest` / `Task` のような fetch-test 用の入力モデルを置く。
  - `infra::atcoder` には AtCoder website の URL 構造、`/tasks_print` や `/tasks` の派生 URL、`task_screen_name` 解析などを残す。
  - `FetchTaskUrl` 相当の責務は「usecase の対象種別」と「AtCoder website adapter の URL 知識」に分ける。

## P3: 品質改善

- [ ] 18. `cargo test` の警告を減らす。
- [ ] 19. ignored テストの目的をコメントで明確にする。
- [ ] 20. AtCoder HTML fixture の更新手順を文書化する。
- [ ] 21. HTML parse の不正状態を単にスキップせず、検知できるエラーまたは警告にする。
- [ ] 22. エラーメッセージを CLI 利用者向けに整える。
- [ ] 23. ログインしていない状態とセッションファイルが存在しない状態を区別して扱う。
- [ ] 24. 相対パスの基準ディレクトリを整理し、設定・テストケース読み込みで一貫させる。
- [ ] 25. Windows 以外でも動くテストに寄せる。

## Done

- [x] 1. `login` の `file_handler::save` 結果を捨てている箇所を修正する。
- [x] 3. `run` 系関数から `unwrap_or_exit` / `process::exit` を外し、`main` だけで終了コードを決める。
- [x] 4. `file_handler::load_config` の `panic!` を通常のエラーにする。
- [x] 5. `config_loader::load_config` の `set_current_dir` を見直し、グローバルなカレントディレクトリ変更に依存しない構造にする。
- [x] 12. `domain::path` から `handler::file_handler` 依存をなくす。
- [x] 13. `handler` という名前が実装詳細に寄りすぎているため、将来的に `infra` や `adapter` への整理を検討する。
- [x] 26. `main` / `cli` / `app` / `usecase` / `infra` の層分けが既存 TODO 13・16 と整合するか整理し、必要なモジュール境界だけを決める。
- [x] 27. `usecase::mod` に残っている DAO setup / session save helper を composition root または infra factory へ移し、usecase が依存の組み立てを持たない形にする。
