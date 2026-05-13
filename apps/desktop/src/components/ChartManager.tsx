import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ChartData } from "../types/chart";
import { theme } from "../theme";

interface Props {
  onLoad: (data: ChartData) => void;
  currentData: ChartData | null;
}

export default function ChartManager({ onLoad, currentData }: Props) {
  const [savedCharts, setSavedCharts] = useState<string[]>([]);
  const [saveName, setSaveName] = useState("");
  const [showManager, setShowManager] = useState(false);

  useEffect(() => { if (showManager) refreshList(); }, [showManager]);

  async function refreshList() {
    const list = await invoke<string[]>("list_charts");
    setSavedCharts(list);
  }

  async function handleSave() {
    if (!currentData || !saveName.trim()) return;
    await invoke("save_chart", { name: saveName.trim(), data: currentData });
    setSaveName("");
    refreshList();
  }

  async function handleLoad(name: string) {
    const data = await invoke<ChartData>("load_chart", { name });
    onLoad(data);
    setShowManager(false);
  }

  async function handleDelete(name: string) {
    await invoke("delete_chart", { name });
    refreshList();
  }

  return (
    <div>
      <button onClick={() => setShowManager(!showManager)} style={toggleBtnStyle}>
        {showManager ? "关闭资料管理" : "资料管理"}
      </button>
      {showManager && (
        <div style={{ marginTop: theme.spacing.md, padding: theme.spacing.md, background: theme.colors.bg.primary, borderRadius: theme.radius.lg, border: `1px solid ${theme.colors.border.default}` }}>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>保存当前星盘</h4>
          <div style={{ display: "flex", gap: theme.spacing.sm, marginBottom: theme.spacing.lg }}>
            <input
              value={saveName}
              onChange={(e) => setSaveName(e.target.value)}
              placeholder="输入星盘名称..."
              style={inputStyle}
            />
            <button onClick={handleSave} disabled={!saveName.trim() || !currentData} style={actionBtnStyle}>
              保存
            </button>
          </div>
          <h4 style={{ color: theme.colors.accent.primary, fontSize: theme.fontSize.lg, marginBottom: theme.spacing.sm }}>已保存的星盘</h4>
          {savedCharts.length === 0 ? (
            <p style={{ color: theme.colors.text.muted, fontSize: theme.fontSize.md }}>暂无保存的星盘</p>
          ) : (
            <ul style={{ listStyle: "none", padding: 0 }}>
              {savedCharts.map((name) => (
                <li key={name} style={{ display: "flex", justifyContent: "space-between", padding: "6px 0", borderBottom: `1px solid ${theme.colors.border.subtle}` }}>
                  <span style={{ color: theme.colors.text.primary, fontSize: theme.fontSize.md }}>{name}</span>
                  <div style={{ display: "flex", gap: 6 }}>
                    <button onClick={() => handleLoad(name)} style={smallBtnStyle}>加载</button>
                    <button onClick={() => handleDelete(name)} style={{ ...smallBtnStyle, color: theme.colors.semantic.error, borderColor: theme.colors.semantic.error }}>删除</button>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  );
}

const toggleBtnStyle: React.CSSProperties = {
  width: "100%", padding: "6px 0", background: "transparent", color: theme.colors.accent.primary,
  border: `1px dashed ${theme.colors.accent.primary}`, borderRadius: theme.radius.md, cursor: "pointer", fontSize: theme.fontSize.md, marginTop: theme.spacing.md,
};

const inputStyle: React.CSSProperties = {
  flex: 1, padding: "6px 10px", background: theme.colors.bg.card, color: theme.colors.text.primary,
  border: `1px solid ${theme.colors.border.default}`, borderRadius: theme.radius.sm, fontSize: theme.fontSize.md, outline: "none",
};

const actionBtnStyle: React.CSSProperties = {
  padding: "6px 14px", background: theme.colors.accent.dark, color: theme.colors.accent.primary,
  border: `1px solid ${theme.colors.accent.primary}`, borderRadius: theme.radius.sm, cursor: "pointer", fontSize: theme.fontSize.sm,
};

const smallBtnStyle: React.CSSProperties = {
  padding: "2px 10px", background: "transparent", color: theme.colors.accent.primary,
  border: `1px solid ${theme.colors.accent.primary}`, borderRadius: theme.radius.sm, cursor: "pointer", fontSize: theme.fontSize.xs,
};
