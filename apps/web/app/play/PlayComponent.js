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
  const [, setSudokuCells] = useAtom(sudokuCellsAtom);
  const { loading, rust } = useRustWasm("../wasm/sudoku_wasm.js");
  const [, setTimerRuning] = useAtom(timerRuningAtom);
  const [, setSudokuSolution] = useAtom(sudokuSolutionAtom);
  const searchParams = useSearchParams();
  const [, setNextStep] = useAtom(nextStepAtom);

  useEffect(() => {
    if (loading) {
      return;
    }

    let timeout;
    let cancelled = false;

    async function prepare() {
      const s = searchParams.get("s");
      if (s && rust) {
        const sudoku = await rust.import_sudoku(s);
        if (cancelled) {
          return;
        }
        const cells = initCellsSudoku(sudoku);
        setSudokuSolution(sudoku.solutions);
        setSudokuCells(cells);
        setTimerRuning(true);
        setNextStep(null);
        timeout = setTimeout(() => {
          router.push("/");
        }, 1000);
      } else {
        router.push("/");
      }
    }
    prepare();

    return () => {
      cancelled = true;
      clearTimeout(timeout);
    };
  }, [
    loading,
    router,
    rust,
    searchParams,
    setNextStep,
    setSudokuCells,
    setSudokuSolution,
    setTimerRuning,
  ]);

  return <Suspense></Suspense>;
}
