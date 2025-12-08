import { atom } from "jotai";
import newCell from "./cell.js";

export const editCellsAtom = atom([]);
export const editSelectedCellAtom = atom(null);

export function initCellsFromOcr(cells) {
  const digitCells = [];
  for (let i = 0; i < 81; i++) {
    const cell = cells[i];
    const digit = Math.trunc(cell["digit"]).toString();
    digitCells.push(newCell(i, digit, [], false));
  }
  return digitCells;
}
