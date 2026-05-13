import { theme } from "../theme";

export function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div style={{ marginBottom: theme.spacing.md }}>
      <label
        style={{
          display: "block",
          fontSize: theme.fontSize.sm,
          color: theme.colors.text.secondary,
          marginBottom: theme.spacing.xs,
        }}
      >
        {label}
      </label>
      {children}
    </div>
  );
}

export const inputStyle: React.CSSProperties = {
  width: "100%",
  padding: "8px 10px",
  boxSizing: "border-box",
  background: theme.colors.bg.input,
  color: theme.colors.text.primary,
  border: `1px solid ${theme.colors.border.default}`,
  borderRadius: theme.radius.sm,
  fontSize: theme.fontSize.md,
  outline: "none",
};

export function SectionTitle({ children }: { children: React.ReactNode }) {
  return (
    <h3 style={{ color: theme.colors.accent.primary, marginTop: 20, marginBottom: 12, fontSize: theme.fontSize.xl, fontWeight: "bold" }}>
      {children}
    </h3>
  );
}

export function TH({ children }: { children: React.ReactNode }) {
  return (
    <th style={{ padding: "6px 8px", textAlign: "left", color: theme.colors.table.th, fontSize: theme.fontSize.md }}>
      {children}
    </th>
  );
}

export function TD({ children }: { children: React.ReactNode }) {
  return (
    <td style={{ padding: "6px 8px", color: theme.colors.table.td, fontSize: theme.fontSize.md }}>
      {children}
    </td>
  );
}

export function MansionTD({ children }: { children: React.ReactNode }) {
  return (
    <td style={{ padding: "6px 8px", color: theme.colors.table.mansionTd, fontSize: theme.fontSize.md }}>
      {children}
    </td>
  );
}
