# DOOM INDEX NFT Mint Contract V1 要件定義書

## 1. 目的

DOOM INDEX において一定時間ごとに生成される作品を、Solana 上の NFT として mint 可能にする。
初期段階では仕様変更の可能性が高いため、本仕様の対象 contract は **`DOOM INDEX NFT Mint Contract V1`** として定義する。
Contract V1 は **upgradeable program** を前提とし、変化しやすい metadata 生成と配信は off-chain に寄せる。

本機能の主目的は以下とする。

- ユーザーが DOOM INDEX の作品を Solana NFT として取得できること
- NFT ごとにアプリケーション独自の `tokenId` を連番で払い出せること
- Solana の標準的な NFT metadata 仕様に沿って、thumbnail image に加えて 3D model を扱えること
- localnet に加えて devnet 上でも一通りの動作を確認できること
- 将来的な仕様変更に耐えられること

## 2. 採用方針

### 2.1 標準とチェーン

- NFT 標準は Metaplex Core を採用する
- 実体識別子は Solana / Metaplex Core 上の `Asset Address` とする
- `tokenId` はアプリケーション独自の連番識別子として on-chain で管理する
- 開発時の動作確認環境は `localnet` と `devnet` を対象とする
- Contract V1 は upgradeable program として deploy する
- 将来の Contract V2 以降に移行可能な前提で、state と instruction を最小構成に保つ

### 2.2 v1 の基本設計

- mint 時の metadata URI は backend が都度承認するのではなく、program が `tokenId` から deterministic に決定する
- metadata URI の標準ルールは `"{base_metadata_url}/{tokenId}.json"` とする
- metadata JSON を mint 前に配置するため、mint 前に `tokenId` を確定する reservation フローを採用する
- 同一生成期間内に metadata の内容が同一であってもよい
- ただし URI 自体は tokenId ごとに一意とする
- on-chain に generation state は持たない
- user は任意の URI を指定できない
- business admin と program upgrade authority は分離する
- collection authority は EOA ではなく program 管理 PDA を第一候補とする

## 3. スコープ

### 3.1 対象

- Metaplex Core Collection の初期化
- DOOM INDEX NFT の mint
- `tokenId` の連番採番
- deterministic metadata URI の付与
- 管理者による運用機能
- devnet E2E 検証フロー
- standard metadata + 3D model 対応

### 3.2 対象外

以下は v1 の対象外とする。

- on-chain generation boundary の厳密管理
- cNFT / compressed NFT
- allowlist / WL mint
- per-wallet mint limit
- mint price / treasury
- 二次流通戦略
- royalties の厳密設計
- permissionless 以外の承認付き mint
- per-token metadata の on-chain 保持
- 既存 NFT の自動 batch metadata 差し替え

## 4. システム概要

### 4.1 全体構成

本システムは、作品 metadata を off-chain で管理し、mint 時点で program が算出した URI を使って Metaplex Core Asset を発行する構成とする。

- on-chain program は `tokenId` 採番、URI 決定、Collection 配下の Asset 作成を担当する
- off-chain backend は thumbnail / 3D model / metadata JSON の生成と配置を担当する
- frontend は user wallet 署名による mint UI を提供する

### 4.2 識別子

- `Asset Address`: Solana / Metaplex Core 上の NFT 実体識別子
- `tokenId`: DOOM INDEX アプリケーション独自の連番 ID
- `Collection Address`: DOOM INDEX Collection の識別子

## 5. On-chain 設計

### 5.1 GlobalConfig

v1 の永続 state は `GlobalConfig` と `MintReservation` とする。
`GlobalConfig` の PDA seed は `["global_config"]` で固定する。

推奨フィールドは以下とする。

- `admin: Pubkey`
- `upgrade_authority: Pubkey`
- `next_token_id: u64`
- `mint_paused: bool`
- `base_metadata_url: String`
- `collection: Pubkey`
- `collection_update_authority: Pubkey`
- `bump: u8`

`next_token_id` は次に払い出す値を保持し、初期値は `1` とする。
Contract V1 は単一カウンタで採番するため、高並列 mint よりも実装単純性を優先する。

### 5.2 MintReservation

`MintReservation` は mint 前に `tokenId` を確定し、metadata 事前配置と競合しないようにするための一時 state とする。
PDA seed は `["reservation", token_id_le_bytes]` を標準とする。

推奨フィールドは以下とする。

- `token_id: u64`
- `reserver: Pubkey`
- `minted: bool`
- `bump: u8`

