# AGENTS.md

日本語で回答してください。

## 開発の進め方

本リポジトリは量子化学計算アプリケーションのモノレポ構成です。以下のディレクトリで開発を行います。

- `electron-ui/` : Electron + React (TypeScript) を用いたフロントエンド
- `rust-core/` : 計算コアロジックを実装する Rust 製 CLI バイナリ

### セットアップ手順

1. Node.js と Rust のコンパイラをインストールしてください。
2. 依存関係をインストールします。
   ```bash
   npm run install:ui
   ```
3. Rust バイナリをビルドします。
   ```bash
   npm run build:rust
   ```
4. Electron アプリを開発モードで起動します。
   ```bash
   npm run dev:ui
   ```
   もしくは全体をまとめて動かす場合は `npm run dev` を使用します。

### ビルド

- すべての成果物を生成する場合は次を実行します。
  ```bash
  npm run build
  ```

### コーディング規約

- フロントエンドは TypeScript、バックエンドは Rust を使用します。
- ディレクトリ名は英語の複数形で統一してください。
- 主要なクラス・関数には Rust では `///`、TypeScript では `/** ... */` 形式でドキュメントコメントを記述します。
- Lint は `npm run lint` (electron-ui 配下) を基準とします。

### Git 運用

- ブランチ名は `(issue番号)-(type)/(内容)` とします。
  - 例: `42-feature/add-parser`
- コミットメッセージは GitMoji を用いて以下の形式で記述します。
  ```
  :emoji: 説明
  ```
  - 例: `✨ 新しい積分エンジンを追加`
- プルリクエストのタイトルも日本語で簡潔にまとめてください。

## 参考

- レポジトリ構成やビルドフローの詳細は `README.md` を参照してください。
