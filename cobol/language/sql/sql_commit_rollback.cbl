       IDENTIFICATION DIVISION.
       PROGRAM-ID. SQL-TXNCTL.
      *
      * Test: COMMIT and ROLLBACK transaction control.
      * Exercises: transaction boundaries, error-driven rollback.
      *
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-EMPNO          PIC 9(6).
       01  WS-ENAME          PIC X(20).
       01  WS-SAL            PIC 9(7)V99.
       01  WS-DEPTNO         PIC 9(4).

       EXEC SQL INCLUDE SQLCA END-EXEC.

       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM INSERT-AND-COMMIT.
           PERFORM INSERT-AND-ROLLBACK.
           STOP RUN.

       INSERT-AND-COMMIT.
           MOVE 888888 TO WS-EMPNO.
           MOVE "COMMITTED EMP" TO WS-ENAME.
           MOVE 40000.00 TO WS-SAL.
           MOVE 20 TO WS-DEPTNO.

           EXEC SQL
               INSERT INTO EMP (EMPNO, ENAME, SAL, DEPTNO)
               VALUES (:WS-EMPNO, :WS-ENAME, :WS-SAL, :WS-DEPTNO)
           END-EXEC.

           IF SQLCODE = 0
               EXEC SQL COMMIT END-EXEC
               DISPLAY "INSERT + COMMIT OK"
           ELSE
               EXEC SQL ROLLBACK END-EXEC
               DISPLAY "INSERT FAILED, ROLLED BACK"
           END-IF.

      * Clean up
           EXEC SQL
               DELETE FROM EMP WHERE EMPNO = :WS-EMPNO
           END-EXEC.
           EXEC SQL COMMIT END-EXEC.

       INSERT-AND-ROLLBACK.
           MOVE 777777 TO WS-EMPNO.
           MOVE "ROLLBACK EMP" TO WS-ENAME.
           MOVE 30000.00 TO WS-SAL.
           MOVE 30 TO WS-DEPTNO.

           EXEC SQL
               INSERT INTO EMP (EMPNO, ENAME, SAL, DEPTNO)
               VALUES (:WS-EMPNO, :WS-ENAME, :WS-SAL, :WS-DEPTNO)
           END-EXEC.

           IF SQLCODE = 0
               EXEC SQL ROLLBACK END-EXEC
               DISPLAY "INSERT THEN ROLLBACK -- ROW SHOULD NOT EXIST"
           ELSE
               DISPLAY "INSERT FAILED: " SQLCODE
           END-IF.
