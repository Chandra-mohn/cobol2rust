       IDENTIFICATION DIVISION.
       PROGRAM-ID. NESTED-CONTROL-TEST.
      *---------------------------------------------------------------
      * Stress test: Deeply nested control flow.
      * Covers: Nested IF in EVALUATE, PERFORM in IF, EVALUATE in
      * PERFORM, multiple nested levels, complex control paths.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-I                 PIC 99.
       01  WS-J                 PIC 99.
       01  WS-TYPE              PIC X VALUE "A".
       01  WS-SCORE             PIC 999 VALUE 85.
       01  WS-COUNT             PIC 999 VALUE 0.
       01  WS-TOTAL             PIC 9(7) VALUE 0.
       01  WS-RESULT            PIC X(30).
       01  WS-TABLE.
           05  WS-ITEM          PIC 9(3) OCCURS 10 TIMES.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * IF inside EVALUATE
           EVALUATE WS-TYPE
               WHEN "A"
                   IF WS-SCORE > 90
                       MOVE "A-HIGH" TO WS-RESULT
                   ELSE IF WS-SCORE > 70
                       MOVE "A-MED" TO WS-RESULT
                   ELSE
                       MOVE "A-LOW" TO WS-RESULT
                   END-IF
               WHEN "B"
                   MOVE "TYPE B" TO WS-RESULT
               WHEN OTHER
                   MOVE "OTHER TYPE" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * EVALUATE inside PERFORM
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 5
               EVALUATE TRUE
                   WHEN WS-I = 1
                       MOVE 100 TO WS-ITEM(WS-I)
                   WHEN WS-I = 2
                       MOVE 200 TO WS-ITEM(WS-I)
                   WHEN WS-I > 2
                       COMPUTE WS-ITEM(WS-I) = WS-I * 100
               END-EVALUATE
           END-PERFORM.
           DISPLAY WS-ITEM(3).
           DISPLAY WS-ITEM(5).
      * Nested PERFORM with IF
           MOVE 0 TO WS-TOTAL.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 10
               IF WS-I > 5
                   PERFORM VARYING WS-J FROM 1 BY 1
                       UNTIL WS-J > WS-I
                       ADD 1 TO WS-TOTAL
                   END-PERFORM
               ELSE
                   ADD WS-I TO WS-TOTAL
               END-IF
           END-PERFORM.
           DISPLAY WS-TOTAL.
      * Complex nested control
           MOVE 0 TO WS-COUNT.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 5
               IF WS-I < 3
                   EVALUATE WS-I
                       WHEN 1
                           ADD 10 TO WS-COUNT
                       WHEN 2
                           ADD 20 TO WS-COUNT
                   END-EVALUATE
               ELSE
                   IF WS-I = 3
                       ADD 30 TO WS-COUNT
                   ELSE
                       ADD WS-I TO WS-COUNT
                   END-IF
               END-IF
           END-PERFORM.
           DISPLAY WS-COUNT.
      * Deeply nested IF (5 levels)
           MOVE 50 TO WS-I.
           IF WS-I > 10
               IF WS-I > 20
                   IF WS-I > 30
                       IF WS-I > 40
                           IF WS-I > 50
                               MOVE "VERY HIGH" TO WS-RESULT
                           ELSE
                               MOVE "HIGH" TO WS-RESULT
                           END-IF
                       ELSE
                           MOVE "MEDIUM-HIGH" TO WS-RESULT
                       END-IF
                   ELSE
                       MOVE "MEDIUM" TO WS-RESULT
                   END-IF
               ELSE
                   MOVE "LOW" TO WS-RESULT
               END-IF
           ELSE
               MOVE "VERY LOW" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
           STOP RUN.
