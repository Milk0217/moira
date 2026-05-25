# Moira — 中国传统星命学应用

> **七政四馀，二十八宿，古法星盘，尽在指尖。**

Moira（Μοῖραι — 希腊命运三女神之名）是一款开源的**中国传统占星学桌面应用**，基于 JPL DE440 高精度星历数据，提供完整的七政四馀星盘计算、四柱八字排盘、神煞推演与星力分析。面向中国传统命理学研究者、天文爱好者与命理软件开发者的深度探索工具。

中文名暂定：**默伊拉**

---

## 功能一览

### ✦ 七政四馀星盘

| 模块 | 说明 |
|------|------|
| **七政** | 太阳、太阴、水星、金星、火星、木星、土星（含天王/海王/冥王） |
| **四馀** | 罗睺、计都、月孛、紫炁（中国传统隐曜） |
| **二十八宿** | 角亢氐房心尾箕…全宿度计算，按四象着色 |
| **十二宫** | 等宫制，显示每宫宿度 |
| **相位** | 合/六分/四分/三分/对分等，支持自定义容许度 |
| **星制切换** | 回归制（Tropical）⇄ 恒星制（Sidereal），实时切换对比 |

### ✦ 四柱八字

- **八字排盘**: 年、月、日、时四柱，含天干地支、纳音五行
- **大运**: 阳男阴女顺行/阴男阳女逆行，节炁起运，自动计算起运年龄
- **十二长生**: 每个天干的十二长生阶段
- **藏干**: 地支藏干解析
- **十神**: 正官、偏印、伤官等十神关系

### ✦ 神煞系统

- **年神煞** (7+): 太岁、天乙贵人、禄神、羊刃、驿马、桃花、华盖等
- **日干神煞** (5-6): 天德、月德、文昌、红艳、天赦等
- **时神煞**: 青龙、朱雀等十二时辰神煞
- 支持自定义启用/禁用

### ✦ 安身立命

- 命宫主（命度主）、身宫主
- 星躔喜忌格（Xijige）
- 十干化曜（Shiganhuayao）

### ✦ 流限系统

- **小限**: 岁数流年地支
- **月限**: 小限+月份
- **洞微飞限**: 12 年周期大限系统

### ✦ 星力规则引擎

- **升殿 / 入垣 / 失躔 / 失垣 / 季星**: 五重星力评估
- 五行生克加成/惩罚
- 宫神煞分析（白虎、丧门、官符、岁破、病符等）

### ✦ 天星择日

选择吉日良辰的天星择日功能（Electional astrology）

### ✦ 日食月食搜索

基于几何方法的日月食搜索

### ✦ 批量命令行

内置 CLI，支持命令行快速排盘输出

---

## 技术架构

```
moira/
├── moira-rs/                    # Rust Cargo workspace
│   ├── crates/moira-core/       # 共享计算引擎库
│   │   └── src/
│   │       ├── lib.rs           # 核心入口：星体、宫位、相位、宿度
│   │       ├── angles.rs        # 天球角度（Ascendant、MC、Part of Fortune）
│   │       ├── bazi.rs          # 四柱八字、大运、十神、纳音
│   │       ├── eclipse.rs       # 日月食搜索
│   │       ├── lunar.rs         # 农历转换（朔望月、干支纪年）
│   │       ├── rule_engine.rs   # 星力规则引擎（升殿入垣等）
│   │       ├── shensha.rs       # 神煞（年、日干、时辰）
│   │       ├── solar_terms.rs   # 二十四节炁计算
│   │       └── utils.rs         # 工具函数
│   ├── src/main.rs              # CLI 入口
│   └── assets/bsp/de440.bsp     # JPL DE440 星历数据（symlink）
└── apps/desktop/                # Tauri v2 桌面应用
    ├── src/                     # React 前端
    │   ├── App.tsx              # 主应用（三区布局）
    │   ├── theme.ts             # 暗色主题系统
    │   ├── types/chart.ts       # TypeScript 类型定义
    │   └── components/          # UI 组件
    │       ├── AstrologyChart.tsx   # 星盘 Canvas 渲染（react-konva）
    │       ├── BaziPanel.tsx        # 八字面板
    │       ├── PlanetTable.tsx      # 星曜数据表
    │       ├── InputForm.tsx        # 输入表单
    │       ├── ElectionalForm.tsx   # 择日表单
    │       ├── ChartManager.tsx     # 星盘管理（保存/加载）
    │       ├── ErrorBoundary.tsx    # 错误边界
    │       └── shared.tsx           # 共享组件
    └── src-tauri/               # Tauri Rust 层
        ├── src/lib.rs           # Tauri commands
        └── tauri.conf.json
```

### 核心技术栈

