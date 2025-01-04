# HACK_FREXISPOT

## 概要

このプロジェクトは、RUSTを用いてFlexispotデバイスを制御するためのツールです。
これを利用しiPhoneやMacなどのssh接続可能なデバイスからFlexSpotの昇降などの制御が可能になります。

## 必要条件

- Rust
- Raspberry Pi model 3
- FlexiSpot
- LANケーブル(RJ45型プラグ)
- GPIOピンとLANケーブルを接続する何か

## インストール

1. リポジトリをRaspberry Piにクローンします。
    ```sh
    git clone https://github.com/JunNakarai/hack_flexispot
    ```
2. プロジェクトディレクトリに移動します。
    ```sh
    cd hack_flexispot
    ```
3. 依存関係をインストールします。
    ```sh
    cargo build
    ```

## 使い方

1. プロジェクトをビルドします。
    ```sh
    cargo build --release
    ```
2. プロジェクトを実行します。
    ```sh
    bash -l -c "cd ~/hack_flexispot && cargo run -- stand"
    ```

## サポートされているコマンド

以下のコマンドがサポートされています：
- `stand`: スタンディング状態
- コマンド2: 説明

## クレジット

このプロジェクトは、[iMicknl/LoctekMotion_IoT](https://github.com/iMicknl/LoctekMotion_IoT)のコードを参考にしています。
