# HACK_FREXISPOT

## 概要

このプロジェクトは、Rustを用いてFlexispotE8デスクの昇降を制御するためのツールです。
これを利用しiPhoneやMacなどのssh接続可能なデバイスからデスクの昇降の制御が可能になります。

## 必要条件

- Rust
- Raspberry Pi model 3
- FlexiSpot
- LANケーブル(RJ45型プラグ)
- GPIOピンとLANケーブルを接続する何か

## インストール

リポジトリをRaspberry Piにクローンします。

```sh
git clone https://github.com/JunNakarai/hack_flexispot
```

## 使い方

1. プロジェクトをビルドします。
    ```sh
    cargo build
    ```
2. プロジェクトを実行します。
    ```sh
    cargo run -- stand
    ```

> [!TIP]
> `Initialization error: Failed to open serial port: Permission denied`と表示される場合は
> `sudo chmod 666 /dev/serial0`を実行し権限を付与して下さい。

## サポートされているコマンド

以下のコマンドがサポートされています：
- `1`: プリセット1
- `2`: プリセット2
- `sit`: 座り
- `stand`: 立ち

## クレジット

このプロジェクトは、[iMicknl/LoctekMotion_IoT](https://github.com/iMicknl/LoctekMotion_IoT)のコードを参考にしています。
