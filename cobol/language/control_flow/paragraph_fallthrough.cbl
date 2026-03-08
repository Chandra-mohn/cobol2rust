       IDENTIFICATION DIVISION.
       PROGRAM-ID. PARA-FALLTHROUGH-TEST.
      *---------------------------------------------------------------
      * Stress test: Paragraph fall-through behavior.
      * Covers: Sequential paragraph execution, fall-through with
      * conditional GO TO, PERFORM range vs fall-through.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-STEP              PIC 99 VALUE 0.
       01  WS-TOTAL             PIC 999 VALUE 0.
       01  WS-RESULT            PIC X(40).
       01  WS-FLAG              PIC 9 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * PERFORM THRU demonstrates fall-through
           PERFORM STEP-A THRU STEP-D.
           DISPLAY WS-TOTAL.
      * Direct call to first para falls through all
           MOVE 0 TO WS-TOTAL.
           PERFORM ACCUM-1.
           DISPLAY WS-TOTAL.
      * Conditional skip with GO TO
           MOVE 1 TO WS-FLAG.
           PERFORM CHECK-PARA THRU CHECK-PARA-EXIT.
           DISPLAY WS-RESULT.
      * Multiple paragraphs in sequence
           MOVE 0 TO WS-STEP.
           PERFORM PIPELINE-START THRU PIPELINE-END.
           DISPLAY WS-STEP.
           STOP RUN.

       STEP-A.
           ADD 10 TO WS-TOTAL.
       STEP-B.
           ADD 20 TO WS-TOTAL.
       STEP-C.
           ADD 30 TO WS-TOTAL.
       STEP-D.
           ADD 40 TO WS-TOTAL.

       ACCUM-1.
           ADD 1 TO WS-TOTAL.
       ACCUM-2.
           ADD 2 TO WS-TOTAL.
       ACCUM-3.
           ADD 3 TO WS-TOTAL.

       CHECK-PARA.
           IF WS-FLAG = 1
               MOVE "FLAG WAS SET" TO WS-RESULT
               GO TO CHECK-PARA-EXIT
           END-IF.
           MOVE "FLAG NOT SET" TO WS-RESULT.
       CHECK-PARA-EXIT.
           EXIT.

       PIPELINE-START.
           ADD 1 TO WS-STEP.
       PIPELINE-VALIDATE.
           ADD 1 TO WS-STEP.
       PIPELINE-PROCESS.
           ADD 1 TO WS-STEP.
       PIPELINE-FINALIZE.
           ADD 1 TO WS-STEP.
       PIPELINE-END.
           ADD 1 TO WS-STEP.
