# Purpose
Testing Cargo.io submissions, versioning, and documentation.

##  Tests will be written around a game of TicTacToe
### Terminal should be drawing this (format lost w/ .txt -> .md):

   |   |                        a1 |a2 |a3 
---|---|---                     ---|---|---
   |   |      --> Positions:    b1 |b2 |b3 
---|---|---                     ---|---|---
   |   |                        c1 |c2 |c3 

1. Randomizer decides which player starts. If rand % 2 = 0. user starts as player 1, and AI plays as player 2.

2. X is assigned to player 1, O to player 2.

3. Turn based, updated board is printed below. Terminal isn't cleared for simplicity.

4. Player is presented with cell ID and types it in to select position when prompted.

5. X / O are drawn when player/AI make their moves.

6. Table state is saved with every move.

7. System checks for wins after positions are submitted. after starting at play #3.

