# Sudoku
This is my implementation of a backtracking solution to the sudoku problem. I took inspiration from several
places but no code was copied from anywhere but the Rust book. This was mainly an exercise in trying to learn the Rust language.
The Wikipidea [entry](https://en.wikipedia.org/wiki/Sudoku_solving_algorithms) for this topic proved to be very helpful as well as 
[this](http://www.norvig.com/sudoku.html) blog post by Peter Norvig. 

## Running the program
The files in the (Examples)(./Examples) directory contain sudoku puzzles in line form. They are grouped into easy(50), hard(95), 
and even harder(11). These were taken from the Peter Norvig blog. When running these files with the Cargo -release profile, all 
puzzles are solved in less than .1 sec. The following are the benchmark outputs from my machine. 
### Easy puzzles
`50 runs took an average of 0 ms`

### Hard puzzles
`
95 runs took an average of 17 ms
`
### Even Harder
`
11 runs took an average of 0 ms
`

## Future Work
I would like to experiment with threading in Rust and i think this could be a good use case for it. The batches of sudoku puzzles could
be done in parallel. 

Additionally, the algorithm could be made more space efficient with the use of the Dancing Links. Right now I think I am using too
much memory for each recursive step. 
