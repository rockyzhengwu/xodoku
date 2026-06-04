# Naked Subsets

Naked Subsets are groups of cells in one house whose combined candidates contain
exactly the same number of digits as cells. Those digits must fill those cells,
so they can be removed from the other cells in that house.

## When to look for it

Look for Naked Subsets after singles. They are most visible when two, three, or
four cells have short candidate lists.

## Implemented in Xodoku

- **Naked Pair**: two cells in one house contain only two combined digits.
- **Naked Triple**: three cells in one house contain only three combined digits.
- **Naked Quadruple**: four cells in one house contain only four combined digits.
- **Locked Subset**: if the same cells also sit inside a second house, Xodoku
  can remove the subset digits from both affected houses.

## Rule

The cells do not need to contain identical candidate lists. For a Naked Triple,
for example, candidates `12`, `23`, and `13` form a valid triple because their
union is `{1, 2, 3}`.

## Scope

The solver searches sizes two through four. Larger naked sets are usually better
handled as hidden subsets or by more targeted techniques.
