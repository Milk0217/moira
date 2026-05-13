# Moira — Agent Guide

中国传统占星学应用。Tauri v2 桌面应用，Rust 计算引擎 + React 前端。

## 快速命令

```bash
cd apps/desktop && npx tauri dev      # 启动开发模式
cd apps/desktop && npx tauri build    # 构建发布版本
cd moira-rs && cargo test -p moira-core # 运行后端测试
cd apps/desktop && npx vitest run     # 运行前端测试
```

## 项目结构

```
moira/
├── moira-rs/                    # Cargo workspace
│   ├── Cargo.toml               # workspace root
│   ├── src/main.rs              # CLI 测试入口
│   ├── crates/moira-core/       # 天文计算共享库
│   │   └── src/lib.rs           # 核心逻辑 (~660 行)
│   └── assets/bsp/de440.bsp     # JPL 星历（symlink 到 apps/desktop）
├── apps/desktop/                # Tauri v2 + React 桌面应用
│   ├── src/                     # React 前端
│   │   ├── App.tsx              # 主应用组件
│   │   ├── components/          # InputForm, AstrologyChart, PlanetTable, ErrorBoundary
│   │   └── types/chart.ts       # TypeScript 类型定义
│   └── src-tauri/               # Tauri Rust 层
│       ├── src/lib.rs           # tauri commands
│       └── tauri.conf.json
├── CONTEXT.md                   # 领域语言定义
├── AGENTS.md                    # ← 本文件
└── docs/
    ├── adr/                     # 架构决策记录
    └── agents/                  # Agent skills 配置
```

## 技术栈

| 层 | 技术 |
|----|------|
| 桌面框架 | Tauri v2 |
| 前端 | React 18 + TypeScript + Vite |
| 星盘渲染 | react-konva (Canvas) |
| 计算引擎 | Rust (moira-core) |
| 星历数据 | JPL DE440 (anise 库) |
| 版本控制 | jj (Jujutsu), 可推送到 GitHub |

## 关键约定

- **领域术语** 使用 `CONTEXT.md` 定义的词汇，不要用西方占星术语
- **Rust** edition 2024，所有错误用 `Result<T, String>` 而非 unwrap
- **TypeScript** strict 模式，`noUnusedLocals` + `noUnusedParameters`
- **测试** moira-core 有 19+ 单元测试；前端 vitest 测试
- **提交** 用 jj（`jj new` → `jj describe -m "..."`）

## Agent skills

### Issue tracker

Issues tracked as local markdown files under `.scratch/`. See `docs/agents/issue-tracker.md`.

### Triage labels

Uses default labels: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context repo. `CONTEXT.md` at root defines domain language. See `docs/agents/domain.md`.
