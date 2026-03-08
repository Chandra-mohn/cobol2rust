       IDENTIFICATION DIVISION.
       PROGRAM-ID. MERGE-VERB-TEST.
      *---------------------------------------------------------------
      * Stress test: MERGE statement.
      * Covers: MERGE with ASCENDING KEY, multiple USING files,
      * GIVING output file.
      *---------------------------------------------------------------
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT MERGE-FILE ASSIGN TO "merge-work.tmp"
               FILE STATUS IS WS-MERGE-STATUS.
           SELECT INPUT-FILE-A ASSIGN TO "merge-a.dat"
               ORGANIZATION IS SEQUENTIAL
               FILE STATUS IS WS-A-STATUS.
           SELECT INPUT-FILE-B ASSIGN TO "merge-b.dat"
               ORGANIZATION IS SEQUENTIAL
               FILE STATUS IS WS-B-STATUS.
           SELECT OUTPUT-FILE ASSIGN TO "merge-out.dat"
               ORGANIZATION IS SEQUENTIAL
               FILE STATUS IS WS-OUT-STATUS.
       DATA DIVISION.
       FILE SECTION.
       SD  MERGE-FILE.
       01  MERGE-RECORD.
           05  MERGE-KEY        PIC 9(5).
           05  MERGE-NAME       PIC X(20).
           05  MERGE-AMT        PIC 9(7)V99.
       FD  INPUT-FILE-A.
       01  INPUT-REC-A.
           05  IN-A-KEY         PIC 9(5).
           05  IN-A-NAME        PIC X(20).
           05  IN-A-AMT         PIC 9(7)V99.
       FD  INPUT-FILE-B.
       01  INPUT-REC-B.
           05  IN-B-KEY         PIC 9(5).
           05  IN-B-NAME        PIC X(20).
           05  IN-B-AMT         PIC 9(7)V99.
       FD  OUTPUT-FILE.
       01  OUTPUT-RECORD.
           05  OUT-KEY          PIC 9(5).
           05  OUT-NAME         PIC X(20).
           05  OUT-AMT          PIC 9(7)V99.
       WORKING-STORAGE SECTION.
       01  WS-MERGE-STATUS      PIC XX.
       01  WS-A-STATUS          PIC XX.
       01  WS-B-STATUS          PIC XX.
       01  WS-OUT-STATUS        PIC XX.
       01  WS-EOF               PIC 9 VALUE 0.
       01  WS-COUNT             PIC 999 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Create sorted input file A
           OPEN OUTPUT INPUT-FILE-A.
           MOVE 00010 TO IN-A-KEY.
           MOVE "ALICE" TO IN-A-NAME.
           MOVE 3000.00 TO IN-A-AMT.
           WRITE INPUT-REC-A.
           MOVE 00030 TO IN-A-KEY.
           MOVE "CHARLIE" TO IN-A-NAME.
           MOVE 5000.00 TO IN-A-AMT.
           WRITE INPUT-REC-A.
           MOVE 00050 TO IN-A-KEY.
           MOVE "EVE" TO IN-A-NAME.
           MOVE 7000.00 TO IN-A-AMT.
           WRITE INPUT-REC-A.
           CLOSE INPUT-FILE-A.
      * Create sorted input file B
           OPEN OUTPUT INPUT-FILE-B.
           MOVE 00020 TO IN-B-KEY.
           MOVE "BOB" TO IN-B-NAME.
           MOVE 4000.00 TO IN-B-AMT.
           WRITE INPUT-REC-B.
           MOVE 00040 TO IN-B-KEY.
           MOVE "DIANA" TO IN-B-NAME.
           MOVE 6000.00 TO IN-B-AMT.
           WRITE INPUT-REC-B.
           CLOSE INPUT-FILE-B.
      * MERGE two sorted files
           MERGE MERGE-FILE
               ON ASCENDING KEY MERGE-KEY
               USING INPUT-FILE-A INPUT-FILE-B
               GIVING OUTPUT-FILE.
      * Read and display merged output
           OPEN INPUT OUTPUT-FILE.
           MOVE 0 TO WS-EOF.
           MOVE 0 TO WS-COUNT.
           PERFORM UNTIL WS-EOF = 1
               READ OUTPUT-FILE
                   AT END MOVE 1 TO WS-EOF
                   NOT AT END
                       ADD 1 TO WS-COUNT
                       DISPLAY OUT-KEY " " OUT-NAME " " OUT-AMT
               END-READ
           END-PERFORM.
           CLOSE OUTPUT-FILE.
           DISPLAY "MERGED RECORDS: " WS-COUNT.
           STOP RUN.
