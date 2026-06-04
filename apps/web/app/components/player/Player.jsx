"use client";

import Board from "../../components/board/Board.jsx";
import PlayerControl from "../../components/playerControl/PlayerControl.jsx";
import { useAtom, useAtomValue } from "jotai";
import {
  sudokuCellsAtom,
  selectedCellAtom,
  autoPmsAtom,
} from "../../atom/PlayerAtoms.js";

import { undoStack, redoStack } from "./PlayerHistory.js";
import { SetDigitCommand } from "./command.js";
import { useCallback, useEffect, useRef } from "react";

export default function Player() {
  const [selectedCell, setSelectedCell] = useAtom(selectedCellAtom);
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);
  const autoPms = useAtomValue(autoPmsAtom);
  const gridRef = useRef(null);

  const handleClick = () => {
    setSelectedCell(null);
    const updatedCells = sudokuCells.map((cell) => {
      return {
        ...cell,
        isSelected: false,
        isSelectedBuddy: false,
        isSelectedSame: false,
      };
    });
    setSudokuCells(updatedCells);
  };

  const handleKeyUp = useCallback((event) => {
    event.preventDefault();
    const key = event.key;
    const validDigits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    if (validDigits.includes(key)) {
      if (selectedCell == null) {
        return;
      }

      const cell = sudokuCells[selectedCell];
      if (cell.isGiven) {
        return;
      }

      redoStack.clear();
      const digit = key;
      const newDigit = cell.digit === digit ? "0" : digit;
      const command = new SetDigitCommand(
        cell.index,
        newDigit,
        cell.digit,
        cell.pms,
        cell.isValidDigit,
      );
      command.execute(sudokuCells, setSudokuCells);
      undoStack.push(command);
    }
  }, [selectedCell, setSudokuCells, sudokuCells]);

  useEffect(() => {
    window.addEventListener("keyup", handleKeyUp);
    return () => window.removeEventListener("keyup", handleKeyUp);
  }, [handleKeyUp]);

  return (
      <div className="mx-auto grid w-full max-w-7xl select-none items-start gap-4 lg:grid-cols-[minmax(0,1fr)_304px]" onClick={handleClick}>
        <div className="flex min-w-0 justify-center">
          <div
            ref={gridRef}
            data-testid="sudoku-grid"
            className="w-full max-w-[min(72vh,760px)] rounded-2xl border border-[var(--panel-border-color)] bg-[var(--panel-background-color)] p-3 shadow-sm outline-none"
            onMouseUpCapture={() => gridRef.current?.focus()}
            tabIndex={0}
          >
            <Board
              sudokuCellsAtom={sudokuCellsAtom}
              selectedCellAtom={selectedCellAtom}
              autoPms={autoPms}
              player={true}
              onCellSelect={() => gridRef.current?.focus()}
            />
          </div>
        </div>
        <div className="rounded-2xl border border-[var(--panel-border-color)] bg-[var(--panel-background-color)] p-3 shadow-sm">
          <PlayerControl />
        </div>
      </div>
  );
}
