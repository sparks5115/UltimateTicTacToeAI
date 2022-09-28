#!/bin/bash
cargo build
cd target/debug
rm first_four_moves
rm nohup.out
nohup python3 game.py player1 player2 >/dev/null 2>&1 &
./UltimateTicTacToeAI
