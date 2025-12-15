# Problem
joltage: a number 1 to 9

each line: bank of batteries

need to turn on 2 batteries per bank

Joltages that are selected are the digits for that bank's output.
```
12345 => 24
_x_x_
```

find the largest possible joltage each bank can produce

# Thinking
- for first digit, pick the largest number from 0..len-1
- if there are duplicates, favor the leftmost
- for the second digit, pick the largest value to the right of the first

## Part Two
- keep an array of Digit objects
- for each battery, 
  - pick largest leftmost number, 
  - to the right of rightmost selected digit
