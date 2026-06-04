# Uniqueness

Uniqueness techniques use the assumption that a published Sudoku has exactly one
solution. They prevent patterns that would otherwise allow two digits to be
exchanged without violating Sudoku rules.

## When to look for it

Look for Uniqueness patterns when four cells form a rectangle across exactly two
rows, two columns, and two blocks, and the same two base candidates appear in
the rectangle.

## Implemented in Xodoku

- **Unique Rectangle Type 1** removes the extra candidate that would leave a
  deadly rectangle.
- **Unique Rectangle Type 2** uses a shared extra candidate to remove that digit
  from peers.
- **Unique Rectangle Type 3** combines the rectangle with a subset outside it.
- **Unique Rectangle Type 4** uses strong-link style pressure on one base digit.
- **Unique Rectangle Type 5** and **Type 6** cover additional rectangle
  placements supported by the solver.
- **Hidden Rectangle** applies the same logic when extra candidates hide part of
  the rectangle.
- **Avoidable Rectangle Type 1 / 2** use non-given placed digits. They are valid
  only when the involved digits were entered by the player, not original givens.
- **BUG+1** handles a Binary Universal Grave with one extra candidate.

## Scope

Xodoku assumes uniqueness techniques are enabled and the puzzle has one
solution. If you want to solve without uniqueness assumptions, skip these hints.
