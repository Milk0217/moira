# 0003: 同心环星盘架构与事件处理

## 状态

实施中

## 背景

星盘渲染经历了从混合式布局到清晰的同心环分层架构的重构。最初各环层（地支、长生、宫名、二十八宿、行星）的径向空间重叠混排，视觉层次不清晰。重构为同心环式分层后，每层为独立环带，层间有隔断环。

在重构过程中，发现 react-konva (Konva) 的事件处理有几个关键陷阱。

## 决策

### 1. 6 层同心环结构

从内到外：

| 层 | 名称 | 径向位置 | 内容 | 分割 |
|---|---|---|---|---|
| 中心 | — | 0 ~ 0.20R | 命度文字 | 无 |
| L1 | 地支 | 0.22R ~ 0.32R | 子丑寅卯... | 12 格 × 30° |
| L2 | 长生 | 0.34R ~ 0.44R | 长生沐浴冠带... | 12 格 × 30° |
| L3 | 洞微大限 | 0.46R ~ 0.56R | 命宫父母福德... | 12 格，按地支映射 |
| L4 | 宫号 | 0.58R ~ 0.68R | 1~12 (跟随 Asc 旋转) | 浮动位置 |
| L5 | 二十八宿 | 0.70R ~ 0.81R | 角亢氐房... | 28 格不等宽 |
| L6 | 行星层 | 0.83R ~ 0.98R | 行星 + 宫头度数 | 无网格 |

所有尺寸使用 `outerR * factor` 相对比值，缩放因子 `S = outerR / 260` 驱动字体/描边/半径的响应式缩放。

### 2. Z-Order 规则

同一 `<Layer>` 内，后绘制的元素在上层。严格遵循以下顺序（从上到下 = 从后到前）：

```
1. 背景 (大圆填充)
2. 外框圆
3. L5 二十八宿 (四象色块、刻度线、宿名)
4. L1 地支 (分割线、文本)
5. L2 长生 (分割线、文本)
6. L3 洞微大限 (分割线、文本、tooltip Circles)
7. 层间隔断环 (环形分隔线)
8. L6 行星层 (轨道、度数、aspect连线、星体)
9. 跨层宫位线 (30° 径向参考线)
10. 中心圆盘 + 文字
11. L4 宫号 ← 浮动层，确保不被遮挡
12. Tooltip
```

关键规则：**需要交互的浮动元素（宫号、星体）必须在最上层绘制**，否则会被后续绘制的结构线/分隔环遮挡事件。

**推论**: 宫位线、隔断环、参考网格等结构元素没有事件监听器（或仅 stroke），但它们的存在仍然会在 hit 检测中占据 z-order 优先级。交互元素必须绘制在所有结构元素之后。

### 3. 事件处理模式（Konva 关键约束）

Konva 的事件系统基于独立的 hit canvas，与视觉 canvas 分离。有以下关键约束：

#### 3.1 Group 事件依赖子元素

```tsx
// ✅ 正确: Group 监听事件，依赖子 Circle 提供 hit 区域
<Group
  onMouseEnter={() => setTooltip(text)}
  onMouseLeave={() => setTooltip(null)}>
  <Circle x={x} y={y} radius={16} fill="#000" opacity={0.001} />
  <Text ... listening={false} />
</Group>

// ❌ 错误: 子元素设 listening=false 导致 Group 无法接收事件
<Group onMouseEnter={...}>
  <Circle ... fill="#000" opacity={0.001} listening={false} />
  {/* Group 收不到事件！因为没有子元素参与 hit 检测 */}
</Group>
```

Group 本身没有几何边界——它的 hit 区域完全由子元素决定。**至少有一个子元素必须 `listening={true}`（默认值）**，否则 Group 的事件处理函数永远无法触发。

#### 3.2 不可见 Hit 区域

使用 `Circle fill="#000" opacity={0.001}` 创建不可见的 hit 检测区：

- `fill="#000"` 确保 Konva 为 Circle 分配 hit canvas 颜色
- `opacity={0.001}` 视觉上不可见，但 `opacity > 0` 满足 Konva 的可见性检查
- 不可与 `fill="transparent"` 或 `listening={false}` 混用

#### 3.3 Text 的 hit 检测

Text 元素的 hit 区域仅覆盖**实际渲染的文本像素**，而非 width/height 设定的包围盒。因此文本交互必须使用 Circle 作为 hit 代理，Text 本身设为 `listening={false}`。

### 4. 响应式尺寸系统

所有尺寸参数基于 `outerR` 的比例系数:

```typescript
const S = outerR / 260; // 1.0 at size=560
const FONT_SIZE = Math.round(14 * S);
const STROKE_WIDTH = 1.5 * S;
```

字体最小值不做硬限制（随 canvas 缩放自然缩小），确保所有元素比例一致。

## 影响

- 星盘从混合重叠布局变为 6 层同心环 + 浮动信息层
- 交互元素集中在顶层（L4 宫号、L6 星体），不被结构线遮挡
- Konva 事件处理模式规范化，避免重复踩坑

## 备选方案

### 多 Layer 方案

每个环层使用独立的 `<Layer>` 而非单个 Layer 内的 z-order 排序。优点是层间完全独立，事件互不干扰。缺点是多 Layer 增加 Konva 的渲染开销（每层独立 canvas），且跨层元素（如跨层宫位线、tooltip）需要额外处理。

决定使用单 Layer + z-order 排序，保持简单且性能更好。

## 参考

- react-konva: https://konvajs.org/docs/react/index.html
- Konva hit detection: https://konvajs.org/docs/events/Event_Delegation.html
