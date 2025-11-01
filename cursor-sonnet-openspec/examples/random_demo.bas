10 REM Random Number Demonstration
20 PRINT "=== Random Number Demo ==="
30 PRINT
40 REM Generate 5 random numbers between 0 and 1
50 PRINT "Five random decimals (0-1):"
60 FOR I = 1 TO 5
70 PRINT RND(1)
80 NEXT I
90 PRINT
100 REM Generate 5 random integers (1-10)
110 PRINT "Five random integers (1-10):"
120 FOR I = 1 TO 5
130 N = INT(RND(1) * 10) + 1
140 PRINT N
150 NEXT I
160 PRINT
170 REM Simple dice roll
180 PRINT "Roll a dice (1-6):"
190 D = INT(RND(1) * 6) + 1
200 PRINT "You rolled:"; D
210 END

