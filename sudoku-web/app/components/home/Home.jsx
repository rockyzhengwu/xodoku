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
import useRustWasm from "../../util/rustWasm.js";
import Player from "../../components/player/Player.jsx";

import styles from "./Home.module.css";

export default function Home() {
  const [nextStep, setNextStep] = useAtom(nextStepAtom);
  const { loading, error, rust } = useRustWasm("../wasm/sudoku_wasm.js");
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);

  return (
    <>
      <div className={styles.player}>
        <Player />
      </div>
      <div
        dangerouslySetInnerHTML={{
          __html: nextStep ? nextStep.explain : "",
        }}
      ></div>
    </>
  );
}
