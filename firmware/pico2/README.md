# Pico 2 ファームウェア

UUaのRP2350 / Raspberry Pi Pico 2向けファームウェアを配置するディレクトリです。PC参照実装から独立したビルド・検証の入口とします。

## このディレクトリの責務

- Pico 2向けのソースコード、ビルド定義、生成物の作成手順、ファームウェア側のテストを後続Issueで管理する。
- PC側のCargo、ウィンドウ表示ライブラリ、PCアプリケーションのビルド成果物に依存しない。
- ハードウェアの候補やUART・USB・HSTXの未決定事項を、このディレクトリ作成だけで採用済み仕様にしない。

## 現在の状態

この段階ではディレクトリの案内のみを用意します。ファームウェアのソースコード、CMake設定、UF2生成、実機動作、CIは未実装・未検証です。Pico 2への書き込みや映像出力ができる状態ではありません。

## 後続作業

- [Impl #12](https://github.com/kkismd/UUa/issues/12)：採用済みのPico 2技術選定ADRを前提に、独立ビルドとUF2生成基盤を実装する。
- [Impl #13](https://github.com/kkismd/UUa/issues/13)：Pico 2側の独立CIを整備する。

## 関連資料

- [リポジトリREADME](../../README.md)
- [ハードウェア設計](../../docs/hardware_design.md)
- [開発・レビューの共通ルール](../../AGENTS.md)
- [ADR #7（採用済み：独立ビルド構成）](https://github.com/kkismd/UUa/issues/7)
