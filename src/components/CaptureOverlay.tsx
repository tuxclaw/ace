import {useEffect, useMemo, useState} from 'react';

import type {SelectionRect} from '../types';
import {LoadingSpinner} from './LoadingSpinner';

interface Point {
  x: number;
  y: number;
}

interface CaptureOverlayProps {
  isLoading: boolean;
  error: string | null;
  onCancel: () => void;
  onSelect: (rect: SelectionRect) => void;
}

const MIN_SELECTION_SIZE = 8;

function toRect(start: Point, end: Point): SelectionRect {
  const x = Math.min(start.x, end.x);
  const y = Math.min(start.y, end.y);
  const width = Math.abs(end.x - start.x);
  const height = Math.abs(end.y - start.y);
  return {x, y, width, height};
}

export function CaptureOverlay({isLoading, error, onCancel, onSelect}: CaptureOverlayProps) {
  const [start, setStart] = useState<Point | null>(null);
  const [current, setCurrent] = useState<Point | null>(null);

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === 'Escape') {
        onCancel();
      }
    }

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onCancel]);

  const selection = useMemo(() => {
    if (!start || !current) {
      return null;
    }
    return toRect(start, current);
  }, [current, start]);

  function handleMouseDown(event: React.MouseEvent<HTMLDivElement>) {
    if (isLoading) {
      return;
    }

    const point = {x: event.clientX, y: event.clientY};
    setStart(point);
    setCurrent(point);
  }

  function handleMouseMove(event: React.MouseEvent<HTMLDivElement>) {
    if (!start || isLoading) {
      return;
    }
    setCurrent({x: event.clientX, y: event.clientY});
  }

  function handleMouseUp() {
    if (!selection || isLoading) {
      return;
    }

    setStart(null);
    setCurrent(null);

    if (selection.width >= MIN_SELECTION_SIZE && selection.height >= MIN_SELECTION_SIZE) {
      onSelect({
        x: Math.round(selection.x),
        y: Math.round(selection.y),
        width: Math.round(selection.width),
        height: Math.round(selection.height),
      });
    }
  }

  return (
    <div
      className="fixed inset-0 cursor-crosshair select-none bg-black/65 text-ace-text"
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
    >
      <div className="pointer-events-none fixed left-6 top-6 rounded-xl border border-ace-border bg-ace-card/90 px-4 py-3 shadow-glow backdrop-blur">
        {isLoading ? (
          <LoadingSpinner />
        ) : (
          <div>
            <p className="text-sm font-semibold text-ace-text">Drag to capture a question</p>
            <p className="mt-1 text-xs text-ace-muted">Press Escape to cancel.</p>
          </div>
        )}
      </div>

      {error ? (
        <div className="pointer-events-none fixed bottom-6 left-6 max-w-lg rounded-xl border border-red-500/40 bg-red-950/85 px-4 py-3 text-sm text-red-100 shadow-lg">
          {error}
        </div>
      ) : null}

      {selection ? (
        <div
          className="pointer-events-none fixed border-2 border-dashed border-ace-accent bg-ace-accent/20 shadow-glow"
          style={{
            left: `${selection.x}px`,
            top: `${selection.y}px`,
            width: `${selection.width}px`,
            height: `${selection.height}px`,
          }}
        />
      ) : null}
    </div>
  );
}
