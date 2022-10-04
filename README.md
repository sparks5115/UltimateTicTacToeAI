# Group members: Sam Parks, Sidney Goldinger, Rusen Emre Sabaz

# how to build executable:
if not already installed, install rust (https://www.rust-lang.org/tools/install)
from project root(wombat/) run command "cargo build" to generate a binary or "cargo run" to build and run\

# utility function:
Returns a heuristic of i32::MAX if the player is winning in the terminal state
Returns a heuristic of i32::MIN if the player is losing in the terminal state
Returns 0 otherwise (for a draw)

# evaluation function:
## check a board state and assign points according to the following:
    total_win_loss: i32::MAX,
    board_win_loss: 100,
    two_boards_in_row: 200,
    block_opponent_board: 150,
    two_in_row: 5,
    block_opponent: 20

# Time limit:
We used a iterative-deepening search to account for time, as well as a timer thread that submits the "best_so_far" move before the time limit is up.
We also used alpha-beta pruning to increase the speed at which we could complete a depth of minimax

# Results:
We tested this program against its self and analyzed the moves that it made while playing
## Strengths:
    Due to using a low level language, and use of lightweight, fast, data structures we are hoping to increase the speed at which we can compute minimax, allowing for deeper thought out moves.
    We believe that the implementation of the timer thread also being the thread that submits the move allows us more time to compute on the main thread without worrying about time.
## Weaknesses:
    As of right now, our board values winning a board that has no actual value to the overall game. This has potential to lead the agent astray.

# Discussion on heuristics:
We use a modified version of the heuristic described in "Powell, S., & Merrill, A. (2021). ULTIMATE TIC-TAC-TOE."
This paper reports to have a 100% win rate against a random and greedy agent using their heuristic.
