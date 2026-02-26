# 🧹 ArteSync

<p align="left">
  <img src="https://img.shields.io/npm/v/artesync" alt="NPM Version" />
  <img src="https://img.shields.io/github/actions/workflow/status/TsuruPong/artesync/release.yml" alt="Build Status" />
  <img src="https://img.shields.io/badge/Rust-High%20Performance-orange" alt="Built with Rust" />
  <img src="https://img.shields.io/badge/License-MIT-blue" alt="License" />
</p>

> 🚧 **Notice**: 現在、テスト用の実験的ツールとして公開しています。事前の予告なく、後方互換性のない変更が入る場合がありますのでご注意ください。

## ArteSync とは？

**ArteSync** はRustで構築された爆速のパッケージマネージャーCLIであり、Claude、Cursor、GeminiなどのAIコーディングアシスタント向けの **Agent Skills**を複数のプロジェクトやリポジトリ間で同期するために特化して設計されています。

## 📦 インストール方法

`npm` を使用してグローバルにインストールできます。

```bash
npm install -g artesync
```

## 🏎️ クイックスタート

### 1. マニフェストの初期化

プロジェクトのルートディレクトリで以下を実行します：

```bash
arsync init
```

対話型のプロンプトが起動し、`name` と `description`、`install-dir` を設定して `skills.arsync` JSON マニフェストファイルを作成します。

### 2. インストール先のディレクトリ設定（オプション）

- `"install-dir"` を指定しない場合、スキルは `skills.arsync` ファイルと同じディレクトリ（通常はプロジェクトルート）に直接インストールされます。
- 任意のディレクトリ（例: `.gemini/antigravity/skills`）を指定すると、ArteSyncがそのディレクトリを自動で作成し、すべてのスキルをその中に整理して配置します。

### 3. スキルのインストール

Agent Skills を含む中央Gitリポジトリのパスを指定します：

```bash
arsync install anthropics/skills/skills/skill-creator#main
```

ArteSyncは指定されたコミットを取得し、`.gemini/antigravity/skills/skill-creator` に配置したうえで、`skills.arsync` マニフェストを更新し、参照コミットのハッシュをロックするための `skills-lock.arsync` ファイルを生成します。

## 🧰 コマンドリファレンス

- **`arsync init`**: 新しい `skills.arsync` マニフェストを対話的に作成します。
- **`arsync install <source>`**: 指定された Git のソースをフェッチしてファイルをコピーし、マニフェストとロックファイルの両方を更新します。 _書式: `owner/repo/path/to/folder#branch`_
  - **明示的フラグ**: フラグを使用してURLの各部分を上書き・明示することも可能です。
    - `--owner <NAME>`, `--repository <NAME>`, `--path <PATH>`
    - `--branch <NAME>` (`--tag`と排他利用)
    - `--tag <NAME>`
  - _例:_ `arsync install --owner anthropics --repository skills --path skills/skill-creator --branch main`
- **`arsync install`**: (引数なし) `skills.arsync` マニフェストと `skills-lock.arsync` を読み込みます。ロックファイルに指定された正確なコミットハッシュに対して **ハードチェックアウト** を実行し、環境を完全に復元します。
- **`arsync update`**: `ls-remote` を経由してリモートリポジトリの更新を確認します。より新しいコミットが存在する場合、変更をフェッチしてスキルを更新し、検証を行い、ロックファイルのハッシュを書き換えます。
- **`arsync list`**: 現在インストールされているすべてのスキルを一覧表示します。
- **`arsync uninstall <skill>`**: スキルのフォルダをファイルシステムから完全に削除し、マニフェストとロックファイルからも削除します。

## ⚙️ 構成ファイル

### マニフェスト (`skills.arsync`)

プロジェクトに必要な Agent Skills を宣言的に定義・管理するマニフェストです。

```json
{
	"name": "my-project",
	"description": "Project description",
	"install-dir": ".gemini/antigravity/skills",
	"dependencies": {
		"skill-creator": "anthropics/skills/skills/skill-creator"
	}
}
```

### ロックファイル (`skills-lock.arsync`)

各スキルと特定の Git コミットハッシュの対応関係を自動生成します。**このファイルは必ずバージョン管理にコミットしてください。**

```json
{
	"dependencies": {
		"skill-creator": "a1b2c3d4e5f6g7h8i9j0"
	}
}
```

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
