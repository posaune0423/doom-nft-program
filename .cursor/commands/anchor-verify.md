# Anchorプログラムの検証

プログラムのビルドとテストを包括的に検証します。

```bash
# 完全な検証スイート
anchor build && anchor test && cargo clippy && cargo fmt --check
```

## 個別検証

### ビルド検証
```bash
anchor build
```

### テスト実行
```bash
anchor test
```

### コード品質チェック
```bash
cargo clippy -- -D warnings
```

### フォーマットチェック
```bash
cargo fmt --check
```

## CI/CDでの使用例

```yaml
- name: Verify Anchor Program
  run: |
    anchor build
    anchor test
    cargo clippy -- -D warnings
    cargo fmt --check
```
