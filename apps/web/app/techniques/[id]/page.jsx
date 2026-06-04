import { notFound } from "next/navigation";

import { getAllPostIds, getPostData, postExists } from "../../lib/posts.js";

const titleFromId = (id) =>
  id
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");

const descriptions = {
  als: "Learn Almost Locked Set techniques in Sudoku, including ALS-XZ, ALS chains, ALS-AIC and Death Blossom.",
  chains:
    "Learn Sudoku chain techniques supported by Xodoku, including Remote Pair, X-Chain, XY-Chain, AIC and Grouped AIC.",
  coloring:
    "Learn Sudoku coloring techniques, including Color Trap, Color Wrap and Multi Colors.",
  expert:
    "Learn Xodoku expert Sudoku hints, including templates, forcing chains and forcing nets.",
  fish: "Learn Sudoku fish techniques, including X-Wing, Swordfish, Jellyfish, finned fish, sashimi fish and complex fish.",
  "hidden-subsets":
    "Learn Hidden Pair, Hidden Triple and Hidden Quadruple Sudoku subset techniques.",
  intersections:
    "Learn Sudoku intersection techniques, including pointing and claiming locked candidates.",
  introduction:
    "Learn the basic Sudoku terminology used by Xodoku hints and solving techniques.",
  "naked-subsets":
    "Learn Naked Pair, Naked Triple and Naked Quadruple Sudoku subset techniques.",
  "single-digit-patterns":
    "Learn single digit Sudoku patterns, including Skyscraper, 2-String Kite, Turbot Fish and Empty Rectangle.",
  singles:
    "Learn Sudoku singles, including Full House, Hidden Single and Naked Single.",
  "sue-de-coq":
    "Learn Sue de Coq and Extended Sue de Coq Sudoku intersection techniques.",
  uniqueness:
    "Learn Sudoku uniqueness techniques, including Unique Rectangles, Hidden Rectangle, Avoidable Rectangle and BUG+1.",
  wings:
    "Learn Sudoku wing techniques, including XY-Wing, XYZ-Wing and W-Wing.",
};

export async function generateMetadata({ params }) {
  const { id } = await params;
  if (!postExists(id)) {
    return {};
  }

  const post = await getPostData(id);
  const title = `${titleFromId(id)} Sudoku Technique`;
  const description =
    post.description ??
    descriptions[id] ??
    `Learn the ${titleFromId(id)} Sudoku technique in Xodoku.`;
  const pageTitle = post.title ?? title;

  return {
    title: pageTitle,
    description,
    alternates: {
      canonical: `/techniques/${id}`,
    },
    openGraph: {
      title: `${pageTitle} | Xodoku`,
      description,
      url: `/techniques/${id}`,
      type: "article",
      images: [
        {
          url: "/images/og.png",
          width: 1200,
          height: 630,
          alt: `${pageTitle} in Xodoku`,
        },
      ],
    },
    twitter: {
      card: "summary_large_image",
      title: `${pageTitle} | Xodoku`,
      description,
      images: ["/images/og.png"],
    },
  };
}

export function generateStaticParams() {
  return getAllPostIds();
}

export default async function TechniqueNote({ params }) {
  const { id } = await params;
  if (!postExists(id)) {
    notFound();
  }

  const { contentHtml } = await getPostData(id);
  return (
    <main className="mx-auto max-w-3xl">
      <article
        dangerouslySetInnerHTML={{ __html: contentHtml }}
        className="article-content"
      />
    </main>
  );
}
