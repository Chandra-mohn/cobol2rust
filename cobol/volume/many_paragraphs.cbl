       IDENTIFICATION DIVISION.
       PROGRAM-ID. MANY-PARAGRAPHS-TEST.
      *---------------------------------------------------------------
      * Volume stress test: 100+ paragraphs.
      * Tests paragraph PERFORM, PERFORM THRU, GO TO, EXIT,
      * paragraph fall-through, and high paragraph counts to
      * verify transpiler handles large procedure divisions.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-COUNT              PIC 999 VALUE 0.
       01  WS-TOTAL              PIC 9(7) VALUE 0.
       01  WS-RESULT             PIC X(30).
       01  WS-I                  PIC 99 VALUE 0.
       01  WS-FLAG               PIC 9 VALUE 0.
       01  WS-ACCUM              PIC 9(5) VALUE 0.
       01  WS-TEMP               PIC 9(5) VALUE 0.
       01  WS-FACTOR             PIC 9(3) VALUE 1.
       01  WS-STATUS             PIC X(10) VALUE SPACES.
       01  WS-CATEGORY           PIC X(20) VALUE SPACES.
       01  WS-SWITCH             PIC 9 VALUE 0.
       01  WS-LOOP-CTR           PIC 99 VALUE 0.
       01  WS-PRODUCT            PIC 9(7) VALUE 0.
       01  WS-REMAINDER          PIC 9(3) VALUE 0.
       01  WS-QUOTIENT           PIC 9(5) VALUE 0.
       01  WS-MSG                PIC X(40) VALUE SPACES.
       01  WS-PHASE              PIC 99 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 0 TO WS-COUNT.
           MOVE 0 TO WS-TOTAL.
           MOVE SPACES TO WS-RESULT.
           PERFORM PARA-001.
           PERFORM PARA-005.
           PERFORM PARA-010 THRU PARA-015.
           PERFORM PARA-020.
           PERFORM PARA-030 THRU PARA-035.
           PERFORM PARA-040.
           PERFORM PARA-050.
           PERFORM PARA-060 THRU PARA-065.
           PERFORM PARA-070.
           PERFORM PARA-080 THRU PARA-085.
           PERFORM PARA-090.
           PERFORM PARA-095 THRU PARA-100.
           GO TO FINAL-PARA.
       PARA-001.
           ADD 1 TO WS-COUNT.
           ADD 1 TO WS-TOTAL.
           MOVE "PARA-001 EXECUTED" TO WS-RESULT.
       PARA-002.
           ADD 1 TO WS-COUNT.
           ADD 2 TO WS-TOTAL.
           MOVE "PARA-002 EXECUTED" TO WS-RESULT.
       PARA-003.
           ADD 1 TO WS-COUNT.
           ADD 3 TO WS-TOTAL.
           PERFORM PARA-001.
       PARA-004.
           ADD 1 TO WS-COUNT.
           ADD 4 TO WS-TOTAL.
           MOVE "PARA-004 DONE" TO WS-RESULT.
       PARA-005.
           ADD 1 TO WS-COUNT.
           ADD 5 TO WS-TOTAL.
           MOVE 1 TO WS-FLAG.
           IF WS-FLAG = 1
               MOVE "FLAG IS SET" TO WS-RESULT
           END-IF.
       PARA-006.
           ADD 1 TO WS-COUNT.
           ADD 6 TO WS-TOTAL.
           MOVE 10 TO WS-FACTOR.
       PARA-007.
           ADD 1 TO WS-COUNT.
           ADD 7 TO WS-TOTAL.
           MULTIPLY WS-FACTOR BY WS-ACCUM.
       PARA-008.
           ADD 1 TO WS-COUNT.
           ADD 8 TO WS-TOTAL.
           MOVE "PARA-008 COMPLETE" TO WS-RESULT.
       PARA-009.
           ADD 1 TO WS-COUNT.
           ADD 9 TO WS-TOTAL.
           MOVE 0 TO WS-SWITCH.
       PARA-010.
           ADD 1 TO WS-COUNT.
           ADD 10 TO WS-TOTAL.
           MOVE "THRU-RANGE-START" TO WS-RESULT.
       PARA-011.
           ADD 1 TO WS-COUNT.
           ADD 11 TO WS-TOTAL.
           MOVE 1 TO WS-PHASE.
       PARA-012.
           ADD 1 TO WS-COUNT.
           ADD 12 TO WS-TOTAL.
           ADD 100 TO WS-ACCUM.
       PARA-013.
           ADD 1 TO WS-COUNT.
           ADD 13 TO WS-TOTAL.
           MOVE "PHASE-1-MID" TO WS-CATEGORY.
       PARA-014.
           ADD 1 TO WS-COUNT.
           ADD 14 TO WS-TOTAL.
           ADD WS-FACTOR TO WS-TEMP.
       PARA-015.
           ADD 1 TO WS-COUNT.
           ADD 15 TO WS-TOTAL.
           MOVE "THRU-RANGE-END" TO WS-RESULT.
       PARA-016.
           ADD 1 TO WS-COUNT.
           ADD 16 TO WS-TOTAL.
           PERFORM PARA-002.
       PARA-017.
           ADD 1 TO WS-COUNT.
           ADD 17 TO WS-TOTAL.
           MOVE 5 TO WS-FACTOR.
       PARA-018.
           ADD 1 TO WS-COUNT.
           ADD 18 TO WS-TOTAL.
           MOVE "PARA-018 OK" TO WS-RESULT.
       PARA-019.
           ADD 1 TO WS-COUNT.
           ADD 19 TO WS-TOTAL.
           ADD WS-FACTOR TO WS-TOTAL.
       PARA-020.
           ADD 1 TO WS-COUNT.
           ADD 20 TO WS-TOTAL.
           PERFORM PARA-021 THRU PARA-025.
       PARA-021.
           ADD 1 TO WS-COUNT.
           ADD 21 TO WS-TOTAL.
           MOVE "NESTED-THRU-START" TO WS-RESULT.
       PARA-022.
           ADD 1 TO WS-COUNT.
           ADD 22 TO WS-TOTAL.
           ADD 50 TO WS-ACCUM.
       PARA-023.
           ADD 1 TO WS-COUNT.
           ADD 23 TO WS-TOTAL.
           MOVE 2 TO WS-PHASE.
       PARA-024.
           ADD 1 TO WS-COUNT.
           ADD 24 TO WS-TOTAL.
           MOVE "PHASE-2" TO WS-CATEGORY.
       PARA-025.
           ADD 1 TO WS-COUNT.
           ADD 25 TO WS-TOTAL.
           MOVE "NESTED-THRU-END" TO WS-RESULT.
       PARA-026.
           ADD 1 TO WS-COUNT.
           ADD 26 TO WS-TOTAL.
           SUBTRACT 10 FROM WS-ACCUM.
       PARA-027.
           ADD 1 TO WS-COUNT.
           ADD 27 TO WS-TOTAL.
           MOVE "PARA-027 DONE" TO WS-MSG.
       PARA-028.
           ADD 1 TO WS-COUNT.
           ADD 28 TO WS-TOTAL.
           MOVE 0 TO WS-FLAG.
       PARA-029.
           ADD 1 TO WS-COUNT.
           ADD 29 TO WS-TOTAL.
           PERFORM PARA-004.
       PARA-030.
           ADD 1 TO WS-COUNT.
           ADD 30 TO WS-TOTAL.
           MOVE "SECOND-THRU-START" TO WS-RESULT.
       PARA-031.
           ADD 1 TO WS-COUNT.
           ADD 31 TO WS-TOTAL.
           ADD 200 TO WS-ACCUM.
       PARA-032.
           ADD 1 TO WS-COUNT.
           ADD 32 TO WS-TOTAL.
           MOVE 3 TO WS-PHASE.
       PARA-033.
           ADD 1 TO WS-COUNT.
           ADD 33 TO WS-TOTAL.
           MOVE "MIDPOINT" TO WS-CATEGORY.
       PARA-034.
           ADD 1 TO WS-COUNT.
           ADD 34 TO WS-TOTAL.
           SUBTRACT 5 FROM WS-TEMP.
       PARA-035.
           ADD 1 TO WS-COUNT.
           ADD 35 TO WS-TOTAL.
           MOVE "SECOND-THRU-END" TO WS-RESULT.
       PARA-036.
           ADD 1 TO WS-COUNT.
           ADD 36 TO WS-TOTAL.
           IF WS-PHASE = 3
               MOVE "IN PHASE 3" TO WS-MSG
           END-IF.
       PARA-037.
           ADD 1 TO WS-COUNT.
           ADD 37 TO WS-TOTAL.
           ADD WS-PHASE TO WS-ACCUM.
       PARA-038.
           ADD 1 TO WS-COUNT.
           ADD 38 TO WS-TOTAL.
           MOVE 1 TO WS-SWITCH.
       PARA-039.
           ADD 1 TO WS-COUNT.
           ADD 39 TO WS-TOTAL.
           MOVE "PARA-039 REACHED" TO WS-RESULT.
       PARA-040.
           ADD 1 TO WS-COUNT.
           ADD 40 TO WS-TOTAL.
           IF WS-SWITCH = 0
               PERFORM PARA-041
           ELSE
               PERFORM PARA-042
           END-IF.
       PARA-041.
           ADD 1 TO WS-COUNT.
           ADD 41 TO WS-TOTAL.
           MOVE "SWITCH-OFF-PATH" TO WS-RESULT.
       PARA-042.
           ADD 1 TO WS-COUNT.
           ADD 42 TO WS-TOTAL.
           MOVE "SWITCH-ON-PATH" TO WS-RESULT.
       PARA-043.
           ADD 1 TO WS-COUNT.
           ADD 43 TO WS-TOTAL.
           MOVE 7 TO WS-FACTOR.
       PARA-044.
           ADD 1 TO WS-COUNT.
           ADD 44 TO WS-TOTAL.
           ADD WS-FACTOR TO WS-PRODUCT.
       PARA-045.
           ADD 1 TO WS-COUNT.
           ADD 45 TO WS-TOTAL.
           MOVE "PARA-045 DONE" TO WS-RESULT.
       PARA-046.
           ADD 1 TO WS-COUNT.
           ADD 46 TO WS-TOTAL.
           PERFORM PARA-006.
       PARA-047.
           ADD 1 TO WS-COUNT.
           ADD 47 TO WS-TOTAL.
           MOVE 4 TO WS-PHASE.
       PARA-048.
           ADD 1 TO WS-COUNT.
           ADD 48 TO WS-TOTAL.
           MOVE "PHASE-4" TO WS-CATEGORY.
       PARA-049.
           ADD 1 TO WS-COUNT.
           ADD 49 TO WS-TOTAL.
           IF WS-COUNT > 30
               GO TO PARA-050-EXIT
           END-IF.
           ADD 500 TO WS-ACCUM.
       PARA-050.
           ADD 1 TO WS-COUNT.
           ADD 50 TO WS-TOTAL.
           MOVE "HALFWAY POINT" TO WS-RESULT.
       PARA-050-EXIT.
           EXIT.
       PARA-051.
           ADD 1 TO WS-COUNT.
           ADD 51 TO WS-TOTAL.
           MOVE "POST-EXIT" TO WS-RESULT.
       PARA-052.
           ADD 1 TO WS-COUNT.
           ADD 52 TO WS-TOTAL.
           ADD 300 TO WS-PRODUCT.
       PARA-053.
           ADD 1 TO WS-COUNT.
           ADD 53 TO WS-TOTAL.
           MOVE 5 TO WS-PHASE.
       PARA-054.
           ADD 1 TO WS-COUNT.
           ADD 54 TO WS-TOTAL.
           MOVE "PHASE-5-START" TO WS-CATEGORY.
       PARA-055.
           ADD 1 TO WS-COUNT.
           ADD 55 TO WS-TOTAL.
           PERFORM PARA-008.
       PARA-056.
           ADD 1 TO WS-COUNT.
           ADD 56 TO WS-TOTAL.
           SUBTRACT WS-FACTOR FROM WS-ACCUM.
       PARA-057.
           ADD 1 TO WS-COUNT.
           ADD 57 TO WS-TOTAL.
           MOVE "PARA-057 REACHED" TO WS-MSG.
       PARA-058.
           ADD 1 TO WS-COUNT.
           ADD 58 TO WS-TOTAL.
           ADD 25 TO WS-TEMP.
       PARA-059.
           ADD 1 TO WS-COUNT.
           ADD 59 TO WS-TOTAL.
           MOVE 0 TO WS-SWITCH.
       PARA-060.
           ADD 1 TO WS-COUNT.
           ADD 60 TO WS-TOTAL.
           MOVE "THIRD-THRU-START" TO WS-RESULT.
           MOVE 6 TO WS-PHASE.
       PARA-061.
           ADD 1 TO WS-COUNT.
           ADD 61 TO WS-TOTAL.
           ADD WS-PHASE TO WS-TOTAL.
       PARA-062.
           ADD 1 TO WS-COUNT.
           ADD 62 TO WS-TOTAL.
           MOVE "ACCUMULATING" TO WS-CATEGORY.
       PARA-063.
           ADD 1 TO WS-COUNT.
           ADD 63 TO WS-TOTAL.
           PERFORM PARA-009.
       PARA-064.
           ADD 1 TO WS-COUNT.
           ADD 64 TO WS-TOTAL.
           ADD 75 TO WS-PRODUCT.
       PARA-065.
           ADD 1 TO WS-COUNT.
           ADD 65 TO WS-TOTAL.
           MOVE "THIRD-THRU-END" TO WS-RESULT.
       PARA-066.
           ADD 1 TO WS-COUNT.
           ADD 66 TO WS-TOTAL.
           MOVE "PARA-066 DONE" TO WS-MSG.
       PARA-067.
           ADD 1 TO WS-COUNT.
           ADD 67 TO WS-TOTAL.
           MOVE 1 TO WS-FLAG.
       PARA-068.
           ADD 1 TO WS-COUNT.
           ADD 68 TO WS-TOTAL.
           IF WS-FLAG = 1
               ADD 1000 TO WS-PRODUCT
           END-IF.
       PARA-069.
           ADD 1 TO WS-COUNT.
           ADD 69 TO WS-TOTAL.
           MOVE 0 TO WS-FLAG.
       PARA-070.
           ADD 1 TO WS-COUNT.
           ADD 70 TO WS-TOTAL.
           PERFORM PARA-071 THRU PARA-075.
           MOVE "PARA-070 DONE" TO WS-RESULT.
       PARA-071.
           ADD 1 TO WS-COUNT.
           ADD 71 TO WS-TOTAL.
           MOVE "INNER-THRU-START" TO WS-RESULT.
       PARA-072.
           ADD 1 TO WS-COUNT.
           ADD 72 TO WS-TOTAL.
           ADD 15 TO WS-FACTOR.
       PARA-073.
           ADD 1 TO WS-COUNT.
           ADD 73 TO WS-TOTAL.
           MOVE 7 TO WS-PHASE.
       PARA-074.
           ADD 1 TO WS-COUNT.
           ADD 74 TO WS-TOTAL.
           MOVE "PHASE-7" TO WS-CATEGORY.
       PARA-075.
           ADD 1 TO WS-COUNT.
           ADD 75 TO WS-TOTAL.
           MOVE "INNER-THRU-END" TO WS-RESULT.
       PARA-075-EXIT.
           EXIT.
       PARA-076.
           ADD 1 TO WS-COUNT.
           ADD 76 TO WS-TOTAL.
           SUBTRACT 3 FROM WS-PHASE.
       PARA-077.
           ADD 1 TO WS-COUNT.
           ADD 77 TO WS-TOTAL.
           MOVE "WINDING DOWN" TO WS-MSG.
       PARA-078.
           ADD 1 TO WS-COUNT.
           ADD 78 TO WS-TOTAL.
           ADD WS-ACCUM TO WS-TOTAL.
       PARA-079.
           ADD 1 TO WS-COUNT.
           ADD 79 TO WS-TOTAL.
           MOVE "PARA-079 REACHED" TO WS-RESULT.
       PARA-080.
           ADD 1 TO WS-COUNT.
           ADD 80 TO WS-TOTAL.
           MOVE "FOURTH-THRU-START" TO WS-RESULT.
       PARA-081.
           ADD 1 TO WS-COUNT.
           ADD 81 TO WS-TOTAL.
           ADD 10 TO WS-REMAINDER.
       PARA-082.
           ADD 1 TO WS-COUNT.
           ADD 82 TO WS-TOTAL.
           MOVE 8 TO WS-PHASE.
       PARA-083.
           ADD 1 TO WS-COUNT.
           ADD 83 TO WS-TOTAL.
           MOVE "NEAR END" TO WS-CATEGORY.
       PARA-084.
           ADD 1 TO WS-COUNT.
           ADD 84 TO WS-TOTAL.
           PERFORM PARA-003.
       PARA-085.
           ADD 1 TO WS-COUNT.
           ADD 85 TO WS-TOTAL.
           MOVE "FOURTH-THRU-END" TO WS-RESULT.
       PARA-085-EXIT.
           EXIT.
       PARA-086.
           ADD 1 TO WS-COUNT.
           ADD 86 TO WS-TOTAL.
           MOVE 1 TO WS-SWITCH.
       PARA-087.
           ADD 1 TO WS-COUNT.
           ADD 87 TO WS-TOTAL.
           IF WS-SWITCH = 1
               MOVE "SWITCHED ON" TO WS-MSG
           ELSE
               MOVE "SWITCHED OFF" TO WS-MSG
           END-IF.
       PARA-088.
           ADD 1 TO WS-COUNT.
           ADD 88 TO WS-TOTAL.
           ADD WS-REMAINDER TO WS-QUOTIENT.
       PARA-089.
           ADD 1 TO WS-COUNT.
           ADD 89 TO WS-TOTAL.
           MOVE "PARA-089 DONE" TO WS-RESULT.
       PARA-090.
           ADD 1 TO WS-COUNT.
           ADD 90 TO WS-TOTAL.
           PERFORM PARA-091 THRU PARA-093.
           MOVE 9 TO WS-PHASE.
       PARA-091.
           ADD 1 TO WS-COUNT.
           ADD 91 TO WS-TOTAL.
           ADD WS-QUOTIENT TO WS-PRODUCT.
       PARA-092.
           ADD 1 TO WS-COUNT.
           ADD 92 TO WS-TOTAL.
           MOVE "FINAL-THRU-MID" TO WS-RESULT.
       PARA-093.
           ADD 1 TO WS-COUNT.
           ADD 93 TO WS-TOTAL.
           MOVE "FINAL-THRU-END" TO WS-RESULT.
       PARA-093-EXIT.
           EXIT.
       PARA-094.
           ADD 1 TO WS-COUNT.
           ADD 94 TO WS-TOTAL.
           MOVE "APPROACHING END" TO WS-MSG.
       PARA-095.
           ADD 1 TO WS-COUNT.
           ADD 95 TO WS-TOTAL.
           MOVE "LAST-THRU-START" TO WS-RESULT.
           MOVE 10 TO WS-PHASE.
       PARA-096.
           ADD 1 TO WS-COUNT.
           ADD 96 TO WS-TOTAL.
           ADD WS-PRODUCT TO WS-TOTAL.
       PARA-097.
           ADD 1 TO WS-COUNT.
           ADD 97 TO WS-TOTAL.
           MOVE "CLEANUP" TO WS-CATEGORY.
       PARA-098.
           ADD 1 TO WS-COUNT.
           ADD 98 TO WS-TOTAL.
           PERFORM PARA-007.
       PARA-099.
           ADD 1 TO WS-COUNT.
           ADD 99 TO WS-TOTAL.
           MOVE "ALMOST DONE" TO WS-MSG.
       PARA-100.
           ADD 1 TO WS-COUNT.
           ADD 100 TO WS-TOTAL.
           MOVE "LAST-THRU-END" TO WS-RESULT.
       PARA-100-EXIT.
           EXIT.
       PARA-101.
           ADD 1 TO WS-COUNT.
           ADD 101 TO WS-TOTAL.
           MOVE "BEYOND 100" TO WS-RESULT.
       PARA-102.
           ADD 1 TO WS-COUNT.
           ADD 102 TO WS-TOTAL.
           MOVE WS-PHASE TO WS-CATEGORY.
       PARA-103.
           ADD 1 TO WS-COUNT.
           ADD 103 TO WS-TOTAL.
           IF WS-TOTAL > 1000
               MOVE "TOTAL EXCEEDED 1000" TO WS-MSG
           END-IF.
       PARA-104.
           ADD 1 TO WS-COUNT.
           ADD 104 TO WS-TOTAL.
           MOVE 0 TO WS-SWITCH.
       PARA-105.
           ADD 1 TO WS-COUNT.
           ADD 105 TO WS-TOTAL.
           MOVE "PARA-105 FINAL" TO WS-RESULT.
       FINAL-PARA.
           DISPLAY "PARAGRAPHS EXECUTED: " WS-COUNT.
           DISPLAY "RUNNING TOTAL: " WS-TOTAL.
           DISPLAY "LAST RESULT: " WS-RESULT.
           DISPLAY "ACCUMULATOR: " WS-ACCUM.
           DISPLAY "PRODUCT: " WS-PRODUCT.
           DISPLAY "PHASE: " WS-PHASE.
           STOP RUN.
