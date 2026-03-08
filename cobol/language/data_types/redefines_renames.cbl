       IDENTIFICATION DIVISION.
       PROGRAM-ID. REDEFINES-RENAMES-TEST.
      *---------------------------------------------------------------
      * Stress test: REDEFINES and RENAMES (level 66).
      * Covers: Simple redefines, group redefines, numeric/alpha
      * overlay, RENAMES single, RENAMES THRU.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Simple REDEFINES
       01  WS-DATE-NUM         PIC 9(8) VALUE 20250115.
       01  WS-DATE-X REDEFINES WS-DATE-NUM PIC X(8).
      * Group REDEFINES
       01  WS-DATE-STRUCT REDEFINES WS-DATE-NUM.
           05  WS-DS-YEAR      PIC 9(4).
           05  WS-DS-MONTH     PIC 99.
           05  WS-DS-DAY       PIC 99.
      * Numeric redefines as alphanumeric
       01  WS-AMOUNT           PIC 9(7)V99 VALUE 12345.67.
       01  WS-AMOUNT-X REDEFINES WS-AMOUNT PIC X(9).
      * Multiple REDEFINES on same field
       01  WS-CODE             PIC X(6) VALUE "AB1234".
       01  WS-CODE-PARTS REDEFINES WS-CODE.
           05  WS-CP-PREFIX    PIC X(2).
           05  WS-CP-NUMBER    PIC 9(4).
      * Nested group with REDEFINES
       01  WS-RECORD.
           05  WS-REC-TYPE     PIC X(2).
           05  WS-REC-DATA     PIC X(20).
           05  WS-REC-DATA-NUM REDEFINES WS-REC-DATA.
               10  WS-RDN-AMT  PIC 9(7)V99.
               10  WS-RDN-QTY  PIC 9(5).
               10  FILLER       PIC X(6).
      * RENAMES (level 66) - single field
       01  WS-PERSON.
           05  WS-FIRST-NAME   PIC X(15).
           05  WS-MIDDLE-INIT  PIC X.
           05  WS-LAST-NAME    PIC X(20).
           05  WS-AGE          PIC 999.
       66  WS-FULL-NAME RENAMES WS-FIRST-NAME THRU WS-LAST-NAME.
       66  WS-NAME-INIT RENAMES WS-FIRST-NAME THRU WS-MIDDLE-INIT.
      * Result
       01  WS-DISPLAY          PIC X(40).
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Test simple REDEFINES
           DISPLAY WS-DATE-NUM.
           DISPLAY WS-DATE-X.
      * Test group REDEFINES
           DISPLAY WS-DS-YEAR.
           DISPLAY WS-DS-MONTH.
           DISPLAY WS-DS-DAY.
      * Test numeric redefines
           DISPLAY WS-AMOUNT.
           DISPLAY WS-AMOUNT-X.
      * Test code parts
           DISPLAY WS-CP-PREFIX.
           DISPLAY WS-CP-NUMBER.
      * Test nested redefines
           MOVE "HD" TO WS-REC-TYPE.
           MOVE "DATA CONTENT HERE" TO WS-REC-DATA.
           DISPLAY WS-REC-TYPE.
           DISPLAY WS-REC-DATA.
      * Test RENAMES
           MOVE "JOHN" TO WS-FIRST-NAME.
           MOVE "Q" TO WS-MIDDLE-INIT.
           MOVE "PUBLIC" TO WS-LAST-NAME.
           MOVE 42 TO WS-AGE.
           MOVE WS-FULL-NAME TO WS-DISPLAY.
           DISPLAY WS-DISPLAY.
           DISPLAY WS-AGE.
           STOP RUN.
