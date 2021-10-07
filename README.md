# human-resource-machine

This project allows you to solve the problems in https://en.wikipedia.org/wiki/Human_Resource_Machine without owning the game.

For this you write `*.human` files, that can't be included by any other `.human` files, each is a standalone program.

There are the following instructions:

| instruction    | description                                                                                                 |
|----------------|-------------------------------------------------------------------------------------------------------------|
| inbox          | Read a number from user input                                                                               |
| output         | Output a number as result                                                                                   |
| copyfrom n     | Copy the number on register n and write it to the current buffer                                            |
| copyto n       | Copy the number from the current buffer to register n                                                       |
| add n          | Add the number of register n to the current buffer                                                          |
| sub n          | Subtract the number of register n from the current buffer                                                   |
| mul n          | Multiply the number of register n with the number from the current buffer and save it to the current buffer |
| bump+ n        | Add one to the number of register n and save it to the current buffer and register n                        |
| bump- n        | Subtract one from the number of register n and save it to the current buffer and register n                 |
| label n        | A label where a `jump` instruction can jump to                                                              |
| jump n         | Jump to the `label m` instruction, where n == m                                                             |
| jumpzero n     | Same as `jump n`, but only jumps if the number in the current buffer in 0                                   |
| jumpnegative n | Same as `jump n`, but only jumps if the number in the current buffer is less than 0                         |

There are 10 registers (register 0 to 10), in which you can write and read. The current buffer and the registers store 32 bit signed integers.

To run your program, run
`cargo r --release -- <filename.human> <enable_logging> <number1, number2, ...>`

Example:

`cargo r --release -- "prog.human" 0 100`
