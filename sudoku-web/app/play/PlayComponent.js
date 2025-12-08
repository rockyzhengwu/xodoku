"use client";

import { useSearchParams, useRouter } from "next/navigation";
import { useAtom } from "jotai";
import { useEffect, Suspense } from "react";

import {
  sudokuCellsAtom,
  initCellsSudoku,
  timerRuningAtom,
  sudokuSolutionAtom,
  nextStepAtom,
} from "../atom/PlayerAtoms.js";
import useRustWasm from "../util/rustWasm.js";

export default function PlayComponent() {
  const router = useRouter();
  const [sudokuCells, setSudokuCells] = useAtom(sudokuCellsAtom);
  const { loading, error, rust } = useRustWasm("../wasm/sudoku_wasm.js");
  const [timerRuning, setTimerRuning] = useAtom(timerRuningAtom);
  const [sudokuSolution, setSudokuSolution] = useAtom(sudokuSolutionAtom);
  const searchParams = useSearchParams();
  const [nextStep, setNextStep] = useAtom(nextStepAtom);

  useEffect(() => {
    async function prepare() {
      const s = searchParams.get("s");
      if (s && !loading) {
        const sudoku = await rust.import_sudoku(s);
        const cells = initCellsSudoku(sudoku);
        setSudokuSolution(sudoku.solution);
        setSudokuCells(cells);
        setTimerRuning(true);
        setNextStep(null);
        const timeout = setTimeout(() => {
          router.push("/");
        }, 1000);
        return () => clearTimeout(timeout);
      } else {
        console.log("s", rust);
        router.push("/");
      }
    }
    prepare();
  }, [searchParams, router, loading]);

  return <Suspense></Suspense>;
}
