       IDENTIFICATION DIVISION.
       PROGRAM-ID. BINARY-TYPES-TEST.
      *---------------------------------------------------------------
      * Stress test: All binary/packed/float USAGE types.
      * Covers: COMP, COMP-3, COMP-5, COMP-1, COMP-2, INDEX.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * COMP / BINARY (PIC-limited range)
       01  WS-COMP-S4         PIC S9(4) COMP.
       01  WS-COMP-S9         PIC S9(9) COMP.
       01  WS-COMP-S18        PIC S9(18) COMP.
       01  WS-COMP-U4         PIC 9(4) COMP.
       01  WS-COMP-U9         PIC 9(9) COMP.
       01  WS-COMP-DEC        PIC S9(5)V99 COMP.
       01  WS-BINARY-F        PIC S9(9) BINARY.
      * COMP-3 / PACKED-DECIMAL
       01  WS-PACK-S4         PIC S9(4) COMP-3.
       01  WS-PACK-S9         PIC S9(9) COMP-3.
       01  WS-PACK-S18        PIC S9(18) COMP-3.
       01  WS-PACK-U4         PIC 9(4) COMP-3.
       01  WS-PACK-DEC        PIC S9(5)V99 COMP-3.
       01  WS-PACKED-F        PIC S9(7)V99 PACKED-DECIMAL.
      * COMP-5 (full binary range)
       01  WS-COMP5-S4        PIC S9(4) COMP-5.
       01  WS-COMP5-S9        PIC S9(9) COMP-5.
       01  WS-COMP5-S18       PIC S9(18) COMP-5.
      * COMP-1 (single precision float)
       01  WS-FLOAT-1A        COMP-1.
       01  WS-FLOAT-1B        COMP-1.
      * COMP-2 (double precision float)
       01  WS-FLOAT-2A        COMP-2.
       01  WS-FLOAT-2B        COMP-2.
      * INDEX
       01  WS-TABLE.
           05  WS-ITEM        PIC X(10) OCCURS 5 TIMES
                               INDEXED BY WS-IDX.
      * Mixed in a group
       01  WS-MIXED-GROUP.
           05  WS-MIX-DISP    PIC 9(5).
           05  WS-MIX-COMP    PIC S9(9) COMP.
           05  WS-MIX-PACK    PIC S9(7)V99 COMP-3.
           05  WS-MIX-FLOAT   COMP-2.
      * Initialized values
       01  WS-INIT-COMP       PIC S9(5) COMP VALUE 12345.
       01  WS-INIT-PACK       PIC S9(5)V99 COMP-3 VALUE 123.45.
       01  WS-INIT-FLOAT1     COMP-1 VALUE 3.14.
       01  WS-INIT-FLOAT2     COMP-2 VALUE 2.71828.
      * Results
       01  WS-RESULT          PIC S9(9)V99.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 1000 TO WS-COMP-S4.
           MOVE 999999999 TO WS-COMP-S9.
           MOVE 5000 TO WS-PACK-S4.
           MOVE 123.45 TO WS-PACK-DEC.
           MOVE 32767 TO WS-COMP5-S4.
           MOVE 3.14 TO WS-FLOAT-1A.
           MOVE 2.71828 TO WS-FLOAT-2A.
           ADD WS-COMP-S4 TO WS-COMP-S9.
           MULTIPLY WS-PACK-DEC BY WS-PACK-S4.
           COMPUTE WS-RESULT =
               WS-INIT-COMP + WS-INIT-PACK * 2.
           DISPLAY WS-COMP-S4.
           DISPLAY WS-COMP-S9.
           DISPLAY WS-PACK-DEC.
           DISPLAY WS-COMP5-S4.
           DISPLAY WS-FLOAT-1A.
           DISPLAY WS-FLOAT-2A.
           DISPLAY WS-INIT-COMP.
           DISPLAY WS-INIT-PACK.
           DISPLAY WS-RESULT.
           STOP RUN.
