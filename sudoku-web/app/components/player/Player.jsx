"use client";

import Board from "../../components/board/Board.jsx";
import PlayerControl from "../../components/playerControl/PlayerControl.jsx";
import { useAtom } from "jotai";
import {
  sudokuCellsAtom,
  selectedCellAtom,
  autoPmsAtom,
} from "../../atom/PlayerAtoms.js";

import { undoStack, redoStack } from "./PlayerHistory.js";
import { SetDigitCommand } from "./command.js";

import styles from "./Player.module.css";

export default function Player() {
  const [selectedCell, setSelectedCell] = useAtom(selectedCellAtom);
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);
  const [autoPms, _] = useAtom(autoPmsAtom);

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

  const handleKeyUp = (event) => {
    event.preventDefault();
    const key = event.key;
    const validDigits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    if (validDigits.includes(key)) {
      if (!selectedCell) {
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
  };

  return (
    <>
      <div className={styles.player} onClick={handleClick}>
        <div className={styles.digitGrid} onKeyUp={handleKeyUp} tabIndex={0}>
          <Board
            sudokuCellsAtom={sudokuCellsAtom}
            selectedCellAtom={selectedCellAtom}
            autoPms={autoPms}
            player={true}
          />
        </div>
        <div className={styles.playerControl}>
          <PlayerControl />
        </div>
      </div>
    </>
  );
}
