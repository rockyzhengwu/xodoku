import { atom } from "jotai";
import newCell from "./cell.js";

export const selectedCellAtom = atom(null);
export const selectedCellListAtom = atom([]);

export const sudokuCellsAtom = atom([]);
export const sudokuSolutionAtom = atom([]);
export const autoPmsAtom = atom(true);
export const modeAtom = atom("digit");
export const usedSecondsAtom = atom(0);
export const nextStepAtom = atom(null);

export const timerRuningAtom = atom(false);

function parseDigit(digit) {
  const v = parseInt(digit);
  if (v >= 0 && v <= 9) {
    return v.toString();
  }
  if (v >= "0" && v <= "9") {
    return v;
  }
  return "0";
}

export function initCells(input) {
  if (input.length < 81) {
  }
  const cells = [];
  for (let i = 0; i < 81; i++) {
    const digit = parseDigit(input[i]);
    const cell = newCell(i, digit, [], digit !== "0");
    cells.push(cell);
  }

  return cells;
}

export function resetCells(cells) {
  const newCells = cells.map((cell) => {
    if (cell.isGiven) {
      return { ...cell, isSelectedSame: false };
    } else {
      return {
        ...cell,
        digit: "0",
        pms: [],
        color: 0,
        isValidDigit: true,
        isSelectedSame: false,
      };
    }
  });
  return newCells;
}

export function initCellsSudoku(sudoku) {
  const digits = sudoku.digits;
  const pms = sudoku.pms;
  const cells = [];
  let is_given = sudoku.is_given;
  for (let i = 0; i < 81; i++) {
    const digit = digits[i].toString();
    let pm = pms[i].split("");
    let given = is_given[i];
    const cell = newCell(i, digit, pm, given);
    cells.push(cell);
  }

  return cells;
}
