import { Component, ReactNode } from "react";
import { theme } from "../theme";

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export default class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, error: null };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  render() {
    if (this.state.hasError) {
      return (
        this.props.fallback || (
          <div
            style={{
              padding: theme.spacing.xl,
              color: theme.colors.semantic.error,
              textAlign: "center",
            }}
          >
            <p>渲染异常</p>
            <p style={{ fontSize: theme.fontSize.sm, color: theme.colors.text.secondary }}>
              {this.state.error?.message}
            </p>
          </div>
        )
      );
    }
    return this.props.children;
  }
}
