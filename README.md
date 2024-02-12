# Compressed Game Notation

This repository contains all the source code for my final year Computer Science dissertation at the University of Warwick (2021-2024). The project revolves around compressing PGN files, which is the standard format for storing chess games. 

Within this project, I propose a new state-of-the-art compression algorithm for PGN files, building on an existing implementation from Lichess.org. In addition to the core library, I also developed a command-line interface and a browser extension (via WebAssembly) to demonstrate the practical applications of the compression algorithm.

## Repository Structure
- `benchmarking`: Jupyter notebooks for benchmarking the compression algorithms.
- `cgn`: The main library for compressing PGN files (git submodule).
- `cgn-cli`: A command-line interface for compressing PGN files (git submodule).
- `cgn-extension`: A browser extension for compressing PGN files (git submodule).
- `JOURNAL.md`: The markdown journal I kept throughout the project.