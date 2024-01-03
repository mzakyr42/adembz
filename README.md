# adembz

adembz (A Dumb Esolang Made By Zaky) is an esoteric programming language that are designed to be just using alphabet, number and parenthases.

adembz is a cell based memory language, 10.000 cell memory filled with unsigned 8 bit integer 0, with external variable called factor with unsigned 8 bit integer 1.

as a cell based memory language, it can move left and right in the memory, its called memory pointer.

it is similiar or even same as brainf*ck.

oh yeah its inspired by brainf*ck.

adembz has 18 command:
|commands|description|
|--------|-----------|
|l|move memory pointer left|
|r|move memory pointer right|
|a|add current memory cell with factor|
|s|subtract current memory cell with factor|
|d|floor divide current memory cell with factor<br>if the current cell or factor is 0 then its gonna be 0|
|m|multiply current memory cell with factor|
|0|set the current memory cell 0|
|A|add factor cell with current memory cell|
|S|subtract factor with current memory cell|
|D|floor divide factor with current memory cell<br>if the current cell or factor is 0 then its gonna be 0 |
|M|multiply factor with current memory cell|
|Z|make the factor 0|
|p|print current memory cell as character|
|P|print current memory cell as number|
|i|input and store it in the current memory cell|
|q|quine|
|(|if the current memory cell is 0 then jump past to matching )|
|)|if the current memory cell isn't 0 then jump back to matching (|
