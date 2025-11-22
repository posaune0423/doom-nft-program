# Solanaエアドロップ

テストネットワークからSOLを受け取ります。

```bash
solana airdrop 2
```

## パラメータ
- `2` - 受け取るSOLの量（SOL単位）

## 前提条件
- devnetまたはtestnetに接続していること
- 1回のリクエストで最大2SOLまで

## ネットワークの切り替え

```bash
# devnet
solana config set --url https://api.devnet.solana.com

# testnet
solana config set --url https://api.testnet.solana.com
```

## 関連コマンド
- `solana balance` - 残高確認
