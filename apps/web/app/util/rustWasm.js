"use client";

import { useState, useEffect, useRef } from "react";

const useRustWasm = (wasmModulePath) => {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [rustExports, setRustExports] = useState(null);

  const moduleRef = useRef(null);

  useEffect(() => {
    const loadRustWasm = async () => {
      try {
        setLoading(true);
        setError(null);

        const rustModule = await import("../wasm/sudoku_wasm.js");
        //const rustModule = await WebAssembly.instantiate(sudokuWasm);
        await rustModule.default();

        setRustExports(rustModule);
        setLoading(false);
      } catch (err) {
        console.error("Error loading Rust WebAssembly module:", err);
        setError(err);
        setLoading(false);
      }
    };

    if (!moduleRef.current || moduleRef.current !== wasmModulePath) {
      moduleRef.current = wasmModulePath;
      loadRustWasm();
    }

    return () => {};
  }, [wasmModulePath]);

  return { loading, error, rust: rustExports };
};

export default useRustWasm;
