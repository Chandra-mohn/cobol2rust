       IDENTIFICATION DIVISION.
       PROGRAM-ID. EDITED-PIC-TEST.
      *---------------------------------------------------------------
      * Stress test: Numeric edited and alphanumeric edited PIC.
      * Covers: Z, *, $, +, -, CR, DB, B, 0, /, comma, period.
      *---------------------------------------------------------------
       DATA DIVISION.
       WORKING-STORAGE SECTION.
      * Source fields
       01  WS-AMT             PIC 9(5)V99 VALUE 12345.67.
       01  WS-SMALL           PIC 9(3)V99 VALUE 1.23.
       01  WS-NEG             PIC S9(5)V99 VALUE -500.25.
       01  WS-ZERO            PIC 9(5)V99 VALUE 0.
      * Numeric edited - zero suppression
       01  WS-EDIT-Z1         PIC Z(5)9.
       01  WS-EDIT-Z2         PIC Z(5)9.99.
       01  WS-EDIT-Z3         PIC ZZ,ZZZ,ZZ9.99.
       01  WS-EDIT-Z4         PIC Z(7).
      * Numeric edited - asterisk fill
       01  WS-EDIT-STAR       PIC *(5)9.99.
       01  WS-EDIT-STAR2      PIC **,***,**9.99.
      * Numeric edited - currency
       01  WS-EDIT-DOLLAR     PIC $ZZ,ZZ9.99.
       01  WS-EDIT-DOLLAR2    PIC $$,$$9.99.
      * Numeric edited - sign
       01  WS-EDIT-PLUS       PIC +ZZ,ZZ9.99.
       01  WS-EDIT-MINUS      PIC -ZZ,ZZ9.99.
       01  WS-EDIT-TRAIL-PLUS PIC ZZ,ZZ9.99+.
       01  WS-EDIT-TRAIL-MINUS PIC ZZ,ZZ9.99-.
       01  WS-EDIT-CR         PIC ZZ,ZZ9.99CR.
       01  WS-EDIT-DB         PIC ZZ,ZZ9.99DB.
      * Numeric edited - insertion
       01  WS-EDIT-B          PIC 9(3)B9(3).
       01  WS-EDIT-ZERO       PIC 9(3)09(3).
       01  WS-EDIT-SLASH      PIC 99/99/9999.
      * Alphanumeric edited
       01  WS-AEDIT-B         PIC X(3)BX(3).
       01  WS-AEDIT-ZERO      PIC X(3)0X(3).
       01  WS-AEDIT-SLASH     PIC XX/XX/XXXX.
      * BLANK WHEN ZERO
       01  WS-BLANK-ZERO      PIC Z(5)9 BLANK WHEN ZERO.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE WS-AMT TO WS-EDIT-Z1.
           MOVE WS-AMT TO WS-EDIT-Z2.
           MOVE WS-AMT TO WS-EDIT-Z3.
           MOVE WS-SMALL TO WS-EDIT-Z4.
           MOVE WS-AMT TO WS-EDIT-STAR.
           MOVE WS-AMT TO WS-EDIT-STAR2.
           MOVE WS-AMT TO WS-EDIT-DOLLAR.
           MOVE WS-AMT TO WS-EDIT-DOLLAR2.
           MOVE WS-NEG TO WS-EDIT-PLUS.
           MOVE WS-NEG TO WS-EDIT-MINUS.
           MOVE WS-NEG TO WS-EDIT-CR.
           MOVE WS-NEG TO WS-EDIT-DB.
           MOVE WS-ZERO TO WS-BLANK-ZERO.
           MOVE 12345678 TO WS-EDIT-SLASH.
           MOVE "ABCDEF" TO WS-AEDIT-B.
           DISPLAY WS-EDIT-Z1.
           DISPLAY WS-EDIT-Z2.
           DISPLAY WS-EDIT-Z3.
           DISPLAY WS-EDIT-STAR.
           DISPLAY WS-EDIT-DOLLAR.
           DISPLAY WS-EDIT-PLUS.
           DISPLAY WS-EDIT-MINUS.
           DISPLAY WS-EDIT-CR.
           DISPLAY WS-EDIT-DB.
           DISPLAY WS-EDIT-SLASH.
           DISPLAY WS-BLANK-ZERO.
           STOP RUN.
