"use client";
import Cell from "./Cell.jsx";
import { useAtom } from "jotai";
import Step from "../../components/step/Step.jsx";
import styles from "./Board.module.css";

export default function Board({
  sudokuCellsAtom,
  selectedCellAtom,
  autoPms,
  player,
}) {
  const [cells, setCells] = useAtom(sudokuCellsAtom);
  const [_, setSelectedCell] = useAtom(selectedCellAtom);

  function handleMouseDown({ index }) {
    const col = cells[index].col;
    const row = cells[index].row;
    const box = cells[index].box;
    const digit = cells[index].digit;
    const updatedCells = cells.map((cell) => {
      const isSelected = cell.index === index;
      let isSelectedBuddy = false;
      let isSelectedSame = false;

      if (cell.row === row || cell.col === col || cell.box === box) {
        isSelectedBuddy = true;
      }
      if (digit && digit !== "0" && cell.digit === digit) {
        isSelectedSame = true;
      }
      return {
        ...cell,
        isSelected: isSelected,
        isSelectedBuddy: isSelectedBuddy,
        isSelectedSame: isSelectedSame,
      };
    });
    setCells(updatedCells);
    setSelectedCell(index);
  }

  function handleMouseOver(event) {}

  return (
    <>
      <svg
        viewBox="0 0 940 940"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        width="100%"
        className={styles["board"]}
      >
        <defs>
          <marker
            id="arrowhead"
            viewBox="0 0 10 10"
            refX="5"
            refY="5"
            markerWidth="6"
            markerHeight="6"
            orient="auto-start-reverse"
          >
            <path d="M 0 0 L 10 5 L 0 10 z" fill="red" />
          </marker>
        </defs>
        <g className={styles["line-bold"]}>
          <rect x="20" y="20" width="900" height="900"></rect>
          <path d="M 20 320 h 900 "></path>
          <path d="M 20 620 h 900 "></path>
          <path d="M 320 20 v 900"></path>
          <path d="M 620 20 v 900"></path>
        </g>
        <g className={styles["line"]}>
          <path d="M 20 120 h 900 "></path>
          <path d="M 20 220 h 900 "></path>
          <path d="M 20 420 h 900 "></path>
          <path d="M 20 520 h 900 "></path>
          <path d="M 20 720 h 900 "></path>
          <path d="M 20 820 h 900 "></path>
          <path d="M 120 20 v 900 "></path>
          <path d="M 220 20 v 900 "></path>
          <path d="M 420 20 v 900 "></path>
          <path d="M 520 20 v 900 "></path>
          <path d="M 720 20 v 900 "></path>
          <path d="M 820 20 v 900 "></path>
        </g>
        {cells.map((cell) => {
          return (
            <Cell
              key={cell.index}
              cell={cell}
              onMouseDown={handleMouseDown}
              onMouseOver={handleMouseOver}
              autoPms={autoPms}
            />
          );
        })}
        {player ? <Step /> : null}
      </svg>
    </>
  );
}
