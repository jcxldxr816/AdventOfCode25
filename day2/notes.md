# Problem
- comma separated values
- "first_id-last_id" (separated by hyphen)
- identify invalid ids by looking for **some sequence of digits repeated twice**
- there are no leading zeroes
- sequences are **only** repeated twice

# Thinking
- invalid id-pattern includes all digits. can't have a partially invalid id
- need to consider all values in range given
- only need to consider **even length** values

for each even value in range, compare the first half of string to last half.

## Part Two
- pattern-length (x) variable, starts at half of string length, decreases to 1

if len % x == 0, get x chars (len / x times), compare them
