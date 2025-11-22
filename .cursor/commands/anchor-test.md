# Anchorテストの実行

プログラムのテストを実行します。

```bash
anchor test
```

## 説明
- ローカルバリデータを起動
- Rust単体テストの実行
- TypeScript統合テストの実行
- テスト後のクリーンアップ

## オプション
- `--skip-build` - ビルドをスキップ
- `--skip-deploy` - デプロイをスキップ
- `--skip-lint` - リンターをスキップ

## テストファイル
- `tests/*.ts` - TypeScript統合テスト
- `programs/*/src/lib.rs` - Rust単体テスト
