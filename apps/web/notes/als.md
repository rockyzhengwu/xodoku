# Almost Locked Sets

An Almost Locked Set (ALS) is a group of `N` unsolved cells in one house whose
combined candidates contain exactly `N + 1` digits. If one digit is removed from
that set, the remaining digits become locked into the cells.

## When to look for it

ALS techniques become useful after ordinary subsets, fish, wings, and short
chains have stopped producing progress. They are especially common in puzzles
where many cells are bivalue or trivalue, but no direct wing is available.

## Restricted common candidates

Two ALS can be linked by a Restricted Common Candidate (RCC). An RCC is a digit
that appears in both ALS, and every occurrence in one ALS sees every occurrence
in the other ALS. Both ALS cannot use the RCC at the same time.

The practical rule is: if a candidate outside the ALS sees all possible places
for a forced digit, that outside candidate can be removed.

## Implemented in Xodoku

- **ALS-XZ**: two ALS connected by one or more RCCs. A shared non-RCC digit can
  be removed from cells seeing that digit in both ALS.
- **ALS-XY-Wing**: three ALS connected by two RCCs. The third digit is forced in
  one of the ALS branches and can produce eliminations.
- **ALS Chain**: a sequence of ALS connected by RCCs. The endpoints justify the
  elimination.
- **ALS-AIC**: ALS nodes are used inside the AIC engine alongside candidate and
  group nodes.
- **Death Blossom**: several ALS are connected through a stem candidate. Each
  branch forces a conclusion that supports the same elimination.
- **WXYZ-Wing**: represented as a compact ALS-XZ shape when the pattern fits the
  ALS rule.

## Scope

Xodoku limits ALS enumeration to small sets for web responsiveness. That covers
the useful teaching forms without turning hints into unbounded search.
