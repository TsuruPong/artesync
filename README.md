# 🌀 ArteSync

<p align="left">
  <img src="https://img.shields.io/npm/v/artesync" alt="NPM Version" />
  <img src="https://img.shields.io/github/actions/workflow/status/TsuruPong/artesync/release.yml" alt="Build Status" />
  <img src="https://img.shields.io/badge/Rust-High%20Performance-orange" alt="Built with Rust" />
  <img src="https://img.shields.io/badge/License-MIT-blue" alt="License" />
</p>

## What is ArteSync?

**ArteSync** is a blazingly fast CLI package manager built in Rust, designed specifically to synchronize **Agent Skills** (workflows, contextual rules, and specialized prompts for AI coding assistants like Claude, Cursor, or Gemini) across multiple projects and repositories.

## ✨ Key Features

- **🚀 Zero NodeJS Dependency**: Installable via NPM, but runs natively as a compiled Rust executable.
- **🔒 Deterministic Lockfiles**: Guarantees identical skill versions for everyone on the project using `skills.arsync.lock`.
- **⚡ Ultra-Fast Git Caching**: Fetches repositories into a global bare cache (`~/.arsync/cache`), making subsequent installs nearly instant via sparse-checkout and worktrees.
- **🛡️ Agent Skills Validation**: Automatically parses `SKILL.md` frontmatter (`name`, `description`) to ensure compliance with Agent Skills specifications, issuing helpful warnings for misconfigured skills.
- **💡 Smart Updates**: Uses `git ls-remote` to check remote hashes _before_ fetching, ensuring zero disk I/O if your skills are already up to date.
- **🎨 Interactive Initialization**: Modern, `npm init`-style interactive prompts to set up your project's manifest.

## 📦 Installation

You can install ArteSync globally via `npm` (utilizing seamless optional dependencies for Windows, Mac, and Linux native binaries).

```bash
npm install -g artesync
```

_Or, if you prefer the Rust ecosystem:_

```bash
cargo install --git https://github.com/TsuruPong/artesync artesync
```

## 🏎️ Quick Start

### 1. Initialize the Manifest

Run the following in the root of your project:

```bash
arsync init
```

This will launch an interactive prompt to set your `name` and `description`, creating a `skills.arsync` JSON manifest.

### 2. Configure Your Source Directory (Optional)

Open `skills.arsync` and tell ArteSync where to put your synchronized skills by setting `"install-dir"`.

- If `"install-dir"` is not specified, skills will be installed in the same directory as the `skills.arsync` manifest file.
- If you specify a directory (e.g., `.gemini/antigravity/skills`), ArteSync will automatically create those folders and place all downloaded skills inside them.

```json
{
	"name": "my-project",
	"description": "My awesome AI-powered project",
	"install-dir": ".gemini/antigravity/skills",
	"dependencies": {}
}
```

### 3. Install a Skill

Point ArteSync to any centralized Git repository containing agent skills:

```bash
arsync install anthropics/skills/skills/skill-creator#main
```

ArteSync will fetch the code, place it in `.gemini/antigravity/skills/skill-creator`, update your `skills.arsync` manifest, and generate a `skills.arsync.lock` file locking the commit hash.

## 🧰 Commands Reference

- **`arsync init`**: Creates a new `skills.arsync` manifest interactively.
- **`arsync install <source>`**: Fetches the specified Git source, copies the files, and updates both the manifest and the lockfile. _Source format: `owner/repo/path/to/folder#branch`_
  - **Explicit Flags**: You can override or explicitly define parts of the URL using flags:
    - `--owner <NAME>`, `--repository <NAME>`, `--path <PATH>`
    - `--branch <NAME>` (Mutually exclusive with `--tag`)
    - `--tag <NAME>`
  - _Example:_ `arsync install --owner anthropics --repository skills --path skills/skill-creator --branch main`
- **`arsync install`**: (No arguments) Reads the `skills.arsync` manifest and `skills.arsync.lock`. Performs a **hard checkout** to the exact commit hashes specified in the lockfile to perfectly restore your environment.
- **`arsync update`**: Checks remote origins for updates via `ls-remote`. If a newer commit exists, it fetches the changes, updates the skills, runs soft validation, and rewrites the lockfile hash.
- **`arsync list`**: Displays all currently installed skills.
- **`arsync uninstall <skill>`**: Completely removes the skill folder from your filesystem, manifest, and lockfile.

## ⚙️ Configuration

### Manifest (`skills.arsync`)

Your declarative truth for what agent skills your project needs.

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

### Lockfile (`skills.arsync.lock`)

Automatically generated mapping of skills to specific Git Commit Hashes. **Commit this to version control.**

```json
{
	"dependencies": {
		"skill-creator": "a1b2c3d4e5f6g7h8i9j0"
	}
}
```

---

_Created with ❤️ by the ArteSync Team. Licensed under the MIT License._
