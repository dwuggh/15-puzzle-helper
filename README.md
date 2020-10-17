# fifteen-puzzle-helper

 `fifteen-puzzle-helper` is a solver and scramble generator for 15-puzzle.

 current status: under develop
 
## about
This program use a heuristic **rank reduction** A* search algorithm to find solutions. Unlike other existing programs, the "distance" to target has a heuristic metric about the status of the most outside layer, say, in a 4x4 puzzle, the most outside layer is `1 2 3 4 5 9 13`. The algorithm's goal is not to solve the whole puzzle; instead, only to reduce its rank. This appoarch is rather simple, but surprisingly effective.

## usage
```
fifteen-puzzle-helper 0.1.0
dwuggh <dwuggh@gmail.com>
solver and scramble generator for 15-puzzle

USAGE:
    fifteen-puzzle-helper [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    scramble    scramble the 15 puzzle
    solve       solve a scramble
```

- geneate a 7x7 scrambled puzzle by `fifteen-puzzle-helper scramble -r 7`:
```
   22   44   13   39   43    8   42
   46   45   27   48   37   32   25
   26   38   47   29        18   31
    2   12   20    3   10   28    1
   24   17   19   21   40   36   23
   16    7   30    6    4   41   15
   34   11    5    9   14   33   35
```

- solve a random scrambled 4x4 puzzle by `fifteen-puzzle-helper solve -r 4`
```
solved! Use 37.974565483 seconds
[Left, Left, Up, Right, Down, Left, Down, Right, Up, Right, Down, Down, Left, Up, Up, Right, Down, Left, Up, Up, Right, Down, Left, Left, Up, Right, Down, Down, Left, Down, Left, Up, Up, Up, Right, Down, Left, Down, Right, Up, Left, Down, Right, Right, Right, Down, Left, Up, Left, Down, Right, Right, Up, Left, Up, Left, Down, Left, Down, Right, Up, Right, Down, Right, Up, Left, Left, Down, Right, Right, Up, Up, Left, Down, Down, Right, Up, Left, Up, Right, Down, Down]

   10   15    2    1
    7   12   13     
    9    3    8   11
    6    5   14    4
```
	note: The move `Up, RIght, Down, Left` represents the direction `empty site` goes.
