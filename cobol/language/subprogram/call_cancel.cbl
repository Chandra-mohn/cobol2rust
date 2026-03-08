       IDENTIFICATION DIVISION.
       PROGRAM-ID. CALL-CANCEL-TEST.
      *---------------------------------------------------------------
      * Stress test: CALL and CANCEL statements.
      * Covers: CALL literal, BY REFERENCE, BY CONTENT, BY VALUE,
      * ON EXCEPTION, NOT ON EXCEPTION, CANCEL, RETURNING.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-PARAM1            PIC X(20) VALUE "HELLO".
       01  WS-PARAM2            PIC 9(5) VALUE 12345.
       01  WS-PARAM3            PIC S9(7)V99 COMP-3 VALUE 100.50.
       01  WS-RETURN-CODE       PIC S9(4) COMP VALUE 0.
       01  WS-RESULT            PIC X(30).
       01  WS-ERR-FLAG          PIC 9 VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
      * Simple CALL by reference (default)
           CALL "SUBPROG1" USING WS-PARAM1 WS-PARAM2.
           DISPLAY WS-PARAM1.
           DISPLAY WS-PARAM2.
      * CALL BY CONTENT (read-only copy)
           CALL "SUBPROG2" USING
               BY CONTENT WS-PARAM1
               BY CONTENT WS-PARAM2.
           DISPLAY WS-PARAM1.
      * CALL BY REFERENCE explicitly
           CALL "SUBPROG3" USING
               BY REFERENCE WS-PARAM1
               BY REFERENCE WS-PARAM2
               BY REFERENCE WS-PARAM3.
           DISPLAY WS-PARAM3.
      * CALL with ON EXCEPTION
           CALL "NONEXISTENT" USING WS-PARAM1
               ON EXCEPTION
                   MOVE 1 TO WS-ERR-FLAG
                   MOVE "CALL FAILED" TO WS-RESULT
               NOT ON EXCEPTION
                   MOVE 0 TO WS-ERR-FLAG
                   MOVE "CALL OK" TO WS-RESULT
           END-CALL.
           DISPLAY WS-ERR-FLAG.
           DISPLAY WS-RESULT.
      * CALL with RETURNING
           CALL "SUBPROG4" USING WS-PARAM1
               RETURNING WS-RETURN-CODE.
           DISPLAY WS-RETURN-CODE.
      * CANCEL subprogram
           CANCEL "SUBPROG1".
      * Call again after CANCEL (reinitializes)
           CALL "SUBPROG1" USING WS-PARAM1 WS-PARAM2.
           DISPLAY WS-PARAM1.
           STOP RUN.
