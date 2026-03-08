       IDENTIFICATION DIVISION.
       PROGRAM-ID. REFERENCE-MOD-TEST.
      *---------------------------------------------------------------
      * Stress test: Reference modification.
      * Covers: Substring extraction, substring assignment,
      * variable position/length, with array elements.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-DATA              PIC X(20) VALUE "ABCDEFGHIJKLMNOPQRST".
       01  WS-RESULT            PIC X(20) VALUE SPACES.
       01  WS-SHORT             PIC X(5).
       01  WS-POS               PIC 99 VALUE 0.
       01  WS-LEN               PIC 99 VALUE 0.
       01  WS-TABLE.
           05  WS-ITEM          PIC X(10) OCCURS 5 TIMES.
       01  WS-I                 PIC 99.
       01  WS-NUM               PIC 9(8) VALUE 20250115.
       01  WS-YEAR              PIC X(4).
       01  WS-MONTH-VAL         PIC X(2).
       01  WS-DAY-VAL           PIC X(2).
       01  WS-DISPLAY           PIC X(30).
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Extract first 5 characters
           MOVE WS-DATA(1:5) TO WS-SHORT.
           DISPLAY WS-SHORT.
      * Extract from middle
           MOVE WS-DATA(6:5) TO WS-SHORT.
           DISPLAY WS-SHORT.
      * Extract last 5 characters
           MOVE WS-DATA(16:5) TO WS-SHORT.
           DISPLAY WS-SHORT.
      * Extract single character
           MOVE WS-DATA(10:1) TO WS-SHORT.
           DISPLAY WS-SHORT.
      * Assign to substring
           MOVE "ABCDEFGHIJKLMNOPQRST" TO WS-DATA.
           MOVE "XXXXX" TO WS-DATA(3:5).
           DISPLAY WS-DATA.
      * Variable position
           MOVE 5 TO WS-POS.
           MOVE 3 TO WS-LEN.
           MOVE WS-DATA(WS-POS:WS-LEN) TO WS-SHORT.
           DISPLAY WS-SHORT.
      * Reference mod on numeric display
           MOVE WS-NUM TO WS-DISPLAY.
           MOVE WS-DISPLAY(1:4) TO WS-YEAR.
           MOVE WS-DISPLAY(5:2) TO WS-MONTH-VAL.
           MOVE WS-DISPLAY(7:2) TO WS-DAY-VAL.
           DISPLAY WS-YEAR.
           DISPLAY WS-MONTH-VAL.
           DISPLAY WS-DAY-VAL.
      * Reference mod with array elements
           MOVE "FIRST ITEM" TO WS-ITEM(1).
           MOVE "SECOND ONE" TO WS-ITEM(2).
           MOVE WS-ITEM(1)(1:5) TO WS-SHORT.
           DISPLAY WS-SHORT.
           MOVE WS-ITEM(2)(8:3) TO WS-SHORT.
           DISPLAY WS-SHORT.
      * Assign into array element substring
           MOVE "XX" TO WS-ITEM(1)(3:2).
           DISPLAY WS-ITEM(1).
      * Length-only reference mod (position 1 implied)
           MOVE WS-DATA(1:10) TO WS-RESULT.
           DISPLAY WS-RESULT.
           STOP RUN.
