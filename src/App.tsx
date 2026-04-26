import {invoke} from '@tauri-apps/api/core';
import {emit, listen} from '@tauri-apps/api/event';
import {LogicalPosition, LogicalSize, getCurrentWindow} from '@tauri-apps/api/window';
import {useCallback, useEffect, useMemo, useState} from 'react';

import {AnswerPanel} from './components/AnswerPanel';
import {CaptureOverlay} from './components/CaptureOverlay';
import {useVeniceAPI} from './hooks/useVeniceAPI';
import type {Answer, SelectionRect} from './types';

const PANEL_WIDTH = 460;
const PANEL_HEIGHT = 620;
const PANEL_MARGIN = 24;

function isCaptureWindow(): boolean {
  const params = new URLSearchParams(window.location.search);
  return params.get('mode') === 'capture';
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export function App() {
  const captureMode = useMemo(() => isCaptureWindow(), []);
  const appWindow = getCurrentWindow();
  const {analyzeScreenshot} = useVeniceAPI();
  const [answer, setAnswer] = useState<Answer | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const showPanel = useCallback(async () => {
    await appWindow.setSize(new LogicalSize(PANEL_WIDTH, PANEL_HEIGHT));
    await appWindow.setPosition(
      new LogicalPosition(Math.max(PANEL_MARGIN, window.screen.availWidth - PANEL_WIDTH - PANEL_MARGIN), PANEL_MARGIN),
    );
    await appWindow.setAlwaysOnTop(true);
    await appWindow.show();
    await appWindow.setFocus();
  }, [appWindow]);

  useEffect(() => {
    if (captureMode) {
      return undefined;
    }

    const unlistenPromise = listen<Answer>('analysis-complete', async (event) => {
      setAnswer(event.payload);
      setIsLoading(false);
      await showPanel();
    });

    return () => {
      void unlistenPromise.then((unlisten) => unlisten());
    };
  }, [captureMode, showPanel]);

  const closeWindow = useCallback(async () => {
    await appWindow.hide();
  }, [appWindow]);

  const cancelCapture = useCallback(async () => {
    await appWindow.close();
  }, [appWindow]);

  const handleSelect = useCallback(
    async (rect: SelectionRect) => {
      setIsLoading(true);
      setError(null);

      try {
        const imageBase64 = await invoke<string>('capture_screen_region', {
          x: rect.x,
          y: rect.y,
          width: rect.width,
          height: rect.height,
        });
        const analysis = await analyzeScreenshot(imageBase64);
        await emit('analysis-complete', analysis);
        await appWindow.close();
      } catch (caught) {
        setError(errorMessage(caught));
        setIsLoading(false);
      }
    },
    [analyzeScreenshot, appWindow],
  );

  if (captureMode) {
    return (
      <CaptureOverlay
        error={error}
        isLoading={isLoading}
        onCancel={cancelCapture}
        onSelect={handleSelect}
      />
    );
  }

  return <AnswerPanel answer={answer} isLoading={isLoading} onClose={closeWindow} />;
}
