       IDENTIFICATION DIVISION.
       PROGRAM-ID. LEVEL88-TEST.
      *---------------------------------------------------------------
      * Stress test: Level 88 condition names.
      * Covers: Single VALUE, multiple VALUES, VALUE THRU,
      * SET TO TRUE/FALSE, use in IF conditions.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Single value conditions
       01  WS-STATUS           PIC X(2).
           88  STATUS-OK       VALUE "OK".
           88  STATUS-ERR      VALUE "ER".
           88  STATUS-EOF      VALUE "EF".
      * Multiple values
       01  WS-GRADE            PIC X.
           88  GRADE-PASS      VALUE "A" "B" "C".
           88  GRADE-FAIL      VALUE "D" "F".
           88  GRADE-HONORS    VALUE "A".
      * Numeric with VALUE THRU
       01  WS-SCORE            PIC 999.
           88  SCORE-HIGH      VALUE 90 THRU 100.
           88  SCORE-MED       VALUE 70 THRU 89.
           88  SCORE-LOW       VALUE 0 THRU 69.
           88  SCORE-PERFECT   VALUE 100.
      * Boolean-like flag
       01  WS-EOF-FLAG         PIC 9.
           88  WS-EOF          VALUE 1.
           88  WS-NOT-EOF      VALUE 0.
      * Multiple condition on same field
       01  WS-MONTH            PIC 99.
           88  MONTH-Q1        VALUE 1 THRU 3.
           88  MONTH-Q2        VALUE 4 THRU 6.
           88  MONTH-Q3        VALUE 7 THRU 9.
           88  MONTH-Q4        VALUE 10 THRU 12.
           88  MONTH-VALID     VALUE 1 THRU 12.
      * Result
       01  WS-RESULT           PIC X(20).
       01  WS-COUNT            PIC 999 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Test single value
           MOVE "OK" TO WS-STATUS.
           IF STATUS-OK
               MOVE "STATUS IS OK" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Test multiple values
           MOVE "B" TO WS-GRADE.
           IF GRADE-PASS
               MOVE "GRADE PASSED" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Test THRU range
           MOVE 95 TO WS-SCORE.
           IF SCORE-HIGH
               MOVE "HIGH SCORE" TO WS-RESULT
           END-IF.
           IF SCORE-PERFECT
               MOVE "PERFECT!" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Test SET TO TRUE
           SET WS-EOF TO TRUE.
           IF WS-EOF
               MOVE "EOF REACHED" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Test SET TO FALSE
           SET WS-NOT-EOF TO TRUE.
           IF WS-NOT-EOF
               MOVE "NOT EOF" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Test quarter detection
           MOVE 7 TO WS-MONTH.
           IF MONTH-Q3
               MOVE "QUARTER 3" TO WS-RESULT
           END-IF.
           IF MONTH-VALID
               ADD 1 TO WS-COUNT
           END-IF.
           DISPLAY WS-RESULT.
           DISPLAY WS-COUNT.
           STOP RUN.
