# PC PoC

RustによるPC参照実装です。描画状態と論理画面はPCウィンドウ／入力ライブラリから独立させ、SBCがアプリケーション状態を所有する設計を維持します。

## 実行基盤

- `render` は320×240、`0x00RRGGBB` の画素バッファと静的テストパターンを管理します。
- `platform` はminifbウィンドウ、入力の取得、論理バッファの表示を担当します。
- `main` はウィンドウの起動、更新ループ、終了を扱います。
- UIオブジェクト、正式な入力意味論、文字描画、通信、GUI DSL/UI IRはこの段階に含みません。

## ビルドとテスト

Rust stable（edition 2024）と、minifbが必要とするホスト側ライブラリを用意してください。Linux環境ではX11/Wayland関連ライブラリが必要になる場合があります。

```sh
cargo build --manifest-path apps/pc-poc/Cargo.toml --locked
cargo test --manifest-path apps/pc-poc/Cargo.toml --locked
cargo fmt --manifest-path apps/pc-poc/Cargo.toml --check
cargo clippy --manifest-path apps/pc-poc/Cargo.toml --all-targets -- -D warnings
```

## 起動と操作

```sh
cargo run --manifest-path apps/pc-poc/Cargo.toml --locked
```

320×240の論理テストパターンが640×480で表示されます。Escまたはウィンドウを閉じると終了します。ウィンドウにはキー押下（リピートなし）、マウス位置、左ボタン状態を取得する入力アダプタがあります。これらはアプリケーション向けSemanticイベントやIME契約を定めるものではありません。

## 関連資料

- [リポジトリREADME](../../README.md)
- [PoC戦略](../../docs/poc_strategy.md)
- [ADR #7（採用済み：独立ビルド構成）](https://github.com/kkismd/UUa/issues/7)
- [ADR #8（採用済み：PC技術スタック）](https://github.com/kkismd/UUa/issues/8)
- [Impl #11](https://github.com/kkismd/UUa/issues/11)
- [Impl #13（独立CI）](https://github.com/kkismd/UUa/issues/13)
