# Rustblocks

Rustblocks is a simple tetromino based falling blocks game that uses crossterm to draw blocks in the terminal. A reasonable Unicode capable font is required as the character \u{2588} is used to draw the blocks.

The game sticks roughly to 'offical' piece dynamics but does not implement 'kicking' off the walls. It is entirely single threaded, with no async.

For performance, the game maintains two board buffers, last and current. Pieces are always recorded on the current board, and when committed only the differences are drawn to the screen.

Controls: 
- Arrow keys or h,j,k,l to move
- space to drop
- q to quit
- t key toggles the tracer block


### Running rustblocks

```rustblocks -h```

```
Usage: rustblocks [OPTIONS]

Options:
  -w, --horizontal <HORIZONTAL>  The width of the board [default: 10]
  -v, --vertical <VERTICAL>      The height of the board [default: 22]
  -h, --help                     Print help
  -V, --version                  Print version

```

### Installing


```cargo install rustblocks```

### Binaries

#### MacOS

[Rustblocks x86_64 (will run on M1/2/3)](./bin/MacOS/rustblocks)

#### Windows

[Rustblocks x86_64 (will run on Windows for Arm)](./bin/Windows/rustblocks.exe)
