# Focus Tracker

[English](#focus-tracker) | [中文](#focus-tracker-中文)

A macOS menubar app that automatically tracks your active windows, sessionizes focus data, and syncs it to a Second Brain backend. Built with **Tauri 2** and **React 19**.

Focus Tracker lives in your menu bar as a lightweight tray icon — no dock icon, no distractions. Click it to see today's working hours, focused time, and an 8-hour goal progress bar at a glance.

## Download

**[⬇️ Download focus-tracker_0.2.0_aarch64.dmg](https://github.com/zhousiyao03-cyber/focus-tracker/releases/latest/download/focus-tracker_0.2.0_aarch64.dmg)** (macOS, Apple Silicon)

After downloading:

1. Open the `.dmg` and drag **focus-tracker.app** into `/Applications`.
2. Launch it; grant **Accessibility** and **Automation** permissions when macOS prompts.
3. Click the tray icon → **Sign in** with your [Knosi](https://www.knosi.xyz) account.

Not code-signed yet — on first launch, right-click the app and choose **Open** to bypass Gatekeeper. See all builds on the [Releases page](https://github.com/zhousiyao03-cyber/focus-tracker/releases).

## Features

- **Automatic window tracking** — reads the frontmost app and window title via macOS APIs
- **Browser URL enrichment** — captures the current URL from browsers via Accessibility APIs, extracts host, path, search query, and surface type
- **Smart sessionization** — debounces rapid switches (3s confirmation delay), filters noise (low-priority apps under 30s ignored), enforces 5s minimum session duration
- **Task group classification** — auto-classifies apps into categories: coding, design, meeting, communication, writing, research
- **Session merging** — adjacent sessions from the same app (or same task group like VS Code + Terminal) within 2 min are merged
- **Idle & away detection** — detects system idle time, screen lock, and screensaver
- **Local persistence** — sessions are stored as JSON in the app data directory with dual queues (pending upload + recent history)
- **Background sync** — decoupled sampling (every 5s) and batch upload (every 120s) to `/api/focus/ingest`
- **Server reconciliation** — periodically pulls canonical day data from `/api/focus/status` to keep the display consistent
- **Device pairing** — secure code-based pairing flow via the web app (no shared API keys)
- **Menubar summary** — tray title shows `Working Hours · 8h progress` without opening the panel
- **Standard popover behavior** — screen-edge clamping, auto-hide on focus loss, Esc to dismiss

## Prerequisites

- macOS (required — uses AppleScript, Core Graphics, Accessibility APIs)
- [Rust toolchain](https://rustup.rs/)
- Xcode Command Line Tools
- [Node.js](https://nodejs.org/) (>= 18) and [pnpm](https://pnpm.io/)
- A running Second Brain web app instance (for sync features)

## Getting Started

```bash
# Install frontend dependencies
pnpm install

# Run in development mode (Rust + React)
pnpm tauri dev --no-watch

# Build frontend only
pnpm build
```

### Rust-only development

```bash
cd src-tauri
cargo check    # Type-check without building
cargo test     # Run tests
```

### Environment Variables

| Variable | Default | Description |
|---|---|---|
| `FOCUS_COLLECTOR_SAMPLE_INTERVAL_SECS` | `5` | Seconds between focus samples |
| `FOCUS_COLLECTOR_UPLOAD_INTERVAL_SECS` | `120` | Seconds between batch uploads |
| `FOCUS_TRACKER_START_VISIBLE` | `false` | Set to `true` to show panel on launch |

## Device Pairing

Focus Tracker uses a secure pairing flow instead of shared API keys:

1. Open `/focus` on the Second Brain web app
2. Click **Generate pairing code** in the "Desktop access" section
3. Paste the code into the desktop app's **Fix setup** screen
4. The app exchanges the code for a device token and saves it locally

If the token is revoked or expired, the app will prompt you to re-pair.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  React Panel (src/App.tsx)                              │
│  Polls get_status via Tauri IPC every 5s                │
└──────────────────────┬──────────────────────────────────┘
                       │ invoke()
┌──────────────────────▼──────────────────────────────────┐
│  Tauri Commands (lib.rs)                                │
│  get_status, pair_device, update_settings, ...          │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│  Background Loop (every sample_interval_secs)           │
│                                                         │
│  tracker.rs ──► sessionizer.rs ──► outbox.rs            │
│  (sample)       (debounce/filter)   (persist)           │
│                                                         │
│  outbox.rs ──► uploader.rs ──► Server                   │
│  (queued)      (batch POST)    /api/focus/ingest        │
│                                                         │
│  status_sync.rs ◄── Server                              │
│  (pull /api/focus/status every 30s)                     │
└─────────────────────────────────────────────────────────┘
```

### Key Modules

| File | Responsibility |
|---|---|
| `src-tauri/src/tracker.rs` | Reads frontmost app/window via `osascript`, detects idle/away |
| `src-tauri/src/accessibility.rs` | Reads browser URL via macOS Accessibility API |
| `src-tauri/src/browser_semantics.rs` | Extracts structured data from browser URLs |
| `src-tauri/src/window_list.rs` | Gets visible windows via Core Graphics |
| `src-tauri/src/sessionizer.rs` | Converts samples into sessions with debounce and filtering |
| `src-tauri/src/outbox.rs` | Local JSON persistence, deduplication, task group classification |
| `src-tauri/src/uploader.rs` | Batch uploads to `/api/focus/ingest` with bearer token auth |
| `src-tauri/src/status_sync.rs` | Pulls canonical day status from the server |
| `src-tauri/src/pairing.rs` | Device pairing code exchange flow |
| `src-tauri/src/state.rs` | Shared runtime state (`SharedState`), settings persistence |
| `src-tauri/src/error_state.rs` | Normalizes errors into user-facing messages |
| `src-tauri/src/lib.rs` | App setup, tray icon, popover, background loop, IPC commands |
| `src/App.tsx` | Panel UI — hours, timeline, progress, setup flow |

## Tech Stack

- **Tauri 2** — native macOS app shell with Rust backend
- **React 19** — panel UI
- **Vite 7** — frontend build
- **chrono** / **chrono-tz** — timezone-aware time handling
- **reqwest** — HTTP client (rustls-tls)
- **uuid** — session identifiers

## License

Private project. All rights reserved.

---

# Focus Tracker (中文)

[English](#focus-tracker) | [中文](#focus-tracker-中文)

一个 macOS 菜单栏应用，自动追踪你当前聚焦的窗口，将焦点数据会话化，并同步到 Second Brain 后端。基于 **Tauri 2** 和 **React 19** 构建。

Focus Tracker 作为轻量级托盘图标驻留在菜单栏中——没有 Dock 图标，没有干扰。点击即可查看今日工作时长、专注时间和 8 小时目标进度。

## 下载

**[⬇️ 下载 focus-tracker_0.2.0_aarch64.dmg](https://github.com/zhousiyao03-cyber/focus-tracker/releases/latest/download/focus-tracker_0.2.0_aarch64.dmg)**（macOS，Apple Silicon）

下载后：

1. 打开 `.dmg` 并将 **focus-tracker.app** 拖入 `/Applications`
2. 启动应用；macOS 弹框时授予**辅助功能**和**自动化**权限
3. 点击托盘图标 → **Sign in**，用你的 [Knosi](https://www.knosi.xyz) 账号登录

当前未做代码签名 —— 首次启动如被 Gatekeeper 拦截，右键点应用选 **Open** 即可。完整历史版本见 [Releases 页面](https://github.com/zhousiyao03-cyber/focus-tracker/releases)。

## 功能特性

- **自动窗口追踪** — 通过 macOS API 读取当前前台应用和窗口标题
- **浏览器 URL 采集** — 通过辅助功能 API 获取浏览器当前 URL，提取域名、路径、搜索词和页面类型
- **智能会话化** — 快速切换防抖（3 秒确认延迟）、噪声过滤（低优先级应用 30 秒内忽略）、最小 5 秒会话时长
- **任务分组分类** — 自动将应用归类为：编码、设计、会议、沟通、写作、研究
- **会话合并** — 相同应用（或相同任务组如 VS Code + Terminal）在 2 分钟内的相邻会话自动合并
- **空闲与离开检测** — 检测系统空闲时间、锁屏和屏保
- **本地持久化** — 会话以 JSON 格式存储在应用数据目录，双队列管理（待上传 + 近期历史）
- **后台同步** — 采样（每 5 秒）与批量上传（每 120 秒）解耦
- **服务端对账** — 周期性拉取服务端权威日数据，保持显示一致
- **设备配对** — 通过 Web 端的安全配对码流程（无需共享 API 密钥）
- **菜单栏摘要** — 托盘标题直接显示 `Working Hours · 8h progress`，无需打开面板
- **标准弹出行为** — 屏幕边缘自适应、失焦自动隐藏、Esc 键收起

## 前置要求

- macOS（必需——依赖 AppleScript、Core Graphics、辅助功能 API）
- [Rust 工具链](https://rustup.rs/)
- Xcode Command Line Tools
- [Node.js](https://nodejs.org/)（>= 18）和 [pnpm](https://pnpm.io/)
- 运行中的 Second Brain Web 应用实例（用于同步功能）

## 快速开始

```bash
# 安装前端依赖
pnpm install

# 开发模式运行（Rust + React）
pnpm tauri dev --no-watch

# 仅构建前端
pnpm build
```

### 仅 Rust 开发

```bash
cd src-tauri
cargo check    # 类型检查
cargo test     # 运行测试
```

### 环境变量

| 变量 | 默认值 | 说明 |
|---|---|---|
| `FOCUS_COLLECTOR_SAMPLE_INTERVAL_SECS` | `5` | 采样间隔（秒） |
| `FOCUS_COLLECTOR_UPLOAD_INTERVAL_SECS` | `120` | 批量上传间隔（秒） |
| `FOCUS_TRACKER_START_VISIBLE` | `false` | 设为 `true` 可在启动时显示面板 |

## 设备配对

Focus Tracker 使用安全配对流程，而非共享 API 密钥：

1. 在 Second Brain Web 端打开 `/focus`
2. 在「Desktop access」区块点击 **Generate pairing code**
3. 将配对码粘贴到桌面端的 **Fix setup** 界面
4. 应用会自动将配对码交换为设备令牌并保存到本地

如果令牌被撤销或过期，应用会提示重新配对。

## 架构概览

```
┌─────────────────────────────────────────────────────────┐
│  React 面板 (src/App.tsx)                                │
│  每 5 秒通过 Tauri IPC 轮询 get_status                    │
└──────────────────────┬──────────────────────────────────┘
                       │ invoke()
┌──────────────────────▼──────────────────────────────────┐
│  Tauri 命令 (lib.rs)                                     │
│  get_status, pair_device, update_settings, ...           │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│  后台循环 (每 sample_interval_secs 执行一次)                │
│                                                         │
│  tracker.rs ──► sessionizer.rs ──► outbox.rs            │
│  (采样)         (防抖/过滤)         (持久化)               │
│                                                         │
│  outbox.rs ──► uploader.rs ──► 服务端                     │
│  (待上传队列)    (批量 POST)    /api/focus/ingest          │
│                                                         │
│  status_sync.rs ◄── 服务端                                │
│  (每 30 秒拉取 /api/focus/status)                         │
└─────────────────────────────────────────────────────────┘
```

### 核心模块

| 文件 | 职责 |
|---|---|
| `src-tauri/src/tracker.rs` | 通过 `osascript` 读取前台应用/窗口，检测空闲/离开 |
| `src-tauri/src/accessibility.rs` | 通过 macOS 辅助功能 API 读取浏览器 URL |
| `src-tauri/src/browser_semantics.rs` | 从浏览器 URL 提取结构化数据 |
| `src-tauri/src/window_list.rs` | 通过 Core Graphics 获取可见窗口列表 |
| `src-tauri/src/sessionizer.rs` | 将采样转换为会话，包含防抖和过滤逻辑 |
| `src-tauri/src/outbox.rs` | 本地 JSON 持久化、去重、任务分组分类 |
| `src-tauri/src/uploader.rs` | 批量上传到 `/api/focus/ingest`（bearer token 认证） |
| `src-tauri/src/status_sync.rs` | 从服务端拉取权威日状态 |
| `src-tauri/src/pairing.rs` | 设备配对码交换流程 |
| `src-tauri/src/state.rs` | 共享运行时状态（`SharedState`）、设置持久化 |
| `src-tauri/src/error_state.rs` | 将错误标准化为用户可读信息 |
| `src-tauri/src/lib.rs` | 应用初始化、托盘图标、弹出窗口、后台循环、IPC 命令 |
| `src/App.tsx` | 面板 UI — 时长、时间线、进度、配置流程 |

## 技术栈

- **Tauri 2** — 原生 macOS 应用壳，Rust 后端
- **React 19** — 面板 UI
- **Vite 7** — 前端构建
- **chrono** / **chrono-tz** — 时区感知的时间处理
- **reqwest** — HTTP 客户端（rustls-tls）
- **uuid** — 会话标识符

## 许可

私有项目，保留所有权利。
