import Scanner from "../components/scanner/Scanner.jsx";

export const metadata = {
  title: "Sudoku Scanner",
  description:
    "Scan a Sudoku puzzle from an image and edit the recognized grid before playing.",
  alternates: {
    canonical: "/scanner",
  },
  openGraph: {
    title: "Sudoku Scanner | Xodoku",
    description:
      "Scan a Sudoku puzzle from an image and edit the recognized grid before playing.",
    url: "/scanner",
  },
};

export default function ScannerPage() {
  return (
    <>
      <Scanner />
    </>
  );
}
