       IDENTIFICATION DIVISION.
       PROGRAM-ID. ARITHMETIC-VERBS-TEST.
      *---------------------------------------------------------------
      * Stress test: ADD, SUBTRACT, MULTIPLY, DIVIDE.
      * Covers: TO, GIVING, ROUNDED, ON SIZE ERROR,
      * NOT ON SIZE ERROR, multiple operands, literals.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-A                 PIC 9(5) VALUE 100.
       01  WS-B                 PIC 9(5) VALUE 200.
       01  WS-C                 PIC 9(5) VALUE 300.
       01  WS-D                 PIC 9(5) VALUE 0.
       01  WS-RESULT            PIC 9(7)V99 VALUE 0.
       01  WS-SMALL             PIC 9(3) VALUE 0.
       01  WS-SDEC1             PIC S9(5)V99 VALUE 123.45.
       01  WS-SDEC2             PIC S9(5)V99 VALUE 67.89.
       01  WS-SDEC3             PIC S9(5)V99 VALUE 0.
       01  WS-PACK1             PIC S9(7)V99 COMP-3 VALUE 1000.00.
       01  WS-PACK2             PIC S9(7)V99 COMP-3 VALUE 500.50.
       01  WS-PACK3             PIC S9(7)V99 COMP-3 VALUE 0.
       01  WS-COMP1             PIC S9(9) COMP VALUE 10000.
       01  WS-COMP2             PIC S9(9) COMP VALUE 3.
       01  WS-COMP3             PIC S9(9) COMP VALUE 0.
       01  WS-REMAINDER         PIC 9(5) VALUE 0.
       01  WS-ERR-FLAG          PIC 9 VALUE 0.
       01  WS-RNDVAL           PIC 9(5)V9 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      *--- ADD variants ---
      * ADD TO
           ADD WS-A TO WS-B.
           DISPLAY WS-B.
      * ADD GIVING
           ADD WS-A WS-C GIVING WS-D.
           DISPLAY WS-D.
      * ADD literal
           ADD 50 TO WS-A.
           DISPLAY WS-A.
      * ADD multiple
           ADD WS-A WS-B WS-C GIVING WS-RESULT.
           DISPLAY WS-RESULT.
      * ADD ROUNDED
           ADD 10.55 TO WS-RNDVAL ROUNDED.
           DISPLAY WS-RNDVAL.
      * ADD ON SIZE ERROR
           MOVE 999 TO WS-SMALL.
           ADD 1 TO WS-SMALL
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
               NOT ON SIZE ERROR
                   MOVE 0 TO WS-ERR-FLAG
           END-ADD.
           DISPLAY WS-ERR-FLAG.
      * ADD signed decimals
           ADD WS-SDEC1 TO WS-SDEC2.
           DISPLAY WS-SDEC2.
      * ADD packed
           ADD WS-PACK1 TO WS-PACK2.
           DISPLAY WS-PACK2.
      * ADD binary
           ADD WS-COMP1 TO WS-COMP2.
           DISPLAY WS-COMP2.
      *--- SUBTRACT variants ---
      * SUBTRACT FROM
           MOVE 500 TO WS-A.
           MOVE 200 TO WS-B.
           SUBTRACT WS-B FROM WS-A.
           DISPLAY WS-A.
      * SUBTRACT GIVING
           SUBTRACT 100 FROM 500 GIVING WS-D.
           DISPLAY WS-D.
      * SUBTRACT ON SIZE ERROR
           MOVE 0 TO WS-SMALL.
           SUBTRACT 1 FROM WS-SMALL
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
           END-SUBTRACT.
           DISPLAY WS-ERR-FLAG.
      *--- MULTIPLY variants ---
      * MULTIPLY BY
           MOVE 25 TO WS-A.
           MULTIPLY 4 BY WS-A.
           DISPLAY WS-A.
      * MULTIPLY GIVING
           MULTIPLY WS-A BY WS-B GIVING WS-RESULT.
           DISPLAY WS-RESULT.
      * MULTIPLY ROUNDED
           MULTIPLY 3.33 BY WS-RNDVAL ROUNDED.
           DISPLAY WS-RNDVAL.
      * MULTIPLY ON SIZE ERROR
           MOVE 999 TO WS-SMALL.
           MULTIPLY 2 BY WS-SMALL
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
           END-MULTIPLY.
           DISPLAY WS-ERR-FLAG.
      * MULTIPLY packed
           MOVE 100.00 TO WS-PACK1.
           MULTIPLY WS-PACK1 BY WS-PACK2.
           DISPLAY WS-PACK2.
      *--- DIVIDE variants ---
      * DIVIDE INTO
           MOVE 100 TO WS-A.
           DIVIDE 5 INTO WS-A.
           DISPLAY WS-A.
      * DIVIDE GIVING
           DIVIDE 100 BY 3 GIVING WS-D.
           DISPLAY WS-D.
      * DIVIDE REMAINDER
           DIVIDE 100 BY 3 GIVING WS-D
               REMAINDER WS-REMAINDER.
           DISPLAY WS-D.
           DISPLAY WS-REMAINDER.
      * DIVIDE ROUNDED
           DIVIDE 10 BY 3 GIVING WS-RNDVAL ROUNDED.
           DISPLAY WS-RNDVAL.
      * DIVIDE ON SIZE ERROR (divide by zero)
           DIVIDE 0 INTO WS-A
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
               NOT ON SIZE ERROR
                   MOVE 0 TO WS-ERR-FLAG
           END-DIVIDE.
           DISPLAY WS-ERR-FLAG.
      * DIVIDE binary
           MOVE 10000 TO WS-COMP1.
           DIVIDE 7 INTO WS-COMP1.
           DISPLAY WS-COMP1.
           STOP RUN.
