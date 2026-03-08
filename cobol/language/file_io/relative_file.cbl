       IDENTIFICATION DIVISION.
       PROGRAM-ID. RELATIVE-FILE-TEST.
      *---------------------------------------------------------------
      * Stress test: Relative file I/O.
      * Covers: OPEN I-O, WRITE, READ, REWRITE, DELETE,
      * START, READ NEXT, INVALID KEY.
      *---------------------------------------------------------------
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT REL-FILE ASSIGN TO "rel-test.dat"
               ORGANIZATION IS RELATIVE
               ACCESS MODE IS DYNAMIC
               RELATIVE KEY IS WS-REL-KEY
               FILE STATUS IS WS-FILE-STATUS.
       DATA DIVISION.
       FILE SECTION.
       FD  REL-FILE.
       01  REL-RECORD.
           05  REL-ID           PIC 9(5).
           05  REL-NAME         PIC X(20).
           05  REL-VALUE        PIC 9(7)V99.
       WORKING-STORAGE SECTION.
       01  WS-FILE-STATUS       PIC XX.
       01  WS-REL-KEY           PIC 9(5) VALUE 0.
       01  WS-EOF               PIC 9 VALUE 0.
       01  WS-COUNT             PIC 999 VALUE 0.
       01  WS-I                 PIC 99.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Create and populate
           OPEN OUTPUT REL-FILE.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 10
               MOVE WS-I TO WS-REL-KEY
               MOVE WS-I TO REL-ID
               MOVE "RELATIVE REC" TO REL-NAME
               COMPUTE REL-VALUE = WS-I * 50.25
               WRITE REL-RECORD
                   INVALID KEY
                       DISPLAY "WRITE ERROR KEY=" WS-REL-KEY
               END-WRITE
           END-PERFORM.
           CLOSE REL-FILE.
      * Random read
           OPEN I-O REL-FILE.
           MOVE 5 TO WS-REL-KEY.
           READ REL-FILE
               INVALID KEY
                   DISPLAY "READ ERROR"
               NOT INVALID KEY
                   DISPLAY REL-ID " " REL-NAME " " REL-VALUE
           END-READ.
      * Rewrite record
           MOVE "UPDATED REC" TO REL-NAME.
           MOVE 99999.99 TO REL-VALUE.
           REWRITE REL-RECORD
               INVALID KEY
                   DISPLAY "REWRITE ERROR"
           END-REWRITE.
      * Verify rewrite
           MOVE 5 TO WS-REL-KEY.
           READ REL-FILE
               INVALID KEY
                   DISPLAY "READ ERROR"
               NOT INVALID KEY
                   DISPLAY REL-ID " " REL-NAME " " REL-VALUE
           END-READ.
      * Delete record
           MOVE 3 TO WS-REL-KEY.
           DELETE REL-FILE
               INVALID KEY
                   DISPLAY "DELETE ERROR"
           END-DELETE.
      * Sequential read all
           MOVE 1 TO WS-REL-KEY.
           START REL-FILE KEY >= WS-REL-KEY
               INVALID KEY
                   DISPLAY "START ERROR"
           END-START.
           MOVE 0 TO WS-EOF.
           MOVE 0 TO WS-COUNT.
           PERFORM UNTIL WS-EOF = 1
               READ REL-FILE NEXT
                   AT END
                       MOVE 1 TO WS-EOF
                   NOT AT END
                       ADD 1 TO WS-COUNT
               END-READ
           END-PERFORM.
           CLOSE REL-FILE.
           DISPLAY "REMAINING RECORDS: " WS-COUNT.
           STOP RUN.
