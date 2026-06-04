# Fish

Fish are single-digit patterns. Choose a set of base houses and the same number
of cover houses. When the cover houses contain all base candidates for a digit,
that digit can be removed from other cells in the cover houses.

Base and cover houses are usually rows and columns, but complex fish can also
use blocks.

## Basic Fish

The number of base and cover houses gives the pattern its name:

- Size 2: **X-Wing**
- Size 3: **Swordfish**
- Size 4: **Jellyfish**
- Size 5: **Squirmbag**
- Size 6: **Whale**
- Size 7: **Leviathan**

Every selected base and cover house must participate in the pattern. Not every
intersection needs to contain the fish digit.

## Finned Fish

A Finned Fish has additional base candidates outside the selected cover houses.
These candidates are fins. A cover candidate can still be removed when it is not
a base candidate and it can see every fin.

Xodoku searches Finned X-Wing, Finned Swordfish, and Finned Jellyfish.

## Sashimi Fish

A Sashimi Fish is a finned fish whose clean base-cover pattern would be
degenerate if the fins were removed. The elimination rule is unchanged: remove
only cover candidates that can see every fin.

Xodoku searches Sashimi X-Wing, Sashimi Swordfish, and Sashimi Jellyfish.

## Complex Fish

Xodoku also searches bounded complex fish:

- **Franken Fish** use blocks as sectors.
- **Mutant Fish** mix rows, columns, and blocks.
- **Endo fins** are fins inside the fish sector system.
- **Cannibalistic eliminations** remove candidates that are part of the cover
  system itself.

The complex fish search is bounded to keep browser hints responsive.

## Reserved

Kraken Fish remains reserved for a later Fish-rooted forcing proof. It is not a
separate solver step today.
