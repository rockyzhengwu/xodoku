"use client";

import Link from "next/link";
import { useAtomValue } from "jotai";
import { nextStepAtom } from "../../atom/PlayerAtoms.js";
const hexColor = (color) => `#${color.toString(16).padStart(6, "0")}`;
const cardClass = "mx-auto mt-3 max-w-6xl rounded-xl border border-gray-300 bg-[var(--mantine-color-body)] p-4 sm:mt-5 sm:px-6 sm:py-5";

export default function HintCard() {
  const hint = useAtomValue(nextStepAtom);
  if (!hint || hint.name === "Nothing") {
    return null;
  }

  if (!hint.summary) {
    return (
      <section
        className={cardClass}
        aria-live="polite"
        dangerouslySetInnerHTML={{ __html: hint.explain ?? "" }}
      />
    );
  }

  return (
    <section className={cardClass} aria-live="polite">
      <div className="flex flex-col justify-between gap-2 sm:flex-row sm:items-start">
        <div>
          <p className="m-0 text-xs font-bold uppercase tracking-widest text-gray-500">{hint.technique_group}</p>
          <h2>{hint.name}</h2>
        </div>
        <span className="whitespace-nowrap text-sm text-gray-500">
          {hint.difficulty_label} · {hint.difficulty_rank}
        </span>
      </div>

      <p className="my-3">{hint.summary}</p>

      {hint.actions?.length ? (
        <div>
          <h3>Next move</h3>
          <ul>
            {hint.actions.map((action, index) => (
              <li key={`${action.kind}-${action.candidate.cell}-${action.candidate.value}-${index}`}>
                {action.text}
              </li>
            ))}
          </ul>
        </div>
      ) : null}

      {hint.reasoning?.length ? (
        <div>
          <h3>Why it works</h3>
          <ul>
            {hint.reasoning.map((reason, index) => (
              <li key={`reason-${index}`}>{reason}</li>
            ))}
          </ul>
        </div>
      ) : null}

      {hint.legend?.length ? (
        <div className="my-3 flex flex-wrap gap-x-4 gap-y-2 text-sm text-gray-500">
          {hint.legend.map((item) => (
            <span className="inline-flex items-center gap-1.5" key={item.role}>
              <i className="h-3 w-3 rounded-full" style={{ backgroundColor: hexColor(item.color) }} />
              {item.label}
            </span>
          ))}
        </div>
      ) : null}

      {hint.technique_url ? (
        <Link href={hint.technique_url} className="font-semibold text-teal-600 hover:underline">
          Learn more
        </Link>
      ) : null}
    </section>
  );
}
