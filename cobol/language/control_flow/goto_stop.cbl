       IDENTIFICATION DIVISION.
       PROGRAM-ID. GOTO-STOP-TEST.
      *---------------------------------------------------------------
      * Stress test: GO TO, STOP RUN, EXIT, CONTINUE.
      * Covers: Simple GO TO, GO TO DEPENDING ON, EXIT PARAGRAPH,
      * CONTINUE, NEXT SENTENCE, paragraph fall-through.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-CODE              PIC 9 VALUE 2.
       01  WS-COUNT             PIC 999 VALUE 0.
       01  WS-RESULT            PIC X(30).
       01  WS-I                 PIC 99 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple GO TO
           GO TO TARGET-PARA.
       SKIP-PARA.
           MOVE "SHOULD NOT REACH" TO WS-RESULT.
       TARGET-PARA.
           MOVE "GOTO WORKED" TO WS-RESULT.
           DISPLAY WS-RESULT.
      * GO TO DEPENDING ON
           MOVE 2 TO WS-CODE.
           GO TO CASE-1-PARA CASE-2-PARA CASE-3-PARA
               DEPENDING ON WS-CODE.
       CASE-1-PARA.
           MOVE "CASE 1" TO WS-RESULT.
           GO TO AFTER-GOTO-DEPEND.
       CASE-2-PARA.
           MOVE "CASE 2" TO WS-RESULT.
           GO TO AFTER-GOTO-DEPEND.
       CASE-3-PARA.
           MOVE "CASE 3" TO WS-RESULT.
           GO TO AFTER-GOTO-DEPEND.
       AFTER-GOTO-DEPEND.
           DISPLAY WS-RESULT.
      * CONTINUE (no-op)
           IF WS-CODE = 2
               CONTINUE
           END-IF.
           MOVE "AFTER CONTINUE" TO WS-RESULT.
           DISPLAY WS-RESULT.
      * Paragraph fall-through
           PERFORM FALL-THROUGH-A.
           DISPLAY WS-COUNT.
      * EXIT PARAGRAPH
           PERFORM EXIT-TEST-PARA.
           DISPLAY WS-RESULT.
      * Final
           DISPLAY "PROGRAM ENDING".
           STOP RUN.

       FALL-THROUGH-A.
           ADD 10 TO WS-COUNT.
       FALL-THROUGH-B.
           ADD 20 TO WS-COUNT.
       FALL-THROUGH-C.
           ADD 30 TO WS-COUNT.

       EXIT-TEST-PARA.
           MOVE "BEFORE EXIT" TO WS-RESULT.
           IF WS-CODE = 2
               EXIT PARAGRAPH
           END-IF.
           MOVE "AFTER EXIT" TO WS-RESULT.
