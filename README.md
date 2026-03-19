# 🚀 My Way CLI

[![Crates.io](https://img.shields.io/crates/v/myway-cli.svg)](https://crates.io/crates/myway-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**A minimalist project manager for your ideas. Built with Rust for speed and mental clarity.**

![banner](./assets/banner.png)


## Summary 

- [🚀 My Way CLI](#-my-way-cli)
  - [Summary](#summary)
  - [🧠 Why My Way?](#-why-my-way)
  - [🛠️ Commands \& Usage](#️-commands--usage)
    - [💡 Quick Examples](#-quick-examples)
  - [📂 Data Storage](#-data-storage)
  - [🚀 Future Roadmap (v0.2.0)](#-future-roadmap-v020)
  - [💻Installation](#installation)
  - [📄 License](#-license)


## 🧠 Why My Way?

Created by **Gabriel Xavier**, My Way was born from the need to organize an "over-mind" of projects. It's not just another CRUD; it's a curated space for your technical journey. 

> *If you have a project, finish it. If you can't, own your choice and give up.*

---

## 🛠️ Commands & Usage

| Command | Action | Key Flags |
| :--- | :--- | :--- |
| `create` | Spawns a new project | -- |
| `way` | Lists all active projects | `--uuid`, `--oneline`, `--complex` |
| `edit` | Modify name, description or tags | `--uuid` or `--name` |
| `version` | Manage project releases | `--add`, `--list` |
| `status` | Manage project status | `--uuid`, `--name` |
| `giveup` | Permanent delete (No graveyard) | `--uuid` or `--name` |
| `graveyard` | Manage your archivees | `--add`, `--list`, `--exject` |
| `finish` | Celebrate completion! | `--uuid` or `--name` |

### 💡 Quick Examples

```bash
# Adding a new project on WAY
mw add

# Adding a new version to a project
mw version --add --name your-project-name

# Looking for a specific project ID
mw way --uuid 8271c6f

# The "Exhumation" Option (Clean the graveyard)
mw graveyard --exject
```

## 📂 Data Storage

MyWay is **zero-cloud** and private by default. Your data is stored locally in:

- Windows: `%USERPROFILE%\Documents\MyWayCli`
- Linux/macOS: `~/Documents/MyWayCli`

## 🚀 Future Roadmap (v0.2.0)

- [ ] `mw sync`: Manual backup via Github Gist.
- [ ] `mw open`: Quick-open project folder in your favorite editor (VSCode, Zed, NVim).


## 💻Installation


1. Install with Cargo:
```
cargo install myway-cli
```

2. Install from **Releases**



## 📄 License

Licensed under the MIT License. Free to use, modify, and distribute.