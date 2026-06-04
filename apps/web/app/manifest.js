export const dynamic = "force-static";

export default function manifest() {
  return {
    name: "Xodoku - Sudoku Scanner, Solver and Generator",
    short_name: "Xodoku",
    description:
      "Scan Sudoku from images, generate puzzles, and get human-style solving hints in your browser.",
    start_url: "/",
    scope: "/",
    display: "standalone",
    background_color: "#ffffff",
    theme_color: "#0d9488",
    icons: [
      {
        src: "/images/logo.png",
        sizes: "512x512",
        type: "image/png",
      },
      {
        src: "/favicon-32x32.png",
        sizes: "32x32",
        type: "image/png",
      },
    ],
  };
}
