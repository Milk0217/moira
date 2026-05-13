import { useState, memo } from "react";
import type { BaziData } from "../types/chart";
import { theme } from "../theme";
import { TH, TD } from "./shared";

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

function BaziPanel({ bazi, ascendant, midheaven, partOfFortune, mingZhu, shenZhu, shiganhuayao, xijige, xiaoxianResult, yuexianResult, dongweifeixianResult }: Props) {
  const [tab, setTab] = useState<"bazi" | "dayun" | "shishen" | "life" | "wuxing" | "liuxian">("bazi");

  const section = (title: string, content: string) => (
    <tr>
      <td style={{ color: theme.colors.text.secondary, padding: "4px 8px", whiteSpace: "nowrap", fontSize: theme.fontSize.md }}>{title}</td>
      <td style={{ color: theme.colors.text.primary, padding: "4px 8px", fontSize: theme.fontSize.md }}>{content}</td>
    </tr>
  );

  const pillarStr = (p: { heavenly_stem: string; earthly_branch: string }) =>
    `${p.heavenly_stem}${p.earthly_branch}`;

  const tabBtnStyle = (isActive: boolean): React.CSSProperties => ({
    padding: "4px 12px",
    fontSize: theme.fontSize.sm,
    borderRadius: theme.radius.sm,
    border: `1px solid ${isActive ? theme.colors.accent.primary : theme.colors.border.default}`,
    background: isActive ? theme.colors.accent.dark : "transparent",
    color: isActive ? theme.colors.accent.primary : theme.colors.text.secondary,
    cursor: "pointer",
  });

  const tdPillar = (color: string) => ({
    padding: "2px 10px",
    color,
  });

  return (
    <div
      style={{
        width: "100%",
        maxWidth: 500,
        marginTop: theme.spacing.lg,
        borderTop: `1px solid ${theme.colors.border.light}`,
        paddingTop: theme.spacing.md,
      }}
    >
      <div style={{ display: "flex", gap: theme.spacing.xs, marginBottom: theme.spacing.sm }}>
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
            style={tabBtnStyle(tab === t.key)}
          >
            {t.label}
          </button>
        ))}
      </div>

      {tab === "bazi" && (
        <div>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>八字</h4>
          <table style={{ borderCollapse: "collapse", fontSize: theme.fontSize.xl, marginBottom: theme.spacing.md }}>
            <tbody>
              <tr>
                <td style={{ padding: "2px 10px", color: theme.colors.text.secondary }}>年</td>
                <td style={tdPillar(theme.colors.semantic.pillar.year)}>{pillarStr(bazi.year_pillar)}</td>
              </tr>
              <tr>
                <td style={{ padding: "2px 10px", color: theme.colors.text.secondary }}>月</td>
                <td style={tdPillar(theme.colors.semantic.pillar.month)}>{pillarStr(bazi.month_pillar)}</td>
              </tr>
              <tr>
                <td style={{ padding: "2px 10px", color: theme.colors.text.secondary }}>日</td>
                <td style={tdPillar(theme.colors.semantic.pillar.day)}>{pillarStr(bazi.day_pillar)}</td>
              </tr>
              <tr>
                <td style={{ padding: "2px 10px", color: theme.colors.text.secondary }}>时</td>
                <td style={tdPillar(theme.colors.semantic.pillar.hour)}>{pillarStr(bazi.hour_pillar)}</td>
              </tr>
            </tbody>
          </table>

          <div style={{ display: "flex", gap: 12, flexWrap: "wrap", margin: "8px 0", fontSize: 13, color: theme.colors.text.primary }}>
            <span><span style={{ color: theme.colors.accent.primary }}>年柱纳音: </span>{bazi.year_pillar.nayin || "—"}</span>
            <span><span style={{ color: theme.colors.accent.primary }}>月柱纳音: </span>{bazi.month_pillar.nayin || "—"}</span>
            <span><span style={{ color: theme.colors.accent.primary }}>日柱纳音: </span>{bazi.day_pillar.nayin || "—"}</span>
            <span><span style={{ color: theme.colors.accent.primary }}>时柱纳音: </span>{bazi.hour_pillar.nayin || "—"}</span>
          </div>

          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>关键点</h4>
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

      {tab === "dayun" && (
        <div>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>大运（每十年一运）</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: theme.fontSize.md }}>
            <thead>
              <tr style={{ borderBottom: `1px solid ${theme.colors.border.default}` }}>
                <TH>年龄</TH>
                <TH>大运</TH>
              </tr>
            </thead>
            <tbody>
              {bazi.dayun.map((d, i) => (
                <tr key={i} style={{ borderBottom: `1px solid ${theme.colors.bg.primary}` }}>
                  <TD>{d.age}-{d.age + 9}</TD>
                  <TD>{d.heavenly_stem}{d.earthly_branch}</TD>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {tab === "shishen" && (
        <div>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>十神</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: theme.fontSize.md }}>
            <thead>
              <tr style={{ borderBottom: `1px solid ${theme.colors.border.default}` }}>
                <TH>柱</TH>
                <TH>天干</TH>
                <TH>十神</TH>
              </tr>
            </thead>
            <tbody>
              {bazi.shishen.map((s, i) => (
                <tr key={i} style={{ borderBottom: `1px solid ${theme.colors.bg.primary}` }}>
                  <TD>{s.pillar_name}</TD>
                  <TD>{s.stem}</TD>
                  <td style={{ padding: "4px 8px", color: theme.colors.accent.primary, fontSize: theme.fontSize.md }}>{s.shishen}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, margin: "12px 0 8px" }}>藏干</h4>
          <div style={{ display: "flex", flexWrap: "wrap", gap: theme.spacing.sm, fontSize: theme.fontSize.md }}>
            {bazi.hidden_stems.map((h, i) => (
              <div key={i} style={{ padding: "4px 8px", background: theme.colors.bg.primary, borderRadius: theme.radius.sm }}>
                <span style={{ color: theme.colors.text.secondary }}>{h.branch_name}: </span>
                <span style={{ color: theme.colors.text.primary }}>{h.stems.join(" ")}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {tab === "life" && (
        <div>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>长生十二宫（日干 {bazi.day_pillar.heavenly_stem}）</h4>
          <div style={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: theme.spacing.xs, fontSize: theme.fontSize.md }}>
            {bazi.life_cycle.map((l, i) => (
              <div
                key={i}
                style={{
                  padding: "4px 8px",
                  background: l.stage === "长生" || l.stage === "帝旺" || l.stage === "临官" ? theme.colors.semantic.lifeCycle.goodBg : theme.colors.semantic.lifeCycle.badBg,
                  borderRadius: theme.radius.sm,
                  display: "flex",
                  justifyContent: "space-between",
                }}
              >
                <span style={{ color: theme.colors.text.secondary }}>{l.branch_name}</span>
                <span
                  style={{
                    color:
                      l.stage === "长生" || l.stage === "帝旺" || l.stage === "临官"
                        ? theme.colors.semantic.lifeCycle.goodText
                        : l.stage === "死" || l.stage === "墓" || l.stage === "绝"
                          ? theme.colors.semantic.lifeCycle.badText
                          : theme.colors.text.primary,
                  }}
                >
                  {l.stage}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {tab === "liuxian" && (
        <div>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>小限</h4>
          <p style={{ fontSize: theme.fontSize.md, color: theme.colors.text.primary, marginBottom: theme.spacing.md }}>
            本年小限: {xiaoxianResult[0]} ({xiaoxianResult[1]}岁)
          </p>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>月限</h4>
          <p style={{ fontSize: theme.fontSize.md, color: theme.colors.text.primary, marginBottom: theme.spacing.md }}>
            本月月限: {yuexianResult[0]}
          </p>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>洞微飞限</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: theme.fontSize.md }}>
            <thead>
              <tr style={{ borderBottom: `1px solid ${theme.colors.border.default}` }}>
                <TH>起始年龄</TH>
                <TH>限</TH>
                <TH>地支</TH>
              </tr>
            </thead>
            <tbody>
              {dongweifeixianResult.map((p, i) => (
                <tr key={i} style={{ borderBottom: `1px solid ${theme.colors.bg.primary}` }}>
                  <TD>{p[0]}岁</TD>
                  <TD>{p[1]}</TD>
                  <td style={{ padding: "4px 8px", color: theme.colors.accent.primary, fontSize: theme.fontSize.md }}>{p[2]}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {tab === "wuxing" && (
        <div>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>十干化曜（日干 {bazi.day_pillar.heavenly_stem}）</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: theme.fontSize.md }}>
            <thead>
              <tr style={{ borderBottom: `1px solid ${theme.colors.border.default}` }}>
                <TH>天干</TH>
                <TH>化曜</TH>
              </tr>
            </thead>
            <tbody>
              {shiganhuayao.map(([stem, yao], i) => (
                <tr key={i} style={{ borderBottom: `1px solid ${theme.colors.bg.primary}` }}>
                  <TD>{stem}</TD>
                  <td style={{ padding: "4px 8px", color: theme.colors.accent.primary, fontSize: theme.fontSize.md }}>{yao}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, margin: "12px 0 8px" }}>喜忌格</h4>
          <table style={{ borderCollapse: "collapse", width: "100%", fontSize: theme.fontSize.md }}>
            <thead>
              <tr style={{ borderBottom: `1px solid ${theme.colors.border.default}` }}>
                <TH>星体</TH>
                <TH>喜忌</TH>
              </tr>
            </thead>
            <tbody>
              {xijige.map(([body, rel], i) => (
                <tr key={i} style={{ borderBottom: `1px solid ${theme.colors.bg.primary}` }}>
                  <TD>{body}</TD>
                  <td style={{ padding: "4px 8px", color: rel === "喜" ? theme.colors.semantic.green : rel === "忌" ? theme.colors.semantic.red : theme.colors.text.primary, fontSize: theme.fontSize.md }}>{rel}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}

export default memo(BaziPanel);
