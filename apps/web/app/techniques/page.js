import Link from "next/link";

export const metadata = {
  title: "Sudoku Solving Techniques",
  description:
    "Learn the Sudoku solving techniques supported by Xodoku, from singles and subsets to fish, chains, ALS and expert hints.",
  alternates: {
    canonical: "/techniques",
  },
  openGraph: {
    title: "Sudoku Solving Techniques | Xodoku",
    description:
      "Learn the Sudoku solving techniques supported by Xodoku, from singles and subsets to fish, chains, ALS and expert hints.",
    url: "/techniques",
  },
};

const STATUS_LABELS = {
  implemented: "Implemented",
  alias: "Alias",
  reserved: "Reserved",
};

const STATUS_CLASSES = {
  implemented: "border-teal-200 bg-teal-50 text-teal-700",
  alias: "border-blue-200 bg-blue-50 text-blue-700",
  reserved: "border-slate-200 bg-slate-50 text-slate-500",
};

const TECHNIQUE_GROUPS = [
  {
    title: "Getting started",
    items: [
      { title: "Introduction", href: "introduction", status: "implemented" },
      { title: "Singles", href: "singles", status: "implemented" },
      { title: "Intersections", href: "intersections", status: "implemented" },
      { title: "Hidden subsets", href: "hidden-subsets", status: "implemented" },
      { title: "Naked subsets", href: "naked-subsets", status: "implemented" },
    ],
  },
  {
    title: "Fish",
    href: "fish",
    items: [
      { title: "X-Wing", status: "implemented" },
      { title: "Swordfish", status: "implemented" },
      { title: "Jellyfish", status: "implemented" },
      { title: "Squirmbag", status: "implemented" },
      { title: "Whale", status: "implemented" },
      { title: "Leviathan", status: "implemented" },
      { title: "Finned fish", status: "implemented" },
      { title: "Sashimi fish", status: "implemented" },
      { title: "Franken / Mutant fish", status: "implemented" },
      { title: "Kraken Fish", status: "reserved" },
    ],
  },
  {
    title: "Single digit patterns",
    href: "single-digit-patterns",
    items: [
      { title: "Skyscraper", status: "implemented" },
      { title: "2-String Kite", status: "implemented" },
      { title: "Turbot Fish", status: "implemented" },
      { title: "Empty Rectangle", status: "implemented" },
    ],
  },
  {
    title: "Uniqueness",
    href: "uniqueness",
    items: [
      { title: "Unique Rectangles 1-6", status: "implemented" },
      { title: "BUG+1", status: "implemented" },
      { title: "Hidden Rectangle", status: "implemented" },
      { title: "Avoidable Rectangles 1-2", status: "implemented" },
    ],
  },
  {
    title: "Wings",
    href: "wings",
    items: [
      { title: "XY-Wing", status: "implemented" },
      { title: "XYZ-Wing", status: "implemented" },
      { title: "W-Wing", status: "implemented" },
      { title: "WXYZ-Wing", status: "alias", note: "Returned as an ALS shape." },
    ],
  },
  {
    title: "Chains",
    href: "chains",
    items: [
      { title: "Remote Pair", status: "implemented" },
      { title: "X-Chain", status: "implemented" },
      { title: "XY-Chain", status: "implemented" },
      { title: "AIC", status: "implemented" },
      { title: "AIC Loop", status: "implemented" },
      { title: "Grouped AIC", status: "implemented" },
      { title: "Nice Loop", status: "alias", note: "Handled by AIC / AIC Loop." },
    ],
  },
  {
    title: "Almost locked sets",
    href: "als",
    items: [
      { title: "ALS-XZ", status: "implemented" },
      { title: "ALS-XY-Wing", status: "implemented" },
      { title: "ALS Chain", status: "implemented" },
      { title: "ALS-AIC", status: "implemented" },
      { title: "Death Blossom", status: "implemented" },
      { title: "WXYZ-Wing", status: "implemented" },
    ],
  },
  {
    title: "Coloring",
    href: "coloring",
    items: [
      { title: "Color Trap", status: "implemented" },
      { title: "Color Wrap", status: "implemented" },
      { title: "Multi Colors", status: "implemented" },
    ],
  },
  {
    title: "Sue de Coq",
    href: "sue-de-coq",
    items: [
      { title: "Sue de Coq", href: "sue-de-coq", status: "implemented" },
      { title: "Extended Sue de Coq", status: "implemented" },
    ],
  },
  {
    title: "Expert techniques",
    href: "expert",
    items: [
      { title: "Templates", status: "implemented" },
      { title: "Forcing Chain", status: "implemented" },
      { title: "Forcing Net", status: "implemented" },
      { title: "Kraken Fish", status: "reserved" },
    ],
  },
];

function TechniqueLink({ title, href }) {
  return href ? (
    <Link
      className="font-semibold text-teal-700 hover:underline focus:outline-none focus:ring-2 focus:ring-teal-500"
      href={`/techniques/${href}`}
    >
      {title}
    </Link>
  ) : (
    <span className="font-semibold">{title}</span>
  );
}

function StatusBadge({ status }) {
  return (
    <span
      className={`rounded-full border px-2 py-0.5 text-[0.68rem] font-semibold uppercase tracking-wide ${STATUS_CLASSES[status]}`}
    >
      {STATUS_LABELS[status]}
    </span>
  );
}

export default function Techniques() {
  return (
    <main className="mx-auto max-w-5xl">
      <h1 className="text-3xl font-bold">Solving techniques</h1>
      <p className="mt-2 max-w-3xl text-gray-500">
        A practical directory of the techniques supported by Xodoku hints and
        puzzle analysis.
      </p>
      <div className="mt-8 grid gap-x-10 gap-y-8 md:grid-cols-2">
        {TECHNIQUE_GROUPS.map((group) => (
          <section className="border-t border-gray-200 pt-4" key={group.title}>
            <h2 className="text-lg">
              <TechniqueLink title={group.title} href={group.href} />
            </h2>
            <ul className="mt-3 space-y-2 text-sm text-gray-600">
              {group.items.map((item) => (
                <li className="flex items-start justify-between gap-3" key={item.title}>
                  <span>
                    <TechniqueLink title={item.title} href={item.href} />
                    {item.note ? (
                      <span className="ml-1 text-xs text-gray-500">{item.note}</span>
                    ) : null}
                  </span>
                  <StatusBadge status={item.status} />
                </li>
              ))}
            </ul>
          </section>
        ))}
      </div>
    </main>
  );
}
