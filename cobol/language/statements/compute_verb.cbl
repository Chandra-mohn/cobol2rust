       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPUTE-VERB-TEST.
      *---------------------------------------------------------------
      * Stress test: COMPUTE statement.
      * Covers: +, -, *, /, ** (exponentiation), parentheses,
      * ROUNDED, ON SIZE ERROR, mixed types, chained expressions.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-A                 PIC 9(5) VALUE 10.
       01  WS-B                 PIC 9(5) VALUE 20.
       01  WS-C                 PIC 9(5) VALUE 30.
       01  WS-D                 PIC 9(5) VALUE 0.
       01  WS-RESULT            PIC 9(9)V99 VALUE 0.
       01  WS-R2                PIC 9(9)V99 VALUE 0.
       01  WS-SMALL             PIC 9(3) VALUE 0.
       01  WS-SDEC1             PIC S9(5)V99 VALUE 123.45.
       01  WS-SDEC2             PIC S9(5)V99 VALUE 67.89.
       01  WS-PACK1             PIC S9(7)V99 COMP-3 VALUE 1000.00.
       01  WS-PACK2             PIC S9(7)V99 COMP-3 VALUE 25.00.
       01  WS-COMP1             PIC S9(9) COMP VALUE 10000.
       01  WS-FLOAT1            COMP-1 VALUE 3.14.
       01  WS-FLOAT2            COMP-2 VALUE 2.71828.
       01  WS-ROUNDED           PIC 9(5)V9 VALUE 0.
       01  WS-ERR-FLAG          PIC 9 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple addition
           COMPUTE WS-RESULT = WS-A + WS-B.
           DISPLAY WS-RESULT.
      * Subtraction
           COMPUTE WS-RESULT = WS-C - WS-A.
           DISPLAY WS-RESULT.
      * Multiplication
           COMPUTE WS-RESULT = WS-A * WS-B.
           DISPLAY WS-RESULT.
      * Division
           COMPUTE WS-RESULT = WS-C / WS-A.
           DISPLAY WS-RESULT.
      * Exponentiation
           COMPUTE WS-RESULT = WS-A ** 3.
           DISPLAY WS-RESULT.
      * Mixed arithmetic
           COMPUTE WS-RESULT = WS-A + WS-B * WS-C.
           DISPLAY WS-RESULT.
      * Parenthesized expression
           COMPUTE WS-RESULT = (WS-A + WS-B) * WS-C.
           DISPLAY WS-RESULT.
      * Nested parentheses
           COMPUTE WS-RESULT =
               ((WS-A + WS-B) * (WS-C - WS-A)) / 2.
           DISPLAY WS-RESULT.
      * Literals in expression
           COMPUTE WS-RESULT = 100 + 200 * 3 - 50.
           DISPLAY WS-RESULT.
      * Mixed field types
           COMPUTE WS-RESULT = WS-SDEC1 + WS-PACK1.
           DISPLAY WS-RESULT.
      * Decimal precision
           COMPUTE WS-RESULT = WS-SDEC1 * WS-SDEC2.
           DISPLAY WS-RESULT.
      * Complex expression
           COMPUTE WS-RESULT =
               WS-PACK1 / WS-PACK2 + WS-SDEC1 - WS-A.
           DISPLAY WS-RESULT.
      * Multiple targets (same expression)
           COMPUTE WS-RESULT WS-R2 = WS-A + WS-B + WS-C.
           DISPLAY WS-RESULT.
           DISPLAY WS-R2.
      * COMPUTE ROUNDED
           COMPUTE WS-ROUNDED ROUNDED = 100 / 3.
           DISPLAY WS-ROUNDED.
      * COMPUTE ON SIZE ERROR
           COMPUTE WS-SMALL = 999 + 1
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
               NOT ON SIZE ERROR
                   MOVE 0 TO WS-ERR-FLAG
           END-COMPUTE.
           DISPLAY WS-ERR-FLAG.
      * Negative result
           COMPUTE WS-SDEC1 = 10 - 500.
           DISPLAY WS-SDEC1.
      * Float in COMPUTE
           COMPUTE WS-RESULT = WS-FLOAT1 * 100.
           DISPLAY WS-RESULT.
      * Chain of operations
           COMPUTE WS-RESULT =
               (WS-A ** 2 + WS-B ** 2) / (WS-A + WS-B).
           DISPLAY WS-RESULT.
      * Zero in denominator with SIZE ERROR
           COMPUTE WS-RESULT = WS-A / 0
               ON SIZE ERROR
                   MOVE 1 TO WS-ERR-FLAG
           END-COMPUTE.
           DISPLAY WS-ERR-FLAG.
           STOP RUN.
