       IDENTIFICATION DIVISION.
       PROGRAM-ID. INDEXED-FILE-TEST.
      *---------------------------------------------------------------
      * Stress test: Indexed (ISAM) file I/O.
      * Covers: OPEN I-O, WRITE, READ, REWRITE, DELETE, START,
      * READ NEXT, ALTERNATE KEY, INVALID KEY.
      *---------------------------------------------------------------
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT IDX-FILE ASSIGN TO "idx-test.dat"
               ORGANIZATION IS INDEXED
               ACCESS MODE IS DYNAMIC
               RECORD KEY IS IDX-KEY
               ALTERNATE RECORD KEY IS IDX-ALT-KEY
                   WITH DUPLICATES
               FILE STATUS IS WS-FILE-STATUS.
       DATA DIVISION.
       FILE SECTION.
       FD  IDX-FILE.
       01  IDX-RECORD.
           05  IDX-KEY          PIC X(5).
           05  IDX-ALT-KEY      PIC X(10).
           05  IDX-NAME         PIC X(20).
           05  IDX-VALUE        PIC 9(7)V99.
       WORKING-STORAGE SECTION.
       01  WS-FILE-STATUS       PIC XX.
       01  WS-EOF               PIC 9 VALUE 0.
       01  WS-COUNT             PIC 999 VALUE 0.
       01  WS-I                 PIC 99.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Create indexed file
           OPEN OUTPUT IDX-FILE.
           MOVE "00001" TO IDX-KEY.
           MOVE "DEPT-A" TO IDX-ALT-KEY.
           MOVE "ALICE" TO IDX-NAME.
           MOVE 50000.00 TO IDX-VALUE.
           WRITE IDX-RECORD
               INVALID KEY DISPLAY "WRITE ERR".
           MOVE "00002" TO IDX-KEY.
           MOVE "DEPT-B" TO IDX-ALT-KEY.
           MOVE "BOB" TO IDX-NAME.
           MOVE 60000.00 TO IDX-VALUE.
           WRITE IDX-RECORD
               INVALID KEY DISPLAY "WRITE ERR".
           MOVE "00003" TO IDX-KEY.
           MOVE "DEPT-A" TO IDX-ALT-KEY.
           MOVE "CHARLIE" TO IDX-NAME.
           MOVE 55000.00 TO IDX-VALUE.
           WRITE IDX-RECORD
               INVALID KEY DISPLAY "WRITE ERR".
           MOVE "00004" TO IDX-KEY.
           MOVE "DEPT-C" TO IDX-ALT-KEY.
           MOVE "DIANA" TO IDX-NAME.
           MOVE 70000.00 TO IDX-VALUE.
           WRITE IDX-RECORD
               INVALID KEY DISPLAY "WRITE ERR".
           MOVE "00005" TO IDX-KEY.
           MOVE "DEPT-B" TO IDX-ALT-KEY.
           MOVE "EVE" TO IDX-NAME.
           MOVE 45000.00 TO IDX-VALUE.
           WRITE IDX-RECORD
               INVALID KEY DISPLAY "WRITE ERR".
           CLOSE IDX-FILE.
      * Random read by primary key
           OPEN I-O IDX-FILE.
           MOVE "00003" TO IDX-KEY.
           READ IDX-FILE
               INVALID KEY
                   DISPLAY "READ ERROR"
               NOT INVALID KEY
                   DISPLAY IDX-KEY " " IDX-NAME " " IDX-VALUE
           END-READ.
      * Rewrite
           MOVE "CHARLES" TO IDX-NAME.
           MOVE 57000.00 TO IDX-VALUE.
           REWRITE IDX-RECORD
               INVALID KEY DISPLAY "REWRITE ERR".
      * Verify rewrite
           MOVE "00003" TO IDX-KEY.
           READ IDX-FILE
               NOT INVALID KEY
                   DISPLAY IDX-KEY " " IDX-NAME " " IDX-VALUE
           END-READ.
      * Delete record
           MOVE "00002" TO IDX-KEY.
           DELETE IDX-FILE
               INVALID KEY DISPLAY "DELETE ERR".
      * Sequential scan
           MOVE LOW-VALUES TO IDX-KEY.
           START IDX-FILE KEY >= IDX-KEY
               INVALID KEY DISPLAY "START ERR".
           MOVE 0 TO WS-EOF.
           MOVE 0 TO WS-COUNT.
           PERFORM UNTIL WS-EOF = 1
               READ IDX-FILE NEXT
                   AT END
                       MOVE 1 TO WS-EOF
                   NOT AT END
                       ADD 1 TO WS-COUNT
                       DISPLAY IDX-KEY " " IDX-NAME
               END-READ
           END-PERFORM.
           CLOSE IDX-FILE.
           DISPLAY "REMAINING: " WS-COUNT.
           STOP RUN.
