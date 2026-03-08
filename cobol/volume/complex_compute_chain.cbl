      *================================================================*
      * PROGRAM:  COMPLEX-COMPUTE-TEST
      * PURPOSE:  Volume stress test for COBOL-to-Rust transpiler.
      *           Tests 50+ COMPUTE expressions with increasing
      *           complexity: simple arithmetic, nested parens,
      *           exponentiation, chained results, mixed numeric
      *           types (DISPLAY, COMP, COMP-3, COMP-1, COMP-2),
      *           ROUNDED, and ON SIZE ERROR handling.
      *================================================================*
       IDENTIFICATION DIVISION.
       PROGRAM-ID. COMPLEX-COMPUTE-TEST.

       DATA DIVISION.
       WORKING-STORAGE SECTION.

      *----------------------------------------------------------------*
      * SOURCE FIELDS: WS-V01 THROUGH WS-V60
      * Mix of PIC 9, S9, V99, COMP, COMP-3, COMP-1, COMP-2
      *----------------------------------------------------------------*
       01  WS-V01          PIC 9(5)        VALUE 100.
       01  WS-V02          PIC 9(5)        VALUE 250.
       01  WS-V03          PIC 9(5)        VALUE 33.
       01  WS-V04          PIC 9(5)        VALUE 7.
       01  WS-V05          PIC S9(5)       VALUE -15.
       01  WS-V06          PIC S9(5)       VALUE 42.
       01  WS-V07          PIC 9(3)V99     VALUE 12.50.
       01  WS-V08          PIC 9(3)V99     VALUE 3.75.
       01  WS-V09          PIC S9(5)V99    VALUE -100.25.
       01  WS-V10          PIC S9(5)V99    VALUE 999.99.
       01  WS-V11          PIC 9(5)        COMP VALUE 500.
       01  WS-V12          PIC 9(5)        COMP VALUE 123.
       01  WS-V13          PIC S9(5)       COMP VALUE -200.
       01  WS-V14          PIC 9(7)        COMP VALUE 10000.
       01  WS-V15          PIC S9(7)       COMP VALUE -5000.
       01  WS-V16          PIC 9(5)        COMP-3 VALUE 300.
       01  WS-V17          PIC 9(5)        COMP-3 VALUE 77.
       01  WS-V18          PIC S9(5)       COMP-3 VALUE -88.
       01  WS-V19          PIC 9(5)V99     COMP-3 VALUE 45.67.
       01  WS-V20          PIC S9(5)V99    COMP-3 VALUE -23.45.
       01  WS-V21          COMP-1          VALUE 3.14.
       01  WS-V22          COMP-1          VALUE 2.718.
       01  WS-V23          COMP-2          VALUE 1.41421356.
       01  WS-V24          COMP-2          VALUE 2.71828183.
       01  WS-V25          PIC 9(3)        VALUE 8.
       01  WS-V26          PIC 9(3)        VALUE 15.
       01  WS-V27          PIC 9(3)V99     VALUE 50.00.
       01  WS-V28          PIC S9(3)V99    VALUE -25.50.
       01  WS-V29          PIC 9(5)        COMP VALUE 1024.
       01  WS-V30          PIC S9(5)       COMP VALUE -512.
       01  WS-V31          PIC 9(5)        VALUE 200.
       01  WS-V32          PIC 9(5)        VALUE 350.
       01  WS-V33          PIC S9(5)       VALUE -75.
       01  WS-V34          PIC 9(3)V99     VALUE 99.99.
       01  WS-V35          PIC S9(5)V99    VALUE 123.45.
       01  WS-V36          PIC 9(5)        COMP VALUE 2048.
       01  WS-V37          PIC S9(5)       COMP VALUE -1000.
       01  WS-V38          PIC 9(5)        COMP-3 VALUE 555.
       01  WS-V39          PIC S9(5)V99    COMP-3 VALUE -67.89.
       01  WS-V40          PIC 9(7)V99     COMP-3 VALUE 12345.67.
       01  WS-V41          COMP-1          VALUE 0.5.
       01  WS-V42          COMP-2          VALUE 100.001.
       01  WS-V43          PIC 9(3)        VALUE 10.
       01  WS-V44          PIC 9(3)        VALUE 20.
       01  WS-V45          PIC S9(3)       VALUE -5.
       01  WS-V46          PIC 9(5)V99     VALUE 750.25.
       01  WS-V47          PIC S9(5)V99    VALUE -333.33.
       01  WS-V48          PIC 9(5)        COMP VALUE 4096.
       01  WS-V49          PIC S9(7)       COMP VALUE -8192.
       01  WS-V50          PIC 9(7)        COMP-3 VALUE 65535.
       01  WS-V51          PIC 9(3)V99     VALUE 1.00.
       01  WS-V52          PIC 9(3)V99     VALUE 2.00.
       01  WS-V53          PIC S9(5)       VALUE -1.
       01  WS-V54          PIC 9(5)        VALUE 999.
       01  WS-V55          PIC 9(5)        COMP VALUE 256.
       01  WS-V56          PIC S9(5)       COMP-3 VALUE -400.
       01  WS-V57          PIC 9(3)V9(4)   VALUE 3.1415.
       01  WS-V58          PIC S9(3)V9(4)  VALUE -2.7182.
       01  WS-V59          COMP-1          VALUE 1.0.
       01  WS-V60          COMP-2          VALUE 0.00001.

      *----------------------------------------------------------------*
      * RESULT FIELDS: WS-RESULT-01 THROUGH WS-RESULT-50
      *----------------------------------------------------------------*
       01  WS-RESULT-01     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-02     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-03     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-04     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-05     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-06     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-07     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-08     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-09     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-10     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-11     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-12     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-13     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-14     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-15     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-16     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-17     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-18     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-19     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-20     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-21     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-22     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-23     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-24     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-25     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-26     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-27     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-28     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-29     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-30     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-31     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-32     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-33     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-34     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-35     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-36     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-37     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-38     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-39     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-40     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-41     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-42     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-43     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-44     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-45     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-46     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-47     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-48     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-49     PIC S9(9)V99   VALUE 0.
       01  WS-RESULT-50     PIC S9(9)V99   VALUE 0.

      * SIZE ERROR FLAG
       01  WS-SIZE-ERR      PIC 9          VALUE 0.

       PROCEDURE DIVISION.
       MAIN-LOGIC.

      *================================================================*
      * SIMPLE ARITHMETIC (01-10)
      *================================================================*

      * 01: Simple addition
           COMPUTE WS-RESULT-01 = WS-V01 + WS-V02.

      * 02: Simple subtraction
           COMPUTE WS-RESULT-02 = WS-V02 - WS-V01.

      * 03: Simple multiplication
           COMPUTE WS-RESULT-03 = WS-V03 * WS-V04.

      * 04: Simple division
           COMPUTE WS-RESULT-04 = WS-V01 / WS-V04.

      * 05: Addition with signed fields
           COMPUTE WS-RESULT-05 = WS-V05 + WS-V06.

      * 06: Subtraction yielding negative
           COMPUTE WS-RESULT-06 = WS-V05 - WS-V06.

      * 07: Multiply decimals
           COMPUTE WS-RESULT-07 = WS-V07 * WS-V08.

      * 08: Divide decimals
           COMPUTE WS-RESULT-08 = WS-V07 / WS-V08.

      * 09: Add signed decimal
           COMPUTE WS-RESULT-09 = WS-V09 + WS-V10.

      * 10: Subtract signed decimal
           COMPUTE WS-RESULT-10 = WS-V10 - WS-V09.

      *================================================================*
      * MEDIUM COMPLEXITY (11-20)
      *================================================================*

      * 11: Three-term addition
           COMPUTE WS-RESULT-11 =
               WS-V01 + WS-V02 + WS-V03.

      * 12: Mixed add/subtract
           COMPUTE WS-RESULT-12 =
               WS-V01 + WS-V02 - WS-V03.

      * 13: Multiply then add
           COMPUTE WS-RESULT-13 =
               WS-V03 * WS-V04 + WS-V01.

      * 14: Divide then subtract
           COMPUTE WS-RESULT-14 =
               WS-V14 / WS-V04 - WS-V01.

      * 15: Parenthesized addition times factor
           COMPUTE WS-RESULT-15 =
               (WS-V01 + WS-V02) * WS-V03.

      * 16: Parenthesized sub divided
           COMPUTE WS-RESULT-16 =
               (WS-V02 - WS-V01) / WS-V04.

      * 17: Two parenthesized groups added
           COMPUTE WS-RESULT-17 =
               (WS-V01 + WS-V02) + (WS-V03 + WS-V04).

      * 18: Two groups multiplied
           COMPUTE WS-RESULT-18 =
               (WS-V01 + WS-V02) * (WS-V03 - WS-V04).

      * 19: Add-multiply-divide chain
           COMPUTE WS-RESULT-19 =
               (WS-V01 + WS-V02) * WS-V03 / WS-V04.

      * 20: Four terms mixed ops
           COMPUTE WS-RESULT-20 =
               WS-V01 * WS-V02 + WS-V03 * WS-V04.

      *================================================================*
      * COMP AND COMP-3 MIXED TYPES (21-30)
      *================================================================*

      * 21: COMP + display numeric
           COMPUTE WS-RESULT-21 =
               WS-V11 + WS-V01.

      * 22: COMP - COMP
           COMPUTE WS-RESULT-22 =
               WS-V11 - WS-V12.

      * 23: COMP * display
           COMPUTE WS-RESULT-23 =
               WS-V12 * WS-V03.

      * 24: COMP-3 + display
           COMPUTE WS-RESULT-24 =
               WS-V16 + WS-V01.

      * 25: COMP-3 * COMP-3
           COMPUTE WS-RESULT-25 =
               WS-V16 * WS-V17.

      * 26: COMP + COMP-3
           COMPUTE WS-RESULT-26 =
               WS-V11 + WS-V16.

      * 27: Signed COMP + signed COMP-3
           COMPUTE WS-RESULT-27 =
               WS-V13 + WS-V18.

      * 28: COMP-3 decimal + display decimal
           COMPUTE WS-RESULT-28 =
               WS-V19 + WS-V07.

      * 29: Signed COMP-3 decimal * display
           COMPUTE WS-RESULT-29 =
               WS-V20 * WS-V03.

      * 30: COMP divided by COMP-3
           COMPUTE WS-RESULT-30 =
               WS-V14 / WS-V16.

      *================================================================*
      * EXPONENTIATION (31-35)
      *================================================================*

      * 31: Simple square
           COMPUTE WS-RESULT-31 = WS-V25 ** 2.

      * 32: Simple cube
           COMPUTE WS-RESULT-32 = WS-V25 ** 3.

      * 33: Sum of squares
           COMPUTE WS-RESULT-33 =
               WS-V01 ** 2 + WS-V02 ** 2.

      * 34: Square times constant
           COMPUTE WS-RESULT-34 =
               WS-V03 ** 2 * WS-V04.

      * 35: Exponent in complex expression
           COMPUTE WS-RESULT-35 =
               (WS-V25 ** 2) + (WS-V26 ** 2)
               - WS-V43 * WS-V44.

      *================================================================*
      * NESTED PARENTHESES (36-40)
      *================================================================*

      * 36: Double nested
           COMPUTE WS-RESULT-36 =
               ((WS-V01 + WS-V02) * WS-V03)
               / WS-V04.

      * 37: Triple nested
           COMPUTE WS-RESULT-37 =
               (((WS-V01 + WS-V02) * WS-V03)
               - WS-V04) / WS-V25.

      * 38: Mixed nesting with subtraction
           COMPUTE WS-RESULT-38 =
               (WS-V01 * (WS-V02 - WS-V03))
               + (WS-V04 * (WS-V25 + WS-V26)).

      * 39: Complex nested with division
           COMPUTE WS-RESULT-39 =
               ((WS-V31 + WS-V32)
               * (WS-V34 - WS-V27))
               / (WS-V43 + WS-V44).

      * 40: Deep nesting four levels
           COMPUTE WS-RESULT-40 =
               (((WS-V01 + WS-V02)
               * (WS-V03 + WS-V04))
               / (WS-V25 + WS-V26))
               + WS-V31.

      *================================================================*
      * CHAINED RESULTS (41-45)
      * These use prior WS-RESULT fields as inputs
      *================================================================*

      * 41: Two results added
           COMPUTE WS-RESULT-41 =
               WS-RESULT-01 + WS-RESULT-02.

      * 42: Result times result
           COMPUTE WS-RESULT-42 =
               WS-RESULT-03 * WS-RESULT-04.

      * 43: Three results combined
           COMPUTE WS-RESULT-43 =
               WS-RESULT-01 + WS-RESULT-10
               * WS-RESULT-05.

      * 44: Result with exponent
           COMPUTE WS-RESULT-44 =
               WS-RESULT-01 ** 2
               + WS-RESULT-02 ** 2.

      * 45: Complex chain
           COMPUTE WS-RESULT-45 =
               (WS-RESULT-01 + WS-RESULT-10)
               * (WS-RESULT-30 - WS-RESULT-05)
               / WS-RESULT-04.

      *================================================================*
      * ROUNDED AND ON SIZE ERROR (46-50)
      *================================================================*

      * 46: Simple with ROUNDED
           COMPUTE WS-RESULT-46 ROUNDED =
               WS-V01 / WS-V03.

      * 47: Complex with ROUNDED
           COMPUTE WS-RESULT-47 ROUNDED =
               (WS-V01 + WS-V02)
               * WS-V07 / WS-V08.

      * 48: With ON SIZE ERROR
           COMPUTE WS-RESULT-48 =
               WS-V14 * WS-V36
               ON SIZE ERROR
               MOVE 1 TO WS-SIZE-ERR
           END-COMPUTE.

      * 49: ROUNDED + ON SIZE ERROR
           COMPUTE WS-RESULT-49 ROUNDED =
               (WS-V46 + WS-V34)
               * WS-V40 / WS-V43
               ON SIZE ERROR
               MOVE 1 TO WS-SIZE-ERR
           END-COMPUTE.

      * 50: Float fields in COMPUTE
           COMPUTE WS-RESULT-50 =
               WS-V21 * WS-V22
               + WS-V23 * WS-V24.

      *================================================================*
      * DISPLAY RESULTS
      *================================================================*
           DISPLAY "=== COMPLEX COMPUTE RESULTS ===".

           DISPLAY "R01 (V01+V02)       = "
               WS-RESULT-01.
           DISPLAY "R02 (V02-V01)       = "
               WS-RESULT-02.
           DISPLAY "R03 (V03*V04)       = "
               WS-RESULT-03.
           DISPLAY "R04 (V01/V04)       = "
               WS-RESULT-04.
           DISPLAY "R05 (V05+V06)       = "
               WS-RESULT-05.
           DISPLAY "R10 (V10-V09)       = "
               WS-RESULT-10.
           DISPLAY "R15 ((V01+V02)*V03) = "
               WS-RESULT-15.
           DISPLAY "R20 (V01*V02+V03*V04)="
               WS-RESULT-20.
           DISPLAY "R25 (V16*V17)       = "
               WS-RESULT-25.
           DISPLAY "R30 (V14/V16)       = "
               WS-RESULT-30.
           DISPLAY "R31 (V25**2)        = "
               WS-RESULT-31.
           DISPLAY "R33 (V01**2+V02**2) = "
               WS-RESULT-33.
           DISPLAY "R35 (exp complex)   = "
               WS-RESULT-35.
           DISPLAY "R40 (deep nest)     = "
               WS-RESULT-40.
           DISPLAY "R41 (R01+R02)       = "
               WS-RESULT-41.
           DISPLAY "R44 (R01**2+R02**2) = "
               WS-RESULT-44.
           DISPLAY "R45 (chain complex) = "
               WS-RESULT-45.
           DISPLAY "R46 (rounded div)   = "
               WS-RESULT-46.
           DISPLAY "R48 (size err test) = "
               WS-RESULT-48.
           DISPLAY "R50 (float compute) = "
               WS-RESULT-50.

           DISPLAY "SIZE-ERR FLAG       = "
               WS-SIZE-ERR.
           DISPLAY "=== END COMPUTE TEST ===".

           STOP RUN.
