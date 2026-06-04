# Single Digit Patterns

Single Digit Patterns use strong links for one digit. A strong link exists when
a house has exactly two candidates for that digit: if one candidate is false,
the other must be true.

## When to look for it

Use these techniques when one digit has several conjugate pairs but no direct
fish is available.

## Implemented in Xodoku

### Skyscraper

A Skyscraper uses two row strong links or two column strong links. One endpoint
from each link shares a house. Since one of the other two endpoints must be
true, the digit can be removed from cells that see both outer endpoints.

### 2-String Kite

A 2-String Kite uses one row strong link and one column strong link. One
endpoint from each link lies in the same block. The digit can be removed from
cells that see both remaining endpoints.

### Turbot Fish

A Turbot Fish is a short single-digit chain. It starts and ends with strong
links, so one endpoint must be true. Candidates seeing both endpoints can be
removed.

### Empty Rectangle

An Empty Rectangle starts with a block where one digit is restricted to one row
and one column. Combined with an external conjugate pair, it can eliminate a
candidate that sees the implied endpoint.

## Scope

Some dual or degenerate forms can be represented by simpler steps or by AIC.
Xodoku reports the shorter named proof when possible.
