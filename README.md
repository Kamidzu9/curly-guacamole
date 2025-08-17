# curly-guacamole

![GitHub repo size](https://img.shields.io/github/repo-size/Kamidzu9/curly-guacamole)
![GitHub license](https://img.shields.io/github/license/Kamidzu9/curly-guacamole)
![GitHub last commit](https://img.shields.io/github/last-commit/Kamidzu9/curly-guacamole)

**Curly Guacamole** is a fully scalable, cross-platform application using a Rust backend (via Tauri) and a modern React web frontend. It aims to provide a fast, lightweight, and portable experience for both desktop and mobile users.

---

## ✨ Features

- 🚀 **Fast & Lightweight:** Powered by Rust and Tauri
- 🖥️ **Cross-Platform:** Android, Linux, Windows, macOS
- ⚛️ **Modern Frontend:** React, Vite, TypeScript
- 📦 **Easy Packaging & Distribution**

---

## 🛠️ Tech Stack

- [Rust](https://www.rust-lang.org/) (backend, Tauri)
- [Tauri](https://tauri.app/) (cross-platform shell)
- [React](https://react.dev/) (frontend)
- [Vite](https://vitejs.dev/) (build tool)
- [TypeScript](https://www.typescriptlang.org/)

---

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (for frontend)
- [Rust](https://www.rust-lang.org/tools/install) (for backend)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Install dependencies

```bash
npm install
```

### Run in development mode

```bash
npm run tauri dev
```

Or, if you want to use the Rust CLI directly:

```bash
cargo tauri dev
```

### Build for production

```bash
npm run tauri build
```

Or, using Rust CLI:

```bash
cargo tauri build
```

---

## 📁 Project Structure

- `src/` – React frontend
- `src-tauri/` – Rust backend (Tauri)
- `public/` – Static assets

---

## 📄 License

This project is licensed under the **Apache 2.0 License**. See the [LICENSE](LICENSE) file for details.
