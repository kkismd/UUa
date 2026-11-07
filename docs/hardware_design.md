# UUa — ハードウェア構成・接続条件

**状態：** 設計検討段階の暫定方針  
**関連資料：** [PoC戦略とソフトウェア設計](poc_strategy.md)

> この資料は技術的な候補・制約・検証手順のみを示す。部品の所有、購入、発注、配送、在庫については扱わない。

## 1. ハードウェアの方向性

- **MCU：** RP2350搭載のRaspberry Pi Pico 2系列を中心に検討。
- **初期試作の有力候補：** Pico 2H（ヘッダ実装済み・無線なし）。Pico 2WH（無線あり）は拡張候補。
- **映像出力：** RP2350のHSTXを利用し、DVI映像信号をMini HDMI端子から出力する。追加の映像出力基板を使う構成を優先。
- **映像出力基板候補：** Adafruit PiCowBell HSTX DVI Output。
- **採用理由：** RP2350のSRAMとHSTXを活用し、小型MCUでVDPを試作できる。外付けDVI基板を用いれば高周波映像配線とコネクタ実装を省力化できる。初期PoCではWi-Fiを必須としない。

## 2. 想定システム

~~~text
8-bit SBC（IDEアプリ／言語処理系／文書の論理的正本）
      │  双方向UART（端子未割当）
      ▼
RP2350 / Pico 2H（候補）
      ├─ HSTX GPIO12..19
      │      ▼
      │   Adafruit PiCowBell HSTX DVI Output（候補）
      │      └─ Mini HDMI → DVI対応HDMIモニター
      ├─ 入力：キーボード／マウス（接続方式は検討中）
      └─ UI object state／renderer／UART protocol
~~~

Mini HDMIは**DVI映像の出力端子**として使う。音声出力を前提としない。

## 3. 表示・処理資源

| 項目 | 方針／数値 | 決定水準・理由 |
|---|---|---|
| VDP MCU | RP2350 | 第一候補のMCU系列。Pico 2H/2WHで共通 |
| CPU | 最高150MHzのデュアルコア（Arm Cortex-M33またはRISC-V構成） | 公称性能。コア割当は未確定 |
| SRAM | 520KiB | renderer、UI状態、入力等と共用 |
| オンボードFlash | 4MB | 標準仕様。ストレージAPI採用を意味しない |
| 論理解像度 | **320×240** | 8×8文字で40列×30行 |
| 映像出力例 | **640×480**への整数2倍拡大 | 実機タイミング確認が必要 |
| 色深度 | 未確定 | 1/4/8/16bppを比較 |
| フォント | 8×8中心、8×16も候補 | レトロGUI・文字表示に適合 |
| 描画方式 | 初期はframebuffer優先 | scanline方式は必要時に検討 |
| SBC通信 | 双方向UART | 115200bps級をPC PoCで検証 |
| 入力 | キーボード、マウス、必要に応じてゲームパッド | USB Host等の接続方式は未確定 |
| ストレージ | 将来VDP側I/O serviceとして追加可能 | 初期PoCでは不要 |

### 320×240フレームバッファのサイズ

| 色深度 | 必要メモリ（1面、行パディングなし） |
|---|---:|
| 1bpp | 9,600B ≒ 9.4KiB |
| 4bpp | 38,400B ≒ 37.5KiB |
| 8bpp | 76,800B = 75KiB |
| 16bpp | 153,600B = 150KiB |

描画作業領域、オブジェクト状態、フォント、EDITOR表示キャッシュ、通信バッファ、スタック、映像信号生成用バッファも必要。520KiBの全てを画面に使えるわけではない。形式とダブルバッファ採否は未決定。

## 4. PiCowBell HSTX DVI Outputの接続条件

Adafruit公式ドキュメントに基づく主な割当：

