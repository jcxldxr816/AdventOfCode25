# Problem
instructions: L12 or R9, etc
dial has numbers 0-99
dial is "circular", 0-1=99
dial starts at 50

real password: number of times the dial is set to 0 after any rotation

# Thoughts
- could use a circular linkedlist, queue, other structure
- array 0-99, conditionals for under/overflow

track ints
L = subtract
if int < 0, subtract 0-int from 99
R = add
if int > 99, add int-99 to 0

increase counter if int == 0
