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

import { Button, Space } from "@mantine/core";
import { useClipboard } from "@mantine/hooks";
import useRustWasm from "../../../util/rustWasm.js";
import { notifications } from "@mantine/notifications";
import { useRouter } from "next/navigation";

import styles from "./Editor.module.css";

export default function Editor({ sudokuImage }) {
  const clipboard = useClipboard({ timeout: 500 });

  const router = useRouter();

  const { loading, error, rust } = useRustWasm("../wasm/sudoku_wasm.js");
  const [_, setSudokuCells] = useAtom(sudokuCellsAtom);
  const [sudokuSolution, setSudokuSolution] = useAtom(sudokuSolutionAtom);

  const [editSudokuCells, setEditSudokuCells] = useAtom(editCellsAtom);
  const [editSelectedCell, setEditSelectedCell] = useAtom(editSelectedCellAtom);
  const inputRef = useRef(null);

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
    const newCells = cells.map((cell, index) => {
      if (cell["digit"] !== "0") {
        cell["isGiven"] = true;
      }
      //cell.pms = pm;
      return cell;
    });
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
      if (!editSelectedCell) {
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
  const handleMouseUp = (event) => {
    inputRef.current.focus();
  };

  return (
    <>
      <div className={styles.editor}>
        <h3>Scanner Preview </h3>
        <p> select and edit the result of scanner</p>
        <div className={styles.previewSection}>
          <div
            className={styles.gridSection}
            onKeyUp={handleKeyUp}
            tabIndex="0"
            onMouseUp={handleMouseUp}
          >
            <Board
              sudokuCellsAtom={editCellsAtom}
              selectedCellAtom={editSelectedCellAtom}
            />
          </div>
          <div className={styles.imageSection}>
            <img src={sudokuImage} className={styles.previewImage}></img>
          </div>
        </div>
        <div className={styles.actionBar}>
          <Button onClick={handlePlay}>Play</Button>
          <Space w="lg" />
          <Button
            onClick={handleCopy}
            color={clipboard.copied ? "blue" : "teal"}
          >
            {clipboard.copied ? "Copied" : "Copy"}
          </Button>
          <input
            ref={inputRef}
            type="text"
            style={{ opacity: 0, position: "absolute", display: "none" }}
          />
        </div>
      </div>
    </>
  );
}
