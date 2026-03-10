       IDENTIFICATION DIVISION.
       PROGRAM-ID. SQL-CURSOR.
      *
      * Test: DECLARE CURSOR, OPEN, FETCH loop, CLOSE.
      * Exercises: cursor lifecycle, multi-row fetch, end-of-data (100).
      *
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-EMPNO          PIC 9(6).
       01  WS-ENAME          PIC X(20).
       01  WS-SAL            PIC 9(7)V99.
       01  WS-DEPTNO         PIC 9(4)     VALUE 10.
       01  WS-ROW-COUNT      PIC 9(5)     VALUE 0.

       EXEC SQL INCLUDE SQLCA END-EXEC.

       EXEC SQL
           DECLARE EMP-CURSOR CURSOR FOR
               SELECT EMPNO, ENAME, SAL
               FROM EMP
               WHERE DEPTNO = :WS-DEPTNO
               ORDER BY EMPNO
       END-EXEC.

       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 10 TO WS-DEPTNO.
           PERFORM OPEN-CURSOR.
           PERFORM FETCH-LOOP UNTIL SQLCODE NOT = 0.
           PERFORM CLOSE-CURSOR.
           DISPLAY "TOTAL ROWS: " WS-ROW-COUNT.
           STOP RUN.

       OPEN-CURSOR.
           EXEC SQL OPEN EMP-CURSOR END-EXEC.
           IF SQLCODE NOT = 0
               DISPLAY "OPEN FAILED: " SQLCODE
               STOP RUN
           END-IF.

       FETCH-LOOP.
           EXEC SQL
               FETCH EMP-CURSOR
               INTO :WS-EMPNO, :WS-ENAME, :WS-SAL
           END-EXEC.

           IF SQLCODE = 0
               ADD 1 TO WS-ROW-COUNT
               DISPLAY WS-EMPNO " " WS-ENAME " " WS-SAL
           ELSE
               IF SQLCODE NOT = 100
                   DISPLAY "FETCH ERROR: " SQLCODE
               END-IF
           END-IF.

       CLOSE-CURSOR.
           EXEC SQL CLOSE EMP-CURSOR END-EXEC.
