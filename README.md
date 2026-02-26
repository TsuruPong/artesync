# 🧹 ArteSync

<p align="left">
  <img src="https://img.shields.io/npm/v/artesync" alt="NPM Version" />
  <img src="https://img.shields.io/github/actions/workflow/status/TsuruPong/artesync/release.yml" alt="Build Status" />
  <img src="https://img.shields.io/badge/Rust-High%20Performance-orange" alt="Built with Rust" />
  <img src="https://img.shields.io/badge/License-MIT-blue" alt="License" />
</p>

> 🚧 **Notice**: Currently published as an experimental tool for testing purposes. Please note that backward-incompatible changes may occur without prior notice.

## What is ArteSync?

**ArteSync** is a blazingly fast CLI package manager built in Rust, designed specifically to synchronize **Agent Skills** for AI coding assistants like Claude, Cursor, or Gemini across multiple projects and repositories.

## 📦 Installation

You can install ArteSync globally via `npm`.

```bash
npm install -g artesync
```

## 🏎️ Quick Start

### 1. Initialize the Manifest

Run the following in the root of your project:

```bash
arsync init
```

This will launch an interactive prompt to set your `name`, `description`, and `install-dir`, creating a `skills.arsync` JSON manifest.

### 2. Configure Your Source Directory (Optional)

- If `"install-dir"` is not specified, skills will be installed in the same directory as the `skills.arsync` manifest file.
- If you specify a directory (e.g., `.gemini/antigravity/skills`), ArteSync will automatically create those folders and place all downloaded skills inside them.

### 3. Install a Skill

Point ArteSync to any centralized Git repository containing agent skills:

```bash
arsync install anthropics/skills/skills/skill-creator#main
```

ArteSync will fetch the code, place it in `.gemini/antigravity/skills/skill-creator`, update your `skills.arsync` manifest, and generate a `skills-lock.arsync` file locking the commit hash.

## 🧰 Commands Reference

- **`arsync init`**: Creates a new `skills.arsync` manifest interactively.
- **`arsync install <source>`**: Fetches the specified Git source, copies the files, and updates both the manifest and the lockfile. _Source format: `owner/repo/path/to/folder#branch`_
  - **Explicit Flags**: You can override or explicitly define parts of the URL using flags:
    - `--owner <NAME>`, `--repository <NAME>`, `--path <PATH>`
    - `--branch <NAME>` (Mutually exclusive with `--tag`)
    - `--tag <NAME>`
  - _Example:_ `arsync install --owner anthropics --repository skills --path skills/skill-creator --branch main`
- **`arsync install`**: (No arguments) Reads the `skills.arsync` manifest and `skills-lock.arsync`. Performs a **hard checkout** to the exact commit hashes specified in the lockfile to perfectly restore your environment.
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

### Lockfile (`skills-lock.arsync`)

Automatically generated mapping of skills to specific Git Commit Hashes. **Commit this to version control.**

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
