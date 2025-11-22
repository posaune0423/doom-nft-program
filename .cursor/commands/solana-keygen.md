# Solanaキーペアの生成

新しいSolanaキーペアを生成します。

```bash
solana-keygen new --outfile ~/.config/solana/id.json
```

## オプション
- `--no-passphrase` - パスフレーズなし
- `--force` - 既存ファイルを上書き
- `--silent` - 詳細出力を抑制

## 公開鍵の確認

```bash
solana-keygen pubkey ~/.config/solana/id.json
```

## 関連コマンド
- `solana config set --keypair ~/.config/solana/id.json` - デフォルトキーペアの設定
