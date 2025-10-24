10 PRINT "Testing string functions"
20 LET S$ = "HELLO WORLD"
30 PRINT "Original: "; S$
40 PRINT "LEFT$(S$, 5): "; LEFT$(S$, 5)
50 PRINT "RIGHT$(S$, 5): "; RIGHT$(S$, 5)
60 PRINT "MID$(S$, 7, 5): "; MID$(S$, 7, 5)
70 PRINT "ASC(H): "; ASC("H")
80 PRINT "CHR$(65): "; CHR$(65)
90 PRINT "STR$(123): "; STR$(123)
100 PRINT "VAL(456): "; VAL("456")
110 PRINT "MID$(S$, 1, 5): "; MID$(S$, 1, 5)
120 PRINT "LEFT$(S$, 20): "; LEFT$(S$, 20)
130 PRINT "RIGHT$(S$, 0): "; RIGHT$(S$, 0)
140 PRINT "MID$(S$, 50, 3): "; MID$(S$, 50, 3)