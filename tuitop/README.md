## What is this?
A bare-metal TUI, using no TUI libraries or frameworks.

## Design
main.rs imports from other crates, and runs a loop that re-renders every 500 ms.
Input is consumed from another thread through a channel, to not block the main thread.

## Why?
To understand fundamentals of TUIs, and to get some coding time in Rust.

## How to use?
