       IDENTIFICATION DIVISION.
       PROGRAM-ID. FIGURATIVE-CONSTANTS-TEST.
      *---------------------------------------------------------------
      * Stress test: Figurative constants.
      * Covers: SPACES, ZEROS, HIGH-VALUES, LOW-VALUES, QUOTES,
      * ALL literal, comparison with figuratives,
      * MOVE figuratives to various types.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-ALPHA             PIC X(10).
       01  WS-NUM               PIC 9(5).
       01  WS-SDEC              PIC S9(5)V99.
       01  WS-PACK              PIC S9(7)V99 COMP-3.
       01  WS-COMP              PIC S9(9) COMP.
       01  WS-GROUP.
           05  WS-GRP-A         PIC X(5).
           05  WS-GRP-B         PIC 9(3).
           05  WS-GRP-C         PIC X(5).
       01  WS-ALL-STARS         PIC X(10).
       01  WS-ALL-HASH          PIC X(10).
       01  WS-ALL-AB            PIC X(10).
       01  WS-RESULT            PIC X(20).
       PROCEDURE DIVISION.
       MAIN-PARA.
      * SPACES
           MOVE SPACES TO WS-ALPHA.
           IF WS-ALPHA = SPACES
               MOVE "IS SPACES" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * SPACE (singular)
           MOVE SPACE TO WS-ALPHA.
           DISPLAY WS-ALPHA.
      * ZEROS to numeric
           MOVE ZEROS TO WS-NUM.
           IF WS-NUM = ZEROS
               MOVE "IS ZEROS" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * ZERO (singular)
           MOVE ZERO TO WS-SDEC.
           DISPLAY WS-SDEC.
      * ZEROES (synonym)
           MOVE ZEROES TO WS-PACK.
           DISPLAY WS-PACK.
      * HIGH-VALUES
           MOVE HIGH-VALUES TO WS-ALPHA.
           IF WS-ALPHA = HIGH-VALUES
               MOVE "IS HIGH-VALUES" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * HIGH-VALUE (singular)
           MOVE HIGH-VALUE TO WS-ALPHA.
           DISPLAY "HIGH-VALUE SET".
      * LOW-VALUES
           MOVE LOW-VALUES TO WS-ALPHA.
           IF WS-ALPHA = LOW-VALUES
               MOVE "IS LOW-VALUES" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * LOW-VALUE (singular)
           MOVE LOW-VALUE TO WS-ALPHA.
           DISPLAY "LOW-VALUE SET".
      * QUOTES
           MOVE QUOTES TO WS-ALPHA.
           DISPLAY WS-ALPHA.
      * QUOTE (singular)
           MOVE QUOTE TO WS-ALPHA.
           DISPLAY WS-ALPHA.
      * ALL literal
           MOVE ALL "*" TO WS-ALL-STARS.
           DISPLAY WS-ALL-STARS.
           MOVE ALL "#" TO WS-ALL-HASH.
           DISPLAY WS-ALL-HASH.
           MOVE ALL "AB" TO WS-ALL-AB.
           DISPLAY WS-ALL-AB.
      * Figurative to group
           MOVE SPACES TO WS-GROUP.
           IF WS-GRP-A = SPACES
               MOVE "GRP SPACES" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * ZEROS to group
           MOVE ZEROS TO WS-GROUP.
           DISPLAY WS-GRP-A.
           DISPLAY WS-GRP-B.
      * Compare numeric with ZERO
           MOVE 0 TO WS-COMP.
           IF WS-COMP = ZERO
               MOVE "COMP IS ZERO" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
           STOP RUN.
