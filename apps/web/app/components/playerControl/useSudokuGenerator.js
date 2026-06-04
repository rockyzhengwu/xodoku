"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { notifications } from "@mantine/notifications";

const initialGeneration = {
  running: false,
  difficulty: null,
  elapsedSeconds: 0,
  budgetSeconds: null,
};

export default function useSudokuGenerator({ onSuccess }) {
  const workerRef = useRef(null);
  const intervalRef = useRef(null);
  const resolveRef = useRef(null);
  const [generation, setGeneration] = useState(initialGeneration);

  const clear = useCallback(() => {
    workerRef.current?.terminate();
    workerRef.current = null;
    clearInterval(intervalRef.current);
    intervalRef.current = null;
    setGeneration((current) => ({ ...current, running: false }));
  }, []);

  const finish = useCallback((success) => {
    const resolve = resolveRef.current;
    resolveRef.current = null;
    clear();
    resolve?.(success);
  }, [clear]);

  useEffect(() => () => {
    workerRef.current?.terminate();
    clearInterval(intervalRef.current);
    resolveRef.current?.(false);
  }, []);

  const cancel = useCallback(() => finish(false), [finish]);

  const generate = useCallback((difficulty) => {
    if (workerRef.current) {
      return Promise.resolve(false);
    }
    const worker = new Worker("/workers/sudokuGenerator.worker.js", { type: "module" });
    workerRef.current = worker;
    setGeneration({ ...initialGeneration, running: true, difficulty });
    intervalRef.current = setInterval(() => {
      setGeneration((current) => ({ ...current, elapsedSeconds: current.elapsedSeconds + 1 }));
    }, 1000);

    return new Promise((resolve) => {
      resolveRef.current = resolve;
      worker.onmessage = ({ data }) => {
        if (data.type === "started") {
          setGeneration((current) => ({ ...current, budgetSeconds: data.budgetSeconds }));
          return;
        }
        if (data.type === "success") {
          onSuccess(data.sudoku);
          finish(true);
          return;
        }
        const runtimeError = data.type === "startup_error" || data.type === "runtime_error";
        console.error("Sudoku generation failed:", data.message);
        notifications.show({
          color: "red",
          title: runtimeError ? "Sudoku generator is unavailable" : "Sudoku generation failed",
          message: runtimeError
            ? "Refresh the page and try again."
            : "Could not generate a puzzle within the selected difficulty budget.",
        });
        finish(false);
      };
      worker.onerror = (error) => {
        console.error("Sudoku generation worker failed:", error);
        notifications.show({
          color: "red",
          title: "Sudoku generator is unavailable",
          message: "Refresh the page and try again.",
        });
        finish(false);
      };
      worker.postMessage({ type: "generate", difficulty });
    });
  }, [finish, onSuccess]);

  return { generation, generate, cancel };
}
