import {invoke} from '@tauri-apps/api/core';

import type {Answer} from '../types';
import {LoadingSpinner} from './LoadingSpinner';

interface AnswerPanelProps {
  answer: Answer | null;
  isLoading: boolean;
  onClose: () => void;
}

function isCorrectOption(option: string, answer: string): boolean {
  const normalizedAnswer = answer.trim().replace(/\.$/, '').toUpperCase();
  return option.trim().toUpperCase().startsWith(`${normalizedAnswer}.`) ||
    option.trim().toUpperCase().startsWith(`${normalizedAnswer})`);
}

export function AnswerPanel({answer, isLoading, onClose}: AnswerPanelProps) {
  async function copyAnswer() {
    if (!answer) {
      return;
    }
    await invoke('copy_to_clipboard', {text: answer.answer});
  }

  return (
    <main className="min-h-screen bg-ace-bg p-4 text-ace-text">
      <section className="rounded-2xl border border-ace-border bg-ace-card p-5 shadow-glow">
        <div className="mb-4 flex items-start justify-between gap-4">
          <div>
            <p className="text-xs font-semibold uppercase tracking-[0.28em] text-ace-accent">ace</p>
            <h1 className="mt-1 text-lg font-semibold">Screenshot Answer</h1>
          </div>
          <button
            className="rounded-lg border border-ace-border px-3 py-1 text-sm text-ace-muted transition hover:border-ace-accent hover:text-ace-text"
            onClick={onClose}
            type="button"
          >
            Close
          </button>
        </div>

        {isLoading ? (
          <div className="flex min-h-40 items-center justify-center">
            <LoadingSpinner />
          </div>
        ) : null}

        {!isLoading && answer ? (
          <div className="space-y-4">
            <div>
              <h2 className="mb-2 text-sm font-semibold text-ace-muted">Question</h2>
              <p className="text-base leading-relaxed">{answer.question}</p>
            </div>

            <div className="space-y-2">
              <h2 className="text-sm font-semibold text-ace-muted">Options</h2>
              {answer.options.map((option) => {
                const correct = isCorrectOption(option, answer.answer);
                return (
                  <div
                    className={`rounded-xl border px-3 py-2 text-sm ${
                      correct
                        ? 'border-ace-accent bg-ace-accent/15 text-white'
                        : 'border-ace-border bg-ace-bg/60 text-ace-text'
                    }`}
                    key={option}
                  >
                    {option}
                  </div>
                );
              })}
            </div>

            <button
              className="w-full rounded-xl border border-ace-accent bg-ace-accent/15 px-4 py-3 text-left transition hover:bg-ace-accent/25"
              onClick={copyAnswer}
              type="button"
            >
              <span className="block text-xs font-semibold uppercase tracking-[0.2em] text-ace-accent">
                Correct answer — click to copy
              </span>
              <span className="mt-1 block font-mono text-2xl font-bold text-white">{answer.answer}</span>
            </button>

            {answer.explanation ? (
              <div>
                <h2 className="mb-2 text-sm font-semibold text-ace-muted">Explanation</h2>
                <p className="text-sm leading-relaxed text-ace-text">{answer.explanation}</p>
              </div>
            ) : null}
          </div>
        ) : null}

        {!isLoading && !answer ? (
          <p className="rounded-xl border border-ace-border bg-ace-bg/70 px-4 py-5 text-sm text-ace-muted">
            Use the ace tray menu to capture a question.
          </p>
        ) : null}
      </section>
    </main>
  );
}
