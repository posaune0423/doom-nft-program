# Doom NFT Program

Solanaブロックチェーン上で動作するNFT（Non-Fungible Token）プログラムです。Anchorフレームワークを使用して実装されています。

## 特徴

- **NFTミント**: 新しいNFTを作成・発行
- **トークン転送**: NFTの所有権移転
- **SPLトークン互換**: Solanaの標準トークン規格に対応
- **セキュリティ**: Anchorフレームワークのセキュリティ機能を使用

## 技術スタック

- **Blockchain**: Solana
- **Framework**: Anchor
- **Language**: Rust
- **Testing**: TypeScript, Mocha, Chai
- **Package Manager**: Bun

## 前提条件

- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/)
- [Bun](https://bun.sh/) または [Node.js](https://nodejs.org/)

## インストール

```bash
# リポジトリをクローン
git clone https://github.com/posaune0423/doom-nft-program.git
cd doom-nft-program

# 依存関係のインストール
bun install

# Solana CLIの設定（devnetを使用する場合）
solana config set --url https://api.devnet.solana.com

# キーペアの生成（初回のみ）
solana-keygen new
```

## ビルド

```bash
# プログラムのビルド
anchor build

# IDLファイルの生成
anchor idl parse -f programs/doom-nft-program/src/lib.rs -o target/idl/doom_nft_program.json
```

## テスト

```bash
# テスト実行
anchor test

# ローカルバリデーターでのテスト
anchor localnet
```

## デプロイ

```bash
# プログラムのデプロイ
anchor deploy

# プログラムIDの確認
solana program show --programs
```

## 使用方法

### NFTミント

```rust
// プログラム内でNFTを作成
create_mint(ctx)?;

// トークンをミント
mint_token(ctx)?;
```

### NFT転送

```rust
// NFTを転送
transfer_token(ctx)?;
```

## プログラム構造

```
programs/doom-nft-program/
├── src/lib.rs          # メインのプログラムロジック
├── Cargo.toml          # Rust依存関係
└── Xargo.toml          # クロスコンパイル設定

tests/
├── src/lib.rs          # テストファイル
└── Cargo.toml          # テスト依存関係

migrations/
├── src/main.rs         # マイグレーションスクリプト
└── Cargo.toml          # マイグレーション依存関係
```

## コントリビューション

1. Forkしてください
2. Featureブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにPush (`git push origin feature/amazing-feature`)
5. Pull Requestを作成してください

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。

## 注意事項

- このプログラムは開発中です
- テストネットでのみ動作確認を行っています
- 本番環境での使用は自己責任でお願いします
