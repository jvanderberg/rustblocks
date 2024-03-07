```
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜🟨🟨⬜⬜⬜⬜🟪⬜⬜⬜⬜⬜⬜⬜⬜⬜🟧⬜⬜⬜🟦⬜⬜⬜⬜⬜🟩🟩⬜⬜🟥🟥⬜⬜⬜
⬜⬜🟨🟨⬜⬜🟪🟪🟪⬜⬜🟫🟫🟫🟫⬜⬜🟧🟧🟧⬜⬜🟦🟦🟦⬜⬜🟩🟩⬜⬜⬜⬜🟥🟥⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
```

# Rustblocks

Rustblocks is a simple tetromino based falling blocks game that uses crossterm to draw blocks in the terminal. A reasonable Unicode capable font is required as the character \u{2588} is used to draw the blocks. There are three different emoji blocks supported as well, the colored square 🟧, circle 🟠, and heart 🧡 emojis.

The game sticks roughly to 'official' piece dynamics but does not implement 'kicking' off the walls. It is entirely single threaded, with no async.

Controls:

    - Arrow keys or h,j,k,l to move
    - space to drop
    - Delete or Backspace to restart
    - b to toggle block emojies
    - d toggle difficulty
    - q to quit
    - u to undo
    - n to toggle next piece display
    - t key toggles the tracer block

### Running rustblocks

`rustblocks -h`

```

Usage: rustblocks [OPTIONS]

Options:
  -x, --horizontal <HORIZONTAL>  The width of the board [default: 10]
  -y, --vertical <VERTICAL>      The height of the board [default: 22]
  -n, --hide-next-piece          Whether to show the next piece
  -e, --emoji <EMOJI>            Use colored emojies instead of unicode block Square, Circle, Heart, or None [default: None]
  -d, --difficulty <DIFFICULTY>  The difficulty of the game, changes the speed of the game. Easy, Medium, Hard, Insane, or 1, 2, 3, 4 [default: Easy]
  -h, --help                     Print help
  -V, --version                  Print version

```

### Installing

`cargo install rustblocks`

### Binaries

#### MacOS

[Rustblocks x86_64 (will run on M1/2/3)](./bin/MacOS/rustblocks)

#### Windows

[Rustblocks x86_64 (will run on Windows for Arm)](./bin/Windows/rustblocks.exe)
