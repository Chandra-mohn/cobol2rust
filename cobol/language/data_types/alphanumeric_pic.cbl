       IDENTIFICATION DIVISION.
       PROGRAM-ID. ALPHANUM-PIC-TEST.
      *---------------------------------------------------------------
      * Stress test: PIC X (alphanumeric) and PIC A (alphabetic).
      * Covers: Various lengths, VALUE, JUSTIFIED RIGHT, BLANK WHEN
      * ZERO, figurative constants.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * PIC X variants
       01  WS-X1              PIC X.
       01  WS-X5              PIC X(5).
       01  WS-X10             PIC X(10).
       01  WS-X50             PIC X(50).
       01  WS-X100            PIC X(100).
       01  WS-X256            PIC X(256).
      * PIC A variants
       01  WS-A1              PIC A.
       01  WS-A5              PIC A(5).
       01  WS-A10             PIC A(10).
       01  WS-A50             PIC A(50).
      * With VALUE clauses
       01  WS-XVAL1           PIC X(10) VALUE "HELLO".
       01  WS-XVAL2           PIC X(20) VALUE "ABCDEFGHIJKLMNOPQRST".
       01  WS-XVAL3           PIC X(5) VALUE "ABCDEFGH".
       01  WS-AVAL1           PIC A(10) VALUE "HELLO".
      * Figurative constants
       01  WS-SPACES-F        PIC X(10) VALUE SPACES.
       01  WS-ZEROS-F         PIC X(10) VALUE ZEROS.
       01  WS-HIGH-F          PIC X(5) VALUE HIGH-VALUES.
       01  WS-LOW-F           PIC X(5) VALUE LOW-VALUES.
       01  WS-QUOTE-F         PIC X(5) VALUE QUOTES.
       01  WS-ALL-A           PIC X(10) VALUE ALL "A".
       01  WS-ALL-XY          PIC X(10) VALUE ALL "XY".
      * JUSTIFIED RIGHT
       01  WS-JUST-R          PIC X(10) JUSTIFIED RIGHT.
      * Level 77
       77  WS-STAND-X         PIC X(15) VALUE "STANDALONE".
      * Mixed usage
       01  WS-RESULT          PIC X(30).
       01  WS-DISPLAY-LINE    PIC X(80).
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE "TEST" TO WS-X5.
           MOVE "A" TO WS-X1.
           MOVE SPACES TO WS-X50.
           MOVE ALL "*" TO WS-X10.
           MOVE WS-XVAL1 TO WS-RESULT.
           MOVE "RIGHT-JUST" TO WS-JUST-R.
           DISPLAY WS-X1.
           DISPLAY WS-X5.
           DISPLAY WS-X10.
           DISPLAY WS-XVAL1.
           DISPLAY WS-XVAL2.
           DISPLAY WS-SPACES-F.
           DISPLAY WS-ALL-A.
           DISPLAY WS-ALL-XY.
           DISPLAY WS-JUST-R.
           DISPLAY WS-RESULT.
           STOP RUN.
