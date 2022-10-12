#Changes since initial submission:
    found a faster compiler - Sam
    removed print statements for speed - Sam
    added two heuristics to account for "useless" moves - Sam and Sid
    bugfixing for working with the referee - Sam
    extensive testing - Sam

#Running:
if there are any issues with running this in the way described in run.txt,</br>
you can compile it with "cargo build --release" which will create an executable </br>
at "./targert/release/wombat" </br>
This will allow you to move this executable to any desired location and run with:</br>
"./wombat" or "./wombat.exe"

In addition, the code will live here if anything is missing: [https://github.com/sparks5115/UltimateTicTacToeAI/tree/optimized-submission](https://github.com/sparks5115/UltimateTicTacToeAI/tree/optimized-submission)

#PLEASE NOTE:
There is still a bug where, due to a race condition on the move_file, the referee will declare an invalid move. </br>
Code was provided by the TA only for python to fix this issue, so we were unable to fix this bug. </br>
Note that we still print the move that we were placing when the referee accessed the file (and it will be in the move_file). </br>
Due to this, we ask that this is forgiven as we are not aware of any way to fix this without modifying the referee code.




