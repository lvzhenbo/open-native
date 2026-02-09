# open-native

一个使用 Rust 编写的高性能跨平台 Node.js 原生模块，用于打开文件、文件夹或在文件管理器中显示文件。

## 特性

- 🚀 **高性能** - 使用 Rust 和 [napi-rs](https://napi.rs/) 构建，无需启动子进程
- 🖥️ **跨平台** - 支持 Windows、macOS 和 Linux
- 📦 **零依赖** - 纯原生实现，无需安装额外依赖
- 🔒 **类型安全** - 完整的 TypeScript 类型定义

## 支持的平台

| 平台    | 架构       |
| ------- | ---------- |
| Windows | x64        |
| macOS   | x64, ARM64 |
| Linux   | x64, ARM64 |

## 安装

```bash
# npm
npm install @lvzhenbo/open-native

# pnpm
pnpm add @lvzhenbo/open-native

# yarn
yarn add @lvzhenbo/open-native
```

> **要求:** Node.js >= 22.0.0

## API

### `open(path: string): void`

使用系统默认程序打开文件或文件夹。

- 如果是文件，使用系统默认程序打开
- 如果是文件夹，在文件管理器中打开

```js
import { open } from '@lvzhenbo/open-native'

// 打开 PDF 文件（使用默认 PDF 阅读器）
open('C:\\Users\\test\\document.pdf')

// 打开文件夹
open('C:\\Users\\test\\Downloads')
```

### `reveal(path: string): void`

在文件管理器中打开文件夹并选中指定文件。

- Windows: 使用 `explorer /select`
- macOS: 使用 `open -R`
- Linux: 使用 DBus 或 `xdg-open`

```js
import { reveal } from '@lvzhenbo/open-native'

// 在文件管理器中显示并选中文件
reveal('C:\\Users\\test\\document.pdf')
```

### `openWith(path: string, app: string): void`

使用指定程序打开文件。

- **Windows**: 可以是程序名（如 `notepad`）或完整路径
- **macOS**: 应用程序名称（如 `TextEdit`）或 bundle identifier
- **Linux**: 可执行文件名称（如 `gedit`）

```js
import { openWith } from '@lvzhenbo/open-native'

// 使用记事本打开文本文件
openWith('C:\\Users\\test\\note.txt', 'notepad')

// 使用 VS Code 打开
openWith('/path/to/file.js', 'code')
```

## 错误处理

如果传入的路径不存在，所有函数都会抛出错误：

```js
import { open } from '@lvzhenbo/open-native'

try {
  open('/non/existent/path')
} catch (error) {
  console.error(error.message) // "Path does not exist: /non/existent/path"
}
```

## 开发

### 环境要求

- [Rust](https://www.rust-lang.org/) 最新稳定版
- [Node.js](https://nodejs.org/) >= 22.0.0
- [pnpm](https://pnpm.io/)

### 构建

```bash
# 安装依赖
pnpm install

# 开发构建
pnpm build:debug

# 生产构建
pnpm build
```

### 测试

```bash
pnpm test
```

### 代码格式化

```bash
pnpm format
```

## License

[MIT](./LICENSE)
