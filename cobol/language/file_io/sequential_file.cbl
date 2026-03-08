       IDENTIFICATION DIVISION.
       PROGRAM-ID. SEQUENTIAL-FILE-TEST.
      *---------------------------------------------------------------
      * Stress test: Sequential file I/O.
      * Covers: OPEN OUTPUT/INPUT/EXTEND, WRITE, READ, CLOSE,
      * FILE STATUS, AT END, NOT AT END.
      *---------------------------------------------------------------
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT SEQ-FILE ASSIGN TO "seq-test.dat"
               ORGANIZATION IS SEQUENTIAL
               FILE STATUS IS WS-FILE-STATUS.
       DATA DIVISION.
       FILE SECTION.
       FD  SEQ-FILE.
       01  SEQ-RECORD.
           05  SEQ-ID           PIC 9(5).
           05  SEQ-NAME         PIC X(20).
           05  SEQ-AMOUNT       PIC 9(7)V99.
       WORKING-STORAGE SECTION.
       01  WS-FILE-STATUS       PIC XX.
       01  WS-EOF               PIC 9 VALUE 0.
       01  WS-COUNT             PIC 999 VALUE 0.
       01  WS-I                 PIC 99 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Write records
           OPEN OUTPUT SEQ-FILE.
           IF WS-FILE-STATUS NOT = "00"
               DISPLAY "OPEN OUTPUT FAILED: " WS-FILE-STATUS
               STOP RUN
           END-IF.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 10
               MOVE WS-I TO SEQ-ID
               MOVE "RECORD NAME" TO SEQ-NAME
               COMPUTE SEQ-AMOUNT = WS-I * 100.50
               WRITE SEQ-RECORD
           END-PERFORM.
           CLOSE SEQ-FILE.
      * Read records back
           OPEN INPUT SEQ-FILE.
           MOVE 0 TO WS-EOF.
           MOVE 0 TO WS-COUNT.
           PERFORM UNTIL WS-EOF = 1
               READ SEQ-FILE
                   AT END
                       MOVE 1 TO WS-EOF
                   NOT AT END
                       ADD 1 TO WS-COUNT
                       DISPLAY SEQ-ID " " SEQ-NAME " " SEQ-AMOUNT
               END-READ
           END-PERFORM.
           CLOSE SEQ-FILE.
           DISPLAY "READ COUNT: " WS-COUNT.
      * Extend file
           OPEN EXTEND SEQ-FILE.
           MOVE 99 TO SEQ-ID.
           MOVE "EXTENDED RECORD" TO SEQ-NAME.
           MOVE 9999.99 TO SEQ-AMOUNT.
           WRITE SEQ-RECORD.
           CLOSE SEQ-FILE.
      * Verify extended
           OPEN INPUT SEQ-FILE.
           MOVE 0 TO WS-EOF.
           MOVE 0 TO WS-COUNT.
           PERFORM UNTIL WS-EOF = 1
               READ SEQ-FILE
                   AT END
                       MOVE 1 TO WS-EOF
                   NOT AT END
                       ADD 1 TO WS-COUNT
               END-READ
           END-PERFORM.
           CLOSE SEQ-FILE.
           DISPLAY "TOTAL RECORDS: " WS-COUNT.
           STOP RUN.
