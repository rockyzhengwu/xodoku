# Coloring

Coloring follows conjugate links for one digit. A conjugate link exists when a
house has exactly two candidates for that digit. If one endpoint is false, the
other must be true, so the endpoints can be colored alternately.

## When to look for it

Coloring is helpful when a digit has several strong links but no simple
Skyscraper, Kite, or Turbot Fish is available.

## Implemented in Xodoku

- **Color Trap** removes an uncolored candidate that sees both colors. One of
  those colors must be true, so the uncolored candidate cannot survive.
- **Color Wrap** removes a color when two candidates of the same color see each
  other. That color would place the same digit twice in one house.
- **Multi Colors** connects multiple coloring components and uses visibility
  between them to remove candidates.

## Scope

Coloring is a teaching-friendly view of short single-digit chains. Some
deductions may also be returned as X-Chain, Turbot Fish, or AIC when that proof
is shorter or appears earlier in the strategy order.
