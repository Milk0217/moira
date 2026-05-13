import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ChartData, BirthInfo } from "./types/chart";
import InputForm from "./components/InputForm";
import ElectionalForm from "./components/ElectionalForm";
import AstrologyChart from "./components/AstrologyChart";
import PlanetTable from "./components/PlanetTable";
import BaziPanel from "./components/BaziPanel";
import ErrorBoundary from "./components/ErrorBoundary";
import ChartManager from "./components/ChartManager";
import { theme } from "./theme";

function App() {
  const [chartData, setChartData] = useState<ChartData | null>(null);
  const [loading, setLoading] = useState(false);
  const [birthInfo, setBirthInfo] = useState<BirthInfo | null>(null);
  const [mode, setMode] = useState<"natal" | "electional">("natal");

  const handleCalculate = useCallback(async (info: BirthInfo) => {
    setLoading(true);
    setBirthInfo(info);
    try {
      const data = await invoke<ChartData>("calculate", {
        year: info.year,
        month: info.month,
        day: info.day,
        hour: info.hour,
        minute: info.minute,
        second: info.second,
        timezone: info.timezone,
        latitude: info.latitude,
        longitude: info.longitude,
        dstApplied: info.dst_applied,
      });
      setChartData(data);
    } catch (e) {
      console.error("计算失败:", e);
      alert(`计算失败: ${e}`);
    } finally {
      setLoading(false);
    }
  }, []);

  const chartBodies = chartData
    ? [
        ...chartData.bodies.map((b) => ({
          name: b.name,
          longitude: b.longitude,
          detail: `${b.name}\n黄经 ${b.longitude.toFixed(1)}° 黄纬 ${b.latitude.toFixed(1)}°\n${b.mansion_name}宿 ${b.mansion_degree.toFixed(1)}°`,
        })),
        ...chartData.extra_bodies.map((e) => ({
          name: e.name,
          longitude: e.longitude,
          detail: `${e.name}\n黄经 ${e.longitude.toFixed(1)}°\n${e.mansion_name}宿 ${e.mansion_degree.toFixed(1)}°`,
        })),
      ]
    : [];

  const handleLoadChart = useCallback((data: ChartData) => {
    setChartData(data);
  }, []);

  async function handleExport() {
    if (!chartData) return;
    const stage = document.querySelector("canvas")?.parentElement;
    if (!stage) return;
    const uri = stage.querySelector("canvas")?.toDataURL("image/png");
    if (!uri) return;
    const a = document.createElement("a");
    a.href = uri;
    a.download = `moira-${birthInfo?.year ?? "chart"}.png`;
    a.click();
  }

  const modeBtnStyle = (isActive: boolean): React.CSSProperties => ({
    flex: 1, padding: "6px 0", fontSize: theme.fontSize.sm, borderRadius: theme.radius.sm, cursor: "pointer",
    border: `1px solid ${isActive ? theme.colors.accent.primary : theme.colors.border.default}`,
    background: isActive ? theme.colors.accent.dark : "transparent",
    color: isActive ? theme.colors.accent.primary : theme.colors.text.secondary,
  });

  const zodiacBtnStyle = (isActive: boolean): React.CSSProperties => ({
    padding: "4px 12px",
    fontSize: theme.fontSize.sm,
    borderRadius: theme.radius.sm,
    border: `1px solid ${isActive ? theme.colors.accent.primary : theme.colors.border.muted}`,
    background: isActive ? theme.colors.accent.dark : "transparent",
    color: isActive ? theme.colors.accent.primary : theme.colors.text.secondary,
    cursor: "pointer",
  });

  return (
    <div style={{ display: "flex", height: "100vh", width: "100vw" }}>
      <div
        style={{
          width: 320,
          minWidth: 320,
          borderRight: `1px solid ${theme.colors.border.light}`,
          padding: theme.spacing.xl,
          overflowY: "auto",
          background: theme.colors.bg.secondary,
        }}
      >
        <div style={{ display: "flex", gap: theme.spacing.xs, marginBottom: theme.spacing.lg }}>
          <button onClick={() => setMode("natal")} style={modeBtnStyle(mode === "natal")}>七政四馀</button>
          <button onClick={() => setMode("electional")} style={modeBtnStyle(mode === "electional")}>天星择日</button>
        </div>
        <h1
          style={{
            fontSize: theme.fontSize.xxl,
            fontWeight: "bold",
            marginBottom: theme.spacing.xl,
            color: theme.colors.accent.primary,
            letterSpacing: 2,
          }}
        >
          Moira 星盘
        </h1>
        {mode === "natal" ? (
          <InputForm onCalculate={handleCalculate} loading={loading} />
        ) : (
          <ElectionalForm onCalculate={handleCalculate} loading={loading} />
        )}
        <ChartManager onLoad={handleLoadChart} currentData={chartData} />
      </div>

      <div
        style={{
          flex: 1,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          padding: theme.spacing.xl,
          overflow: "auto",
          background: theme.colors.bg.primary,
        }}
      >
        {chartData ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              gap: theme.spacing.xxl,
            }}
          >
            {birthInfo && (
              <div style={{ fontSize: theme.fontSize.md, color: theme.colors.text.secondary, textAlign: "center" }}>
                {birthInfo.year}-{String(birthInfo.month).padStart(2, "0")}-
                {String(birthInfo.day).padStart(2, "0")}{" "}
                {String(birthInfo.hour).padStart(2, "0")}:
                {String(birthInfo.minute).padStart(2, "0")} UTC{birthInfo.timezone >= 0 ? "+" : ""}
                {birthInfo.timezone}
              </div>
            )}
            <ErrorBoundary
              fallback={
                <div style={{ color: theme.colors.semantic.error, padding: 40 }}>星盘渲染失败</div>
              }
            >
              <AstrologyChart bodies={chartBodies} houses={chartData.houses} size={500} />
            </ErrorBoundary>
            <div style={{ display: "flex", gap: theme.spacing.sm }}>
              <button onClick={handleExport} style={btnStyle}>
                导出 PNG
              </button>
            </div>
            <div style={{ display: "flex", gap: theme.spacing.sm, alignItems: "center" }}>
              <span style={{ color: theme.colors.text.secondary, fontSize: theme.fontSize.md }}>星制:</span>
              <button
                onClick={() => {}}
                style={zodiacBtnStyle(chartData.zodiac_type === "回归")}
              >
                回归制 {chartData.zodiac_type === "回归" ? "✓" : ""}
              </button>
              <span style={{ color: theme.colors.text.tertiary, fontSize: theme.fontSize.xs }}>
                岁差: {chartData.ayanamsa.toFixed(2)}°
              </span>
            </div>
            <PlanetTable
              bodies={chartData.bodies}
              extras={chartData.extra_bodies}
              aspects={chartData.aspects}
              houses={chartData.houses}
              shen_sha={chartData.shen_sha}
            />
            <BaziPanel
              bazi={chartData.bazi}
              ascendant={chartData.ascendant}
              midheaven={chartData.midheaven}
              partOfFortune={chartData.part_of_fortune}
              mingZhu={chartData.ming_zhu}
              shenZhu={chartData.shen_zhu}
              shiganhuayao={chartData.shiganhuayao}
              xijige={chartData.xijige}
              xiaoxianResult={chartData.xiaoxian_result}
              yuexianResult={chartData.yuexian_result}
              dongweifeixianResult={chartData.dongweifeixian_result}
            />
          </div>
        ) : (
          <div style={{ color: theme.colors.text.muted, fontSize: theme.fontSize.xl, textAlign: "center" }}>
            <p style={{ marginBottom: theme.spacing.md, fontSize: 48 }}>✦</p>
            <p>输入出生信息，点击"计算星盘"</p>
          </div>
        )}
      </div>
    </div>
  );
}

const btnStyle: React.CSSProperties = {
  padding: "6px 16px",
  background: "transparent",
  color: theme.colors.accent.primary,
  border: `1px solid ${theme.colors.accent.primary}`,
  borderRadius: theme.radius.md,
  cursor: "pointer",
  fontSize: theme.fontSize.md,
};

export default App;
