export const pmBackgroundCircleSize = 12;

export const pmPositionOffset = {
  1: [25, 25],
  2: [50, 25],
  3: [75, 25],
  4: [25, 50],
  5: [50, 50],
  6: [75, 50],
  7: [25, 75],
  8: [50, 75],
  9: [75, 75],
};

export const colorPosition = [
  { code: 1, x: 0, y: 0, color: "rgb(255,192,89)" },
  { code: 2, x: 100, y: 0, color: "rgb(177, 165, 243)" },
  { code: 3, x: 200, y: 0, color: "rgb(247, 165, 167)" },
  { code: 4, x: 0, y: 100, color: "rgb(134, 232, 208)" },
  { code: 5, x: 100, y: 100, color: "rgb(134, 242, 128)" },
  { code: 6, x: 200, y: 100, color: "rgb(215, 255, 215)" },
  { code: 7, x: 0, y: 200, color: "rgb(51, 204, 255)" },
  { code: 8, x: 100, y: 200, color: "rgb(247, 247, 197)" },
  { code: 9, x: 200, y: 200, color: "rgb(247, 228, 197)" },
];

export const colorPositionMobile = [
  { code: 1, x: 0, y: 0, color: "rgb(255,192,89)" },
  { code: 2, x: 100, y: 0, color: "rgb(177, 165, 243)" },
  { code: 3, x: 200, y: 0, color: "rgb(247, 165, 167)" },
  { code: 4, x: 300, y: 0, color: "rgb(134, 232, 208)" },
  { code: 5, x: 400, y: 0, color: "rgb(134, 242, 128)" },
  { code: 6, x: 500, y: 0, color: "rgb(215, 255, 215)" },
  { code: 7, x: 600, y: 0, color: "rgb(51, 204, 255)" },
  { code: 8, x: 700, y: 0, color: "rgb(247, 247, 197)" },
  { code: 9, x: 800, y: 0, color: "rgb(247, 228, 197)" },
];

export const cellPosition = (index) => {
  const row = Math.floor(index / 9) + 1;
  const col = (index % 9) + 1;
  const x = (col - 1) * 100 + 20;
  const y = (row - 1) * 100 + 20;
  return [x, y];
};

export const pmPosition = (index, pm) => {
  const pos = cellPosition(index);
  const pmOffset = pmPositionOffset[pm];
  const x = pos[0] + pmOffset[0];
  const y = pos[1] + pmOffset[1];
  return [x, y];
};

export const StepHintValueColor = "rgb(63, 218, 101)";
export const StepDeleteCandidateColor = "rgb(255, 118, 132)";
export const StepFinColor = "rgb(127, 187, 255)";
export const StepEndoFinColor = "rgb(216, 178, 255)";
export const StepHintAlsColors = [
  "rgb(197, 232, 140)",
  "rgb(255, 203, 203)",
  "rgb(178, 223, 223)",
  "rgb(252, 220, 165)",
];
export function AlsColor(index) {
  return StepHintAlsColors[index % 4];
}
