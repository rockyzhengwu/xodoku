# Hidden Subsets

Hidden Subsets are groups of digits that can appear only in a specific set of
cells within one house. Once those cells are identified, every other candidate
can be removed from them.

## When to look for it

Look inside one row, column, or block and ask where each digit can still go. A
Hidden Subset is easier to find by scanning digits than by scanning cells.

## Implemented in Xodoku

### Hidden Pair

Two digits can appear only in the same two cells of a house. Remove every other
candidate from those two cells.

![Example](../images/hidden_pair.webp)

### Hidden Triple

Three digits can appear only in three cells of a house. The cells do not need to
contain all three digits individually.

![Example](../images/hidden_tripe.webp)

### Hidden Quadruple

Four digits can appear only in four cells of a house. Remove all other
candidates from those cells.

![Example](../images/hidden_quadruple.webp)

## Scope

Xodoku searches Hidden Pairs, Triples, and Quadruples. Larger hidden subsets are
rare and usually show up as smaller or more direct deductions first.
