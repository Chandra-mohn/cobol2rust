       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPARISON-OPS-TEST.
      *---------------------------------------------------------------
      * Stress test: Comparison operators and conditions.
      * Covers: =, <, >, <=, >=, NOT =, numeric compare,
      * alphanumeric compare, mixed type compare,
      * class conditions (NUMERIC, ALPHABETIC).
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-NUM-A             PIC 9(5) VALUE 100.
       01  WS-NUM-B             PIC 9(5) VALUE 200.
       01  WS-SDEC-A            PIC S9(5)V99 VALUE -50.25.
       01  WS-SDEC-B            PIC S9(5)V99 VALUE 50.25.
       01  WS-ALPHA-A           PIC X(10) VALUE "APPLE".
       01  WS-ALPHA-B           PIC X(10) VALUE "BANANA".
       01  WS-MIXED             PIC X(5) VALUE "12345".
       01  WS-ALPHA-ONLY        PIC X(5) VALUE "ABCDE".
       01  WS-RESULT            PIC X(30).
       01  WS-PACK-A            PIC S9(7)V99 COMP-3 VALUE 100.50.
       01  WS-PACK-B            PIC S9(7)V99 COMP-3 VALUE 100.50.
       01  WS-COMP-A            PIC S9(9) COMP VALUE 1000.
       01  WS-COMP-B            PIC S9(9) COMP VALUE 999.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Equal
           IF WS-NUM-A = 100
               MOVE "NUM = 100" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Not equal
           IF WS-NUM-A NOT = WS-NUM-B
               MOVE "A != B" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Greater than
           IF WS-NUM-B > WS-NUM-A
               MOVE "B > A" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Less than
           IF WS-NUM-A < WS-NUM-B
               MOVE "A < B" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Greater or equal
           IF WS-NUM-A >= 100
               MOVE "A >= 100" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Less or equal
           IF WS-NUM-A <= 100
               MOVE "A <= 100" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Signed decimal comparison
           IF WS-SDEC-A < WS-SDEC-B
               MOVE "NEG < POS" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Negative comparison
           IF WS-SDEC-A < 0
               MOVE "A IS NEGATIVE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Alphanumeric comparison
           IF WS-ALPHA-A < WS-ALPHA-B
               MOVE "APPLE < BANANA" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Alpha equal
           IF WS-ALPHA-A = "APPLE"
               MOVE "MATCH APPLE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Packed decimal equal
           IF WS-PACK-A = WS-PACK-B
               MOVE "PACKS EQUAL" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Binary comparison
           IF WS-COMP-A > WS-COMP-B
               MOVE "COMP A > B" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IS NUMERIC
           IF WS-MIXED IS NUMERIC
               MOVE "MIXED IS NUMERIC" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IS ALPHABETIC
           IF WS-ALPHA-ONLY IS ALPHABETIC
               MOVE "IS ALPHABETIC" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IS NOT NUMERIC
           IF WS-ALPHA-ONLY IS NOT NUMERIC
               MOVE "NOT NUMERIC" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Compare with SPACES
           IF WS-ALPHA-A NOT = SPACES
               MOVE "NOT SPACES" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Compare with ZEROS
           IF WS-NUM-A NOT = ZEROS
               MOVE "NOT ZEROS" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
           STOP RUN.
