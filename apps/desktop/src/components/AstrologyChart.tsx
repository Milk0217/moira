import { useState } from "react";
import { Stage, Layer, Arc, Circle, Line, Rect, Text, Group } from "react-konva";
import type { StarPower, HouseAnalysis, MansionInfo } from "../types/chart";

/* ── 星曜常量 ── */
const BODY_SYMBOLS: Record<string, string> = {
  太阳: "☉", 太阴: "☽", 水星: "☿", 金星: "♀",
  火星: "♂", 木星: "♃", 土星: "♄",
  天王星: "♅", 海王星: "♆", 冥王星: "♇",
  罗睺: "☊", 计都: "☋", 月孛: "⚸", 紫炁: "✦",
};

const BODY_COLORS: Record<string, string> = {
  太阳: "#FFD700", 太阴: "#C0C0C0", 水星: "#B5A5D5", 金星: "#FF69B4",
  火星: "#FF4444", 木星: "#DAA520", 土星: "#CD853F",
  天王星: "#7ECBA1", 海王星: "#4B70DD", 冥王星: "#8B7355",
  罗睺: "#9370DB", 计都: "#8B0000", 月孛: "#2F4F4F", 紫炁: "#FFE4B5",
};

/* ── 层数据 ── */
const BRANCHES = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

const STAGES_12 = ["长生", "沐浴", "冠带", "临官", "帝旺", "衰", "病", "死", "墓", "绝", "胎", "养"];

const FALLBACK_MANSIONS: { name: string; dir: string; width: number }[] = [
  { name: "角", dir: "青龙", width: 12 }, { name: "亢", dir: "青龙", width: 9 },
  { name: "氐", dir: "青龙", width: 15 }, { name: "房", dir: "青龙", width: 5 },
  { name: "心", dir: "青龙", width: 5 }, { name: "尾", dir: "青龙", width: 18 },
  { name: "箕", dir: "青龙", width: 11 }, { name: "斗", dir: "玄武", width: 26 },
  { name: "牛", dir: "玄武", width: 8 }, { name: "女", dir: "玄武", width: 12 },
  { name: "虚", dir: "玄武", width: 10 }, { name: "危", dir: "玄武", width: 17 },
  { name: "室", dir: "玄武", width: 16 }, { name: "壁", dir: "玄武", width: 9 },
  { name: "奎", dir: "白虎", width: 16 }, { name: "娄", dir: "白虎", width: 12 },
  { name: "胃", dir: "白虎", width: 14 }, { name: "昴", dir: "白虎", width: 11 },
  { name: "毕", dir: "白虎", width: 16 }, { name: "觜", dir: "白虎", width: 2 },
  { name: "参", dir: "白虎", width: 9 }, { name: "井", dir: "朱雀", width: 33 },
  { name: "鬼", dir: "朱雀", width: 4 }, { name: "柳", dir: "朱雀", width: 15 },
  { name: "星", dir: "朱雀", width: 7 }, { name: "张", dir: "朱雀", width: 18 },
  { name: "翼", dir: "朱雀", width: 18 }, { name: "轸", dir: "朱雀", width: 17 },
];

const DIR_COLORS: Record<string, string> = {
  青龙: "rgba(0,168,107,0.12)", 玄武: "rgba(54,94,184,0.12)",
  白虎: "rgba(192,192,192,0.10)", 朱雀: "rgba(230,57,70,0.12)",
};



const ASPECT_ANGLES: [number, string, string][] = [
  [0, "合相", "#FFD700"],
  [30, "半六分相", "#88CC88"],
  [60, "六分相", "#44AA44"],
  [90, "四分相", "#E63946"],
  [120, "三分相", "#4A90D9"],
  [150, "半四分相", "#888"],
  [180, "对分相", "#CC66CC"],
];
const ASPECT_ORB = 8;

