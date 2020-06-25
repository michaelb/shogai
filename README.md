# shogai

a rust interface for shogi engines

## What it is

an helper programm that can create, manage, display (on terminal), and modify a shogiban (the board of shogi)

What it means:
You can create a board, ask for possible (legal) moves, play such a move, print a board on the CLI, get things like the value (good position?) and nice things.

## Who it is for

Honestly, I don't really know. Do you want to implement a shogi _engine_?
Maybe that's the right place...
I mean, you can do a shogi engine in a few lines (see ai::greedy) with the help of shogai, but as long as performance is a concern, I cannot offer any garanties. It is still quite optimized, but code clarity was more important than pure speed here.

DISCLAIMER:
I do not have a way to compare performance with other shogi engines, as implementing a shogi engine is a completely different story, tons of if/elses branches etc...

If you are interested in building your own "high-perf" shogi engine in rust, you can use this helper to fasten development. You may have to redo some parts yourself is performance is not up to your expectations.
Advantages of using shogai ?:

- CLI interface? Done
- move-checking (in case you are not 100% sure of yours)? Done
- Simple structure for pieces, position, movements with parsing? Done

## Installation

add

```
shogai="*"
```

to your Cargo.toml

## How to make it work

(or whatever it means for you)
Depending on your goals, (i mean: performance goals), your usage can conform to examples as shown in documentation, or get _quite_ optimized, but then, more complex.

The structure of your basic shogi program should look like this:

- create a Board
- loop
- - read, get, generate ... a move ( as &str)
- - play the move
- - check if the game is over (usually a checkmate)

## Contributing

PR can be accepted, and contributors are welcome!
