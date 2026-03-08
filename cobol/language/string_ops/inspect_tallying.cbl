       IDENTIFICATION DIVISION.
       PROGRAM-ID. INSPECT-TALLYING-TEST.
      *---------------------------------------------------------------
      * Stress test: INSPECT TALLYING and REPLACING.
      * Covers: TALLYING ALL, LEADING, CHARACTERS, BEFORE/AFTER,
      * REPLACING ALL, LEADING, FIRST.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-DATA              PIC X(30).
       01  WS-TALLY             PIC 999 VALUE 0.
       01  WS-TALLY2            PIC 999 VALUE 0.
       01  WS-WORK              PIC X(30).
       PROCEDURE DIVISION.
       MAIN-PARA.
      * TALLYING ALL
           MOVE "HELLO WORLD HELLO" TO WS-DATA.
           MOVE 0 TO WS-TALLY.
           INSPECT WS-DATA TALLYING
               WS-TALLY FOR ALL "L".
           DISPLAY WS-TALLY.
      * TALLYING ALL substring
           MOVE 0 TO WS-TALLY.
           INSPECT WS-DATA TALLYING
               WS-TALLY FOR ALL "HELLO".
           DISPLAY WS-TALLY.
      * TALLYING LEADING
           MOVE "000123000" TO WS-DATA.
           MOVE 0 TO WS-TALLY.
           INSPECT WS-DATA TALLYING
               WS-TALLY FOR LEADING "0".
           DISPLAY WS-TALLY.
      * TALLYING CHARACTERS
           MOVE "ABCDEF" TO WS-DATA.
           MOVE 0 TO WS-TALLY.
           INSPECT WS-DATA TALLYING
               WS-TALLY FOR CHARACTERS.
           DISPLAY WS-TALLY.
      * TALLYING BEFORE INITIAL
           MOVE "ABC.DEF.GHI" TO WS-DATA.
           MOVE 0 TO WS-TALLY.
           INSPECT WS-DATA TALLYING
               WS-TALLY FOR CHARACTERS BEFORE INITIAL ".".
           DISPLAY WS-TALLY.
      * TALLYING AFTER INITIAL
           MOVE 0 TO WS-TALLY.
           INSPECT WS-DATA TALLYING
               WS-TALLY FOR ALL "." AFTER INITIAL ".".
           DISPLAY WS-TALLY.
      * Multiple TALLYING
           MOVE "AABBBCCCC" TO WS-DATA.
           MOVE 0 TO WS-TALLY.
           MOVE 0 TO WS-TALLY2.
           INSPECT WS-DATA TALLYING
               WS-TALLY FOR ALL "A"
               WS-TALLY2 FOR ALL "C".
           DISPLAY WS-TALLY.
           DISPLAY WS-TALLY2.
      * REPLACING ALL
           MOVE "HELLO WORLD" TO WS-WORK.
           INSPECT WS-WORK REPLACING
               ALL "L" BY "R".
           DISPLAY WS-WORK.
      * REPLACING LEADING
           MOVE "000123" TO WS-WORK.
           INSPECT WS-WORK REPLACING
               LEADING "0" BY " ".
           DISPLAY WS-WORK.
      * REPLACING FIRST
           MOVE "ABCABC" TO WS-WORK.
           INSPECT WS-WORK REPLACING
               FIRST "A" BY "X".
           DISPLAY WS-WORK.
      * REPLACING CHARACTERS BY
           MOVE "ABCDEF" TO WS-WORK.
           INSPECT WS-WORK REPLACING
               CHARACTERS BY "*" BEFORE INITIAL "D".
           DISPLAY WS-WORK.
      * CONVERTING
           MOVE "HELLO WORLD" TO WS-WORK.
           INSPECT WS-WORK CONVERTING
               "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
               TO "abcdefghijklmnopqrstuvwxyz".
           DISPLAY WS-WORK.
           STOP RUN.