/* ── 类型 ── */
interface BodyEntry {
  name: string; longitude: number; latitude?: number; speed?: number;
  detail?: string; mansion_name?: string; mansion_degree?: number;
}
interface HouseEntry { index: number; longitude: number; }
interface Props {
  bodies: BodyEntry[]; houses?: HouseEntry[]; size?: number;
  centerText?: string; stageRef?: React.RefObject<any>;
  dongweifeixian?: [number, string, string][];
  starPowers?: StarPower[];
  houseAnalyses?: HouseAnalysis[];
  mansions?: MansionInfo[];
}

/* ── 工具函数 ── */
function degToVec(deg: number, radius: number, cx: number, cy: number) {
  const a = ((deg - 90) * Math.PI) / 180;
  return { x: cx + radius * Math.cos(a), y: cy + radius * Math.sin(a) };
}

function toMansionEntry(m: { name: string; width: number }): { name: string; width: number } {
  return { name: m.name, width: m.width };
}

export function getLunarMansion(lon: number, mansions?: MansionInfo[]): { name: string; degree: number } {
  const data = (mansions ?? FALLBACK_MANSIONS).map(toMansionEntry);
  const total = data.reduce((s, m) => s + m.width, 0);
  const pos = (((lon % 360) + 360) % 360 / 360) * total;
  let cum = 0;
  for (const m of data) {
    cum += m.width;
    if (pos < cum) return { name: m.name, degree: pos - (cum - m.width) };
  }
  const last = data[data.length - 1];
  return { name: last.name, degree: last.width };
}

/* ── 行星布局算法 ── */
function layoutBodies(bodies: BodyEntry[], baseR: number, cx: number, cy: number) {
  if (bodies.length === 0) return [];
  const CLUSTER_DEG = 8, MIN_SEP = 4.5, RADII = [baseR, baseR * 0.9, baseR * 1.08];
  let sorted: any[] = [...bodies].sort((a: any, b: any) => a.longitude - b.longitude);

  const rawSpan = sorted[sorted.length - 1].longitude - sorted[0].longitude;
  if (rawSpan > 300 && sorted.length > 1) {
    let maxGap = 0, gapIdx = 0;
    for (let i = 0; i < sorted.length; i++) {
      const curr = sorted[i].longitude;
      const next = sorted[(i + 1) % sorted.length].longitude;
      const gap = next <= curr ? next + 360 - curr : next - curr;
      if (gap > maxGap) { maxGap = gap; gapIdx = i; }
    }
    sorted = [...sorted.slice(gapIdx + 1), ...sorted.slice(0, gapIdx + 1)];
    const adj = sorted.map(b => ({ ...b, _lon: b.longitude }));
    for (let i = 1; i < adj.length; i++) {
      if (adj[i].longitude < adj[i - 1].longitude) {
        for (let j = i; j < adj.length; j++) adj[j] = { ...adj[j], _lon: adj[j].longitude + 360 };
        break;
      }
    }
    sorted = adj;
  } else {
    sorted = sorted.map(b => ({ ...b, _lon: b.longitude }));
  }

  const groups: any[][] = [];
  for (const b of sorted) {
    if (groups.length === 0) { groups.push([b]); continue; }
    const last = groups[groups.length - 1];
    if (Math.abs(b._lon - last[last.length - 1]._lon) < CLUSTER_DEG) last.push(b);
    else groups.push([b]);
  }

  return groups.flatMap((g) => {
    if (g.length === 1) {
      const b = g[0];
      const pos = degToVec(b.longitude, baseR, cx, cy);
      return [{ name: b.name, symbol: BODY_SYMBOLS[b.name] || b.name[0],
        color: BODY_COLORS[b.name] || "#c9a0dc", detail: b.detail, speed: b.speed,
        x: pos.x, y: pos.y, dotR: 7, hitR: 16 }];
    }
    const minLon = g[0]._lon, maxLon = g[g.length - 1]._lon;
    const totalSpan = Math.max(maxLon - minLon, (g.length - 1) * MIN_SEP);
    return g.map((b, i) => {
      const t = i / (g.length - 1);
      const dl = minLon + t * totalSpan;
      const r = RADII[i % RADII.length];
      const pos = degToVec(dl > 360 ? dl - 360 : dl, r, cx, cy);
      return { name: b.name, symbol: BODY_SYMBOLS[b.name] || b.name[0],
        color: BODY_COLORS[b.name] || "#c9a0dc", detail: b.detail, speed: b.speed,
        x: pos.x, y: pos.y, dotR: 7, hitR: 16 };
    });
  });
}

