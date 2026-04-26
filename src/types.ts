export interface Answer {
  question: string;
  options: string[];
  answer: string;
  explanation: string;
}

export interface SelectionRect {
  x: number;
  y: number;
  width: number;
  height: number;
}
