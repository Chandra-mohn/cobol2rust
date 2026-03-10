       IDENTIFICATION DIVISION.
       PROGRAM-ID. SQL-DML-OPS.
      *
      * Test: INSERT, UPDATE, DELETE with host variables.
      * Exercises: DML operations, SQLERRD(3) row count, error handling.
      *
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-EMPNO          PIC 9(6).
       01  WS-ENAME          PIC X(20).
       01  WS-SAL            PIC 9(7)V99.
       01  WS-DEPTNO         PIC 9(4).
       01  WS-NEW-SAL        PIC 9(7)V99.
       01  WS-ROWS-AFFECTED  PIC 9(5).

       EXEC SQL INCLUDE SQLCA END-EXEC.

       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM INSERT-EMPLOYEE.
           PERFORM UPDATE-SALARY.
           PERFORM DELETE-EMPLOYEE.
           STOP RUN.

       INSERT-EMPLOYEE.
           MOVE 999999 TO WS-EMPNO.
           MOVE "TEST EMPLOYEE" TO WS-ENAME.
           MOVE 50000.00 TO WS-SAL.
           MOVE 10 TO WS-DEPTNO.

           EXEC SQL
               INSERT INTO EMP (EMPNO, ENAME, SAL, DEPTNO)
               VALUES (:WS-EMPNO, :WS-ENAME, :WS-SAL, :WS-DEPTNO)
           END-EXEC.

           IF SQLCODE = 0
               DISPLAY "INSERT OK"
           ELSE
               DISPLAY "INSERT FAILED: " SQLCODE
           END-IF.

       UPDATE-SALARY.
           MOVE 55000.00 TO WS-NEW-SAL.
           MOVE 999999 TO WS-EMPNO.

           EXEC SQL
               UPDATE EMP
               SET SAL = :WS-NEW-SAL
               WHERE EMPNO = :WS-EMPNO
           END-EXEC.

           IF SQLCODE = 0
               DISPLAY "UPDATE OK"
           ELSE
               DISPLAY "UPDATE FAILED: " SQLCODE
           END-IF.

       DELETE-EMPLOYEE.
           MOVE 999999 TO WS-EMPNO.

           EXEC SQL
               DELETE FROM EMP
               WHERE EMPNO = :WS-EMPNO
           END-EXEC.

           IF SQLCODE = 0
               DISPLAY "DELETE OK"
           ELSE
               DISPLAY "DELETE FAILED: " SQLCODE
           END-IF.
