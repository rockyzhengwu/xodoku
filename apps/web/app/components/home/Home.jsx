"use client";

import Link from "next/link";

import Player from "../../components/player/Player.jsx";
import HintCard from "../../components/hintCard/HintCard.jsx";

export default function Home() {
  return (
    <>
      <div className="mt-1">
        <Player />
      </div>
      <HintCard />
      <section className="mx-auto mt-10 max-w-4xl rounded-3xl border border-slate-200 bg-white/80 p-6 shadow-sm dark:border-slate-700 dark:bg-slate-900/70 sm:p-8">
        <p className="text-sm font-semibold uppercase tracking-[0.18em] text-teal-600">
          Sudoku tools
        </p>
        <h1 className="mt-2 text-2xl font-bold tracking-tight text-slate-950 dark:text-slate-50 sm:text-3xl">
          Free Sudoku scanner, generator, and human-style solver
        </h1>
        <p className="mt-3 max-w-3xl text-sm leading-7 text-slate-600 dark:text-slate-300 sm:text-base">
          Xodoku helps you play Sudoku in the browser, scan puzzles from images,
          generate new boards, and ask for logical hints that explain the next
          solving step instead of jumping straight to the answer.
        </p>
        <div className="mt-5 flex flex-wrap gap-3 text-sm font-semibold">
          <Link
            className="rounded-full bg-teal-600 px-4 py-2 text-white hover:bg-teal-700 focus:outline-none focus:ring-2 focus:ring-teal-500"
            href="/scanner"
          >
            Scan a Sudoku
          </Link>
          <Link
            className="rounded-full border border-slate-300 px-4 py-2 text-slate-800 hover:border-teal-500 hover:text-teal-700 focus:outline-none focus:ring-2 focus:ring-teal-500 dark:border-slate-600 dark:text-slate-100"
            href="/techniques"
          >
            Learn solving techniques
          </Link>
        </div>
      </section>
    </>
  );
}
