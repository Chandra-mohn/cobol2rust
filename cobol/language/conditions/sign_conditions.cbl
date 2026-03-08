       IDENTIFICATION DIVISION.
       PROGRAM-ID. SIGN-CONDITIONS-TEST.
      *---------------------------------------------------------------
      * Stress test: Sign conditions and POSITIVE/NEGATIVE/ZERO.
      * Covers: IS POSITIVE, IS NEGATIVE, IS ZERO,
      * IS NOT POSITIVE, IS NOT NEGATIVE, IS NOT ZERO.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-POS               PIC S9(5)V99 VALUE 123.45.
       01  WS-NEG               PIC S9(5)V99 VALUE -67.89.
       01  WS-ZERO              PIC S9(5)V99 VALUE 0.
       01  WS-PACK-POS          PIC S9(7)V99 COMP-3 VALUE 500.00.
       01  WS-PACK-NEG          PIC S9(7)V99 COMP-3 VALUE -250.00.
       01  WS-COMP-ZERO         PIC S9(9) COMP VALUE 0.
       01  WS-RESULT            PIC X(30).
       01  WS-COUNT             PIC 999 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * IS POSITIVE
           IF WS-POS IS POSITIVE
               MOVE "POS IS POSITIVE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IS NEGATIVE
           IF WS-NEG IS NEGATIVE
               MOVE "NEG IS NEGATIVE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IS ZERO
           IF WS-ZERO IS ZERO
               MOVE "ZERO IS ZERO" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IS NOT POSITIVE
           IF WS-NEG IS NOT POSITIVE
               MOVE "NEG NOT POSITIVE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IS NOT NEGATIVE
           IF WS-POS IS NOT NEGATIVE
               MOVE "POS NOT NEGATIVE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IS NOT ZERO
           IF WS-POS IS NOT ZERO
               MOVE "POS NOT ZERO" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Packed positive
           IF WS-PACK-POS IS POSITIVE
               MOVE "PACK POSITIVE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Packed negative
           IF WS-PACK-NEG IS NEGATIVE
               MOVE "PACK NEGATIVE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Binary zero
           IF WS-COMP-ZERO IS ZERO
               MOVE "COMP ZERO" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Sign condition in compound
           IF WS-POS IS POSITIVE AND WS-NEG IS NEGATIVE
               MOVE "BOTH SIGN OK" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Count positives, negatives, zeros
           MOVE 0 TO WS-COUNT.
           IF WS-POS IS POSITIVE ADD 1 TO WS-COUNT END-IF.
           IF WS-NEG IS POSITIVE ADD 1 TO WS-COUNT END-IF.
           IF WS-ZERO IS POSITIVE ADD 1 TO WS-COUNT END-IF.
           DISPLAY WS-COUNT.
      * Zero becomes positive after MOVE
           MOVE 1 TO WS-ZERO.
           IF WS-ZERO IS POSITIVE
               MOVE "NOW POSITIVE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
           STOP RUN.