| 层 | 技术 |
|----|------|
| 桌面框架 | **Tauri v2** — 轻量、安全、跨平台 |
| 前端 | **React 18 + TypeScript + Vite** |
| 星盘渲染 | **react-konva** (Canvas) |
| 天文计算 | **moira-core** (Rust 库) |
| 星历数据 | **JPL DE440** (anise 库) |
| Rust 版本 | **1.95** (edition 2024) |
| 版本控制 | jj (Jujutsu) / Git |

---

## 快速开始

### 前置依赖

- [Rust](https://rustup.rs/) 1.95+ (edition 2024)
- [Node.js](https://nodejs.org/) >= 18
- Tauri v2 系统依赖（[参考文档](https://v2.tauri.app/start/prerequisites/)）

### 安装与运行

```bash
# 克隆仓库
git clone https://github.com/Milk0217/moira.git
cd moira

# 获取星历数据（~40MB，存放于 assets/bsp/）
# 需要下载 JPL DE440 BSP 文件：
# https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de440.bsp

# 启动桌面应用
cd apps/desktop && npm install && npx tauri dev

# 或运行 CLI
cd moira-rs && cargo run
```

### 测试

```bash
# 后端测试（19+ test cases）
cd moira-rs && cargo test -p moira-core

# 前端测试
cd apps/desktop && npx vitest run
```

### 构建发布版本

```bash
cd apps/desktop && npx tauri build
```

---

## 领域术语说明

本项目严格使用中国传统占星学领域语言，详见 [CONTEXT.md](./CONTEXT.md)。核心约定：

- **七政** → 太阳、太阴（非"月亮"）、水星、金星、火星、木星、土星
- **四馀** → 罗睺、计都、月孛、紫炁
- **二十八宿** → 角亢氐房心尾箕… 四象划分
- **星制** → 回归制（Tropical）与恒星制（Sidereal）双支持
- **神煞** → 吉神/凶神/中性，分类推演
- **大运** → 节炁起运，十年一运
- **流限** → 小限/月限/洞微飞限

避免使用西方占星术语（星座名用中国传统十二宫名称）。

---

## 项目状态

该项目处于 **早期开发阶段**。当前版本已实现完整的七政四馀星盘计算、八字排盘、神煞系统等核心功能，正在进行界面优化和功能扩展。

### 开发路线

- [x] JPL DE440 星历集成
- [x] 七政四馀星体位置计算（黄道/赤道）
- [x] 二十八宿度与四象着色
- [x] 回归制 / 恒星制双模式
- [x] 等宫制十二宫
- [x] 相位计算（含自定义容许度）
- [x] 四柱八字 + 大运 + 纳音
- [x] 神煞系统（年/日干/时辰）
- [x] 安身立命与度主星
- [x] 星力规则引擎（升殿入垣等）
- [x] 流限系统（小限/月限/洞微飞限）
- [x] 宫神煞分析
- [x] 天星择日
- [x] 日食月食搜索
- [x] 暗色主题星盘 Canvas 渲染
- [ ] 十二宫宫神煞标注
- [ ] 多语言支持（中/英）
- [ ] 命盘报告导出（PDF）
- [ ] 比较盘（合盘）功能
- [ ] 流年星盘叠加
- [ ] macOS / Windows / Linux 正式发布

---

## 贡献指南

**欢迎任何人加入 Moira 的开发！** 🙌

无论你是：
- 🦀 Rust 开发者——改进天文计算精度、优化性能
- ⚛️ React 前端开发者——美化星盘渲染、完善交互体验
- 📚 中国传统文化研究者——校对领域逻辑、补充神煞定义
- 🐛 Bug 发现者——提交 Issue 或 PR

都欢迎贡献代码、文档、想法！

### 如何参与

1. **Fork 本仓库**，创建你的功能分支
2. **提交代码**，保持代码风格一致（Rust edition 2024，TypeScript strict 模式）
3. **确保测试通过**：`cargo test` + `npx vitest run`
4. **发起 Pull Request**，描述你的改动

### 开发约定

- 使用 `CONTEXT.md` 定义的领域术语
- Rust 用 `Result<T, String>` 而非 unwrap
- TypeScript strict 模式，`noUnusedLocals` + `noUnusedParameters`
- 提交信息用中文或英文，保持清晰

---

## 许可证

MIT License

---

## 致谢

- [JPL NAIF](https://naif.jpl.nasa.gov/naif/) — DE440 高精度星历
- [anise](https://github.com/nyx-space/anise) — Rust 天文计算库
- [Tauri](https://v2.tauri.app/) — 跨平台桌面框架
- [react-konva](https://konvajs.org/docs/react/) — Canvas 渲染引擎

---

<p align="center"><b>Moira</b> — 让星辰讲述你的故事。</p>
