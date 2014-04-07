The entry point (`main()`) is in `msweeper.rs`.

The `Board` is defined in `board/mod.rs`. The `Show` trait is implemented in `board/board_show.rs`. There is also a `ConsoleInput` trait which is implemented in `board/console_input.rs`.

A `Board` contains two-dimension vector of `Square`, which is defined in `board/square.rs`. There is also an iterator struct called `SquareIter` that is used to iterate through all the squares of a `Board` and is defined in `board/square_iter.rs`.