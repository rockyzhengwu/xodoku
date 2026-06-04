# Singles

Singles place a digit immediately. They are the first techniques Xodoku tries
and the easiest steps to apply by hand.

## Full House

A Full House is a row, column, or block with one empty cell. The missing digit
must go in that cell.

![example](../images/full_house.webp)

## Hidden Single

A Hidden Single occurs when one digit has only one possible cell in a house,
even if that cell has several candidates.

![hidden single example](../images/hidden_single.webp)

## Naked Single

A Naked Single is a cell with exactly one remaining candidate.

![naked single example](../images/naked_single.webp)

## Scope

Xodoku treats Full House, Hidden Single, and Naked Single as placement steps.
When a hint applies one of these techniques, the next move is to set the shown
cell to the shown digit.