function separatorRing(cx: number, cy: number, r: number, sw: number, color: string, opacity: number) {
  return <Circle x={cx} y={cy} radius={r} stroke={color} strokeWidth={sw} opacity={opacity} />;
}

/* 躔度歌示例 (sample from sample_s.rule) */
const MANSION_POEMS: Record<string, string> = {
  "太阳_角": "日躔角宿号天慈，命里居之比华岳。",
  "太阳_亢": "日躔亢宿号天富，冠世文章更清秀。",
  "太阴_角": "月躔角宿号天贵，子孙荣贵更光辉。",
  "太阴_亢": "月躔亢宿号天英，冠世文章更清秀。",
  "水星_角": "水躔角宿号天元，年少成名入金门。",
  "金星_角": "金躔角宿号天英，金玉满堂更有声。",
  "火星_角": "火躔角宿号天燠，不习诗书也自高。",
  "木星_角": "木躔角宿号天贵，平生衣食自然丰。",
  "土星_角": "土躔角宿号天仓，家积金银富贵强。",
  "罗睺_角": "罗躔角宿号天骄，一生名利两相饶。",
  "计都_角": "计躔角宿号天符，一生多难更忧虞。",
  "紫炁_角": "炁躔角宿号天端，衣食丰盈不用叹。",
  "月孛_角": "孛躔角宿号天微，骨肉亲知各自离。",
};

/* ══════════════════════════════════════════════════════════════════
   主组件
   ══════════════════════════════════════════════════════════════════ */
