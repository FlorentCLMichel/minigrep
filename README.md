# minigrep

This is a Rust implementation of a grep-like command, inspired by (although slightly different than) an example given in the [Rust book](https://doc.rust-lang.org/stable/book). 

## Build 

You will need Rust (edition 2018) and Cargo to build this project. Instructions to install them can be found on the [rust-lang](https://www.rust-lang.org/tools/install) website. 

Run either 

    make 
or

    cargo build --release

This will generate the executable file will be in target/release/minigrep.

The documentation can be generated by running

    make doc
or
 
    cargo doc

and found in the target/html/minigrep folder.

## Arguments 

Two required arguments: 

* the string to be searched

* the file in which to search

One optional argument determining how the matches are highlighted:
    0 → no highlight
    1 → bold
    2 → dimmed
    3 → italic
    4 → underline
    5 → blink
    7 → reversed
    8 → hidden
    9 → strikethrough

## Example use 

    minigrep you poem.txt 1

will search the word ‘you’ in the file ‘poem.txt’, print the lines that countain it, and highlight the matches in bold. 
