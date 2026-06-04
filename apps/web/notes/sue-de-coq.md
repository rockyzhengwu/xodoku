# Sue de Coq

Sue de Coq is an intersection technique using one row or column, one block, and
their shared intersection cells. The candidates in the intersection split into
restricted sets outside the intersection.

## When to look for it

Look for Sue de Coq when a row-block or column-block intersection has two or
three unsolved cells and their combined candidates are tightly constrained.

## Rule

The intersection candidates can be divided between:

- candidates that must live in the row or column side,
- candidates that must live in the block side,
- candidates that remain inside the intersection.

Once those sets are known, matching candidates can be removed from the rest of
the row, column, or block.

## Implemented in Xodoku

- **Sue de Coq**: the standard form with a compact intersection and bivalue
  support cells outside it.
- **Extended Sue de Coq**: a broader search over two- and three-cell
  intersections with restricted outside sets.

## Scope

Sue de Coq can overlap conceptually with ALS reasoning. Xodoku keeps it as a
named step because the row/column-block intersection proof is easier to read for
many puzzles.
