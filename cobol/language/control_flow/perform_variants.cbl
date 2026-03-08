       IDENTIFICATION DIVISION.
       PROGRAM-ID. PERFORM-VARIANTS-TEST.
      *---------------------------------------------------------------
      * Stress test: PERFORM statement variants.
      * Covers: Inline PERFORM, PERFORM paragraph, PERFORM THRU,
      * PERFORM TIMES, PERFORM UNTIL, PERFORM VARYING,
      * nested PERFORM, PERFORM with TEST BEFORE/AFTER.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-I                 PIC 99 VALUE 0.
       01  WS-J                 PIC 99 VALUE 0.
       01  WS-K                 PIC 99 VALUE 0.
       01  WS-COUNT             PIC 999 VALUE 0.
       01  WS-SUM               PIC 9(7) VALUE 0.
       01  WS-RESULT            PIC X(30).
       01  WS-TABLE.
           05  WS-ITEM          PIC 9(3) OCCURS 10 TIMES.
       01  WS-MATRIX.
           05  WS-ROW           OCCURS 3 TIMES.
               10  WS-COL       PIC 9(3) OCCURS 3 TIMES.
       01  WS-FLAG              PIC 9 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Inline PERFORM
           PERFORM
               ADD 1 TO WS-COUNT
               ADD 1 TO WS-COUNT
               ADD 1 TO WS-COUNT
           END-PERFORM.
           DISPLAY WS-COUNT.
      * PERFORM paragraph
           PERFORM PROCESS-PARA.
           DISPLAY WS-RESULT.
      * PERFORM THRU
           PERFORM INIT-PARA THRU INIT-PARA-EXIT.
           DISPLAY WS-COUNT.
      * PERFORM TIMES
           MOVE 0 TO WS-COUNT.
           PERFORM 5 TIMES
               ADD 1 TO WS-COUNT
           END-PERFORM.
           DISPLAY WS-COUNT.
      * PERFORM UNTIL
           MOVE 0 TO WS-SUM.
           MOVE 1 TO WS-I.
           PERFORM UNTIL WS-I > 10
               ADD WS-I TO WS-SUM
               ADD 1 TO WS-I
           END-PERFORM.
           DISPLAY WS-SUM.
      * PERFORM VARYING
           MOVE 0 TO WS-SUM.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 10
               ADD WS-I TO WS-SUM
           END-PERFORM.
           DISPLAY WS-SUM.
      * PERFORM VARYING with AFTER (nested loops)
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 3
               AFTER WS-J FROM 1 BY 1
               UNTIL WS-J > 3
               COMPUTE WS-COL(WS-I, WS-J) =
                   WS-I * 10 + WS-J
           END-PERFORM.
           DISPLAY WS-COL(2, 3).
      * PERFORM with TEST AFTER
           MOVE 0 TO WS-COUNT.
           MOVE 0 TO WS-I.
           PERFORM WITH TEST AFTER
               UNTIL WS-I >= 5
               ADD 1 TO WS-I
               ADD 1 TO WS-COUNT
           END-PERFORM.
           DISPLAY WS-COUNT.
      * Nested PERFORM paragraphs
           PERFORM OUTER-PARA.
           DISPLAY WS-COUNT.
      * Fill table with PERFORM VARYING
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 10
               MOVE WS-I TO WS-ITEM(WS-I)
           END-PERFORM.
           DISPLAY WS-ITEM(5).
           DISPLAY WS-ITEM(10).
           STOP RUN.

       PROCESS-PARA.
           MOVE "PROCESSED" TO WS-RESULT.

       INIT-PARA.
           MOVE 0 TO WS-COUNT.
           ADD 100 TO WS-COUNT.
       INIT-PARA-EXIT.
           EXIT.

       OUTER-PARA.
           MOVE 0 TO WS-COUNT.
           PERFORM INNER-PARA 3 TIMES.

       INNER-PARA.
           ADD 10 TO WS-COUNT.
