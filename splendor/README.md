# splendor
A solver for the board game [Splendor](https://www.spacecowboys.fr/splendor)!

## How I play
This solver is meant for how I play, in which players try to reach 15 points. The nobles are ignored when I play, so they are not included in this (however they might be added in the future).

## Usage
Make a file called `board.txt` (in the working directory of the command). This will contain the board state. Every time the board changes in one of the following events, re-run this program:
- A card is bought
- A card is reserved
- You buy a card


### Cards
The board file contains a list of cards. The format for cards is:
```
<card points><card gem><requirement #1 count><requirement #1 gems><requirement #2 count><requirement #2 gems>...
```
For example, the card
```
3r3d5s3e3o
```
Would be a ruby card worth 3 points that required:
- 3 diamonds
- 5 sapphires
- 3 emeralds
- 3 onyx

Here are all the available gems:

| Letter | Gem |
| ------ | --- |
| `r` | Ruby |
| `e` | Emerald |
| `o` | Onyx |
| `s` | Sapphire |
| `d` | Diamond |

### Ordering
The file is composed of 12 lines of cards that tell the cards on the board, followed by the cards the you have. 

I usually order them as shown below (x, y coordinates) so that when the board changes, the file can easily be updated:
```
1, 1
1, 2
1, 3
1, 4
2, 1
2, 2
2, 3
2, 4
3, 1
3, 2
3, 3
3, 4
My card #1
My card #2
My card #3
```

### Output
The solver will output this (on the example board):
```
Checking depth 1...
Checking depth 2...
Checking depth 3...
Checking depth 4...
Checking depth 5...
Checking depth 6...
Checking depth 7...
Checking depth 8...
Checking depth 9...
Checking depth 10...
Checking depth 11...
Checking depth 12...
Checking depth 13...
Checking depth 14...
Checking depth 15...
Checking depth 16...
Checking depth 17...
Checking depth 18...
Checking depth 19...
Checking depth 20...
(1, 2) - 4r
(2, 3) - 1r
(1, 4) - 3o
(2, 2) - 2o
(2, 1) - 2s
(1, 3) - 3e
```
The last `Checking Depth` statement indicates the number of turns it believes it will take (note that it assumes beyond best-case scenarios).

After that, it has a list of statements in the format
```
(x, y) - <points><gem>
```
This says the position of the card on the board along with it's gem (in the same format as inputted) and point value. 

You should get the cards in the order they are listed to win.

## Tips
Follow these tips for the most success:
- If you need to reserve a card in order to obtain gold, add all the cards you have already reserved to your inventory in the board file and re-run the program, then reserve the first card it shows
- Reserve the first few cards in its plan in the start of the game and work on these before executing more of it's plan
- Pay attention to other player's moves so that your board file doesn't get out of date!
