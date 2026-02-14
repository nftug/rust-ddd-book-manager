# rust-ddd-book-manager

Rust で DDD（Domain-Driven Design） をもとにしてWeb API を実装するサンプルプロジェクトです。

書籍管理 API を例に、ドメインモデルの設計からアプリケーションサービス、永続化、HTTP API までを実装しています。

API は Axum、永続化は SeaORM、DB は PostgreSQL、認証は Keycloak（OIDC / JWT）を前提にしています。

## 構成（workspace）

このリポジトリは Cargo workspace で、以下の crate で構成されています。

- `domain/`: ドメインモデル（エンティティ・値オブジェクト・ドメインサービス・監査/認可など）
- `application/`: ユースケース（コマンド/クエリ・DTO・アプリケーションサービス）
- `infrastructure/`: DB 実装（Repository / QueryService）や設定、外部接続
- `api/`: HTTP API（Axum）、OpenAPI（debug ビルド時）
- `migration/`: SeaORM の migration

依存関係は概ね `api -> application -> domain`、`api -> infrastructure -> domain` の方向です。

## 前提

- Rust（Edition 2024 を使用）
- Docker / Docker Compose

任意（開発が楽になります）

- `cargo-make`: `Makefile.toml` のタスク実行に使用
- `cargo-watch`: `cargo make run` が内部で使います

インストール例：

```sh
cargo install cargo-make cargo-watch
```

## クイックスタート（推奨：cargo-make）

`Makefile.toml` にローカル向けの環境変数が定義されており、DB と Keycloak の起動 + migration 適用までまとめて実行できます。

```sh
cargo make run
```

- API: `http://localhost:8080/api`
- PostgreSQL: `localhost:5432`
- Keycloak: `http://localhost:8081`（admin / admin）

停止：

```sh
cargo make compose-down
```

### OpenAPI（debug ビルド時）

debug ビルド（通常の `cargo run` 相当）では API ドキュメントが有効です。

- ReDoc: `http://localhost:8080/api/doc`
- OpenAPI JSON: `http://localhost:8080/api/doc/openapi.json`

debug ビルドでは起動時に OpenAPI スキーマをリポジトリルートの [openapi.json](/openapi.json) に自動出力します。

※ release ビルドではドキュメントルートは無効です。

## 認証（Keycloak / Bearer Token）

JWT は Keycloak の JWKS で検証します（RS256）。`OIDC_AUTHORITY` は realm まで含めた URL を指定します。

- 例（ローカルデフォルト）: `http://localhost:8081/realms/master`

また、認証が必要な API では、トークンのクレームに `name` と `email` が含まれている必要があります（欠けていると 400 を返します）。

### 認証が「必須」のエンドポイント例

- `GET /api/users/me`
- `POST /api/books/`
- `PUT /api/books/{book_id}`
- `DELETE /api/books/{book_id}`
- `POST /api/books/{book_id}/checkouts`
- `POST /api/books/{book_id}/return`

### 認証が「任意」のエンドポイント例

- `GET /api/books/`
- `GET /api/books/{book_id}`

（Authorization ヘッダがあればユーザーを作成/取得して監査に利用します。無ければ匿名扱いです）

### curl 例

未認証（一覧取得）：

```sh
curl -sS "http://localhost:8080/api/books/?limit=20" | jq
```

認証あり（作成）：

```sh
curl -sS -X POST "http://localhost:8080/api/books/" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title":"DDD入門","isbn":"9780000000000"}'
```

## 環境変数

最低限、API 起動には以下が必要です（`cargo make run` の場合はローカル値が自動セットされます）。

- `PORT`
- `DATABASE_HOST`
- `DATABASE_PORT`
- `DATABASE_USERNAME`
- `DATABASE_PASSWORD`
- `DATABASE_NAME`
- `OIDC_AUTHORITY`
- `OIDC_CLIENT_ID`
- （任意）`OIDC_AUDIENCE`（設定すると `aud` 検証が有効になります）

## よく使う開発コマンド

- DB/Keycloak 起動：
  ```sh
  cargo make compose-up
  ```
- migration 適用：
  ```sh
  cargo make migrate
  ```
- entity 生成（SeaORM）：
  ```sh
  cargo make generate-entity
  ```
- ビルド：
  ```sh
  cargo make build
  ```
- Lint（clippy）：
  ```sh
  cargo make check
  ```
- テスト：
  ```sh
  cargo make test
  ```

## Migration について

`migration/` は SeaORM migration の crate です。詳細は `migration/README.md` を参照してください。

## ライセンス

[MIT License](LICENSE)
