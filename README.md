# VEMESS

Electron + React をフロントエンドに、Rust 製の並列バイナリをバックエンドに据えた量子化学計算アプリケーションのモノレポ構成と実行フローをまとめたリポジトリです。

## 📚 Table of Contents

- [Overview](#overview)
- [Directory Structure](#directory-structure)
- [Build & Development](#build--development)
- [Execution Flow](#execution-flow)
- [Key Points](#key-points)
- [License](#license)

## 🔍 Overview

- **UI**: Electron + React (WebView) でユーザー操作と可視化を実装
- **Controller**: Electron Main が Rust バイナリを `child_process.spawn()` で呼び出し
- **Calculation Core**: Rust の CLI バイナリ（`cargo new --bin rust-core`）でファイル I/O → 計算 → JSON 出力

この構成により、UI／コントローラ／計算コアを疎結合に保ちつつ、高速なネイティブ処理とリッチな可視化を両立します。

## 🗂 Directory Structure

```plaintext
vemess/
├─ package.json               # ワークスペース定義 (Yarn Workspaces / pnpm)
├─ yarn.lock / pnpm-lock.yaml
│
├─ rust-core/                 # Rust 計算エンジン
│  ├─ Cargo.toml              # バイナリクレート設定
│  └─ src/
│     ├─ main.rs              # CLI: --input 指示ファイル → 計算 → --output JSON
│     └─ lib.rs               # 計算ロジック（HF-SCF, AO 演算など）
│
├─ electron-ui/               # Electron アプリケーション
│  ├─ package.json            # Electron / React 依存定義
│  ├─ tsconfig.json
│  ├─ public/                 # 静的ファイル (index.html, CSS)
│  └─ src/
│     ├─ main.ts              # Electron Main: Rust バイナリ呼び出し & IPC
│     ├─ preload.ts           # contextBridge 経由で Renderer と橋渡し
│     └─ renderer/
│        ├─ index.tsx         # React エントリポイント
│        ├─ App.tsx           # UI: ファイル選択 → 結果可視化
│        └─ components/       # Three.js, Chart.js など可視化パーツ
│
└─ instruction-schema/        # （任意）入力 JSON スキーマ定義
   └─ schema.json
```

## 🛠 Build & Development

ルートの `package.json` で一括操作可能:

```jsonc
{
  "private": true,
  "workspaces": ["electron-ui"],
  "scripts": {
    "build:rust": "cd rust-core && cargo build --release",
    "build:ui": "cd electron-ui && yarn build",
    "build": "yarn build:rust && yarn build:ui",
    "dev:ui": "cd electron-ui && yarn dev",
    "dev": "yarn build:rust && yarn dev:ui",
    "dev:rust": "cd rust-core && cargo run"
  }
}
```

- **Rust**: `cd rust-core && cargo build --release` → `target/release/rust-core`
- **Electron**: `cd electron-ui && yarn dev` (開発)、`yarn build` (本番バンドル)
- **Packaging**: `electron-builder` 等で Rust バイナリをバンドル

## ▶️ Execution Flow

1. ユーザーが UI で指示ファイル（JSON）を選択
2. Renderer → Preload 経由で `ipcRenderer.invoke('run-calculation', filePath)`
3. Main (`main.ts`) で Rust バイナリを `spawn()`:

   ```ts
   const child = spawn(binaryPath, ["--input", filePath]);
   child.stdout.on("data", (data) => {
     /* JSON取得 */
   });
   child.on("close", () => {
     /* 結果を返却 */
   });
   ```

4. Renderer で受け取った JSON を Three.js / Chart.js で可視化

## ⚙️ Key Points

- **バイナリ呼び出し**: Node の `child_process` で純粋高速 Rust プロセスを利用
- **IPC 設計**: シンプルな `invoke('run-calculation', path)` / `handle('run-calculation', …)`
- **JSON スキーマ**: `instruction-schema/schema.json` + Rust (serde + schemars) で一貫性保証
- **並列処理**: Rust 側は `rayon` / `crossbeam` でマルチスレッド対応、CLI 引数でスレッド指定可能
- **パッケージサイズ**: Electron ＋ Rust バイナリで数百 MB に。軽量化検討（Tauri など）も可
