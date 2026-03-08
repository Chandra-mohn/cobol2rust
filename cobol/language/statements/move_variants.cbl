       IDENTIFICATION DIVISION.
       PROGRAM-ID. MOVE-VARIANTS-TEST.
      *---------------------------------------------------------------
      * Stress test: MOVE statement variants.
      * Covers: Simple MOVE, MOVE CORR, figurative constants,
      * numeric-to-alpha, alpha-to-numeric, group MOVE,
      * reference modification, MOVE to array elements.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Simple fields
       01  WS-ALPHA-SRC         PIC X(20) VALUE "HELLO WORLD".
       01  WS-ALPHA-DST         PIC X(20).
       01  WS-ALPHA-SHORT       PIC X(5).
       01  WS-ALPHA-LONG        PIC X(40).
       01  WS-NUM-SRC           PIC 9(5) VALUE 12345.
       01  WS-NUM-DST           PIC 9(5).
       01  WS-NUM-SHORT         PIC 9(3).
       01  WS-NUM-LONG          PIC 9(8).
       01  WS-SDEC-SRC          PIC S9(5)V99 VALUE -123.45.
       01  WS-SDEC-DST          PIC S9(5)V99.
       01  WS-PACK-DST          PIC S9(7)V99 COMP-3.
       01  WS-COMP-DST          PIC S9(9) COMP.
      * Figurative constant targets
       01  WS-FIG-SPACES        PIC X(10).
       01  WS-FIG-ZEROS         PIC 9(5).
       01  WS-FIG-HIGHS         PIC X(5).
       01  WS-FIG-LOWS          PIC X(5).
       01  WS-FIG-QUOTES        PIC X(5).
       01  WS-FIG-ALL           PIC X(10).
      * MOVE CORRESPONDING source
       01  WS-SRC-REC.
           05  WS-NAME          PIC X(20) VALUE "ALICE".
           05  WS-AGE           PIC 999 VALUE 30.
           05  WS-SALARY        PIC 9(7)V99 VALUE 50000.00.
           05  WS-DEPT          PIC X(10) VALUE "ENGG".
      * MOVE CORRESPONDING target (partial overlap)
       01  WS-DST-REC.
           05  WS-NAME          PIC X(20).
           05  WS-AGE           PIC 999.
           05  WS-TITLE         PIC X(15).
           05  WS-SALARY        PIC 9(7)V99.
      * Group MOVE
       01  WS-GROUP-SRC.
           05  WS-GS-A          PIC X(5) VALUE "ABCDE".
           05  WS-GS-B          PIC 9(3) VALUE 123.
           05  WS-GS-C          PIC X(2) VALUE "XY".
       01  WS-GROUP-DST.
           05  WS-GD-A          PIC X(5).
           05  WS-GD-B          PIC 9(3).
           05  WS-GD-C          PIC X(2).
      * Array targets
       01  WS-TABLE.
           05  WS-ITEM          PIC X(10) OCCURS 5 TIMES.
       01  WS-IDX              PIC 99.
      * Cross-type MOVE targets
       01  WS-NUM-TO-ALPHA      PIC X(10).
       01  WS-ALPHA-TO-NUM      PIC 9(5).
       01  WS-EDITED-DST        PIC ZZ,ZZ9.99.
      * Reference modification target
       01  WS-REFMOD            PIC X(20) VALUE SPACES.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple alphanumeric MOVE
           MOVE WS-ALPHA-SRC TO WS-ALPHA-DST.
           DISPLAY WS-ALPHA-DST.
      * Truncation (long to short)
           MOVE WS-ALPHA-SRC TO WS-ALPHA-SHORT.
           DISPLAY WS-ALPHA-SHORT.
      * Padding (short to long)
           MOVE WS-ALPHA-SRC TO WS-ALPHA-LONG.
           DISPLAY WS-ALPHA-LONG.
      * Simple numeric MOVE
           MOVE WS-NUM-SRC TO WS-NUM-DST.
           DISPLAY WS-NUM-DST.
      * Numeric truncation
           MOVE WS-NUM-SRC TO WS-NUM-SHORT.
           DISPLAY WS-NUM-SHORT.
      * Numeric padding
           MOVE WS-NUM-SRC TO WS-NUM-LONG.
           DISPLAY WS-NUM-LONG.
      * Signed decimal MOVE
           MOVE WS-SDEC-SRC TO WS-SDEC-DST.
           DISPLAY WS-SDEC-DST.
      * Numeric to packed
           MOVE WS-SDEC-SRC TO WS-PACK-DST.
           DISPLAY WS-PACK-DST.
      * Numeric to binary
           MOVE WS-NUM-SRC TO WS-COMP-DST.
           DISPLAY WS-COMP-DST.
      * Figurative constants
           MOVE SPACES TO WS-FIG-SPACES.
           DISPLAY WS-FIG-SPACES.
           MOVE ZEROS TO WS-FIG-ZEROS.
           DISPLAY WS-FIG-ZEROS.
           MOVE HIGH-VALUES TO WS-FIG-HIGHS.
           MOVE LOW-VALUES TO WS-FIG-LOWS.
           MOVE QUOTES TO WS-FIG-QUOTES.
           MOVE ALL "*" TO WS-FIG-ALL.
           DISPLAY WS-FIG-ALL.
      * MOVE CORRESPONDING
           MOVE CORRESPONDING WS-SRC-REC TO WS-DST-REC.
           DISPLAY WS-NAME OF WS-DST-REC.
           DISPLAY WS-AGE OF WS-DST-REC.
           DISPLAY WS-SALARY OF WS-DST-REC.
      * Group MOVE (alphanumeric overlay)
           MOVE WS-GROUP-SRC TO WS-GROUP-DST.
           DISPLAY WS-GD-A.
           DISPLAY WS-GD-B.
           DISPLAY WS-GD-C.
      * MOVE to array elements
           PERFORM VARYING WS-IDX FROM 1 BY 1
               UNTIL WS-IDX > 5
               MOVE "ELEMENT" TO WS-ITEM(WS-IDX)
           END-PERFORM.
           DISPLAY WS-ITEM(3).
      * Numeric to alphanumeric
           MOVE WS-NUM-SRC TO WS-NUM-TO-ALPHA.
           DISPLAY WS-NUM-TO-ALPHA.
      * Alphanumeric to numeric
           MOVE "00042" TO WS-ALPHA-TO-NUM.
           DISPLAY WS-ALPHA-TO-NUM.
      * Numeric to edited
           MOVE 12345.67 TO WS-EDITED-DST.
           DISPLAY WS-EDITED-DST.
      * Literal MOVEs
           MOVE 0 TO WS-NUM-DST.
           MOVE "LITERAL" TO WS-ALPHA-DST.
           DISPLAY WS-NUM-DST.
           DISPLAY WS-ALPHA-DST.
      * Reference modification MOVE
           MOVE "INSERT" TO WS-REFMOD(3:6).
           DISPLAY WS-REFMOD.
           STOP RUN.
