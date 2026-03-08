       IDENTIFICATION DIVISION.
       PROGRAM-ID. ARRAYS-OCCURS-TEST.
      *---------------------------------------------------------------
      * Stress test: OCCURS (fixed), OCCURS DEPENDING ON, subscripts,
      * INDEXED BY, multi-dimensional arrays.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Simple fixed array
       01  WS-SIMPLE-TABLE.
           05  WS-SIMPLE-ITEM PIC X(10) OCCURS 10 TIMES.
      * Numeric array
       01  WS-NUM-TABLE.
           05  WS-NUM-ITEM    PIC 9(5)V99 OCCURS 20 TIMES.
      * Array with INDEXED BY
       01  WS-IDX-TABLE.
           05  WS-IDX-ITEM    PIC X(15) OCCURS 5 TIMES
                               INDEXED BY WS-IX1.
      * Nested arrays (2D)
       01  WS-MATRIX.
           05  WS-ROW          OCCURS 5 TIMES.
               10  WS-COL      PIC 9(3) OCCURS 10 TIMES.
      * Array of groups
       01  WS-EMP-TABLE.
           05  WS-EMP-ENTRY    OCCURS 10 TIMES.
               10  WS-ET-ID    PIC 9(5).
               10  WS-ET-NAME  PIC X(20).
               10  WS-ET-SAL   PIC 9(7)V99.
      * OCCURS DEPENDING ON
       01  WS-VAR-COUNT        PIC 99 VALUE 5.
       01  WS-VAR-TABLE.
           05  WS-VAR-ITEM    PIC X(10)
                               OCCURS 1 TO 20 TIMES
                               DEPENDING ON WS-VAR-COUNT.
      * Loop counter
       01  WS-I               PIC 99.
       01  WS-J               PIC 99.
       01  WS-SUM             PIC 9(7)V99.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Fill simple table
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 10
               MOVE "ITEM" TO WS-SIMPLE-ITEM(WS-I)
           END-PERFORM.
      * Fill numeric table
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 20
               MOVE WS-I TO WS-NUM-ITEM(WS-I)
           END-PERFORM.
      * Fill matrix (2D)
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 5
               PERFORM VARYING WS-J FROM 1 BY 1
                   UNTIL WS-J > 10
                   COMPUTE WS-COL(WS-I, WS-J) =
                       WS-I * 10 + WS-J
               END-PERFORM
           END-PERFORM.
      * Fill employee table
           MOVE 10001 TO WS-ET-ID(1).
           MOVE "ALICE" TO WS-ET-NAME(1).
           MOVE 50000.00 TO WS-ET-SAL(1).
           MOVE 10002 TO WS-ET-ID(2).
           MOVE "BOB" TO WS-ET-NAME(2).
           MOVE 60000.00 TO WS-ET-SAL(2).
      * Use OCCURS DEPENDING ON
           MOVE 3 TO WS-VAR-COUNT.
           MOVE "FIRST" TO WS-VAR-ITEM(1).
           MOVE "SECOND" TO WS-VAR-ITEM(2).
           MOVE "THIRD" TO WS-VAR-ITEM(3).
      * Sum numeric table
           MOVE 0 TO WS-SUM.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 10
               ADD WS-NUM-ITEM(WS-I) TO WS-SUM
           END-PERFORM.
      * Display results
           DISPLAY WS-SIMPLE-ITEM(1).
           DISPLAY WS-SIMPLE-ITEM(10).
           DISPLAY WS-NUM-ITEM(5).
           DISPLAY WS-COL(3, 7).
           DISPLAY WS-ET-NAME(1).
           DISPLAY WS-ET-SAL(2).
           DISPLAY WS-VAR-ITEM(1).
           DISPLAY WS-SUM.
           STOP RUN.