### 5.3 Anchor instructions

#### `initialize_global_config`

- 実行者は `admin`
- `GlobalConfig` を初期化する
- `admin` と `upgrade_authority` を設定する
- `next_token_id = 1`
- `mint_paused = false`
- `base_metadata_url` を設定する

#### `initialize_collection`

- 実行者は `admin`
- Metaplex Core Collection を作成する
- 作成した Collection Address を `GlobalConfig.collection` に保存する
- Collection の update authority は program 管理 PDA を第一候補とする

#### `reserve_token_id`

- 実行者は user
- `mint_paused == false` を必須とする
- `tokenId = next_token_id`
- `next_token_id += 1`
- `MintReservation` を作成する
- reservation 作成後、off-chain backend は `"{base_metadata_url}/{tokenId}.json"` を配置する
- v1 では reservation expiry は持たず、未使用 reservation は将来運用または migration で整理可能とする

#### `mint_doom_index_nft`

- 実行者は user
- `mint_paused == false` を必須とする
- user 自身の `MintReservation` を必須とする
- `MintReservation.minted == false` を必須とする
- name は `DOOM INDEX #<tokenId>` とする
- uri は `"{base_metadata_url}/{tokenId}.json"` とする
- user を owner とする Metaplex Core Asset を Collection 配下で作成する
- mint 成功時に `MintReservation.minted = true` とする

#### `update_base_metadata_url`

- 実行者は `admin`
- `base_metadata_url` を更新する
- 変更は future mint にのみ反映する
- 既存 NFT の on-chain URI は変更しない

#### `set_mint_paused`

- 実行者は `admin`
- mint の pause / unpause を切り替える

#### `transfer_admin`

- 実行者は現 `admin`
- `admin` を更新する

#### `set_upgrade_authority`

- 実行者は現 `upgrade_authority`
- `upgrade_authority` を更新する
- business admin の操作権とは独立に扱う

## 6. Functional Requirements

### 6.1 Mint

- FR-01: user は `reserve_token_id` により mint 前に `tokenId` を確保できること
- FR-02: user は予約済み `tokenId` を使って DOOM INDEX NFT を mint できること
- FR-03: 表示名は `DOOM INDEX #<tokenId>` とすること
- FR-04: mint 時に deterministic な metadata URI を設定すること
- FR-05: user は任意 URI を指定できないこと
- FR-06: NFT は Metaplex Core Asset として発行されること
- FR-07: mint された Asset は DOOM INDEX Collection に属すること
- FR-08: reservation 済みでない `tokenId` では mint できないこと
- FR-09: 同一 reservation を二重使用できないこと

### 6.2 管理機能

- FR-10: 管理者は `GlobalConfig` を初期化できること
- FR-11: 管理者は Collection を初期化できること
- FR-12: 管理者は `base_metadata_url` を更新できること
- FR-13: 管理者は mint を停止 / 再開できること
- FR-14: 管理者は admin 権限を移譲できること
- FR-15: upgrade authority は admin と分離されること

### 6.3 Devnet 検証

- FR-16: program は devnet に deploy 可能であること
- FR-17: devnet 上で Collection 初期化、reservation、metadata 配置、mint、状態確認まで一通り実行できること
- FR-18: devnet 上で発行された NFT は public metadata URI を参照できること
- FR-19: devnet 上で `image` と `animation_url` の両方が取得できること

## 7. Metadata 仕様

### 7.1 基本方針

Metaplex Core の off-chain JSON Schema に合わせる。
Core の JSON metadata は Token Metadata 系の標準に近く、`name` `description` `image` `category` を必須とし、`animation_url` `external_url` `attributes` `properties.files` を任意で持てる。

DOOM INDEX では以下を v1 の標準とする。

- `image`: thumbnail image の公開 URI
- `animation_url`: 3D model の公開 URI
- `properties.files`: image と 3D model を必ず列挙
- `external_url`: DOOM INDEX の作品詳細ページ
- `attributes`: 作品生成に使った最低限の識別情報、visual parameter、prompt 情報を格納
- `doom_index`: DOOM INDEX 独自の補助フィールド。長文 prompt や生値を機械可読に保持したい場合に使う

### 7.2 Attributes 最低要件

v1 では、各 token の metadata に少なくとも以下の情報を含める。
viewer 互換性のため、表示したい値は `attributes` に入れる。

#### Basic information

- `Generated`
- `ID`
- `Seed`
- `Params Hash`
- `File Size`

#### Visual parameters

