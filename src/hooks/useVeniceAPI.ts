import {invoke} from '@tauri-apps/api/core';

import type {Answer} from '../types';

interface UseVeniceAPIResult {
  analyzeScreenshot: (imageBase64: string) => Promise<Answer>;
}

export function useVeniceAPI(): UseVeniceAPIResult {
  async function analyzeScreenshot(imageBase64: string): Promise<Answer> {
    return await invoke<Answer>('analyze_screenshot', {
      imageBase64,
      apiKey: '',
    });
  }

  return {analyzeScreenshot};
}
