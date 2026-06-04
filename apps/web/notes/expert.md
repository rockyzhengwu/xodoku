# Expert Techniques

Expert Hint runs bounded fallback searches only when the normal logical
strategy list cannot find a step. These hints are intentionally budgeted so the
web UI stays responsive.

## When to look for it

Use Expert Hint when the puzzle still has candidates but ordinary hints return
no progress. Expert techniques are proof-oriented: they explain what every valid
branch has in common, rather than naming a small local pattern.

## Implemented in Xodoku

- **Templates** enumerate valid placements for one digit and remove candidates
  that appear in no valid placement.
- **Forcing Chain** follows two-way assumptions from a candidate or cell and
  keeps conclusions shared by every branch.
- **Forcing Net** allows branching through cells with more than two candidates,
  then intersects the resulting conclusions.

## Reserved

- **Kraken Fish** is reserved for a later Fish-rooted forcing proof. Xodoku can
  already find complex fish and forcing deductions, but it does not currently
  combine them into a named Kraken Fish hint.

## Search budget

The web solver stops expert search after a small deadline and node budget. A
truncated search is reported separately from a complete search that found no
hint.
