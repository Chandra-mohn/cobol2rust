       IDENTIFICATION DIVISION.
       PROGRAM-ID. INITIALIZE-SET-TEST.
      *---------------------------------------------------------------
      * Stress test: INITIALIZE and SET statements.
      * Covers: INITIALIZE (default), REPLACING NUMERIC/ALPHA,
      * SET condition TO TRUE, SET index UP BY/DOWN BY.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Group for INITIALIZE
       01  WS-RECORD.
           05  WS-REC-ALPHA     PIC X(10) VALUE "DIRTY DATA".
           05  WS-REC-NUM       PIC 9(5) VALUE 99999.
           05  WS-REC-SDEC      PIC S9(5)V99 VALUE -123.45.
           05  WS-REC-PACK      PIC S9(7)V99 COMP-3 VALUE 9999.99.
           05  WS-REC-COMP      PIC S9(9) COMP VALUE 42.
           05  WS-REC-ALPHA2    PIC X(5) VALUE "HELLO".
      * Nested group for INITIALIZE
       01  WS-NESTED.
           05  WS-N-HDR         PIC X(5) VALUE "HEAD".
           05  WS-N-BODY.
               10  WS-NB-NAME   PIC X(15) VALUE "ALICE".
               10  WS-NB-AMT    PIC 9(7)V99 VALUE 50000.00.
               10  WS-NB-CODE   PIC X(3) VALUE "ABC".
      * Level-88 conditions for SET
       01  WS-STATUS            PIC X(2).
           88  STATUS-ACTIVE    VALUE "AC".
           88  STATUS-INACTIVE  VALUE "IN".
           88  STATUS-PENDING   VALUE "PE".
       01  WS-FLAG              PIC 9.
           88  FLAG-ON          VALUE 1.
           88  FLAG-OFF         VALUE 0.
       01  WS-TYPE              PIC X.
           88  TYPE-A           VALUE "A".
           88  TYPE-B           VALUE "B".
           88  TYPE-C           VALUE "C".
      * Indexed table for SET index
       01  WS-TABLE.
           05  WS-ITEM          PIC X(10) OCCURS 10 TIMES
                                 INDEXED BY WS-IX.
       01  WS-I                 PIC 99.
       01  WS-DISPLAY           PIC X(40).
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Default INITIALIZE - spaces for alpha, zeros for numeric
           INITIALIZE WS-RECORD.
           DISPLAY WS-REC-ALPHA.
           DISPLAY WS-REC-NUM.
           DISPLAY WS-REC-SDEC.
           DISPLAY WS-REC-PACK.
           DISPLAY WS-REC-COMP.
           DISPLAY WS-REC-ALPHA2.
      * Re-fill and INITIALIZE nested group
           MOVE "RESTORED" TO WS-REC-ALPHA.
           MOVE 12345 TO WS-REC-NUM.
           INITIALIZE WS-NESTED.
           DISPLAY WS-N-HDR.
           DISPLAY WS-NB-NAME.
           DISPLAY WS-NB-AMT.
           DISPLAY WS-NB-CODE.
      * INITIALIZE REPLACING NUMERIC BY
           MOVE "ORIGINAL" TO WS-REC-ALPHA.
           MOVE 99999 TO WS-REC-NUM.
           INITIALIZE WS-RECORD
               REPLACING NUMERIC DATA BY 42.
           DISPLAY WS-REC-ALPHA.
           DISPLAY WS-REC-NUM.
      * INITIALIZE REPLACING ALPHANUMERIC BY
           INITIALIZE WS-RECORD
               REPLACING ALPHANUMERIC DATA BY "INIT".
           DISPLAY WS-REC-ALPHA.
           DISPLAY WS-REC-NUM.
      * SET condition TO TRUE
           SET STATUS-ACTIVE TO TRUE.
           IF STATUS-ACTIVE
               DISPLAY "STATUS IS ACTIVE"
           END-IF.
           SET STATUS-PENDING TO TRUE.
           DISPLAY WS-STATUS.
      * SET flag
           SET FLAG-ON TO TRUE.
           IF FLAG-ON
               DISPLAY "FLAG IS ON"
           END-IF.
           SET FLAG-OFF TO TRUE.
           IF FLAG-OFF
               DISPLAY "FLAG IS OFF"
           END-IF.
      * SET type
           SET TYPE-B TO TRUE.
           IF TYPE-B
               DISPLAY "TYPE IS B"
           END-IF.
      * SET index
           SET WS-IX TO 1.
           MOVE "FIRST" TO WS-ITEM(WS-IX).
           SET WS-IX UP BY 1.
           MOVE "SECOND" TO WS-ITEM(WS-IX).
           SET WS-IX UP BY 3.
           MOVE "FIFTH" TO WS-ITEM(WS-IX).
           SET WS-IX DOWN BY 2.
           MOVE "THIRD" TO WS-ITEM(WS-IX).
      * Display results
           DISPLAY WS-ITEM(1).
           DISPLAY WS-ITEM(2).
           DISPLAY WS-ITEM(3).
           DISPLAY WS-ITEM(5).
           STOP RUN.
