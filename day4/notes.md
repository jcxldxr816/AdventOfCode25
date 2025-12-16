# Problem
rolls of paper (@) can only be accessed if there are fewer than four adjacent rolls (out of eight possible spots)

# Thinking
2d array
loop through each line
  loop through each char
    if char = @ //(i, j)
      check (i-1, j-1), etc.