export default function AstrologyChart({
  bodies, houses, size = 560, centerText = "命盘", stageRef, dongweifeixian,
  starPowers, houseAnalyses, mansions,
}: Props) {
  const cx = size / 2, cy = size / 2;
  const outerR = size / 2 - 20;

  /* 响应式缩放因子: 1.0 at size=560 (outerR=260) */
  const S = outerR / 260;

  /* 6层同心环半径 */
  const L1_I = outerR * 0.22, L1_O = outerR * 0.32; // 地支
  const L2_I = outerR * 0.34, L2_O = outerR * 0.44; // 长生
  const L3_I = outerR * 0.46, L3_O = outerR * 0.56; // 洞微大限
  const L4_I = outerR * 0.58, L4_O = outerR * 0.68; // 宫名
  const L5_I = outerR * 0.70, L5_O = outerR * 0.81; // 二十八宿
  const L6_I = outerR * 0.83, L6_O = outerR * 0.98; // 行星

  /* 二十八宿数据（从 props 或 fallback） */
  const source = (mansions ?? FALLBACK_MANSIONS) as ({ name: string; dir?: string; direction?: string; width: number })[];
  const mansionsData = source.map(m => ({
    name: m.name,
    dir: m.dir ?? m.direction ?? '',
    width: m.width,
  }));
  const totalWidth = mansionsData.reduce((s, m) => s + m.width, 0);
  const mansionStartAngles: number[] = (() => {
    let cum = 0;
    return mansionsData.map((m) => {
      const angle = (cum / totalWidth) * 360;
      cum += m.width;
      return angle;
    });
  })();

  /* State */
  const [tooltip, setTooltip] = useState<{ text: string; x: number; y: number } | null>(null);
  const [selectedBody, setSelectedBody] = useState<string | null>(null);
  const [hoveredAspect, setHoveredAspect] = useState<number | null>(null);
  const [activeAngles, setActiveAngles] = useState<Set<number>>(new Set([60, 90, 120, 180]));

  const positionedBodies = layoutBodies(bodies, (L6_I + L6_O) / 2, cx, cy);

  function toggleAngle(angle: number) {
    setActiveAngles((prev) => {
      const next = new Set(prev);
      if (next.has(angle)) next.delete(angle); else next.add(angle);
      return next;
    });
  }

  function getAspectLines(selected: string) {
     const sel = positionedBodies.find((b) => b.name === selected);
     if (!sel) return [];
     const lines: { x1: number; y1: number; x2: number; y2: number; color: string; label: string; angle: number; orb: number; body1: string; body2: string }[] = [];
     for (const other of positionedBodies) {
       if (other.name === selected) continue;
       const selLon = bodies.find((b) => b.name === selected)?.longitude ?? 0;
       const otherLon = bodies.find((b) => b.name === other.name)?.longitude ?? 0;
       let raw = Math.abs(selLon - otherLon);
       if (raw > 180) raw = 360 - raw;
       for (const [target, name, color] of ASPECT_ANGLES) {
         if (Math.abs(raw - target) <= ASPECT_ORB && target !== 0 && activeAngles.has(target)) {
           const orb = Math.abs(raw - target);
           lines.push({ x1: sel.x, y1: sel.y, x2: other.x, y2: other.y, color, label: name, angle: raw, body1: selected, body2: other.name, orb });
           break;
         }
       }
     }
     return lines;
   }

  const aspectLines = selectedBody ? getAspectLines(selectedBody) : [];

  /* ── 响应式字体/尺寸 ── */
  const FZ_BRANCH    = Math.round(14 * S);
  const FZ_STAGE     = Math.round(11 * S);
  const FZ_DONGWEI   = Math.round(10 * S);
  const FZ_MANSION   = Math.round(10 * S);
  const FZ_CENTER    = Math.round(14 * S);
  const FZ_CUSP      = Math.round(9 * S);
  const FZ_HOUSE_NUM = Math.round(9 * S);
  const FZ_CLASSIFY  = Math.round(8 * S);
  const SEG_W        = Math.round(60 * S);
  const SEG_H        = Math.round(14 * S);
  const CENTER_RADIUS = Math.round(50 * S);

  /* Tooltip 位置: 在右侧时向左翻，避免被画布裁切 */
  function tp(x: number, y: number) {
    const tw = 140, th = 52, offset = 15;
    return {
      x: x > size / 2 ? x - tw - offset : x + offset,
      y: Math.max(offset, Math.min(y - offset, size - th - offset)),
    };
  }

  /* ════════════════════════════════════════════
     渲染
     ════════════════════════════════════════════ */
  return (
    <div style={{ position: "relative", display: "inline-block" }}>
      {/* 相位过滤器 */}
      {selectedBody && (
        <div style={{ position: "absolute", top: -30, left: "50%", transform: "translateX(-50%)",
          display: "flex", gap: 4, zIndex: 10, background: "rgba(15,15,35,0.9)", padding: "4px 10px",
          borderRadius: 8, border: "1px solid #333", whiteSpace: "nowrap" }}>
          {ASPECT_ANGLES.filter(([a]) => a !== 0).map(([angle, name, color]) => (
            <button key={angle} onClick={() => toggleAngle(angle)}
              style={{ padding: "2px 8px", fontSize: 11, borderRadius: 4, border: `1px solid ${color}`,
                background: activeAngles.has(angle) ? color : "transparent",
                color: activeAngles.has(angle) ? "#fff" : color, cursor: "pointer",
                fontWeight: activeAngles.has(angle) ? "bold" : "normal" }}>
              {name}
            </button>
          ))}
        </div>
      )}

      <Stage ref={stageRef} width={size} height={size}>
        <Layer>
          {/* ── 背景 ── */}
          <Circle x={cx} y={cy} radius={outerR + 2} fill="#0f0f23" />
          <Rect x={0} y={0} width={size} height={size} fill="transparent"
            listening={!!selectedBody} onClick={() => setSelectedBody(null)} onTap={() => setSelectedBody(null)} />

          {/* ── 外框 ── */}
          <Circle x={cx} y={cy} radius={outerR} stroke="#5a1a70" strokeWidth={1.5 * S} opacity={0.7} />
          <Circle x={cx} y={cy} radius={L6_O} stroke="#3a1050" strokeWidth={0.5 * S} opacity={0.3} />

          {/* ══════════════════════════════════ L5: 二十八宿 ══════════════════════════════════ */}
          {/* 四象背景色块 */}
          {(() => {
            const dirAngles: { dir: string; start: number; end: number }[] = [
              { dir: "青龙", start: mansionStartAngles[0], end: mansionStartAngles[7] },
              { dir: "玄武", start: mansionStartAngles[7], end: mansionStartAngles[14] },
              { dir: "白虎", start: mansionStartAngles[14], end: mansionStartAngles[21] },
              { dir: "朱雀", start: mansionStartAngles[21], end: mansionStartAngles[28] },
            ];
            return dirAngles.map((q) => {
              let ang = q.end - q.start;
              if (ang < 0) ang += 360;
              return (
                <Arc key={`dir-bg-${q.dir}`} x={cx} y={cy}
                  innerRadius={L5_I} outerRadius={L5_O}
                  angle={ang} rotation={q.start - 90}
                  fill={DIR_COLORS[q.dir]} listening={false} />
              );
            });
          })()}

          {/* 二十八宿径向分割线 */}
          {mansionStartAngles.map((deg, i) => {
            const p1 = degToVec(deg, L5_I, cx, cy);
            const p2 = degToVec(deg, L5_O, cx, cy);
            return (
              <Line key={`mline-${i}`} points={[p1.x, p1.y, p2.x, p2.y]}
                stroke="#7b2d8e" strokeWidth={(i % 7 === 0 ? 2.5 : 1) * S}
                opacity={i % 7 === 0 ? 0.7 : 0.3} />
            );
          })}

          {/* 二十八宿名 */}
          {mansionsData.map((m, i) => {
            const ang = mansionStartAngles[i] + (m.width / totalWidth) * 180;
            const p = degToVec(ang, (L5_I + L5_O) / 2, cx, cy);
            return (
              <Text key={`mname-${i}`} x={p.x - SEG_W/4} y={p.y - SEG_H/2}
                text={m.name} fontSize={FZ_MANSION} fill="#b388ff" align="center" width={SEG_W/2} height={SEG_H} />
            );
          })}

          {/* ══════════════════════════════════ L1: 十二地支 ══════════════════════════════════ */}
          {(() => {
            const midR = (L1_I + L1_O) / 2;
            return BRANCHES.map((b, i) => {
              const deg = i * 30 + 15;
              const p = degToVec(deg, midR, cx, cy);
              return (
                <Group key={`branch-${i}`}>
                  <Line points={[degToVec(i*30, L1_I, cx, cy).x, degToVec(i*30, L1_I, cx, cy).y,
                    degToVec(i*30, L1_O, cx, cy).x, degToVec(i*30, L1_O, cx, cy).y]}
                    stroke="#3a1050" strokeWidth={0.5 * S} opacity={0.12} />
                  <Text x={p.x - SEG_W/2} y={p.y - SEG_H/2}
                    text={b} fontSize={FZ_BRANCH} fill="#D4A574" fontStyle="bold"
                    width={SEG_W} height={SEG_H} align="center" />
                </Group>
              );
            });
          })()}

          {/* ══════════════════════════════════ L2: 十二长生 ══════════════════════════════════ */}
          {STAGES_12.map((s, i) => {
            const deg = i * 30 + 15;
            const p = degToVec(deg, (L2_I + L2_O) / 2, cx, cy);
            return (
              <Group key={`stage-${i}`}>
                <Line points={[degToVec(i*30, L2_I, cx, cy).x, degToVec(i*30, L2_I, cx, cy).y,
                  degToVec(i*30, L2_O, cx, cy).x, degToVec(i*30, L2_O, cx, cy).y]}
                  stroke="#3a1050" strokeWidth={0.5 * S} opacity={0.12} />
                <Text x={p.x - SEG_W/2} y={p.y - SEG_H/2}
                  text={s} fontSize={FZ_STAGE} fill="#b39ddb" width={SEG_W} height={SEG_H} align="center" />
              </Group>
            );
          })}

          {/* ══════════════════════════════════ L3: 洞微大限 ══════════════════════════════════ */}
          {dongweifeixian && dongweifeixian.length > 0 && dongweifeixian.map((d, i) => {
            const [age, name, branch] = d;
            const branchIdx = BRANCHES.indexOf(branch);
            if (branchIdx < 0) return null;
            const deg = branchIdx * 30 + 15;
            const p = degToVec(deg, (L3_I + L3_O) / 2, cx, cy);
            const nextAge = i < 11 ? dongweifeixian[i + 1][0] : age + 6;
            return (
              <Group key={`dongwei-${i}`}>
                <Line points={[degToVec(branchIdx*30, L3_I, cx, cy).x, degToVec(branchIdx*30, L3_I, cx, cy).y,
                  degToVec(branchIdx*30, L3_O, cx, cy).x, degToVec(branchIdx*30, L3_O, cx, cy).y]}
                  stroke="#3a1050" strokeWidth={0.5 * S} opacity={0.12} />
                <Text x={p.x - SEG_W/2} y={p.y - SEG_H/2}
                  text={name} fontSize={FZ_DONGWEI} fill="#b39ddb" width={SEG_W} height={SEG_H} align="center"
                  onMouseEnter={() => { const t = tp(p.x, p.y); setTooltip({ text: `${name}\n地支 ${branch}\n${age}~${nextAge-1}岁`, x: t.x, y: t.y }); }}
                  onMouseLeave={() => setTooltip(null)} />
              </Group>
            );
          })}

          {/* ══════════════════════════════════ 层间隔断环 ══════════════════════════════════ */}
          {separatorRing(cx, cy, L1_I - outerR*0.01, 1.5*S, "#7b2d8e", 0.5)}
          {separatorRing(cx, cy, L1_O + outerR*0.01, 1.5*S, "#7b2d8e", 0.5)}
          {separatorRing(cx, cy, L2_O + outerR*0.01, 1.5*S, "#7b2d8e", 0.5)}
          {separatorRing(cx, cy, L3_O + outerR*0.01, 1.5*S, "#7b2d8e", 0.5)}
          {separatorRing(cx, cy, L4_O + outerR*0.01, 1.5*S, "#7b2d8e", 0.5)}
          {separatorRing(cx, cy, L5_O + outerR*0.01, 1.5*S, "#7b2d8e", 0.5)}
          {separatorRing(cx, cy, L5_I - outerR*0.01, 1.0*S, "#3a1050", 0.35)}
          {separatorRing(cx, cy, L6_I - outerR*0.01, 1.0*S, "#3a1050", 0.35)}

          {/* ══════════════════════════════════ L6: 行星层 ══════════════════════════════════ */}
          {/* 行星轨道 */}
          <Circle x={cx} y={cy} radius={(L6_I + L6_O) / 2} stroke="#3a1050" strokeWidth={0.5 * S} opacity={0.12} />

          {/* 宫头度数标记 (L6 外缘) */}
          {houses?.map((h) => {
            const pos = degToVec(h.longitude, L6_O - outerR * 0.03, cx, cy);
            return (
              <Text key={`cusp-${h.index}`} x={pos.x - 12} y={pos.y - SEG_H/2}
                text={`${Math.floor(h.longitude)}°`} fontSize={FZ_CUSP} fill="#4a4a6a"
                width={24} height={SEG_H} align="center" />
            );
          })}

          {/* 相位连线 */}

          {/* 行星 */}
          {positionedBodies.map((body) => {
            const isSelected = selectedBody === body.name;
            const sp = starPowers?.find(p => p.body_name === body.name);
            const classifyLabel = sp?.classification?.replace("星", "") ?? "";
            const classifyColor = sp?.classification === "难星" ? "#FF5252"
              : sp?.classification === "恩星" ? "#69F0AE"
              : sp?.classification === "仇星" ? "#FFB347"
              : sp?.classification === "用星" ? "#40C4FF" : "";
            return (
              <Group key={body.name}
                onMouseEnter={() => {
                  const t = tp(body.x, body.y);
                  const bodyEntry = bodies.find(b => b.name === body.name);
                  const poemKey = `${body.name}_${bodyEntry?.mansion_name ?? ""}`;
                  const poem = MANSION_POEMS[poemKey];
                  setTooltip({ text: body.detail || body.name + (poem ? `\n${poem}` : ""), x: t.x, y: t.y });
                }}
                onMouseLeave={() => setTooltip(null)}
                onClick={() => setSelectedBody(isSelected ? null : body.name)}
                onTap={() => setSelectedBody(isSelected ? null : body.name)}>
                <Circle x={body.x} y={body.y} radius={body.hitR} fill="#000" opacity={0.001} />
                {isSelected && (
                  <>
                    <Circle x={body.x} y={body.y} radius={body.dotR + 6} stroke="#FFD700" strokeWidth={2} opacity={0.7} />
                    <Circle x={body.x} y={body.y} radius={body.dotR + 10} stroke="#FFD700" strokeWidth={1} opacity={0.3} />
                  </>
                )}
                <Circle x={body.x} y={body.y} radius={body.dotR} fill={body.color} opacity={0.85} />
                <Text x={body.x - 10} y={body.y - 13} text={body.symbol} fontSize={Math.round(13*S)}
                  fill="#fff" width={20} height={16} align="center" />
                {body.speed !== undefined && (
                   <Text x={body.x - 6} y={body.y - 24} text={body.speed < 0 ? "逆" : "顺"}
                     fontSize={Math.round(9*S)} fill={body.speed < 0 ? "#FF5252" : "#69F0AE"}
                     fontStyle="bold" width={12} height={10} align="center" />
                 )}
                 {classifyLabel && (
                   <Text x={body.x - 8} y={body.y + body.dotR + 2} text={classifyLabel}
                     fontSize={FZ_CLASSIFY} fill={classifyColor} fontStyle="bold"
                     width={16} height={10} align="center" />
                 )}
              </Group>
            );
          })}

          {/* 跨层宫位线 (从 L1 内到 L6 外, 30° 间隔) */}
          {Array.from({ length: 12 }).map((_, i) => {
            const deg = (i * 30 - 90) * (Math.PI / 180);
            return (
              <Line key={`hline-${i}`}
                points={[cx + L1_I * Math.cos(deg), cy + L1_I * Math.sin(deg),
                  cx + L6_O * Math.cos(deg), cy + L6_O * Math.sin(deg)]}
                stroke="#3a1050" strokeWidth={0.5 * S} opacity={0.08} />
            );
          })}

          {/* ══════════════════════════════════ 中心 ══════════════════════════════════ */}
          <Circle x={cx} y={cy} radius={CENTER_RADIUS} stroke="#3a1050" strokeWidth={1 * S}
            fill="#0f0f23" opacity={0.8} />
          <Text x={cx - 45 * S} y={cy - 10 * S}
            text={centerText || "命盘"} fontSize={FZ_CENTER}
            fill="#ce93d8" fontStyle="bold" width={90 * S} height={20 * S} align="center" />

          {/* ══════════════════════════════════ L4: 宫号 (顶层) ══════════════════════════════════ */}
          {houses && (() => {
            const sorted = [...houses].sort((a, b) => a.index - b.index);
            return sorted.map((h, idx) => {
              const pos = degToVec(h.longitude, (L4_I + L4_O) / 2, cx, cy);
              const nextLon = idx < sorted.length - 1 ? sorted[idx + 1].longitude : sorted[0].longitude + 360;
              const houseBodies = bodies.filter(b => {
                if (nextLon > h.longitude) return b.longitude >= h.longitude && b.longitude < nextLon;
                else return b.longitude >= h.longitude || b.longitude < nextLon;
              });
              const names = houseBodies.map(b => b.name).join(", ") || "无星曜";
              const ha = houseAnalyses?.find(a => a.house_name === ["命宫","财帛","兄弟","田宅","男女","奴仆","夫妻","疾厄","迁移","官禄","福德","相貌"][idx]);
              const spStr = ha ? [...ha.auspicious_spirits, ...ha.inauspicious_spirits].filter(Boolean).join(" ") : "";
              return (
                <Group key={`house-${h.index}`}
                  onMouseEnter={() => { const t = tp(pos.x, pos.y); setTooltip({ text: `${idx+1}宫(${["命","财","兄","田","男","仆","妻","疾","迁","官","福","相"][idx]}): ${names}${spStr ? `\n神煞: ${spStr}` : ""}`, x: t.x, y: t.y }); }}
                  onMouseLeave={() => setTooltip(null)}>
                  <Circle x={pos.x} y={pos.y} radius={18} fill="#000" opacity={0.001} />
                  <Text x={pos.x - 12} y={pos.y - 10}
                    text={String(h.index)} fontSize={FZ_HOUSE_NUM} fill="#ce93d8" fontStyle="bold"
                    width={24} height={20} align="center" listening={false} />
                </Group>
              );
            });
          })()}

          {/* 相位连线: 单遍渲染，每根线自带 12px 隐形 hit 区域 */}
          {aspectLines.map((line, i) => {
            const isHovered = hoveredAspect === i;
            const lx = (line.x1 + line.x2) / 2;
            const ly = (line.y1 + line.y2) / 2;
            return (
              <Group key={`aspect-${i}`}>
                {/* 可见的线: 悬停加粗实线，否则虚线 */}
                <Line points={[line.x1, line.y1, line.x2, line.y2]}
                  stroke={line.color} strokeWidth={(isHovered ? 3 : 2) * S}
                  dash={isHovered ? undefined : [6 * S, 4 * S]} opacity={isHovered ? 1 : 0.6} />
                {/* 隐形 hit 线: 覆盖整根线，悬停时显示 tooltip */}
                <Line points={[line.x1, line.y1, line.x2, line.y2]}
                  stroke="#000" strokeWidth={12 * S} opacity={0.001}
                  onMouseEnter={() => {
                    setHoveredAspect(i);
                    const t = tp(lx, ly);
                    setTooltip({ text: `${line.body1} ↔ ${line.body2}\n${line.label} ${line.angle.toFixed(1)}° orb ${(line as any).orb?.toFixed(1)}°`, x: t.x, y: t.y });
                  }}
                  onMouseLeave={() => { setHoveredAspect(null); setTooltip(null); }} />
              </Group>
            );
          })}

          {/* ══════════════════════════════════ Tooltip ══════════════════════════════════ */}
          {tooltip && (
            <Group>
              <Rect x={tooltip.x} y={tooltip.y} width={140} height={52}
                fill="rgba(0,0,0,0.85)" cornerRadius={4} stroke="#6a1b9a" strokeWidth={1} />
              <Text x={tooltip.x + 8} y={tooltip.y + 6} text={tooltip.text}
                fontSize={11} fill="#e0e0e0" width={124} height={44} />
            </Group>
          )}
        </Layer>
      </Stage>
    </div>
  );
}