- `fogDensity`
- `skyTint`
- `reflectivity`
- `blueBalance`
- `vegetationDensity`
- `organicPattern`
- `radiationGlow`
- `debrisIntensity`
- `mechanicalPattern`
- `metallicRatio`
- `fractalDensity`
- `bioluminescence`
- `shadowDepth`
- `redHighlight`
- `lightIntensity`
- `warmHue`

#### Prompt information

- `Prompt`
- `Negative Prompt`

数値は小数のまま保持してよい。
`Generated` は ISO 8601 UTC 文字列を標準とする。
`File Size` は viewer 表示を優先し、`"472.62 KB"` のような文字列で保持してよい。

長文である `Prompt` と `Negative Prompt` は、`attributes` に加えて top-level custom field にも同値を保持してよい。

### 7.3 推奨 JSON 形式

```json
{
  "name": "DOOM INDEX #1",
  "description": "AI-generated market artwork from DOOM INDEX.",
  "image": "https://cdn.example.com/doom-index/1.png",
  "animation_url": "https://cdn.example.com/doom-index/1.glb",
  "external_url": "https://doomindex.fun/artworks/1",
  "category": "vr",
  "attributes": [
    {
      "trait_type": "Generated",
      "value": "2026-03-12T03:00:00Z"
    },
    {
      "trait_type": "ID",
      "value": "DOOM_202603111800_c92dfdb3_0a44d00ee2c7"
    },
    {
      "trait_type": "Seed",
      "value": "0a44d00ee2c7"
    },
    {
      "trait_type": "Params Hash",
      "value": "c92dfdb3"
    },
    {
      "trait_type": "File Size",
      "value": "472.62 KB"
    },
    {
      "trait_type": "fogDensity",
      "value": 0.4745098039215686
    },
    {
      "trait_type": "skyTint",
      "value": 0.9254901960784314
    },
    {
      "trait_type": "reflectivity",
      "value": 0.1411764705882353
    },
    {
      "trait_type": "blueBalance",
      "value": 0.41568627450980394
    },
    {
      "trait_type": "vegetationDensity",
      "value": 0.4549019607843137
    },
    {
      "trait_type": "organicPattern",
      "value": 0.6549019607843137
    },
    {
      "trait_type": "radiationGlow",
      "value": 0.8470588235294118
    },
    {
      "trait_type": "debrisIntensity",
      "value": 0.6666666666666666
    },
    {
      "trait_type": "mechanicalPattern",
      "value": 0.9254901960784314
    },
    {
      "trait_type": "metallicRatio",
      "value": 0.8745098039215686
    },
    {
      "trait_type": "fractalDensity",
      "value": 1
    },
    {
      "trait_type": "bioluminescence",
      "value": 0.9764705882352941
    },
    {
      "trait_type": "shadowDepth",
      "value": 0.9725490196078431
    },
    {
      "trait_type": "redHighlight",
      "value": 0.6941176470588235
    },
    {
      "trait_type": "lightIntensity",
      "value": 0.3686274509803922
    },
    {
      "trait_type": "warmHue",
      "value": 0.6588235294117647
    },
    {
      "trait_type": "Prompt",
      "value": "Use the reference image as a mysterious ancient symbol reflected in the surface of a mystical pool at the foreground..."
    },
    {
      "trait_type": "Negative Prompt",
      "value": "watermark, text, oversaturated colors, low detail hands, extra limbs"
    }
  ],
  "doom_index": {
    "generated_at": "2026-03-12T03:00:00Z",
    "source_id": "DOOM_202603111800_c92dfdb3_0a44d00ee2c7",
    "seed": "0a44d00ee2c7",
    "params_hash": "c92dfdb3",
    "file_size_kb": 472.62,
    "prompt": "Use the reference image as a mysterious ancient symbol reflected in the surface of a mystical pool at the foreground. Ensure the token logo is subtly integrated into the classical oil painting composition without dominating the overall renaissance master style...",
    "negative_prompt": "watermark, text, oversaturated colors, low detail hands, extra limbs",
    "visual_parameters": {
      "fogDensity": 0.4745098039215686,
      "skyTint": 0.9254901960784314,
      "reflectivity": 0.1411764705882353,
      "blueBalance": 0.41568627450980394,
      "vegetationDensity": 0.4549019607843137,
      "organicPattern": 0.6549019607843137,
      "radiationGlow": 0.8470588235294118,
      "debrisIntensity": 0.6666666666666666,
      "mechanicalPattern": 0.9254901960784314,
      "metallicRatio": 0.8745098039215686,
      "fractalDensity": 1,
      "bioluminescence": 0.9764705882352941,
      "shadowDepth": 0.9725490196078431,
      "redHighlight": 0.6941176470588235,
      "lightIntensity": 0.3686274509803922,
      "warmHue": 0.6588235294117647
    }
  },
  "properties": {
    "files": [
      {
        "uri": "https://cdn.example.com/doom-index/1.png",
        "type": "image/png"
      },
      {
        "uri": "https://cdn.example.com/doom-index/1.glb",
        "type": "model/gltf-binary"
      }
    ]
  }
}
```

