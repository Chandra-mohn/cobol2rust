       IDENTIFICATION DIVISION.
       PROGRAM-ID. STRING-UNSTRING-TEST.
      *---------------------------------------------------------------
      * Stress test: STRING and UNSTRING statements.
      * Covers: STRING with DELIMITED BY, multiple sources,
      * POINTER, OVERFLOW. UNSTRING with DELIMITED BY,
      * COUNT IN, TALLYING IN, multiple targets.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-FIRST             PIC X(10) VALUE "JOHN".
       01  WS-MIDDLE            PIC X(2) VALUE "Q".
       01  WS-LAST              PIC X(15) VALUE "PUBLIC".
       01  WS-RESULT            PIC X(40) VALUE SPACES.
       01  WS-PTR               PIC 99 VALUE 1.
       01  WS-OVERFLOW-FLAG     PIC 9 VALUE 0.
      * UNSTRING fields
       01  WS-CSV-LINE          PIC X(50).
       01  WS-FIELD1            PIC X(10).
       01  WS-FIELD2            PIC X(10).
       01  WS-FIELD3            PIC X(10).
       01  WS-FIELD4            PIC X(10).
       01  WS-COUNT1            PIC 99 VALUE 0.
       01  WS-COUNT2            PIC 99 VALUE 0.
       01  WS-COUNT3            PIC 99 VALUE 0.
       01  WS-DELIM1            PIC X VALUE SPACES.
       01  WS-DELIM2            PIC X VALUE SPACES.
       01  WS-TALLY-COUNT       PIC 99 VALUE 0.
       01  WS-UPTR              PIC 99 VALUE 1.
       01  WS-SHORT             PIC X(10) VALUE SPACES.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple STRING
           MOVE SPACES TO WS-RESULT.
           STRING WS-FIRST DELIMITED BY SPACES
               " " DELIMITED BY SIZE
               WS-LAST DELIMITED BY SPACES
               INTO WS-RESULT.
           DISPLAY WS-RESULT.
      * STRING with POINTER
           MOVE SPACES TO WS-RESULT.
           MOVE 1 TO WS-PTR.
           STRING "HELLO" DELIMITED BY SIZE
               " " DELIMITED BY SIZE
               "WORLD" DELIMITED BY SIZE
               INTO WS-RESULT
               WITH POINTER WS-PTR.
           DISPLAY WS-RESULT.
           DISPLAY WS-PTR.
      * STRING with OVERFLOW
           MOVE SPACES TO WS-SHORT.
           STRING "THIS IS A VERY LONG STRING" DELIMITED BY SIZE
               INTO WS-SHORT
               ON OVERFLOW
                   MOVE 1 TO WS-OVERFLOW-FLAG
               NOT ON OVERFLOW
                   MOVE 0 TO WS-OVERFLOW-FLAG
           END-STRING.
           DISPLAY WS-OVERFLOW-FLAG.
           DISPLAY WS-SHORT.
      * Multiple strings concatenated
           MOVE SPACES TO WS-RESULT.
           STRING WS-FIRST DELIMITED BY SPACES
               ", " DELIMITED BY SIZE
               WS-MIDDLE DELIMITED BY SPACES
               ". " DELIMITED BY SIZE
               WS-LAST DELIMITED BY SPACES
               INTO WS-RESULT.
           DISPLAY WS-RESULT.
      * Simple UNSTRING
           MOVE "ALICE,BOB,CHARLIE" TO WS-CSV-LINE.
           UNSTRING WS-CSV-LINE DELIMITED BY ","
               INTO WS-FIELD1 WS-FIELD2 WS-FIELD3.
           DISPLAY WS-FIELD1.
           DISPLAY WS-FIELD2.
           DISPLAY WS-FIELD3.
      * UNSTRING with COUNT IN
           MOVE "HELLO,WORLD,TEST" TO WS-CSV-LINE.
           UNSTRING WS-CSV-LINE DELIMITED BY ","
               INTO WS-FIELD1 COUNT IN WS-COUNT1
                    WS-FIELD2 COUNT IN WS-COUNT2
                    WS-FIELD3 COUNT IN WS-COUNT3.
           DISPLAY WS-COUNT1.
           DISPLAY WS-COUNT2.
           DISPLAY WS-COUNT3.
      * UNSTRING with DELIMITER IN
           MOVE "A|B,C" TO WS-CSV-LINE.
           UNSTRING WS-CSV-LINE DELIMITED BY "|" OR ","
               INTO WS-FIELD1 DELIMITER IN WS-DELIM1
                    WS-FIELD2 DELIMITER IN WS-DELIM2
                    WS-FIELD3.
           DISPLAY WS-FIELD1.
           DISPLAY WS-DELIM1.
           DISPLAY WS-FIELD2.
           DISPLAY WS-DELIM2.
      * UNSTRING with TALLYING IN
           MOVE "ONE,TWO,THREE,FOUR" TO WS-CSV-LINE.
           MOVE 0 TO WS-TALLY-COUNT.
           UNSTRING WS-CSV-LINE DELIMITED BY ","
               INTO WS-FIELD1 WS-FIELD2 WS-FIELD3 WS-FIELD4
               TALLYING IN WS-TALLY-COUNT.
           DISPLAY WS-TALLY-COUNT.
      * UNSTRING with POINTER
           MOVE "ABCDEFGHIJ" TO WS-CSV-LINE.
           MOVE 4 TO WS-UPTR.
           UNSTRING WS-CSV-LINE DELIMITED BY SIZE
               INTO WS-FIELD1
               WITH POINTER WS-UPTR.
           DISPLAY WS-FIELD1.
           DISPLAY WS-UPTR.
           STOP RUN.
