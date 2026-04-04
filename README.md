# Focus Tracker

> 基于 Tauri 2 + React 的桌面专注时间追踪器，常驻菜单栏，自动采集并同步工作数据到 Second Brain。

## 核心特性

### 智能采样与降噪

- macOS 下通过 `osascript` 读取前台 app / window title
- 新窗口需稳定 **10s** 才确认切换，避免误判
- 短于 **30s** 的 session 不单独入队
- `Finder / focus-tracker / Rize / 系统窗口` 等低权重 app 在 **< 2min** 时自动忽略

### Session 合并

- 相同 app、间隔 **< 2min** 的片段自动合并
- `VS Code / Terminal / Docs / Postman` 等 coding workflow 按任务组合并

### 数据同步

- 本地 JSON outbox 持久化，采样与上传解耦
- 默认 **5s** 采样、**120s** 批量上传
- 仅在 outbox 有待上传数据时触发上传
- 通过 bearer token 上传到 `/api/focus/ingest`
- 周期性拉取 `/api/focus/status`，面板数据优先对齐服务端
- 本地未上传 session 会叠加到远端快照上，避免显示倒退

### Idle 判定

- 默认 **30min** 无输入才视为 idle

### 菜单栏体验

- Tray title 直接显示 `Working Hours · 8h progress`，无需打开面板
- 点击 tray icon 弹出固定宽度 popover，自动屏幕边界 clamp
- 失焦自动隐藏，`Esc` 收起

### React 面板

- Working hours + Focused time + 8h progress
- Compact local timeline
- 快速跳转 `/focus` 页面
- Setup needed / Upload failed 时显示修复入口
- 支持 pairing code 配对

## 快速开始

### 前置依赖

- [Rust toolchain](https://rustup.rs/)
- Xcode Command Line Tools / Xcode
- Node.js + pnpm
- 一个正在运行的 Second Brain Web app

### 安装与运行

```bash
pnpm install
pnpm tauri dev --no-watch
```

### 构建

```bash
pnpm build
```

### 仅验证 Rust 侧逻辑

```bash
cd src-tauri
cargo test
cargo check
```

## 配对流程

1. 在 Web 端 `/focus` 点击 **Generate pairing code**
2. 在桌面端 **Fix setup** 里粘贴 code
3. Collector 自动换成正式 device token 并保存到本地

无需共享全局 ingest key，token 被 revoke 或过期时会提示重连。

## 环境变量

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `FOCUS_COLLECTOR_SAMPLE_INTERVAL_SECS` | `5` | 采样间隔（秒） |
| `FOCUS_COLLECTOR_UPLOAD_INTERVAL_SECS` | `120` | 上传间隔（秒） |
| `FOCUS_INGEST_API_KEY` | — | 服务端 ingest API key |
| `FOCUS_INGEST_USER_ID` | — | 服务端 user ID |

## 项目结构

```
src/
  App.tsx                  # React 控制面板
src-tauri/src/
  tracker.rs               # macOS 前台窗口采样
  sessionizer.rs           # Session merge / flush / 降噪
  outbox.rs                # 本地 JSON 持久化
  uploader.rs              # 上传 /api/focus/ingest
  state.rs                 # 共享运行时状态
  status_sync.rs           # 服务端日视图同步
  pairing.rs               # Pairing code 配对
  browser_semantics.rs     # 浏览器窗口语义解析
  window_list.rs           # 窗口列表
  accessibility.rs         # 辅助功能权限检测
  error_state.rs           # 错误状态管理
  lib.rs                   # Tauri 入口 / 命令注册
```
