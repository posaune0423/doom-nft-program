# Anchorプログラムのデプロイ

AnchorプログラムをSolanaブロックチェーンにデプロイします。

```bash
anchor deploy
```

## 前提条件
- Solana CLIのインストール
- ウォレットの設定
- 十分なSOL残高

## オプション
- `--provider.cluster mainnet` - メインネットへのデプロイ
- `--provider.cluster devnet` - デブネットへのデプロイ

## 関連コマンド
- `solana balance` - 残高確認
- `solana airdrop 2` - テストSOLの取得
