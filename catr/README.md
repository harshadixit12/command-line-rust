# Problem statement
Implement clone of `cat` command.

Features:
1. Read given files sequentially and output to stdout
    1a. If file name is not provided, accept input from stdin
2. Support flags for outputting number of lines, and non-blank lines
3. Handle errors gracefully - per file
4. Return an exit code - non zero when there are errors