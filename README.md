# Turing Machine

This is a simple terminal based turing machine written in Rust.
The `code.txt` file contains the instructions for the turing machine.
If any line starts with a `#` it is ignored.
The first line contains the initial tape
The second contains the starting position from the left of the tape.
The third line contains the starting state.
For every next set of four lines, the first line contains the name of the state, the next three contains the transition rules for `0`, `1` and `_` respectively.
Each transition rule consists of the symbol set then the direction (`<` for left and `>` for right) then the state it switches to.
Furthermore the first state defined is the halting state.

