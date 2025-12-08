export default function newCell(index, digit, pms, isGiven) {
  const row = Math.floor(index / 9) + 1;
  const col = (index % 9) + 1;
  const box = Math.floor((row - 1) / 3) * 3 + Math.floor((col - 1) / 3) + 1;
  const x = (col - 1) * 100 + 20;
  const y = (row - 1) * 100 + 20;
  return {
    index: index,
    col: col,
    box: box,
    row: row,
    digit: digit,
    x: x,
    y: y,
    isGiven: isGiven,
    pms: pms,
    userSetPms: false,
    color: 0,
    isSelectedBuddy: false,
    isSelected: false,
    isSelectedSame: false,
    isValidDigit: true,
  };
}

export function cellsValueString(cells) {
  const digits = cells.map((cell) => cell.digit);
  return digits.join("");
}
