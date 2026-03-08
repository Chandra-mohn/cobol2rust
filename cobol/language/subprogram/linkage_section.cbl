       IDENTIFICATION DIVISION.
       PROGRAM-ID. LINKAGE-SECTION-TEST.
      *---------------------------------------------------------------
      * Stress test: Subprogram with LINKAGE SECTION.
      * Covers: LINKAGE SECTION fields, PROCEDURE DIVISION USING,
      * multiple parameter types, group parameters.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-LOCAL             PIC X(20) VALUE "LOCAL DATA".
       01  WS-COUNTER           PIC 999 VALUE 0.
       LINKAGE SECTION.
       01  LS-ALPHA-PARAM       PIC X(20).
       01  LS-NUM-PARAM         PIC 9(5).
       01  LS-DEC-PARAM         PIC S9(7)V99 COMP-3.
       01  LS-GROUP-PARAM.
           05  LS-GP-NAME       PIC X(15).
           05  LS-GP-CODE       PIC X(3).
           05  LS-GP-AMT        PIC 9(7)V99.
       PROCEDURE DIVISION USING
           LS-ALPHA-PARAM
           LS-NUM-PARAM
           LS-DEC-PARAM
           LS-GROUP-PARAM.
       MAIN-PARA.
      * Process parameters
           DISPLAY "RECEIVED: " LS-ALPHA-PARAM.
           DISPLAY "NUM: " LS-NUM-PARAM.
           DISPLAY "DEC: " LS-DEC-PARAM.
      * Modify by reference
           MOVE "MODIFIED" TO LS-ALPHA-PARAM.
           ADD 100 TO LS-NUM-PARAM.
           MULTIPLY 2 BY LS-DEC-PARAM.
      * Work with group parameter
           DISPLAY "NAME: " LS-GP-NAME.
           DISPLAY "CODE: " LS-GP-CODE.
           MOVE "PROCESSED" TO LS-GP-NAME.
           MOVE "OK" TO LS-GP-CODE.
           ADD 500.00 TO LS-GP-AMT.
      * Use local storage
           ADD 1 TO WS-COUNTER.
           DISPLAY "CALL COUNT: " WS-COUNTER.
           STOP RUN.
