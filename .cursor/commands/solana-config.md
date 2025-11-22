# Solana設定の確認・変更

現在のSolana CLI設定を確認・変更します。

## 現在の設定確認

```bash
solana config get
```

## ネットワークの設定

```bash
# localnet
solana config set --url http://127.0.0.1:8899

# devnet
solana config set --url https://api.devnet.solana.com

# mainnet
solana config set --url https://api.mainnet.solana.com
```

## キーペアの設定

```bash
solana config set --keypair ~/.config/solana/id.json
```

## 設定例（開発環境）

```bash
solana config set --url http://127.0.0.1:8899
solana config set --keypair ~/.config/solana/id.json
```
