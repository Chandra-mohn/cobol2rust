       IDENTIFICATION DIVISION.
       PROGRAM-ID. COPY-REPLACING-TEST.
      *---------------------------------------------------------------
      * Stress test: COPY and REPLACING directives.
      * Note: This tests the preprocessor's ability to handle COPY.
      * Since copybooks may not exist, this tests parse resilience.
      * Real COPY testing requires copybook files to be present.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Inline data (no COPY dependency)
       01  WS-FIELD1            PIC X(20) VALUE "TEST DATA".
       01  WS-FIELD2            PIC 9(5) VALUE 12345.
       01  WS-RESULT            PIC X(30).
      * Simulated COPY result (what a copybook would expand to)
       01  WS-CUSTOMER-REC.
           05  WS-CUST-ID       PIC 9(5).
           05  WS-CUST-NAME     PIC X(30).
           05  WS-CUST-BAL      PIC S9(7)V99.
       01  WS-ORDER-REC.
           05  WS-ORD-ID        PIC 9(8).
           05  WS-ORD-CUST      PIC 9(5).
           05  WS-ORD-AMT       PIC 9(7)V99.
           05  WS-ORD-DATE      PIC 9(8).
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Use the "copied" structures
           MOVE 10001 TO WS-CUST-ID.
           MOVE "JOHN DOE" TO WS-CUST-NAME.
           MOVE 5000.50 TO WS-CUST-BAL.
           DISPLAY WS-CUST-ID.
           DISPLAY WS-CUST-NAME.
           DISPLAY WS-CUST-BAL.
           MOVE 20250101 TO WS-ORD-ID.
           MOVE 10001 TO WS-ORD-CUST.
           MOVE 250.00 TO WS-ORD-AMT.
           MOVE 20250115 TO WS-ORD-DATE.
           DISPLAY WS-ORD-ID.
           DISPLAY WS-ORD-CUST.
           DISPLAY WS-ORD-AMT.
      * Simple operations
           MOVE "PREPROCESSING OK" TO WS-RESULT.
           DISPLAY WS-RESULT.
           STOP RUN.
