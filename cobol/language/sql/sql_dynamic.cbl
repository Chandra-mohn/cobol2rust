       IDENTIFICATION DIVISION.
       PROGRAM-ID. SQL-DYNAMIC.
      *
      * Test: Dynamic SQL (PREPARE, EXECUTE, EXECUTE IMMEDIATE).
      * Exercises: runtime-built SQL strings, parameterized execution.
      *
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-SQL-TEXT       PIC X(200).
       01  WS-EMPNO          PIC 9(6)     VALUE 100.
       01  WS-ENAME          PIC X(20).
       01  WS-COUNT          PIC 9(5).

       EXEC SQL INCLUDE SQLCA END-EXEC.

       PROCEDURE DIVISION.
       MAIN-PARA.
           PERFORM TEST-EXECUTE-IMMEDIATE.
           PERFORM TEST-PREPARE-EXECUTE.
           STOP RUN.

       TEST-EXECUTE-IMMEDIATE.
           MOVE "DELETE FROM TEMP_TABLE WHERE STATUS = 'X'"
               TO WS-SQL-TEXT.

           EXEC SQL
               EXECUTE IMMEDIATE :WS-SQL-TEXT
           END-EXEC.

           IF SQLCODE = 0
               DISPLAY "EXECUTE IMMEDIATE OK"
           ELSE
               DISPLAY "EXECUTE IMMEDIATE: " SQLCODE
           END-IF.

       TEST-PREPARE-EXECUTE.
           MOVE "SELECT ENAME FROM EMP WHERE EMPNO = ?"
               TO WS-SQL-TEXT.

           EXEC SQL
               PREPARE STMT1 FROM :WS-SQL-TEXT
           END-EXEC.

           IF SQLCODE NOT = 0
               DISPLAY "PREPARE FAILED: " SQLCODE
               GO TO TEST-PREPARE-EXECUTE-EXIT
           END-IF.

           MOVE 100 TO WS-EMPNO.

           EXEC SQL
               EXECUTE STMT1 USING :WS-EMPNO
           END-EXEC.

           IF SQLCODE = 0
               DISPLAY "EXECUTE OK"
           ELSE
               DISPLAY "EXECUTE FAILED: " SQLCODE
           END-IF.

       TEST-PREPARE-EXECUTE-EXIT.
           EXIT.
