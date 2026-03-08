       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPOUND-CONDITIONS-TEST.
      *---------------------------------------------------------------
      * Stress test: Compound conditions.
      * Covers: AND, OR, NOT, nested parentheses, implied subjects,
      * mixed condition types, negated compound conditions.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-A                 PIC 9(3) VALUE 50.
       01  WS-B                 PIC 9(3) VALUE 75.
       01  WS-C                 PIC 9(3) VALUE 100.
       01  WS-D                 PIC 9(3) VALUE 25.
       01  WS-STATUS            PIC X(2) VALUE "OK".
       01  WS-FLAG1             PIC 9 VALUE 1.
       01  WS-FLAG2             PIC 9 VALUE 0.
       01  WS-RESULT            PIC X(30).
       01  WS-GRADE             PIC X VALUE "B".
           88  IS-PASS          VALUE "A" "B" "C".
           88  IS-FAIL          VALUE "D" "F".
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple AND
           IF WS-A > 25 AND WS-B > 50
               MOVE "AND TRUE" TO WS-RESULT
           ELSE
               MOVE "AND FALSE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Simple OR
           IF WS-A > 100 OR WS-B > 50
               MOVE "OR TRUE" TO WS-RESULT
           ELSE
               MOVE "OR FALSE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * NOT condition
           IF NOT (WS-A > 100)
               MOVE "NOT TRUE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * AND with OR
           IF WS-A > 25 AND (WS-B > 100 OR WS-C = 100)
               MOVE "AND-OR TRUE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Multiple AND
           IF WS-A > 10 AND WS-B > 10 AND WS-C > 10 AND WS-D > 10
               MOVE "ALL > 10" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Multiple OR
           IF WS-A = 50 OR WS-B = 50 OR WS-C = 50
               MOVE "ONE IS 50" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Complex parenthesized
           IF (WS-A > 25 AND WS-B > 50)
               OR (WS-C > 75 AND WS-D < 50)
               MOVE "COMPLEX TRUE" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * NOT compound
           IF NOT (WS-A > 100 AND WS-B > 100)
               MOVE "NOT COMPOUND" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Implied subject (A = 50 OR 75)
           IF WS-A = 50 OR 75
               MOVE "IMPLIED SUBJ" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * 88-level in compound
           IF IS-PASS AND WS-A > 25
               MOVE "PASS AND > 25" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Mixed flag conditions
           IF WS-FLAG1 = 1 AND WS-FLAG2 = 0
               MOVE "FLAGS MATCH" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * String comparison in compound
           IF WS-STATUS = "OK" AND WS-A > 0
               MOVE "STATUS OK A>0" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
      * Triple nested
           IF (WS-A > 25 OR WS-B > 25)
               AND (WS-C > 50 OR WS-D > 50)
               AND WS-STATUS = "OK"
               MOVE "TRIPLE PASS" TO WS-RESULT
           END-IF.
           DISPLAY WS-RESULT.
           STOP RUN.
