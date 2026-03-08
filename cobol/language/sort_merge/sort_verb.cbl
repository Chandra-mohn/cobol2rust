       IDENTIFICATION DIVISION.
       PROGRAM-ID. SORT-VERB-TEST.
      *---------------------------------------------------------------
      * Stress test: SORT statement.
      * Covers: SORT ASCENDING/DESCENDING, USING/GIVING,
      * INPUT/OUTPUT PROCEDURE, RELEASE, RETURN.
      *---------------------------------------------------------------
       ENVIRONMENT DIVISION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT SORT-FILE ASSIGN TO "sort-work.tmp"
               FILE STATUS IS WS-SORT-STATUS.
           SELECT INPUT-FILE ASSIGN TO "sort-input.dat"
               ORGANIZATION IS SEQUENTIAL
               FILE STATUS IS WS-IN-STATUS.
           SELECT OUTPUT-FILE ASSIGN TO "sort-output.dat"
               ORGANIZATION IS SEQUENTIAL
               FILE STATUS IS WS-OUT-STATUS.
       DATA DIVISION.
       FILE SECTION.
       SD  SORT-FILE.
       01  SORT-RECORD.
           05  SORT-KEY         PIC 9(5).
           05  SORT-NAME        PIC X(20).
           05  SORT-AMT         PIC 9(7)V99.
       FD  INPUT-FILE.
       01  INPUT-RECORD.
           05  IN-KEY           PIC 9(5).
           05  IN-NAME          PIC X(20).
           05  IN-AMT           PIC 9(7)V99.
       FD  OUTPUT-FILE.
       01  OUTPUT-RECORD.
           05  OUT-KEY          PIC 9(5).
           05  OUT-NAME         PIC X(20).
           05  OUT-AMT          PIC 9(7)V99.
       WORKING-STORAGE SECTION.
       01  WS-SORT-STATUS       PIC XX.
       01  WS-IN-STATUS         PIC XX.
       01  WS-OUT-STATUS        PIC XX.
       01  WS-EOF               PIC 9 VALUE 0.
       01  WS-COUNT             PIC 999 VALUE 0.
       01  WS-I                 PIC 99.
       01  WS-KEYS              PIC 9(5).
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Create input file with unsorted data
           OPEN OUTPUT INPUT-FILE.
           MOVE 00050 TO IN-KEY.
           MOVE "CHARLIE" TO IN-NAME.
           MOVE 5000.00 TO IN-AMT.
           WRITE INPUT-RECORD.
           MOVE 00010 TO IN-KEY.
           MOVE "ALICE" TO IN-NAME.
           MOVE 3000.00 TO IN-AMT.
           WRITE INPUT-RECORD.
           MOVE 00030 TO IN-KEY.
           MOVE "BOB" TO IN-NAME.
           MOVE 4000.00 TO IN-AMT.
           WRITE INPUT-RECORD.
           MOVE 00020 TO IN-KEY.
           MOVE "DIANA" TO IN-NAME.
           MOVE 6000.00 TO IN-AMT.
           WRITE INPUT-RECORD.
           MOVE 00040 TO IN-KEY.
           MOVE "EVE" TO IN-NAME.
           MOVE 2000.00 TO IN-AMT.
           WRITE INPUT-RECORD.
           CLOSE INPUT-FILE.
      * SORT USING/GIVING (ascending)
           SORT SORT-FILE
               ON ASCENDING KEY SORT-KEY
               USING INPUT-FILE
               GIVING OUTPUT-FILE.
      * Read and display sorted output
           OPEN INPUT OUTPUT-FILE.
           MOVE 0 TO WS-EOF.
           PERFORM UNTIL WS-EOF = 1
               READ OUTPUT-FILE
                   AT END MOVE 1 TO WS-EOF
                   NOT AT END
                       DISPLAY OUT-KEY " " OUT-NAME " " OUT-AMT
               END-READ
           END-PERFORM.
           CLOSE OUTPUT-FILE.
      * SORT with INPUT PROCEDURE
           SORT SORT-FILE
               ON DESCENDING KEY SORT-AMT
               INPUT PROCEDURE IS GENERATE-DATA
               GIVING OUTPUT-FILE.
      * Display descending result
           OPEN INPUT OUTPUT-FILE.
           MOVE 0 TO WS-EOF.
           DISPLAY "--- DESCENDING BY AMT ---".
           PERFORM UNTIL WS-EOF = 1
               READ OUTPUT-FILE
                   AT END MOVE 1 TO WS-EOF
                   NOT AT END
                       DISPLAY OUT-KEY " " OUT-NAME " " OUT-AMT
               END-READ
           END-PERFORM.
           CLOSE OUTPUT-FILE.
           STOP RUN.

       GENERATE-DATA SECTION.
       GEN-PARA.
           MOVE 00001 TO SORT-KEY.
           MOVE "ZEBRA" TO SORT-NAME.
           MOVE 9000.00 TO SORT-AMT.
           RELEASE SORT-RECORD.
           MOVE 00002 TO SORT-KEY.
           MOVE "APPLE" TO SORT-NAME.
           MOVE 1000.00 TO SORT-AMT.
           RELEASE SORT-RECORD.
           MOVE 00003 TO SORT-KEY.
           MOVE "MANGO" TO SORT-NAME.
           MOVE 5000.00 TO SORT-AMT.
           RELEASE SORT-RECORD.
