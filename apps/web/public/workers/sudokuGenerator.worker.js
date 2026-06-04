import init, {
  generate_sudoku,
  generation_budget_seconds,
} from "/wasm/sudoku_wasm.js";

let initialized;

const initialize = () => {
  initialized ??= init();
  return initialized;
};

self.onmessage = async ({ data }) => {
  if (data.type !== "generate") {
    return;
  }

  try {
    await initialize();
  } catch (error) {
    self.postMessage({ type: "startup_error", message: String(error) });
    return;
  }

  try {
    const budgetSeconds = generation_budget_seconds(data.difficulty);
    self.postMessage({ type: "started", budgetSeconds });
    const sudoku = generate_sudoku(data.difficulty);
    self.postMessage({ type: "success", sudoku });
  } catch (error) {
    self.postMessage({
      type: String(error) === "GenerateFailed" ? "generation_error" : "runtime_error",
      message: String(error),
    });
  }
};
