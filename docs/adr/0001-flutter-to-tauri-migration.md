# 0001: Flutter 迁移至 Tauri v2

Moira 初始前端选型为 Flutter 桌面端。经过早期开发后评估，决定迁移到 Tauri v2 + React + Vite 技术栈。

## 原因

Flutter 桌面端在当时是一个合理起点，但项目的中期目标——共享 Rust 代码派生的天文计算、更小的打包体积、基于 Web 技术栈的前端、以及桌面优先的跨平台策略——与 Tauri 的模型自然对齐。具体来说：

- Rust 后端 (`moira-core`) 可以直接嵌入 Tauri 的 native 层，通过 `tauri::command` 暴露给前端，无需 HTTP 通信
- Tauri 打包体积远小于 Flutter（不携带 Dart VM 和 Flutter 引擎）
- React + Vite 生态对 Canvas/图形绘制库（如 react-konva）支持更成熟
- 迁移时仅丢弃前端 UI 层，Rust 天文计算代码全部保留并重构为共享库

## 技术选型

| 层 | 选择 | 替代考虑 |
|----|------|---------|
| 桌面框架 | Tauri v2 | Tauri v1（无移动端可能） |
| 前端框架 | React | Svelte（更强"轻量"定位）、Vue |
| 构建工具 | Vite | Next.js（桌面场景过重）、CRA（已废弃） |
| 星盘渲染 | react-konva | Canvas 2D（更底层）、SVG（复杂径向布局不便） |
| 后端集成 | 嵌入式（Rust 库 → Tauri command） | HTTP 服务模式（需多进程部署） |

## 项目结构（实际）

```
moira/                              # 项目根（非 git 仓库）
├── moira-rs/                       # Cargo workspace root（git 仓库，原 moira-rs）
│   ├── Cargo.toml                  # workspace 清单
│   ├── src/main.rs                 # CLI 测试入口
│   ├── crates/moira-core/          # ← 天文计算共享库
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── assets/bsp/de440.bsp        # JPL 星历
├── apps/desktop/                   # Tauri 桌面应用（独立 npm 项目）
│   ├── src/                        # React 前端
│   │   ├── App.tsx
│   │   ├── components/
│   │   │   ├── InputForm.tsx
│   │   │   └── AstrologyChart.tsx
│   │   └── types/chart.ts
│   ├── src-tauri/                  # Tauri Rust 层
│   │   ├── Cargo.toml              # 依赖 moira-core = { path = "../../../moira-rs/crates/moira-core" }
│   │   ├── src/lib.rs              # tauri commands
│   │   └── tauri.conf.json
│   └── package.json
├── CONTEXT.md                      # 领域语言定义
├── docs/adr/                       # 架构决策记录
├── SPICE/                          # CSPICE 工具包
└── eph/                            # JPL 星历数据备份
```

## 与原始方案的差异

原始方案预计 `crates/moira-core/` 和 `apps/desktop/` 同级位于根目录。实际因保留 `moira-rs/` 的 git 历史作为 workspace root，`moira-core/` 位于 `moira-rs/crates/` 下。Tauri 通过相对路径引用。这增加了路径深度，但保留了完整的 git 历史。
