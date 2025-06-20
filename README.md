# oxre ⚡

> Blazingly fast React project scaffolder written in Rust

[![Crates.io](https://img.shields.io/crates/v/oxre)](https://crates.io/crates/oxre)

**oxre** scaffolds React projects in **~0.33 seconds**. No network requests, no waiting.

## 🚀 Quick Start

```bash
# Install
cargo install oxre

# Create React app
oxre my-app

# Create TypeScript app  
oxre my-ts-app --typescript

# Start coding
cd my-app && npm install && npm run dev
```

## ⚡ Performance

| Tool | Time | Network | Size |
|------|------|---------|------|
| `vite create react` | ~15s | Required | ~50MB |
| **`oxre`** | **0.33s** | None | **2MB** |

*Tested on MacBook Air M4*

## 🛠️ What You Get

- React 18 + Vite 5
- TypeScript support (`--typescript`)
- ESLint configured
- Modern project structure
- Ready to run

## 🤔 Why I Built oxre

Tired of waiting for project scaffolding. Wanted something that:

- Works offline
- Takes < 1 second  
- Uses modern tools
- Just works

## 📦 Installation

```bash
cargo install oxre
```

## 📄 License

MIT - see [LICENSE](LICENSE)

**Made by [@ddoemonn](https://github.com/ddoemonn)**
