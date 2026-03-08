       IDENTIFICATION DIVISION.
       PROGRAM-ID. DISPLAY-ACCEPT-TEST.
      *---------------------------------------------------------------
      * Stress test: DISPLAY and ACCEPT statements.
      * Covers: DISPLAY single, multiple, WITH NO ADVANCING,
      * DISPLAY literal, group, numeric, all types.
      * ACCEPT FROM DATE/TIME/DAY/DAY-OF-WEEK.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-NAME              PIC X(20) VALUE "TEST PROGRAM".
       01  WS-NUM               PIC 9(5) VALUE 12345.
       01  WS-SDEC              PIC S9(5)V99 VALUE -123.45.
       01  WS-PACK              PIC S9(7)V99 COMP-3 VALUE 9999.99.
       01  WS-COMP              PIC S9(9) COMP VALUE 42.
       01  WS-FLOAT             COMP-1 VALUE 3.14.
       01  WS-EDITED            PIC ZZ,ZZ9.99.
       01  WS-GROUP.
           05  WS-GRP-A         PIC X(5) VALUE "HELLO".
           05  WS-GRP-B         PIC 9(3) VALUE 42.
           05  WS-GRP-C         PIC X(5) VALUE "WORLD".
       01  WS-TABLE.
           05  WS-ITEM          PIC X(10) OCCURS 3 TIMES.
       01  WS-DATE-VAL          PIC 9(8).
       01  WS-TIME-VAL          PIC 9(8).
       01  WS-DAY-VAL           PIC 9(7).
       01  WS-DOW-VAL           PIC 9.
       01  WS-DATE-YYYYMMDD     PIC 9(8).
       01  WS-DAY-YYYYDDD       PIC 9(7).
       PROCEDURE DIVISION.
       MAIN-PARA.
      * DISPLAY single alphanumeric
           DISPLAY WS-NAME.
      * DISPLAY literal
           DISPLAY "HELLO WORLD".
      * DISPLAY numeric
           DISPLAY WS-NUM.
      * DISPLAY signed decimal
           DISPLAY WS-SDEC.
      * DISPLAY packed
           DISPLAY WS-PACK.
      * DISPLAY binary
           DISPLAY WS-COMP.
      * DISPLAY float
           DISPLAY WS-FLOAT.
      * DISPLAY edited
           MOVE 12345.67 TO WS-EDITED.
           DISPLAY WS-EDITED.
      * DISPLAY group (shows flat bytes)
           DISPLAY WS-GROUP.
      * DISPLAY multiple items
           DISPLAY "NAME=" WS-NAME " NUM=" WS-NUM.
      * DISPLAY WITH NO ADVANCING
           DISPLAY "NO-ADV: " WITH NO ADVANCING.
           DISPLAY WS-NUM.
      * DISPLAY array element
           MOVE "FIRST" TO WS-ITEM(1).
           MOVE "SECOND" TO WS-ITEM(2).
           MOVE "THIRD" TO WS-ITEM(3).
           DISPLAY WS-ITEM(1).
           DISPLAY WS-ITEM(2).
           DISPLAY WS-ITEM(3).
      * ACCEPT FROM DATE
           ACCEPT WS-DATE-VAL FROM DATE.
           DISPLAY WS-DATE-VAL.
      * ACCEPT FROM TIME
           ACCEPT WS-TIME-VAL FROM TIME.
           DISPLAY WS-TIME-VAL.
      * ACCEPT FROM DAY
           ACCEPT WS-DAY-VAL FROM DAY.
           DISPLAY WS-DAY-VAL.
      * ACCEPT FROM DAY-OF-WEEK
           ACCEPT WS-DOW-VAL FROM DAY-OF-WEEK.
           DISPLAY WS-DOW-VAL.
      * ACCEPT FROM DATE YYYYMMDD
           ACCEPT WS-DATE-YYYYMMDD FROM DATE YYYYMMDD.
           DISPLAY WS-DATE-YYYYMMDD.
      * ACCEPT FROM DAY YYYYDDD
           ACCEPT WS-DAY-YYYYDDD FROM DAY YYYYDDD.
           DISPLAY WS-DAY-YYYYDDD.
           STOP RUN.
