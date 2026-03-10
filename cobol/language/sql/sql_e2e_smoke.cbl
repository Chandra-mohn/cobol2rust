       IDENTIFICATION DIVISION.
       PROGRAM-ID. SQL-TEST-E2E.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-EMPNO          PIC 9(6)     VALUE 100.
       01  WS-ENAME          PIC X(20).
       01  WS-SAL            PIC 9(7)V99.
       01  WS-DEPTNO         PIC 9(4).
       01  WS-RC             PIC S9(9)    VALUE 0.

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

           DISPLAY "QUERY DONE".
           DISPLAY "ENAME: " WS-ENAME.

           EXEC SQL
               INSERT INTO AUDIT_LOG (ACTION, EMPNO)
               VALUES ('QUERY', :WS-EMPNO)
           END-EXEC.

           DISPLAY "INSERT DONE".

           EXEC SQL COMMIT END-EXEC.

           DISPLAY "COMMIT DONE".
           STOP RUN.
