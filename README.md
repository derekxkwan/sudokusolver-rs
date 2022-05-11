# sudokusolver-rs

References the backtracking search algorithm as described in this [towardsdatascience article](https://towardsdatascience.com/solving-sudoku-with-ai-d6008993c7de) by Justin Svegliato and solves a sudoku puzzle using constraint propagation (in Rust!).

## usage
problem file to open is passed as the first argument and takes the form of comma-separated values (9 per row, see the problems folder for examples). If no file path is given, it defaults to testprob1 (described below)

## example problems
are in the problems folder
- testprob1 is from the [Wikipedia page](https://en.wikipedia.org/wiki/Sudoku)
- the rest are from [this arizona.edu website](https://sandiway.arizona.edu/sudoku/examples.html)
    - testprob2 is Tuesday Jan 17 2006 from the Arizona Daily Wildcat
    - testprob3 is challenge 1 from sudoku solver from logic
