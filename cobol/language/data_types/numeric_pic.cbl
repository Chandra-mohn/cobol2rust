       IDENTIFICATION DIVISION.
       PROGRAM-ID. NUMERIC-PIC-TEST.
      *---------------------------------------------------------------
      * Stress test: All numeric PIC clause variants.
      * Covers: PIC 9, S9, V, P, sign position, precision/scale.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Simple unsigned integers
       01  WS-INT1            PIC 9.
       01  WS-INT3            PIC 999.
       01  WS-INT5            PIC 9(5).
       01  WS-INT9            PIC 9(9).
       01  WS-INT18           PIC 9(18).
      * Signed integers
       01  WS-SINT1           PIC S9.
       01  WS-SINT5           PIC S9(5).
       01  WS-SINT9           PIC S9(9).
       01  WS-SINT18          PIC S9(18).
      * Decimal with implied decimal point
       01  WS-DEC1            PIC 9V9.
       01  WS-DEC2            PIC 9(3)V99.
       01  WS-DEC3            PIC 9(5)V9(3).
       01  WS-DEC4            PIC 9(7)V9(5).
      * Signed decimal
       01  WS-SDEC1           PIC S9V9.
       01  WS-SDEC2           PIC S9(3)V99.
       01  WS-SDEC3           PIC S9(5)V9(3).
       01  WS-SDEC4           PIC S9(7)V9(5).
      * P (assumed decimal scaling)
       01  WS-PSCALE1         PIC 9(3)PP.
       01  WS-PSCALE2         PIC PP9(3).
       01  WS-PSCALE3         PIC S9(5)PPP.
      * Large precision
       01  WS-LARGE1          PIC 9(15)V9(3).
       01  WS-LARGE2          PIC S9(18).
      * Initialized values
       01  WS-INIT-INT        PIC 9(5) VALUE 12345.
       01  WS-INIT-DEC        PIC 9(3)V99 VALUE 123.45.
       01  WS-INIT-SINT       PIC S9(5) VALUE -500.
       01  WS-INIT-SDEC       PIC S9(3)V99 VALUE -12.34.
       01  WS-INIT-ZERO       PIC 9(5) VALUE ZERO.
       01  WS-INIT-ZEROS      PIC 9(5) VALUE ZEROS.
      * Level 77 standalone
       77  WS-STANDALONE      PIC 9(7)V99.
       77  WS-STAND-INIT      PIC S9(5) VALUE 99999.
      * Result fields
       01  WS-RESULT1         PIC 9(9)V99.
       01  WS-RESULT2         PIC S9(9)V99.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 42 TO WS-INT5.
           MOVE 12345 TO WS-INT5.
           MOVE -100 TO WS-SINT5.
           MOVE 123.45 TO WS-DEC2.
           MOVE -99.99 TO WS-SDEC2.
           ADD WS-INIT-INT TO WS-RESULT1.
           SUBTRACT WS-INIT-DEC FROM WS-RESULT2.
           DISPLAY WS-INT5.
           DISPLAY WS-SINT5.
           DISPLAY WS-DEC2.
           DISPLAY WS-SDEC2.
           DISPLAY WS-INIT-INT.
           DISPLAY WS-INIT-DEC.
           DISPLAY WS-INIT-SINT.
           DISPLAY WS-INIT-SDEC.
           DISPLAY WS-RESULT1.
           DISPLAY WS-RESULT2.
           STOP RUN.
