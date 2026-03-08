       IDENTIFICATION DIVISION.
       PROGRAM-ID. IF-EVALUATE-TEST.
      *---------------------------------------------------------------
      * Stress test: IF and EVALUATE statements.
      * Covers: IF/ELSE, nested IF, EVALUATE TRUE, EVALUATE var,
      * WHEN OTHER, ALSO, THRU.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-A                 PIC 9(3) VALUE 50.
       01  WS-B                 PIC 9(3) VALUE 75.
       01  WS-C                 PIC 9(3) VALUE 100.
       01  WS-STATUS            PIC X(2) VALUE "OK".
       01  WS-GRADE             PIC X VALUE "B".
       01  WS-MONTH             PIC 99 VALUE 7.
       01  WS-REGION            PIC X(5) VALUE "NORTH".
       01  WS-RESULT            PIC X(30).
       01  WS-CODE              PIC 99 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple IF
           IF WS-A > 25
               MOVE "A > 25" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IF/ELSE
           IF WS-A > WS-B
               MOVE "A > B" TO WS-RESULT
           ELSE
               MOVE "A <= B" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Nested IF
           IF WS-A > 25
               IF WS-B > 50
                   IF WS-C > 75
                       MOVE "ALL PASS" TO WS-RESULT
                   ELSE
                       MOVE "C FAIL" TO WS-RESULT
                   END-IF
               ELSE
                   MOVE "B FAIL" TO WS-RESULT
               END-IF
           ELSE
               MOVE "A FAIL" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IF with AND
           IF WS-A > 25 AND WS-B > 50
               MOVE "BOTH PASS" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IF with OR
           IF WS-A > 100 OR WS-B > 50
               MOVE "ONE PASS" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IF with NOT
           IF NOT WS-A > 100
               MOVE "A NOT > 100" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IF with compound condition
           IF (WS-A > 25 AND WS-B > 50) OR WS-C = 100
               MOVE "COMPOUND PASS" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * IF NUMERIC/ALPHABETIC
           IF WS-STATUS IS ALPHABETIC
               MOVE "STATUS IS ALPHA" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * EVALUATE TRUE
           EVALUATE TRUE
               WHEN WS-A > 75
                   MOVE "HIGH" TO WS-RESULT
               WHEN WS-A > 50
                   MOVE "MEDIUM" TO WS-RESULT
               WHEN WS-A > 25
                   MOVE "LOW" TO WS-RESULT
               WHEN OTHER
                   MOVE "VERY LOW" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * EVALUATE variable
           EVALUATE WS-GRADE
               WHEN "A"
                   MOVE "EXCELLENT" TO WS-RESULT
               WHEN "B"
                   MOVE "GOOD" TO WS-RESULT
               WHEN "C"
                   MOVE "AVERAGE" TO WS-RESULT
               WHEN "D" THRU "F"
                   MOVE "BELOW AVG" TO WS-RESULT
               WHEN OTHER
                   MOVE "UNKNOWN GRADE" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * EVALUATE with ALSO
           EVALUATE WS-MONTH ALSO WS-REGION
               WHEN 1 THRU 3 ALSO "NORTH"
                   MOVE "Q1 NORTH" TO WS-RESULT
               WHEN 1 THRU 3 ALSO "SOUTH"
                   MOVE "Q1 SOUTH" TO WS-RESULT
               WHEN 7 THRU 9 ALSO ANY
                   MOVE "Q3 ANY REGION" TO WS-RESULT
               WHEN OTHER
                   MOVE "OTHER COMBO" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * EVALUATE TRUE/FALSE
           EVALUATE TRUE ALSO FALSE
               WHEN WS-A > 25 ALSO WS-B > 100
                   MOVE "MATCH 1" TO WS-RESULT
               WHEN OTHER
                   MOVE "NO MATCH" TO WS-RESULT
           END-EVALUATE.
           DISPLAY WS-RESULT.
      * Nested EVALUATE
           EVALUATE WS-STATUS
               WHEN "OK"
                   EVALUATE WS-GRADE
                       WHEN "A"
                           MOVE 1 TO WS-CODE
                       WHEN "B"
                           MOVE 2 TO WS-CODE
                       WHEN OTHER
                           MOVE 3 TO WS-CODE
                   END-EVALUATE
               WHEN "ER"
                   MOVE 99 TO WS-CODE
               WHEN OTHER
                   MOVE 0 TO WS-CODE
           END-EVALUATE.
           DISPLAY WS-CODE.
           STOP RUN.
