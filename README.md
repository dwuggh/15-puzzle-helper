# fifteen-puzzle-helper

 `fifteen-puzzle-helper` is a solver and scramble generator for 15-puzzle.

 current status: under develop
 
## about
  This program use a heuristic **rank reduction** A* search algorithm to find solutions. Unlike other existing programs, the "distance" to target has a heuristic metric about the status of the most outside layer, say, in a 4x4 puzzle, the most outside layer is `1 2 3 4 5 9 13`. The algorithm's goal is not to solve the whole puzzle; instead, only to reduce its rank. This appoarch is rather simple, but surprisingly effective.
 
## usage
```bash
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
solved! Use 37.440843819 seconds
[Up, Right, Right, Down, Down, Left, Up, Left, Down, Right, Up, Up, Right, Up, Left, Down, Left, Down, Left, Down, Right, Up, Right, Up, Left, Left, Up, Right, Right, Down, Left, Up, Right, Down, Left, Down, Right, Right, Up, Up, Left, Down, Down, Left, Up, Left, Up, Right, Right, Down, Right, Down, Left, Up, Left, Down, Right, Right, Down, Left, Up, Right, Up, Left, Down, Right, Down, Left, Left, Up, Right, Down, Right, Up, Left, Down, Right]

   12    3    8   10
    4   11         2
   14    5   13   15
    9    7    6    1
```
