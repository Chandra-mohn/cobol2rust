       IDENTIFICATION DIVISION.
       PROGRAM-ID. SQL-INDICATORS.
      *
      * Test: Null indicator variables.
      * Exercises: :HOST :INDICATOR syntax, null detection, truncation.
      *
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-EMPNO          PIC 9(6)     VALUE 100.
       01  WS-ENAME          PIC X(20).
       01  WS-ENAME-IND      PIC S9(4) COMP VALUE 0.
       01  WS-COMM           PIC 9(7)V99.
       01  WS-COMM-IND       PIC S9(4) COMP VALUE 0.

       EXEC SQL INCLUDE SQLCA END-EXEC.

       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 100 TO WS-EMPNO.

           EXEC SQL
               SELECT ENAME, COMM
               INTO :WS-ENAME :WS-ENAME-IND,
                    :WS-COMM :WS-COMM-IND
               FROM EMP
               WHERE EMPNO = :WS-EMPNO
           END-EXEC.

           IF SQLCODE = 0
               IF WS-ENAME-IND < 0
                   DISPLAY "NAME IS NULL"
               ELSE
                   DISPLAY "NAME: " WS-ENAME
               END-IF

               IF WS-COMM-IND < 0
                   DISPLAY "COMMISSION IS NULL"
               ELSE
                   DISPLAY "COMM: " WS-COMM
               END-IF
           ELSE
               DISPLAY "QUERY FAILED: " SQLCODE
           END-IF.

           STOP RUN.
