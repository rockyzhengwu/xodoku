# Introduction

## The Language of Sudoku

Understanding Sudoku is easier when you know the terminology used by solvers. This guide will introduce you to the fundamental terms for navigating and discussing the puzzle grid.

![The Grid's Basic Elements](../images/sudoku_terminology.webp)

### Cell

A cell is the smallest unit of the Sudoku grid. It is a single square where you will place a number from 1 to 9. A standard Sudoku grid has 81 cells.

### Row

A row is a horizontal line of 9 cells. There are 9 rows in total on the grid.

### Column

A column is a vertical line of 9 cells. There are 9 columns in total.

### Block (or Box)

A block (also called a box or region) is one of the nine 3x3 squares that make up the grid.

### House

A house is a collective term for any of the three types of groups a number must be unique in: a row, a column, or a block. Every cell belongs to exactly three houses.

### Givens

The givens are the numbers that are already filled in on the grid when you start the puzzle. These numbers are fixed and cannot be changed.

### Pencil Marks

Pencil marks are small numbers written in a cell to indicate the possible numbers (candidates) that could go in that cell. They are not the final solution but are used as a helpful note-taking tool for difficult puzzles.

### Peers

The peers of a cell are all the other cells in the same row, column, and block. A key rule of Sudoku is that a cell's number cannot be the same as any of its peers.

### Band (or Chute)

A band is a horizontal group of three blocks. A standard Sudoku grid has three bands: the top band (blocks 1, 2, 3), the middle band (blocks 4, 5, 6), and the bottom band (blocks 7, 8, 9).

### Stack (or Tower)

A stack is a vertical group of three blocks. A standard Sudoku grid has three stacks: the left stack (blocks 1, 4, 7), the middle stack (blocks 2, 5, 8), and the right stack (blocks 3, 6, 9).

## how to express a solver step

We use Hodoku like expression.
such as r3c5 for the cell in row 3, column 5, is often used to refer to a single cell.

The compressed notation r1c234 is a shorthand for the cells r1c2, r1c3, and r1c4, which are all in row 1. Similarly, r123c5 would refer to the cells r1c5, r2c5, and r3c5, which are all in column 5.

Another form of compressed notation, which uses the box, point format, is useful for expressing a group of cells within a single box. The notation b1p159 is a concise way to refer to the cells in box 1 at points 1, 5, and 9.
Think of the cells in a 3x3 box as being numbered from 1 to 9, starting from the top-left and moving across each row. So, in box 1 (the top-left box), point 1 is r1c1, point 5 is r2c2, and point 9 is r3c3. Therefore, b1p159 is a simple way of writing the set of cells {r1c1, r2c2, r3c3}.