### 7.4 Media 要件

- thumbnail は PNG / JPEG のいずれかを標準とする
- 3D model は GLB を標準とする
- 将来的に glTF を扱う場合は MIME type を `model/gltf+json` とする
- metadata URI、image URI、animation_url URI は public HTTPS で参照可能であること
- devnet 検証時も mainnet を意識した URI 形式を用いること

### 7.5 Collection metadata

Collection 側の metadata も standard JSON 形式に合わせる。
最低限以下を持つこと。

- `name`
- `description`
- `image`
- `external_url`
- `category`

## 8. Mint フロー

1. 作品生成ワーカーが一定期間ごとに作品を生成する
2. user が `reserve_token_id` transaction を送信する
3. on-chain program が `next_token_id` を読み、`tokenId` を採番し、`MintReservation` を作成する
4. backend が reservation 済み `tokenId` に対して thumbnail、3D model、metadata JSON を生成し、`{base_metadata_url}/{tokenId}.json` に配置する
5. user が `mint_doom_index_nft` transaction を送信する
6. program が reservation を検証し、name と URI を決定する
7. program が Collection 配下で Metaplex Core Asset を mint する
8. program が reservation を使用済みに更新する
9. user は Asset Address を受け取り、wallet / explorer / Metaplex Core viewer で確認できる

## 9. Devnet 動作確認要件

### 9.1 必須確認項目

- upgradeable program を devnet に deploy できること
- `initialize_global_config` を devnet で実行できること
- `initialize_collection` を devnet で実行できること
- `reserve_token_id` を devnet で実行できること
- user wallet から `mint_doom_index_nft` を devnet で実行できること
- 発行後に Asset Address, owner, collection, name, uri を取得できること
- metadata JSON が取得できること
- thumbnail と 3D model の両方が取得できること
- 同じ reservation で 2 回 mint できないこと

### 9.2 推奨検証手段

- Solana Explorer の devnet
- Metaplex Core viewer
- script / frontend からの fetch 検証

### 9.3 実装に含めるべき補助物

devnet での確認容易性のため、実装時には以下を同梱する。

- devnet 用の初期化スクリプト
- devnet 用の mint スクリプトまたは UI
- Asset / Collection / metadata を検証する確認手順

## 10. 非機能要件

### 10.1 拡張性

- 生成期間は将来的に 10 分以外へ変更できること
- pricing, allowlist, per-wallet limit を将来的に追加できること
- metadata を per-token 固有化したまま運用拡張できること
- 将来的に asset immutability policy を強化できること
- Contract V1 から Contract V2 以降へ段階的に移行できること

### 10.2 保守性

- on-chain state は最小限に保つこと
- metadata 生成ロジックは off-chain に寄せること
- program upgrade で機能追加しやすいこと
- 単一 `next_token_id` カウンタによる直列化を許容し、Contract V1 は高頻度大量 mint を最適化対象としない

### 10.3 互換性

- Solana wallet と標準的な NFT viewer が理解できる metadata 形式を使うこと
- `image` を primary preview としつつ、`animation_url` で 3D model を提供すること

## 11. Open Items

以下は Contract V1 定義時点では未確定とする。

1. 生成期間の境界を UTC 基準にするか JST 基準にするか
2. storage を IPFS / Arweave / Irys / R2+CDN のどれにするか
3. metadata URI の immutability をどこまで強制するか
4. Asset / Collection の update authority を将来 `None` に落とすか
5. marketplace 表示上で `tokenId` をどこまで明示したいか
6. 未使用 reservation を将来 reclaim / cleanup するか

## 12. 参考

- Metaplex Core は off-chain metadata を参照する `name` + `uri` ベースで Asset を作成する
- Metaplex Core JSON Schema では `animation_url` に GLB を持てる
- `properties.files` には image と 3D model の両方を MIME type 付きで列挙できる
