interface LoadingSpinnerProps {
  label?: string;
}

export function LoadingSpinner({label = 'Analyzing...'}: LoadingSpinnerProps) {
  return (
    <div className="flex items-center gap-3 text-ace-text">
      <div className="h-5 w-5 animate-spin rounded-full border-2 border-ace-border border-t-ace-accent" />
      <span className="text-sm font-medium tracking-wide">{label}</span>
    </div>
  );
}
