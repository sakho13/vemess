# 開発フロー

1. [入出力系（I/O）](#1-入出力系io)
2. [分子・基底関数モデル](#2-分子・基底関数モデル)
3. [一電子積分エンジン](#3-一電子積分エンジン)
4. [二電子積分エンジン](#4-二電子積分エンジン)
5. [ハミルトニアン＆行列演算](#5-ハミルトニアン-行列演算)
6. [SCF ループ制御](#6-scf-ループ制御)
7. [ユーティリティ／共通機能](#7-ユーティリティ-共通機能)

---

## 1. 入出力系（I/O）

### 実装概要

- **InputParser**
  - 様々な入力フォーマット（XYZ, Gaussian, custom）の解析器を実装
  - 分子情報（原子リスト、座標、電荷、多重度、計算手法など）を内部データ構造へ変換
- **OutputWriter**
  - エネルギー、軌道エネルギー、密度行列、収束履歴などをファイル（テキスト・JSON）／標準出力へ出力

### 必要な情報

- サポートする入力フォーマットの仕様（ファイル例・フォーマット定義）
- 単位系（距離：Å vs Bohr、エネルギー：Hartree vs eV）の取り扱いルール
- 出力項目の仕様とフォーマット（項目名、桁数、ファイル拡張子）

---

## 2. 分子・基底関数モデル

### 実装概要

- **Molecule**
  - 原子番号、座標（Vec\<f64\>）、電荷、多重度を保持
  - 分子全体の情報アクセスメソッド（中心位置、全電荷チェックなど）
- **BasisSet**
  - STO-nG、Pople 系、cc-pVXZ などの基底関数パラメータ管理
  - 契約ガウス関数の展開係数とプリ指数（exponent）の読み込み・生成

### 必要な情報

- 各基底セットのパラメータデータ（契約係数・プリ指数リスト）
- データファイル形式（XML／JSON／自作テキスト）と読込方法
- データ検証やエラー処理ルール

---

## 3. 一電子積分エンジン

### 実装概要

- **OneElectronIntegrals** モジュールで以下を実装
  1. オーバーラップ積分 \(S*{\mu\nu} = \langle \chi*\mu|\chi\_\nu\rangle\)
  2. 運動エネルギー積分 \(T*{\mu\nu} = \langle \chi*\mu|-\tfrac12\nabla^2|\chi\_\nu\rangle\)
  3. 原子核引力積分 \(V*{\mu\nu} = \langle \chi*\mu|\sum*A -\tfrac{Z_A}{r*{A}}|\chi\_\nu\rangle\)

### 必要な情報

- ガウス型基底関数の解析積分公式
- 数値精度要件（tolerance）とスクリーニング条件
- テスト用ベンチマーク分子（H₂, He, H₂O など）

---

## 4. 二電子積分エンジン

### 実装概要

- **TwoElectronIntegrals** で四中心 ERI \((\mu\nu|\lambda\sigma)\) を計算
- スクリーニング（Schwarz 関係など）による計算削減
- ディスクキャッシュまたはオン-ザ-フライ生成の選択肢

### 必要な情報

- 解析積分アルゴリズム（Obara–Saika, McMurchie–Davidson など）
- スクリーニングしきい値のデフォルト値
- メモリ・ディスク I/O 制約（最大キャッシュサイズ）

---

## 5. ハミルトニアン＆行列演算

### 実装概要

- **CoreHamiltonian**: \(H*{\rm core} = T + V*{\rm nuc}\) の構築
- **FockBuilder**:
  - 二電子項から \(G\)-行列を構築
  - \(F = H\_{\rm core} + G\)
- **DensityMatrix**:
  - 軌道係数行列 \(C\) から密度行列 \(P\) を計算
- **Diagonalizer**:
  - Fock 行列の対角化ラッパー（LAPACK／nalgebra／eig2）

### 必要な情報

- 利用する線形代数ライブラリと API
- 行列サイズ（基底関数数）に応じたストレージ形式（フル／疎行列）
- 繰り返しごとのエネルギー評価式

---

## 6. SCF ループ制御

### 実装概要

- **SCFSolver**:
  1. 初期密度行列の生成（最適化済み Guess, Hückel 法など）
  2. 反復：
     - Fock 構築 → 対角化 → 密度行列更新 → エネルギー計算
  3. **DIIS** による収束加速
- 収束判定：エネルギー差 \(\Delta E\)、密度差 \(\|\Delta P\|\)

### 必要な情報

- DIIS サブスペースサイズとリミット
- 収束基準値（例：\(\Delta E<10^{-8}\) Hartree）
- 最大反復回数

---

## 7. ユーティリティ／共通機能

### 実装概要

- **LinearAlgebraWrapper**: BLAS/LAPACK 呼び出しの統一インターフェース
- **ParallelEngine**: Rayon 等による並列化設定
- **Logger & ErrorHandling**:
  - ログレベル（INFO/DEBUG/WARN/ERROR）
  - 計算ステップごとのトレース出力
  - 一貫した例外ハンドリング

### 必要な情報

- 並列度の設定方法（環境変数／API）
- ログ出力フォーマットと出力先
- エラーメッセージ・コード一覧
