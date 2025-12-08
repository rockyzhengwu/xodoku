"use client";

import { useAtom } from "jotai";
import {
  initCells,
  usedSecondsAtom,
  nextStepAtom,
  sudokuSolutionAtom,
  timerRuningAtom,
  sudokuCellsAtom,
  initCellsSudoku,
} from "../../atom/PlayerAtoms.js";
import Timer from "../../components/timer/Timer.jsx";
import useRustWasm from "../../util/rustWasm.js";
import Player from "../../components/player/Player.jsx";
import RandomGenerateDialog from "../../components/randomGenerateDialog/RandomGenerateDialog.jsx";
import PlayerActions from "../../components/playerActions/PlayerActions.jsx";

import styles from "./Home.module.css";

export default function Home() {
  const [usedSeconds, setUsedSeconds] = useAtom(usedSecondsAtom);
  const [_, setNextStep] = useAtom(nextStepAtom);
  const { loading, error, rust } = useRustWasm("../wasm/sudoku_wasm.js");
  const [sudokuSolution, setSudokuSolution] = useAtom(sudokuSolutionAtom);
  const [timerRuning, setTimerRuning] = useAtom(timerRuningAtom);
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);

  const gerateRandomSudoku = (difficulty_level) => {
    setUsedSeconds(0);
    if (rust) {
      const sudokuResult = rust.generate_sudoku(difficulty_level);
      const cells = initCellsSudoku(sudokuResult);
      setNextStep(null);
      setSudokuSolution(sudokuResult.solutions);
      setSudokuCells(cells);
      setTimerRuning(true);
    }
  };

  return (
    <>
      <div className={styles.player}>
        <Player />
      </div>
      <div className={styles.gridFooter}>
        <div className={styles.timer}>
          <Timer />
        </div>
        <RandomGenerateDialog onInput={gerateRandomSudoku} />
        <PlayerActions />
      </div>
    </>
  );
}
