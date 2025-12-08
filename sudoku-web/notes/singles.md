# Singles

## Last Digit / Full House 🏡

A Last Digit or Full House is the easiest single to spot. It's the final, missing number in a row, column, or block that already has eight of the nine digits filled in.

![example](../images/full_house.webp)
Example: If a row has all the numbers from 1 to 9 except for the number 4, and there is only one empty cell left, that cell must be a 4. In the image above, if row 4 has only one empty cell at r4c6, you can immediately fill it with the missing number, which is 4. r4c6 = 4.

## Hidden Single 🕵️‍♂️

A Hidden Single is when a number can only be placed in a single cell within a specific house (row, column, or block), even if that cell has other numbers as possibilities. The key is that the number you're looking for can't go anywhere else in that house.

![hidden single example](../images/hidden_single.webp)

Example:
In the image above, let's focus on row 1. Even though other numbers could potentially go in cell r1c4, the number 5 can't be placed anywhere else in that row. Therefore, cell r1c4 must be a 5. r1c4 = 5.

## Naked Single 🎯

A Naked Single is a cell that has only one possible number left after you've eliminated all other options. It's the most straightforward "single" to find.

![naked single example](../images/naked_single.webp)

Example: If cell r8c4 has been narrowed down so that the only remaining possibility is the number 7, you can confidently place 7 in that cell. r8c4 = 7.
