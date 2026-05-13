import { useState } from "react";
import type { BaziData } from "../types/chart";

interface Props {
  bazi: BaziData;
  ascendant: number;
  midheaven: number;
  partOfFortune: number;
  mingZhu: string;
  shenZhu: string;
  shiganhuayao: [string, string][];
  xijige: [string, string][];
  xiaoxianResult: [string, number];
  yuexianResult: [string, number];
  dongweifeixianResult: [number, string, string][];
}

export default function BaziPanel({ bazi, ascendant, midheaven, partOfFortune, mingZhu, shenZhu, shiganhuayao, xijige, xiaoxianResult, yuexianResult, dongweifeixianResult }: Props) {
  const [tab, setTab] = useState<"bazi" | "dayun" | "shishen" | "life" | "wuxing" | "liuxian">("bazi");

  const section = (title: string, content: string) => (
    <tr>
      <td style={{ color: "#888", padding: "4px 8px", whiteSpace: "nowrap", fontSize: 13 }}>{title}</td>
      <td style={{ color: "#e0e0e0", padding: "4px 8px", fontSize: 13 }}>{content}</td>
    </tr>
  );

  const pillarStr = (p: { heavenly_stem: string; earthly_branch: string }) =>
    `${p.heavenly_stem}${p.earthly_branch}`;

  return (
    <div
      style={{
        width: "100%",
        maxWidth: 500,
        marginTop: 16,
        borderTop: "1px solid #2a2a4a",
        paddingTop: 12,
      }}
    >
      {/* Tab bar */}
      <div style={{ display: "flex", gap: 4, marginBottom: 8 }}>
        {[
          { key: "bazi" as const, label: "四柱八字" },
          { key: "dayun" as const, label: "大运" },
          { key: "shishen" as const, label: "十神" },
          { key: "life" as const, label: "长生十二宫" },
          { key: "wuxing" as const, label: "五行" },
          { key: "liuxian" as const, label: "流限" },
        ].map((t) => (
          <button
            key={t.key}
            onClick={() => setTab(t.key)}
            style={{
              padding: "4px 12px",
              fontSize: 12,
              borderRadius: 4,
              border: `1px solid ${tab === t.key ? "#c9a0dc" : "#333"}`,
              background: tab === t.key ? "#3a1a5e" : "transparent",
              color: tab === t.key ? "#c9a0dc" : "#888",
              cursor: "pointer",
            }}
          >
            {t.label}
          </button>
        ))}
      </div>

      {/* Bazi tab */}
      {tab === "bazi" && (
        <div>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>八字</h4>
          <table style={{ borderCollapse: "collapse", fontSize: 16, marginBottom: 12 }}>
            <tbody>
              <tr>
                <td style={{ padding: "2px 10px", color: "#888" }}>年</td>
                <td style={{ padding: "2px 10px", color: "#FFD700" }}>{pillarStr(bazi.year_pillar)}</td>
              </tr>
              <tr>
                <td style={{ padding: "2px 10px", color: "#888" }}>月</td>
                <td style={{ padding: "2px 10px", color: "#69F0AE" }}>{pillarStr(bazi.month_pillar)}</td>
              </tr>
              <tr>
                <td style={{ padding: "2px 10px", color: "#888" }}>日</td>
                <td style={{ padding: "2px 10px", color: "#FF5252" }}>{pillarStr(bazi.day_pillar)}</td>
              </tr>
              <tr>
                <td style={{ padding: "2px 10px", color: "#888" }}>时</td>
                <td style={{ padding: "2px 10px", color: "#40C4FF" }}>{pillarStr(bazi.hour_pillar)}</td>
              </tr>
            </tbody>
          </table>

          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>关键点</h4>
          <table style={{ borderCollapse: "collapse", width: "100%" }}>
            <tbody>
              {section("上升 (Asc)", `${ascendant.toFixed(2)}°`)}
              {section("天顶 (MC)", `${midheaven.toFixed(2)}°`)}
              {section("福点", `${partOfFortune.toFixed(2)}°`)}
              {section("胎元", pillarStr(bazi.taiyuan))}
              {section("命主", mingZhu)}
              {section("身主", shenZhu)}
            </tbody>
          </table>
        </div>
      )}

      {/* Dayun tab */}
      {tab === "dayun" && (
        <div>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>大运（每十年一运）</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: 13 }}>
            <thead>
              <tr style={{ borderBottom: "1px solid #333" }}>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>年龄</th>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>大运</th>
              </tr>
            </thead>
            <tbody>
              {bazi.dayun.map((d, i) => (
                <tr key={i} style={{ borderBottom: "1px solid #1a1a2e" }}>
                  <td style={{ padding: "4px 8px", color: "#888" }}>{d.age}-{d.age + 9}</td>
                  <td style={{ padding: "4px 8px", color: "#e0e0e0" }}>
                    {d.heavenly_stem}{d.earthly_branch}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {/* Shishen tab */}
      {tab === "shishen" && (
        <div>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>十神</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: 13 }}>
            <thead>
              <tr style={{ borderBottom: "1px solid #333" }}>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>柱</th>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>天干</th>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>十神</th>
              </tr>
            </thead>
            <tbody>
              {bazi.shishen.map((s, i) => (
                <tr key={i} style={{ borderBottom: "1px solid #1a1a2e" }}>
                  <td style={{ padding: "4px 8px", color: "#e0e0e0" }}>{s.pillar_name}</td>
                  <td style={{ padding: "4px 8px", color: "#e0e0e0" }}>{s.stem}</td>
                  <td style={{ padding: "4px 8px", color: "#c9a0dc" }}>{s.shishen}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h4 style={{ color: "#c9a0dc", fontSize: 14, margin: "12px 0 8px" }}>藏干</h4>
          <div style={{ display: "flex", flexWrap: "wrap", gap: 8, fontSize: 13 }}>
            {bazi.hidden_stems.map((h, i) => (
              <div key={i} style={{ padding: "4px 8px", background: "#1a1a2e", borderRadius: 4 }}>
                <span style={{ color: "#888" }}>{h.branch_name}: </span>
                <span style={{ color: "#e0e0e0" }}>{h.stems.join(" ")}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Life cycle tab */}
      {tab === "life" && (
        <div>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>长生十二宫（日干 {bazi.day_pillar.heavenly_stem}）</h4>
          <div style={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: 4, fontSize: 13 }}>
            {bazi.life_cycle.map((l, i) => (
              <div
                key={i}
                style={{
                  padding: "4px 8px",
                  background: l.stage === "长生" || l.stage === "帝旺" || l.stage === "临官" ? "#1a3a1a" : "#2a1a1a",
                  borderRadius: 4,
                  display: "flex",
                  justifyContent: "space-between",
                }}
              >
                <span style={{ color: "#888" }}>{l.branch_name}</span>
                <span
                  style={{
                    color:
                      l.stage === "长生" || l.stage === "帝旺" || l.stage === "临官"
                        ? "#69F0AE"
                        : l.stage === "死" || l.stage === "墓" || l.stage === "绝"
                          ? "#FF5252"
                          : "#e0e0e0",
                  }}
                >
                  {l.stage}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Liuxian tab */}
      {tab === "liuxian" && (
        <div>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>小限</h4>
          <p style={{ fontSize: 13, color: "#e0e0e0", marginBottom: 12 }}>
            本年小限: {xiaoxianResult[0]} ({xiaoxianResult[1]}岁)
          </p>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>月限</h4>
          <p style={{ fontSize: 13, color: "#e0e0e0", marginBottom: 12 }}>
            本月月限: {yuexianResult[0]}
          </p>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>洞微飞限</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: 13 }}>
            <thead>
              <tr style={{ borderBottom: "1px solid #333" }}>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>起始年龄</th>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>限</th>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>地支</th>
              </tr>
            </thead>
            <tbody>
              {dongweifeixianResult.map((p, i) => (
                <tr key={i} style={{ borderBottom: "1px solid #1a1a2e" }}>
                  <td style={{ padding: "4px 8px", color: "#888" }}>{p[0]}岁</td>
                  <td style={{ padding: "4px 8px", color: "#e0e0e0" }}>{p[1]}</td>
                  <td style={{ padding: "4px 8px", color: "#c9a0dc" }}>{p[2]}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {/* Wuxing tab */}
      {tab === "wuxing" && (
        <div>
          <h4 style={{ color: "#c9a0dc", fontSize: 14, marginBottom: 8 }}>十干化曜（日干 {bazi.day_pillar.heavenly_stem}）</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: 13 }}>
            <thead>
              <tr style={{ borderBottom: "1px solid #333" }}>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>天干</th>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>化曜</th>
              </tr>
            </thead>
            <tbody>
              {shiganhuayao.map(([stem, yao], i) => (
                <tr key={i} style={{ borderBottom: "1px solid #1a1a2e" }}>
                  <td style={{ padding: "4px 8px", color: "#e0e0e0" }}>{stem}</td>
                  <td style={{ padding: "4px 8px", color: "#c9a0dc" }}>{yao}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h4 style={{ color: "#c9a0dc", fontSize: 14, margin: "12px 0 8px" }}>喜忌格</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: 13 }}>
            <thead>
              <tr style={{ borderBottom: "1px solid #333" }}>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>星体</th>
                <th style={{ padding: "4px 8px", textAlign: "left", color: "#888" }}>喜忌</th>
              </tr>
            </thead>
            <tbody>
              {xijige.map(([body, rel], i) => (
                <tr key={i} style={{ borderBottom: "1px solid #1a1a2e" }}>
                  <td style={{ padding: "4px 8px", color: "#e0e0e0" }}>{body}</td>
                  <td style={{ padding: "4px 8px", color: rel === "喜" ? "#69F0AE" : rel === "忌" ? "#FF5252" : "#e0e0e0" }}>{rel}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
