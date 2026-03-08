       IDENTIFICATION DIVISION.
       PROGRAM-ID. SECTION-PERFORM-TEST.
      *---------------------------------------------------------------
      * Stress test: SECTION-based PERFORM and paragraph interaction.
      * Covers: PERFORM SECTION, paragraphs within sections,
      * PERFORM para THRU para within section.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-RESULT            PIC X(30).
       01  WS-COUNT             PIC 999 VALUE 0.
       01  WS-I                 PIC 99 VALUE 0.
       01  WS-SUM               PIC 9(7) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-SECTION SECTION.
       MAIN-PARA.
      * PERFORM a section
           PERFORM PROCESSING-SECTION.
           DISPLAY WS-COUNT.
      * PERFORM individual paragraphs
           PERFORM CALC-PARA.
           DISPLAY WS-SUM.
      * PERFORM THRU in section
           PERFORM STEP-1 THRU STEP-3.
           DISPLAY WS-COUNT.
           STOP RUN.

       PROCESSING-SECTION SECTION.
       PROC-INIT.
           MOVE 0 TO WS-COUNT.
       PROC-WORK.
           ADD 100 TO WS-COUNT.
       PROC-FINISH.
           ADD 50 TO WS-COUNT.

       CALCULATION-SECTION SECTION.
       CALC-PARA.
           MOVE 0 TO WS-SUM.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 5
               ADD WS-I TO WS-SUM
           END-PERFORM.

       STEPS-SECTION SECTION.
       STEP-1.
           ADD 10 TO WS-COUNT.
       STEP-2.
           ADD 20 TO WS-COUNT.
       STEP-3.
           ADD 30 TO WS-COUNT.
