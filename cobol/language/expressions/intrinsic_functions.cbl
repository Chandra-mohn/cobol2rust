       IDENTIFICATION DIVISION.
       PROGRAM-ID. INTRINSIC-FUNCTIONS-TEST.
      *---------------------------------------------------------------
      * Stress test: Intrinsic functions.
      * Covers: FUNCTION LENGTH, UPPER-CASE, LOWER-CASE, TRIM,
      * REVERSE, NUMVAL, NUMVAL-C, MAX, MIN, MOD,
      * INTEGER, INTEGER-PART, REM, ABS, CURRENT-DATE,
      * WHEN-COMPILED, ORD, CHAR, CONCATENATE.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-STR               PIC X(30).
       01  WS-NUM               PIC 9(9)V99.
       01  WS-INT               PIC 9(9).
       01  WS-LEN               PIC 999.
       01  WS-RESULT            PIC X(40).
       01  WS-A                 PIC 9(5) VALUE 100.
       01  WS-B                 PIC 9(5) VALUE 75.
       01  WS-C                 PIC 9(5) VALUE 200.
       01  WS-SDEC              PIC S9(5)V99 VALUE -123.45.
       01  WS-CURR-DATE         PIC X(21).
       01  WS-COMPILE-DATE      PIC X(21).
       01  WS-ORD-VAL           PIC 999.
       01  WS-CHAR-VAL          PIC X.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * FUNCTION LENGTH
           MOVE FUNCTION LENGTH(WS-STR) TO WS-LEN.
           DISPLAY "LENGTH: " WS-LEN.
      * FUNCTION UPPER-CASE
           MOVE "hello world" TO WS-STR.
           MOVE FUNCTION UPPER-CASE(WS-STR) TO WS-RESULT.
           DISPLAY "UPPER: " WS-RESULT.
      * FUNCTION LOWER-CASE
           MOVE "HELLO WORLD" TO WS-STR.
           MOVE FUNCTION LOWER-CASE(WS-STR) TO WS-RESULT.
           DISPLAY "LOWER: " WS-RESULT.
      * FUNCTION TRIM
           MOVE "  TRIMMED  " TO WS-STR.
           MOVE FUNCTION TRIM(WS-STR) TO WS-RESULT.
           DISPLAY "TRIM: >" WS-RESULT "<".
      * FUNCTION REVERSE
           MOVE "ABCDEF" TO WS-STR.
           MOVE FUNCTION REVERSE(WS-STR) TO WS-RESULT.
           DISPLAY "REVERSE: " WS-RESULT.
      * FUNCTION NUMVAL
           MOVE "12345" TO WS-STR.
           COMPUTE WS-NUM = FUNCTION NUMVAL(WS-STR).
           DISPLAY "NUMVAL: " WS-NUM.
      * FUNCTION MAX
           COMPUTE WS-INT = FUNCTION MAX(WS-A WS-B WS-C).
           DISPLAY "MAX: " WS-INT.
      * FUNCTION MIN
           COMPUTE WS-INT = FUNCTION MIN(WS-A WS-B WS-C).
           DISPLAY "MIN: " WS-INT.
      * FUNCTION MOD
           COMPUTE WS-INT = FUNCTION MOD(WS-A 7).
           DISPLAY "MOD: " WS-INT.
      * FUNCTION INTEGER
           COMPUTE WS-INT = FUNCTION INTEGER(123.99).
           DISPLAY "INTEGER: " WS-INT.
      * FUNCTION INTEGER-PART
           COMPUTE WS-INT = FUNCTION INTEGER-PART(123.99).
           DISPLAY "INT-PART: " WS-INT.
      * FUNCTION REM
           COMPUTE WS-NUM = FUNCTION REM(100 7).
           DISPLAY "REM: " WS-NUM.
      * FUNCTION ABS
           COMPUTE WS-NUM = FUNCTION ABS(WS-SDEC).
           DISPLAY "ABS: " WS-NUM.
      * FUNCTION CURRENT-DATE
           MOVE FUNCTION CURRENT-DATE TO WS-CURR-DATE.
           DISPLAY "CURR-DATE: " WS-CURR-DATE.
      * FUNCTION WHEN-COMPILED
           MOVE FUNCTION WHEN-COMPILED TO WS-COMPILE-DATE.
           DISPLAY "COMPILED: " WS-COMPILE-DATE.
      * FUNCTION ORD
           COMPUTE WS-ORD-VAL = FUNCTION ORD("A").
           DISPLAY "ORD(A): " WS-ORD-VAL.
      * FUNCTION CHAR
           MOVE FUNCTION CHAR(66) TO WS-CHAR-VAL.
           DISPLAY "CHAR(66): " WS-CHAR-VAL.
           STOP RUN.
