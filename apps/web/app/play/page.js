import { Suspense } from "react";
import PlayComponent from "./PlayComponent";

export const metadata = {
  title: "Play Sudoku",
  description: "Load and play a shared Sudoku puzzle in Xodoku.",
  robots: {
    index: false,
    follow: true,
  },
};

export default function Play() {
  return (
    <Suspense>
      <PlayComponent />
    </Suspense>
  );
}
