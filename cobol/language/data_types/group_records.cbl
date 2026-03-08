       IDENTIFICATION DIVISION.
       PROGRAM-ID. GROUP-RECORDS-TEST.
      *---------------------------------------------------------------
      * Stress test: Multi-level group records, FILLER, nested groups.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Simple group
       01  WS-EMPLOYEE.
           05  WS-EMP-ID      PIC 9(5).
           05  WS-EMP-NAME    PIC X(30).
           05  WS-EMP-SALARY  PIC 9(7)V99.
      * Nested group
       01  WS-ADDRESS.
           05  WS-STREET.
               10  WS-STR-NUM PIC 9(5).
               10  WS-STR-NAME PIC X(30).
           05  WS-CITY         PIC X(20).
           05  WS-STATE        PIC X(2).
           05  WS-ZIP.
               10  WS-ZIP-MAIN PIC 9(5).
               10  WS-ZIP-EXT  PIC 9(4).
      * Deep nesting (5 levels)
       01  WS-DEEP.
           05  WS-LVL2.
               10  WS-LVL3.
                   15  WS-LVL4.
                       20  WS-LVL5 PIC X(10).
      * FILLER fields
       01  WS-HEADER.
           05  FILLER          PIC X(5) VALUE "HDR: ".
           05  WS-HDR-DATE     PIC 9(8).
           05  FILLER          PIC X(3) VALUE " - ".
           05  WS-HDR-NAME     PIC X(20).
           05  FILLER          PIC X(40) VALUE SPACES.
      * Mixed types in group
       01  WS-MIXED-REC.
           05  WS-MX-ALPHA     PIC A(10).
           05  WS-MX-ALNUM     PIC X(10).
           05  WS-MX-INT       PIC 9(5).
           05  WS-MX-SDEC      PIC S9(5)V99.
           05  WS-MX-PACK      PIC S9(7)V99 COMP-3.
           05  WS-MX-BIN       PIC S9(9) COMP.
      * Multiple groups at same level
       01  WS-REC-A.
           05  WS-RA-FIELD1    PIC X(10).
           05  WS-RA-FIELD2    PIC 9(5).
       01  WS-REC-B.
           05  WS-RB-FIELD1    PIC X(10).
           05  WS-RB-FIELD2    PIC 9(5).
      * Group MOVE target
       01  WS-DISPLAY-REC      PIC X(80).
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 10001 TO WS-EMP-ID.
           MOVE "JOHN DOE" TO WS-EMP-NAME.
           MOVE 75000.00 TO WS-EMP-SALARY.
           MOVE 123 TO WS-STR-NUM.
           MOVE "MAIN STREET" TO WS-STR-NAME.
           MOVE "SPRINGFIELD" TO WS-CITY.
           MOVE "IL" TO WS-STATE.
           MOVE 62701 TO WS-ZIP-MAIN.
           MOVE 1234 TO WS-ZIP-EXT.
           MOVE "DEEP VALUE" TO WS-LVL5.
           MOVE 20250101 TO WS-HDR-DATE.
           MOVE "TEST REPORT" TO WS-HDR-NAME.
           MOVE WS-EMPLOYEE TO WS-DISPLAY-REC.
           DISPLAY WS-EMP-ID.
           DISPLAY WS-EMP-NAME.
           DISPLAY WS-EMP-SALARY.
           DISPLAY WS-CITY.
           DISPLAY WS-STATE.
           DISPLAY WS-ZIP-MAIN.
           DISPLAY WS-LVL5.
           DISPLAY WS-HEADER.
           DISPLAY WS-DISPLAY-REC.
           STOP RUN.
