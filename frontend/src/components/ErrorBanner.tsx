import type { ClientError } from "../platform";

interface Props {
  error?: ClientError;
  onRetry?: () => void;
  onDismiss?: () => void;
}

/** Surfaces a generation/IPC failure with a retry affordance (FR-017, SC-006). */
export function ErrorBanner({ error, onRetry, onDismiss }: Props) {
  if (!error) return null;
  return (
    <div className="error-banner" role="alert" data-testid="error-banner">
      <span className="error-banner__msg">
        Something went wrong ({error.code}): {error.message}
      </span>
      <span className="error-banner__actions">
        {error.retryable && onRetry && (
          <button type="button" onClick={onRetry} data-testid="retry">
            Retry
          </button>
        )}
        {onDismiss && (
          <button type="button" onClick={onDismiss} aria-label="dismiss">
            ✕
          </button>
        )}
      </span>
    </div>
  );
}
