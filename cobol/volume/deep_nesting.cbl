       IDENTIFICATION DIVISION.
       PROGRAM-ID. DEEP-NESTING-TEST.
      *---------------------------------------------------------------
      * Volume stress test: Deeply nested structures (20+ levels).
      * Tests transpiler handling of:
      *   - Data hierarchy 22 levels deep (levels 01 through 22)
      *   - Nested IF statements 12 levels deep
      *   - EVALUATE inside nested IF inside PERFORM
      *   - PERFORM VARYING nested 4 levels deep
      *   - Assignment and retrieval of deeply nested fields
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      *---------------------------------------------------------------
      * 22-level deep record hierarchy (01 -> 02 -> ... -> 22)
      * COBOL allows levels 01-49 for data items.
      *---------------------------------------------------------------
       01  WS-DEEP-RECORD.
         02  WS-D02.
           03  WS-D03.
             04  WS-D04.
               05  WS-D05.
                 06  WS-D06.
                   07  WS-D07.
                     08  WS-D08.
                       09  WS-D09.
                         10  WS-D10.
                           11  WS-D11.
                             12  WS-D12.
                               13  WS-D13.
                                 14  WS-D14.
                                   15  WS-D15.
                                     16  WS-D16.
                                       17  WS-D17.
                                         18  WS-D18.
                                           19  WS-D19.
                                             20  WS-D20.
                                               21  WS-D21.
                                                 22  WS-LEAF
                                                   PIC X(10).
      *---------------------------------------------------------------
      * Second deep record with numeric leaf nodes
      *---------------------------------------------------------------
       01  WS-NUM-DEEP.
         02  WS-N02.
           03  WS-N03.
             04  WS-N04.
               05  WS-N05.
                 06  WS-N06.
                   07  WS-N07.
                     08  WS-N08.
                       09  WS-N09.
                         10  WS-N10.
                           11  WS-N11.
                             12  WS-N12.
                               13  WS-N13.
                                 14  WS-N14.
                                   15  WS-N15.
                                     16  WS-N16.
                                       17  WS-N17.
                                         18  WS-N18.
                                           19  WS-N19.
                                             20  WS-N20
                                               PIC 9(5).
      *---------------------------------------------------------------
      * Third deep record: mixed types at various depths
      *---------------------------------------------------------------
       01  WS-MIXED-DEEP.
         02  WS-M-GRP-A.
           03  WS-M-GRP-B.
             04  WS-M-GRP-C.
               05  WS-M-GRP-D.
                 06  WS-M-GRP-E.
                   07  WS-M-GRP-F.
                     08  WS-M-GRP-G.
                       09  WS-M-GRP-H.
                         10  WS-M-GRP-I.
                           11  WS-M-GRP-J.
                             12  WS-M-ALPHA
                                 PIC A(8).
                             12  WS-M-NUM
                                 PIC 9(7)V99.
                             12  WS-M-EDITED
                                 PIC Z(5)9.99.
                           11  WS-M-FLAG
                               PIC X.
                             12  WS-M-FLAG-V
                                 PIC X.
                               13  WS-M-FLAG-W
                                   PIC X.
                                 14  WS-M-FLAG-X
                                     PIC X.
                                   15  WS-M-FLAG-Y
                                       PIC X.
                                     16  WS-M-DEEPFLAG
                                         PIC 9.
       88  WS-M-DEEP-ON    VALUE 1.
       88  WS-M-DEEP-OFF   VALUE 0.
      *---------------------------------------------------------------
      * Loop and result variables
      *---------------------------------------------------------------
       01  WS-I                    PIC 99.
       01  WS-J                    PIC 99.
       01  WS-K                    PIC 99.
       01  WS-L                    PIC 99.
       01  WS-DEPTH               PIC 99 VALUE 0.
       01  WS-NEST-COUNT          PIC 9(5) VALUE 0.
       01  WS-RESULT              PIC X(40).
       01  WS-STATUS              PIC X(20).
       01  WS-ACCUMULATOR         PIC 9(7) VALUE 0.
       01  WS-TEMP                PIC 9(5) VALUE 0.
       01  WS-CATEGORY            PIC X(10).
       01  WS-INNER-VAL           PIC 9(3) VALUE 0.
       01  WS-PASS-COUNT          PIC 9(5) VALUE 0.
       01  WS-FAIL-COUNT          PIC 9(5) VALUE 0.
      *---------------------------------------------------------------
      * Table for nested PERFORM tests
      *---------------------------------------------------------------
       01  WS-4D-TABLE.
         02  WS-DIM1 OCCURS 3 TIMES.
           03  WS-DIM2 OCCURS 3 TIMES.
             04  WS-DIM3 OCCURS 3 TIMES.
               05  WS-DIM4 OCCURS 3 TIMES.
                 06  WS-CELL    PIC 9(4).
       PROCEDURE DIVISION.
       MAIN-PARA.
           DISPLAY "=== DEEP NESTING TEST START ===".
           PERFORM INIT-DEEP-DATA.
           PERFORM TEST-DEEP-ACCESS.
           PERFORM TEST-NESTED-IFS.
           PERFORM TEST-NESTED-PERFORM.
           PERFORM TEST-EVAL-IN-NEST.
           DISPLAY "=== RESULTS ===".
           DISPLAY "PASS: " WS-PASS-COUNT.
           DISPLAY "FAIL: " WS-FAIL-COUNT.
           DISPLAY "=== DEEP NESTING TEST END ===".
           STOP RUN.
      *---------------------------------------------------------------
      * Initialize deeply nested fields
      *---------------------------------------------------------------
       INIT-DEEP-DATA.
           MOVE "DEEP-LEAF!" TO WS-LEAF.
           MOVE 12345 TO WS-N20.
           MOVE "ALPHABET" TO WS-M-ALPHA.
           MOVE 9876543.21 TO WS-M-NUM.
           MOVE 1 TO WS-M-DEEPFLAG.
           MOVE "A" TO WS-M-FLAG-V.
           MOVE "B" TO WS-M-FLAG-W.
           MOVE "C" TO WS-M-FLAG-X.
           MOVE "D" TO WS-M-FLAG-Y.
           INITIALIZE WS-4D-TABLE.
      *---------------------------------------------------------------
      * Test access to deeply nested fields
      *---------------------------------------------------------------
       TEST-DEEP-ACCESS.
           DISPLAY "--- Deep Field Access ---".
      * Read back the 22-level deep leaf
           IF WS-LEAF = "DEEP-LEAF!"
               DISPLAY "PASS: 22-level leaf OK"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: 22-level leaf"
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
      * Read back the 20-level numeric leaf
           IF WS-N20 = 12345
               DISPLAY "PASS: 20-level numeric OK"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: 20-level numeric"
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
      * Read back a 16-level deep flag
           IF WS-M-DEEP-ON
               DISPLAY "PASS: 16-level flag OK"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: 16-level flag"
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
      * Access sibling at level 12 in mixed tree
           IF WS-M-ALPHA = "ALPHABET"
               DISPLAY "PASS: 12-level alpha OK"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: 12-level alpha"
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
      *---------------------------------------------------------------
      * 12-level deep nested IF
      *---------------------------------------------------------------
       TEST-NESTED-IFS.
           DISPLAY "--- Nested IF (12 deep) ---".
           MOVE 50 TO WS-DEPTH.
           IF WS-DEPTH > 0
            IF WS-DEPTH > 5
             IF WS-DEPTH > 10
              IF WS-DEPTH > 15
               IF WS-DEPTH > 20
                IF WS-DEPTH > 25
                 IF WS-DEPTH > 30
                  IF WS-DEPTH > 35
                   IF WS-DEPTH > 40
                    IF WS-DEPTH > 42
                     IF WS-DEPTH > 45
                      IF WS-DEPTH > 48
                       MOVE "ALL-PASS" TO WS-RESULT
                       DISPLAY "PASS: IF-12"
                       ADD 1 TO WS-PASS-COUNT
                      ELSE
                       MOVE "FAIL-12" TO WS-RESULT
                       ADD 1 TO WS-FAIL-COUNT
                      END-IF
                     ELSE
                      MOVE "FAIL-11" TO WS-RESULT
                      ADD 1 TO WS-FAIL-COUNT
                     END-IF
                    ELSE
                     MOVE "FAIL-10" TO WS-RESULT
                     ADD 1 TO WS-FAIL-COUNT
                    END-IF
                   ELSE
                    MOVE "FAIL-9" TO WS-RESULT
                    ADD 1 TO WS-FAIL-COUNT
                   END-IF
                  ELSE
                   MOVE "FAIL-8" TO WS-RESULT
                   ADD 1 TO WS-FAIL-COUNT
                  END-IF
                 ELSE
                  MOVE "FAIL-7" TO WS-RESULT
                  ADD 1 TO WS-FAIL-COUNT
                 END-IF
                ELSE
                 MOVE "FAIL-6" TO WS-RESULT
                 ADD 1 TO WS-FAIL-COUNT
                END-IF
               ELSE
                MOVE "FAIL-5" TO WS-RESULT
                ADD 1 TO WS-FAIL-COUNT
               END-IF
              ELSE
               MOVE "FAIL-4" TO WS-RESULT
               ADD 1 TO WS-FAIL-COUNT
              END-IF
             ELSE
              MOVE "FAIL-3" TO WS-RESULT
              ADD 1 TO WS-FAIL-COUNT
             END-IF
            ELSE
             MOVE "FAIL-2" TO WS-RESULT
             ADD 1 TO WS-FAIL-COUNT
            END-IF
           ELSE
            MOVE "FAIL-1" TO WS-RESULT
            ADD 1 TO WS-FAIL-COUNT
           END-IF.
           DISPLAY "Nested IF result: " WS-RESULT.
      * Second nested IF with mixed conditions
           MOVE 7 TO WS-INNER-VAL.
           IF WS-INNER-VAL > 0
            IF WS-INNER-VAL < 99
             IF WS-INNER-VAL NOT = 0
              IF WS-INNER-VAL >= 5
               IF WS-INNER-VAL <= 10
                IF WS-INNER-VAL NOT < 7
                 IF WS-INNER-VAL NOT > 7
                  IF WS-INNER-VAL = 7
                   IF WS-M-DEEP-ON
                    IF WS-LEAF = "DEEP-LEAF!"
                     MOVE "MIX-OK" TO WS-STATUS
                     DISPLAY "PASS: Mixed IF"
                     ADD 1 TO WS-PASS-COUNT
                    END-IF
                   END-IF
                  END-IF
                 END-IF
                END-IF
               END-IF
              END-IF
             END-IF
            END-IF
           END-IF.
      *---------------------------------------------------------------
      * 4-level nested PERFORM VARYING
      *---------------------------------------------------------------
       TEST-NESTED-PERFORM.
           DISPLAY "--- Nested PERFORM (4 deep) ---".
           MOVE 0 TO WS-ACCUMULATOR.
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 3
               PERFORM VARYING WS-J FROM 1 BY 1
                   UNTIL WS-J > 3
                   PERFORM VARYING WS-K FROM 1 BY 1
                       UNTIL WS-K > 3
                       PERFORM VARYING WS-L FROM 1
                           BY 1 UNTIL WS-L > 3
                           COMPUTE WS-CELL(WS-I,
                               WS-J, WS-K, WS-L) =
                               WS-I * 1000
                               + WS-J * 100
                               + WS-K * 10
                               + WS-L
                           ADD 1 TO
                               WS-ACCUMULATOR
                       END-PERFORM
                   END-PERFORM
               END-PERFORM
           END-PERFORM.
      * Should be 3^4 = 81 iterations
           IF WS-ACCUMULATOR = 81
               DISPLAY "PASS: 4-deep PERFORM (81)"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: 4-deep PERFORM: "
                   WS-ACCUMULATOR
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
      * Verify a specific cell value
           IF WS-CELL(2, 3, 1, 2) = 2312
               DISPLAY "PASS: Cell(2,3,1,2)"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: Cell(2,3,1,2)="
                   WS-CELL(2, 3, 1, 2)
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
      * Verify corner cells
           IF WS-CELL(1, 1, 1, 1) = 1111
               DISPLAY "PASS: Cell(1,1,1,1)"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: Cell(1,1,1,1)="
                   WS-CELL(1, 1, 1, 1)
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
           IF WS-CELL(3, 3, 3, 3) = 3333
               DISPLAY "PASS: Cell(3,3,3,3)"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: Cell(3,3,3,3)="
                   WS-CELL(3, 3, 3, 3)
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
      *---------------------------------------------------------------
      * EVALUATE inside nested IF inside PERFORM
      *---------------------------------------------------------------
       TEST-EVAL-IN-NEST.
           DISPLAY "--- EVALUATE in Nested IF ---".
           PERFORM VARYING WS-I FROM 1 BY 1
               UNTIL WS-I > 5
               IF WS-I NOT = 0
                IF WS-I <= 5
                 IF WS-I > 0
                  EVALUATE TRUE
                   WHEN WS-I = 1
                    IF WS-M-DEEP-ON
                     MOVE "CAT-A" TO WS-CATEGORY
                    END-IF
                   WHEN WS-I = 2
                    IF WS-N20 > 10000
                     IF WS-N20 < 20000
                      MOVE "CAT-B"
                          TO WS-CATEGORY
                     END-IF
                    END-IF
                   WHEN WS-I = 3
                    EVALUATE WS-LEAF
                     WHEN "DEEP-LEAF!"
                      MOVE "CAT-C"
                          TO WS-CATEGORY
                     WHEN OTHER
                      MOVE "CAT-X"
                          TO WS-CATEGORY
                    END-EVALUATE
                   WHEN WS-I = 4
                    IF WS-M-ALPHA = "ALPHABET"
                     IF WS-M-FLAG-V = "A"
                      IF WS-M-FLAG-W = "B"
                       MOVE "CAT-D"
                           TO WS-CATEGORY
                      END-IF
                     END-IF
                    END-IF
                   WHEN WS-I = 5
                    IF WS-DEPTH > 0
                     EVALUATE TRUE
                      WHEN WS-DEPTH > 40
                       MOVE "CAT-E"
                           TO WS-CATEGORY
                      WHEN WS-DEPTH > 20
                       MOVE "CAT-F"
                           TO WS-CATEGORY
                      WHEN OTHER
                       MOVE "CAT-G"
                           TO WS-CATEGORY
                     END-EVALUATE
                    END-IF
                   WHEN OTHER
                    MOVE "CAT-Z"
                        TO WS-CATEGORY
                  END-EVALUATE
                  DISPLAY "  I=" WS-I
                      " CAT=" WS-CATEGORY
                 END-IF
                END-IF
               END-IF
           END-PERFORM.
      * Verify final category (I=5, DEPTH=50)
           IF WS-CATEGORY = "CAT-E"
               DISPLAY "PASS: Eval-in-nest"
               ADD 1 TO WS-PASS-COUNT
           ELSE
               DISPLAY "FAIL: Eval-in-nest="
                   WS-CATEGORY
               ADD 1 TO WS-FAIL-COUNT
           END-IF.