| 用途 | GPIO等 | 備考 |
|---|---|---|
| HSTX DVI | **GPIO12〜19** | DVIデータ／クロックの4差動ペアに割当 |
| PIO-USBホスト用パッド | **D+ = GPIO6、D− = GPIO7** | USB-A接続、電源、ソフトウェア対応は別途必要 |
| Host用5V/GNDパッド | **5V = VSYS、G = GND** | 給電可否と電源予算を検証 |
| スライドスイッチ | GPIO3 | 競合に注意 |
| STEMMA QT / I2C | GPIO4/5 | I2C機器接続候補 |
| SBC用UART | **未割当** | 上記ピンを避けて対応UART機能と配線を確定 |

**理由：** HSTXに映像を担当させ、PIOやCPU資源をUI処理へ残す。USB Host用パッドがあっても、それだけでUSBキーボードが動作するわけではない。

## 5. 構成部品の候補と確認事項

| 品目 | 設計上の位置づけ | 確認事項 |
|---|---|---|
| Adafruit PiCowBell HSTX DVI Output | DVI出力用基板の候補 | HSTXの配線、物理的な嵌合、組立方法 |
| Raspberry Pi Pico 2H | MCU基板の第一候補 | RP2350、ヘッダ、無線不要の要件 |
| Raspberry Pi Pico 2WH | 代替候補 | 無線機能が必要になった場合の選択 |
| Mini HDMI → HDMIケーブル | 映像接続に必要な構成要素 | 端子・モニターとの互換性 |
| HDMI入力モニター | 表示試験用 | DVI信号の受信互換性 |
| ソケットヘッダ／積層ヘッダ | 基板接続の候補 | ヘッダ実装済みボードとの嵌合 |
| USB Host用ケーブル／コネクタ／給電部品 | 入力機能を追加する段階で検討 | 接続規格と電源予算 |
| SBCとのUART接続部品 | プロトコル検証後に選定 | GPIO、共通GND、電圧レベル、保護 |

**その他の技術候補：** DVI出力端子を備えたRP2350系ボードも比較対象。選定はピン割当、実装容易性、映像信号、UART/USBとの競合を基準にする。

## 6. 実機立ち上げの検証手順

PC版PoCと切り離して、ハードウェア単体で次を確認する。

1. MCU基板と映像出力基板の物理的嵌合、向き、接続方法、はんだ付けを確認。
2. 公式／実績のあるRP2350 HSTX DVIサンプルで**テストパターンの安定表示**を確認。
3. 320×240論理描画から640×480への2倍拡大を検証。
4. framebufferとUI描画処理のRAM・フレーム時間を計測。
5. UART、キーボード／マウスの同時利用を段階的に試験。

**理由：** DVI配線／タイミング問題をGUIプロトコル・IDE・言語処理系から切り離して追跡する。初期PoCの完了条件を早期の実機映像実験で変更しない。

## 7. 未決定事項・要確認事項

- Pico 2H/2WHの最終選択と基板構成。
- PiCowBellとヘッダ実装済みPico 2Hの組立方法、向き、クリアランス。
- DVIタイミング・モニター互換性、色深度、描画・転送方式。
- UARTのGPIO割当、電圧レベル、8ビットSBCとの電気的接続。**RS-232電圧と3.3V GPIO UARTを直結しない。**
- PIO-USB／その他USB Host方式、機器への給電、入力機器の同時利用。
- UI object／EDITOR用RAM上限、映像出力時の負荷、DMAとCPUコア分担。
- 将来の保存メディアとストレージ抽象化。**初期PoCでは着手しない。**

## 8. 公開技術資料

- [Adafruit：PiCowBell HSTX DVI Output 概要・配線・組み立て](https://learn.adafruit.com/adafruit-picowbell-hstx-dvi-output)
- [Adafruit：Pinouts](https://learn.adafruit.com/adafruit-picowbell-hstx-dvi-output/pinouts)
- [Adafruit：Socket Headers](https://learn.adafruit.com/adafruit-picowbell-hstx-dvi-output/socket-headers)
- [Raspberry Pi：Picoシリーズ仕様](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html)

---

**要点：** RP2350/Pico 2Hを試作第一候補とし、HSTX＋PiCowBellによるDVIを想定する。実機立ち上げはテストパターンから始め、GUI・UART・USBを後で統合する。製品の選択は技術上の候補を示すものであり、入手状況を表さない。
