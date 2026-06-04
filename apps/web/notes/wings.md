# Wings

Wings combine candidates from a small number of cells. Their endpoints force a
shared candidate to be true in at least one location, allowing that candidate to
be removed from cells that can see every relevant endpoint.

## When to look for it

Look for Wings when the grid has many bivalue or trivalue cells. They often
appear before longer chains and ALS techniques.

## Implemented in Xodoku

### XY-Wing

An XY-Wing has a two-candidate pivot `XY` and two pincers `XZ` and `YZ`. Both
pincers see the pivot. Candidate `Z` can be removed from cells that see both
pincers.

### XYZ-Wing

An XYZ-Wing has a three-candidate pivot `XYZ` and two pincers `XZ` and `YZ`.
Candidate `Z` can be removed from cells that see the pivot and both pincers.

### W-Wing

A W-Wing uses two identical bivalue cells. A strong link for one candidate
connects the endpoints, allowing the other candidate to be removed from cells
that see both endpoints.

## Alias

WXYZ-Wing is recognized as a useful teaching name, but in Xodoku it is returned
as an ALS shape when the proof is ALS-XZ-like.
