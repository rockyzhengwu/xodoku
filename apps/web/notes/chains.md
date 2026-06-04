# Chains and Loops

Chains connect candidate conclusions with alternating strong and weak
inferences.

- A **strong inference** means at least one endpoint must be true.
- A **weak inference** means both endpoints cannot be true.

Following those links lets the solver prove that a candidate must be removed or
that a contradiction would occur if one endpoint were chosen.

## When to look for it

Chains are useful when local patterns are exhausted but the grid still has many
strong links. They are often easier to understand when drawn as endpoints and
links rather than as a long text proof.

## Implemented in Xodoku

- **Remote Pair**: a chain of bivalue cells with the same two digits. A digit can
  be removed from a cell seeing both opposite-colored endpoints.
- **X-Chain**: a single-digit chain using conjugate links.
- **XY-Chain**: a chain through bivalue cells where each step changes the active
  digit.
- **AIC**: a general Alternating Inference Chain using candidate nodes.
- **AIC Loop**: a closed AIC. Older references often call this a Nice Loop.
- **Grouped AIC**: a row-block or column-block group can act as one chain node.

## Nice Loop naming

HoDoKu and older Sudoku references distinguish several Nice Loop categories.
Xodoku uses a single AIC engine instead. If a classic Nice Loop proof is found,
it is surfaced as AIC or AIC Loop.

## Scope

ALS-AIC is documented under Almost Locked Sets because its proof contains ALS
nodes. Expert forcing chains are documented under Expert Techniques because they
come from bounded fallback search rather than the normal chain engine.
