"use client";
import { useRef } from "react";
import {
  editCellsAtom,
  editSelectedCellAtom,
} from "../../../atom/EditorAtom.js";
import { useAtom } from "jotai";
import { useAtomCallback } from "jotai/utils";
import {
  sudokuCellsAtom,
  sudokuSolutionAtom,
} from "../../../atom/PlayerAtoms.js";
import Board from "../../../components/board/Board.jsx";
import { cellsValueString } from "../../../atom/cell.js";
import { computePms } from "../../../lib/sudoku_util.js";

import { Button, Group } from "@mantine/core";
import { useClipboard } from "@mantine/hooks";
import useRustWasm from "../../../util/rustWasm.js";
import { notifications } from "@mantine/notifications";
import { useRouter } from "next/navigation";

export default function Editor({ sudokuImage }) {
  const clipboard = useClipboard({ timeout: 500 });

  const router = useRouter();

  const { rust } = useRustWasm("../wasm/sudoku_wasm.js");
  const [, setSudokuCells] = useAtom(sudokuCellsAtom);
  const [, setSudokuSolution] = useAtom(sudokuSolutionAtom);

  const [editSudokuCells, setEditSudokuCells] = useAtom(editCellsAtom);
  const [editSelectedCell, setEditSelectedCell] = useAtom(editSelectedCellAtom);
  const gridRef = useRef(null);

  const handlePlay = useAtomCallback((get) => {
    const cells = get(editCellsAtom);
    const digitStr = cellsValueString(cells);
    let solution = {};
    try {
      solution = rust.solve_backtracing(digitStr);
    } catch (error) {
      notifications.show({
        color: "red",
        title: "Invalid Sudoku",
        message: "Sudoku Has no Unique Solution",
      });
      return;
    }
    if (solution.count != 1) {
      notifications.show({
        color: "red",
        title: "Invalid Sudoku",
        message: "Sudoku Has no Unique Solution",
      });
      return false;
    } else {
      setSudokuSolution(solution.solutions);
    }
    const newCells = cells.map((cell) => ({
      ...cell,
      isGiven: cell.digit !== "0",
    }));
    const cellsWithPms = computePms(newCells);
    setSudokuCells(cellsWithPms);
    router.push("/");
  });

  const handleKeyUp = (event) => {
    let key = event.key;
    if (key === "Delete") {
      key = "0";
    }

    const validDigits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    if (validDigits.includes(key)) {
      if (editSelectedCell == null) {
        return;
      }
      const newCells = editSudokuCells.map((cell) => {
        if (cell.index === editSelectedCell) {
          return { ...cell, digit: key };
        } else {
          return cell;
        }
      });
      setEditSudokuCells(newCells);
    }
  };
  const handleCopy = (event) => {
    event.stopPropagation();
    const digits = editSudokuCells.map((cell) => {
      return cell.digit;
    });
    const content = digits.join("");
    if (!navigator.clipboard && document.execCommand) {
      try {
        const tempTextArea = document.createElement("textarea");
        tempTextArea.value = content;
        document.body.appendChild(tempTextArea);
        tempTextArea.select();
        document.execCommand("copy");
        document.body.removeChild(tempTextArea);
        clipboard.copied = true; // Use the hook to update state
        notifications.show({
          color: "blue",
          title: "Copied",
          message: "Sudoku copied",
        });
      } catch (err) {
        console.error("Fallback copy failed:", err);
      }
    } else {
      clipboard.copy(content);
    }
  };
  return (
    <>
      <div className="flex w-full flex-col items-start">
        <h3>Scanner Preview </h3>
        <p className="text-gray-500">Select cells and correct the scanned digits before playing.</p>
        <div className="grid w-full gap-4 lg:grid-cols-2">
          <div
            className="min-w-0 outline-none"
            ref={gridRef}
            onKeyUp={handleKeyUp}
            onMouseUpCapture={() => gridRef.current?.focus()}
            tabIndex="0"
          >
            <Board
              sudokuCellsAtom={editCellsAtom}
              selectedCellAtom={editSelectedCellAtom}
              onCellSelect={() => gridRef.current?.focus()}
            />
          </div>
          <div className="flex min-h-64 min-w-0 rounded-xl border border-gray-200 p-4 shadow-sm">
            <img src={sudokuImage} alt="Uploaded Sudoku preview" className="h-auto max-h-[60vh] w-full object-contain"></img>
          </div>
        </div>
        <Group className="mt-4 w-full" justify="center">
          <Button onClick={handlePlay}>Play</Button>
          <Button
            onClick={handleCopy}
            color={clipboard.copied ? "blue" : "teal"}
          >
            {clipboard.copied ? "Copied" : "Copy"}
          </Button>
        </Group>
      </div>
    </>
  );
}
