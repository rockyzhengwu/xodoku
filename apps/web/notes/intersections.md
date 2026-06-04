# Intersections

Intersections use the overlap between a row or column and a block. If a digit is
locked into that overlap, it can be removed from the rest of the other house.

## When to look for it

Look for one digit whose candidates in a block are all in one row or column, or
whose candidates in a row or column are all in one block.

## Implemented in Xodoku

### Locked Candidates: Pointing

If all possible locations for a digit in a block are in one row or column, that
digit cannot appear elsewhere in that row or column.

![example](../images/locked_candidates_type_1.webp)

### Locked Candidates: Claiming

If all possible locations for a digit in a row or column are inside one block,
that digit can be removed from the rest of the block.

![example](../images/locked_candidates_type_2.webp)

## Rule

Both forms rely on the same idea: the digit must be placed somewhere in the
intersection, so candidates outside the intersection but inside the affected
house are impossible.
