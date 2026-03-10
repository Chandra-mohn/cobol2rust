       IDENTIFICATION DIVISION.
       PROGRAM-ID. SQL-SELECT-INTO.
      *
      * Test: Basic SELECT INTO with host variables.
      * Exercises: single-row query, input/output host vars, SQLCODE check.
      *
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-EMPNO          PIC 9(6)     VALUE 100.
       01  WS-ENAME          PIC X(20).
       01  WS-SAL            PIC 9(7)V99.
       01  WS-DEPTNO         PIC 9(4).
       01  WS-SQLCODE        PIC S9(9)    VALUE 0.

       EXEC SQL INCLUDE SQLCA END-EXEC.

       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 100 TO WS-EMPNO.

           EXEC SQL
               SELECT ENAME, SAL, DEPTNO
               INTO :WS-ENAME, :WS-SAL, :WS-DEPTNO
               FROM EMP
               WHERE EMPNO = :WS-EMPNO
           END-EXEC.

           IF SQLCODE = 0
               DISPLAY "FOUND: " WS-ENAME
               DISPLAY "SAL:   " WS-SAL
               DISPLAY "DEPT:  " WS-DEPTNO
           ELSE
               IF SQLCODE = 100
                   DISPLAY "EMPLOYEE NOT FOUND"
               ELSE
                   DISPLAY "SQL ERROR: " SQLCODE
               END-IF
           END-IF.

           STOP RUN.
