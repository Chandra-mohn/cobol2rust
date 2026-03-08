       IDENTIFICATION DIVISION.
       PROGRAM-ID. REALISTIC-BATCH-TEST.
      *---------------------------------------------------------------
      * Volume stress test: Realistic batch processing program.
      * Simulates an employee payroll batch job that reads input
      * records, validates data, calculates pay, accumulates
      * department and grand totals, writes a formatted report,
      * and logs errors to a separate error file.
      * Exercises: file I/O, EVALUATE, PERFORM VARYING, STRING,
      * INSPECT, COMPUTE with ON SIZE ERROR, level-88 conditions,
      * table processing, edited pictures, and accumulators.
      *---------------------------------------------------------------
       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.
       SOURCE-COMPUTER. IBM-MAINFRAME.
       OBJECT-COMPUTER. IBM-MAINFRAME.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT INPUT-FILE ASSIGN TO "PAYROLL.DAT"
               ORGANIZATION IS SEQUENTIAL
               ACCESS MODE IS SEQUENTIAL
               FILE STATUS IS WS-INPUT-STATUS.
           SELECT REPORT-FILE ASSIGN TO "PAYREPORT.RPT"
               ORGANIZATION IS SEQUENTIAL
               ACCESS MODE IS SEQUENTIAL
               FILE STATUS IS WS-REPORT-STATUS.
           SELECT ERROR-FILE ASSIGN TO "PAYERROR.LOG"
               ORGANIZATION IS SEQUENTIAL
               ACCESS MODE IS SEQUENTIAL
               FILE STATUS IS WS-ERROR-STATUS.
       DATA DIVISION.
       FILE SECTION.
       FD  INPUT-FILE
           RECORDING MODE IS F
           RECORD CONTAINS 120 CHARACTERS.
       01  INPUT-RECORD.
           05  IR-EMPLOYEE-ID         PIC 9(6).
           05  IR-EMPLOYEE-NAME.
               10  IR-LAST-NAME       PIC X(20).
               10  IR-FIRST-NAME      PIC X(15).
           05  IR-DEPARTMENT          PIC X(4).
           05  IR-PAY-TYPE            PIC X(1).
               88  IR-HOURLY          VALUE "H".
               88  IR-SALARIED        VALUE "S".
               88  IR-COMMISSION      VALUE "C".
           05  IR-HOURS-WORKED        PIC 9(3)V99.
           05  IR-PAY-RATE            PIC 9(5)V99.
           05  IR-DEDUCTIONS.
               10  IR-HEALTH-DED      PIC 9(4)V99.
               10  IR-RETIRE-DED      PIC 9(4)V99.
               10  IR-OTHER-DED       PIC 9(4)V99.
           05  IR-YTD-GROSS           PIC 9(8)V99.
           05  IR-YTD-TAX             PIC 9(7)V99.
           05  IR-YTD-NET             PIC 9(8)V99.
           05  IR-FILLER              PIC X(15).
       FD  REPORT-FILE
           RECORDING MODE IS F
           RECORD CONTAINS 132 CHARACTERS.
       01  REPORT-RECORD              PIC X(132).
       FD  ERROR-FILE
           RECORDING MODE IS F
           RECORD CONTAINS 120 CHARACTERS.
       01  ERROR-RECORD               PIC X(120).
       WORKING-STORAGE SECTION.
      * File status fields
       01  WS-INPUT-STATUS            PIC XX.
           88  INPUT-OK               VALUE "00".
           88  INPUT-EOF              VALUE "10".
       01  WS-REPORT-STATUS           PIC XX.
           88  REPORT-OK              VALUE "00".
       01  WS-ERROR-STATUS            PIC XX.
           88  ERROR-OK               VALUE "00".
      * End of file flag
       01  WS-EOF-FLAG                PIC 9 VALUE 0.
           88  END-OF-FILE            VALUE 1.
           88  NOT-END-OF-FILE        VALUE 0.
      * Processing flags and error codes
       01  WS-VALID-RECORD            PIC 9 VALUE 0.
           88  RECORD-IS-VALID        VALUE 1.
           88  RECORD-IS-INVALID      VALUE 0.
       01  WS-ERROR-CODE              PIC 9(3) VALUE 0.
           88  NO-ERROR               VALUE 0.
           88  ERR-INVALID-ID         VALUE 100.
           88  ERR-INVALID-DEPT       VALUE 200.
           88  ERR-INVALID-HOURS      VALUE 300.
           88  ERR-INVALID-RATE       VALUE 400.
           88  ERR-CALC-OVERFLOW      VALUE 500.
           88  ERR-INVALID-PAY-TYPE   VALUE 600.
           88  ERR-NEGATIVE-NET       VALUE 700.
       01  WS-ERROR-MESSAGE           PIC X(50) VALUE SPACES.
       01  WS-DEPT-FOUND              PIC 9 VALUE 0.
           88  DEPT-WAS-FOUND         VALUE 1.
           88  DEPT-NOT-FOUND         VALUE 0.
      * Record counters
       01  WS-RECORDS-READ            PIC 9(7) VALUE 0.
       01  WS-RECORDS-PROCESSED       PIC 9(7) VALUE 0.
       01  WS-RECORDS-ERROR           PIC 9(7) VALUE 0.
       01  WS-RECORDS-WRITTEN         PIC 9(7) VALUE 0.
       01  WS-REPORT-LINES            PIC 9(7) VALUE 0.
       01  WS-ERROR-LINES             PIC 9(7) VALUE 0.
       01  WS-PAGE-COUNT              PIC 9(3) VALUE 0.
       01  WS-LINE-COUNT              PIC 9(3) VALUE 0.
       01  WS-LINES-PER-PAGE          PIC 9(3) VALUE 55.
      * Tax rates and limits (constants)
       01  WS-FED-TAX-RATE            PIC V9(4) VALUE 0.2200.
       01  WS-STATE-TAX-RATE          PIC V9(4) VALUE 0.0650.
       01  WS-FICA-RATE               PIC V9(4) VALUE 0.0765.
       01  WS-OVERTIME-FACTOR         PIC 9V99 VALUE 1.50.
       01  WS-MAX-REGULAR-HOURS       PIC 9(3)V99 VALUE 40.00.
       01  WS-FICA-WAGE-LIMIT         PIC 9(8)V99
                                      VALUE 160200.00.
       01  WS-MIN-WAGE                PIC 9(3)V99 VALUE 7.25.
       01  WS-MAX-HOURS-ALLOWED       PIC 9(3)V99 VALUE 80.00.
       01  WS-COMMISSION-RATE         PIC V9(4) VALUE 0.0800.
      * Calculated pay fields
       01  WS-GROSS-PAY               PIC 9(8)V99 VALUE 0.
       01  WS-REGULAR-PAY             PIC 9(7)V99 VALUE 0.
       01  WS-OVERTIME-PAY            PIC 9(7)V99 VALUE 0.
       01  WS-OVERTIME-HOURS          PIC 9(3)V99 VALUE 0.
       01  WS-REGULAR-HOURS           PIC 9(3)V99 VALUE 0.
       01  WS-FED-TAX                 PIC 9(7)V99 VALUE 0.
       01  WS-STATE-TAX              PIC 9(6)V99 VALUE 0.
       01  WS-FICA-TAX                PIC 9(6)V99 VALUE 0.
       01  WS-TOTAL-TAX               PIC 9(7)V99 VALUE 0.
       01  WS-TOTAL-DEDUCTIONS        PIC 9(7)V99 VALUE 0.
       01  WS-NET-PAY                 PIC S9(8)V99 VALUE 0.
       01  WS-COMMISSION-PAY          PIC 9(7)V99 VALUE 0.
       01  WS-CALC-OVERFLOW           PIC 9 VALUE 0.
           88  CALC-HAS-OVERFLOW      VALUE 1.
           88  CALC-NO-OVERFLOW       VALUE 0.
      * Department table (10 departments)
       01  WS-DEPT-TABLE.
           05  WS-DEPT-ENTRY OCCURS 10 TIMES.
               10  WS-DEPT-CODE       PIC X(4).
               10  WS-DEPT-NAME       PIC X(25).
               10  WS-DEPT-EMP-COUNT  PIC 9(5) VALUE 0.
               10  WS-DEPT-GROSS-TOT  PIC 9(10)V99 VALUE 0.
               10  WS-DEPT-TAX-TOT    PIC 9(9)V99 VALUE 0.
               10  WS-DEPT-NET-TOT    PIC 9(10)V99 VALUE 0.
               10  WS-DEPT-DED-TOT    PIC 9(9)V99 VALUE 0.
               10  WS-DEPT-HOURS-TOT  PIC 9(7)V99 VALUE 0.
               10  WS-DEPT-OT-HOURS   PIC 9(6)V99 VALUE 0.
       01  WS-DEPT-IDX                PIC 99 VALUE 0.
       01  WS-DEPT-SUB               PIC 99 VALUE 0.
       01  WS-TABLE-IDX               PIC 99 VALUE 0.
      * Grand total accumulators
       01  WS-GRAND-TOTALS.
           05  WS-GRAND-GROSS         PIC 9(12)V99 VALUE 0.
           05  WS-GRAND-TAX           PIC 9(11)V99 VALUE 0.
           05  WS-GRAND-NET           PIC 9(12)V99 VALUE 0.
           05  WS-GRAND-DEDUCTIONS    PIC 9(11)V99 VALUE 0.
           05  WS-GRAND-HOURS         PIC 9(9)V99 VALUE 0.
           05  WS-GRAND-OT-HOURS      PIC 9(8)V99 VALUE 0.
           05  WS-GRAND-EMP-COUNT     PIC 9(7) VALUE 0.
           05  WS-GRAND-HOURLY-CT     PIC 9(7) VALUE 0.
           05  WS-GRAND-SALARY-CT     PIC 9(7) VALUE 0.
           05  WS-GRAND-COMM-CT       PIC 9(7) VALUE 0.
      * Date fields
       01  WS-CURRENT-DATE-DATA.
           05  WS-CURRENT-DATE.
               10  WS-CURRENT-YEAR   PIC 9(4).
               10  WS-CURRENT-MONTH  PIC 9(2).
               10  WS-CURRENT-DAY    PIC 9(2).
           05  WS-CURRENT-TIME.
               10  WS-CURRENT-HOUR   PIC 9(2).
               10  WS-CURRENT-MIN    PIC 9(2).
               10  WS-CURRENT-SEC    PIC 9(2).
               10  WS-CURRENT-HUND   PIC 9(2).
           05  WS-GMT-OFFSET          PIC S9(4).
       01  WS-FORMATTED-DATE          PIC X(10).
       01  WS-FORMATTED-TIME          PIC X(8).
       01  WS-PAY-PERIOD              PIC X(7) VALUE SPACES.
      * Edited picture fields for report formatting
       01  WS-EDIT-FIELDS.
           05  WS-EDIT-EMP-ID         PIC Z(5)9.
           05  WS-EDIT-HOURS          PIC ZZ9.99.
           05  WS-EDIT-RATE           PIC Z(4)9.99.
           05  WS-EDIT-GROSS          PIC Z(6)9.99.
           05  WS-EDIT-TAX            PIC Z(5)9.99.
           05  WS-EDIT-DED            PIC Z(5)9.99.
           05  WS-EDIT-NET            PIC Z(6)9.99-.
           05  WS-EDIT-TOTAL          PIC Z(10)9.99.
           05  WS-EDIT-COUNT          PIC Z(6)9.
           05  WS-EDIT-OT-HOURS       PIC ZZ9.99.
           05  WS-EDIT-PCT            PIC Z9.99.
           05  WS-EDIT-PAGE           PIC ZZ9.
      * Report header lines
       01  WS-HEADER-LINE-1.
           05  FILLER                  PIC X(30)
               VALUE "EMPLOYEE PAYROLL BATCH REPORT".
           05  FILLER                  PIC X(52) VALUE SPACES.
           05  FILLER                  PIC X(6) VALUE "DATE: ".
           05  WS-HDR-DATE            PIC X(10).
           05  FILLER                  PIC X(34) VALUE SPACES.
       01  WS-HEADER-LINE-2.
           05  FILLER                  PIC X(30)
               VALUE "------------------------------".
           05  FILLER                  PIC X(52) VALUE SPACES.
           05  FILLER                  PIC X(6) VALUE "TIME: ".
           05  WS-HDR-TIME            PIC X(8).
           05  FILLER                  PIC X(6) VALUE " PAGE ".
           05  WS-HDR-PAGE            PIC ZZ9.
           05  FILLER                  PIC X(27) VALUE SPACES.
       01  WS-COLUMN-HDR-1.
           05  FILLER     PIC X(7) VALUE "EMP-ID ".
           05  FILLER     PIC X(21) VALUE "EMPLOYEE NAME       ".
           05  FILLER     PIC X(5) VALUE "DEPT ".
           05  FILLER     PIC X(2) VALUE "T ".
           05  FILLER     PIC X(8) VALUE "  HOURS ".
           05  FILLER     PIC X(10) VALUE "     RATE ".
           05  FILLER     PIC X(11) VALUE "     GROSS ".
           05  FILLER     PIC X(10) VALUE "      TAX ".
           05  FILLER     PIC X(10) VALUE "      DED ".
           05  FILLER     PIC X(11) VALUE "      NET  ".
           05  FILLER     PIC X(37) VALUE SPACES.
       01  WS-COLUMN-HDR-2.
           05  FILLER     PIC X(7) VALUE "------ ".
           05  FILLER     PIC X(21) VALUE "--------------------".
           05  FILLER     PIC X(5) VALUE "---- ".
           05  FILLER     PIC X(2) VALUE "- ".
           05  FILLER     PIC X(8) VALUE "-------- ".
           05  FILLER     PIC X(10) VALUE "--------- ".
           05  FILLER     PIC X(11) VALUE "---------- ".
           05  FILLER     PIC X(10) VALUE "--------- ".
           05  FILLER     PIC X(10) VALUE "--------- ".
           05  FILLER     PIC X(11) VALUE "---------- ".
           05  FILLER     PIC X(37) VALUE SPACES.
      * Detail line for report
       01  WS-DETAIL-LINE.
           05  WS-DET-EMP-ID          PIC Z(5)9.
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-NAME            PIC X(20).
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-DEPT            PIC X(4).
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-TYPE            PIC X(1).
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-HOURS           PIC ZZ9.99.
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-RATE            PIC Z(4)9.99.
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-GROSS           PIC Z(6)9.99.
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-TAX             PIC Z(5)9.99.
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-DED             PIC Z(5)9.99.
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-DET-NET             PIC Z(6)9.99-.
           05  FILLER                  PIC X(28) VALUE SPACES.
      * Summary line for department totals
       01  WS-DEPT-SUMMARY-LINE.
           05  FILLER                  PIC X(5) VALUE "DEPT ".
           05  WS-SUM-DEPT-CODE       PIC X(4).
           05  FILLER                  PIC X(2) VALUE ": ".
           05  WS-SUM-DEPT-NAME       PIC X(25).
           05  FILLER                  PIC X(7) VALUE "  EMP: ".
           05  WS-SUM-EMP-CT          PIC Z(4)9.
           05  FILLER                  PIC X(9) VALUE "  GROSS: ".
           05  WS-SUM-GROSS           PIC Z(8)9.99.
           05  FILLER                  PIC X(7) VALUE "  NET: ".
           05  WS-SUM-NET             PIC Z(8)9.99.
           05  FILLER                  PIC X(44) VALUE SPACES.
      * Grand total line
       01  WS-GRAND-TOTAL-LINE.
           05  FILLER        PIC X(14) VALUE "GRAND TOTALS: ".
           05  FILLER        PIC X(6) VALUE "EMP: ".
           05  WS-GT-EMP-CT  PIC Z(6)9.
           05  FILLER        PIC X(9) VALUE "  GROSS: ".
           05  WS-GT-GROSS   PIC Z(10)9.99.
           05  FILLER        PIC X(7) VALUE "  TAX: ".
           05  WS-GT-TAX     PIC Z(9)9.99.
           05  FILLER        PIC X(7) VALUE "  NET: ".
           05  WS-GT-NET     PIC Z(10)9.99.
           05  FILLER        PIC X(44) VALUE SPACES.
      * Error log record layout
       01  WS-ERROR-LOG-REC.
           05  WS-ERR-DATE            PIC X(10).
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-ERR-TIME            PIC X(8).
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-ERR-EMP-ID          PIC 9(6).
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-ERR-CODE            PIC 9(3).
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-ERR-MSG             PIC X(50).
           05  FILLER                  PIC X VALUE SPACE.
           05  WS-ERR-FIELD           PIC X(20).
           05  FILLER                  PIC X(18) VALUE SPACES.
      * Temporary work fields
       01  WS-TEMP-STRING             PIC X(80) VALUE SPACES.
       01  WS-STRING-PTR              PIC 99 VALUE 1.
       01  WS-INSPECT-COUNT           PIC 9(3) VALUE 0.
       01  WS-TEMP-NUM                PIC 9(10)V99 VALUE 0.
       01  WS-TEMP-RATE               PIC 9V9(4) VALUE 0.
       01  WS-CLEANED-NAME            PIC X(20) VALUE SPACES.
       01  WS-DISPLAY-NAME            PIC X(36) VALUE SPACES.
       01  WS-SEPARATOR-LINE          PIC X(132) VALUE ALL "-".
       01  WS-BLANK-LINE              PIC X(132) VALUE SPACES.
       01  WS-FICA-REMAINING          PIC 9(8)V99 VALUE 0.
       01  WS-TAXABLE-GROSS           PIC 9(8)V99 VALUE 0.
      * Statistics tracking
       01  WS-STATS.
           05  WS-MIN-GROSS           PIC 9(8)V99
                                      VALUE 99999999.
           05  WS-MAX-GROSS           PIC 9(8)V99 VALUE 0.
           05  WS-MIN-NET             PIC S9(8)V99
                                      VALUE 99999999.
           05  WS-MAX-NET             PIC S9(8)V99 VALUE 0.
           05  WS-AVG-GROSS           PIC 9(8)V99 VALUE 0.
           05  WS-AVG-NET             PIC 9(8)V99 VALUE 0.
           05  WS-HOURLY-COUNT        PIC 9(5) VALUE 0.
           05  WS-SALARY-COUNT        PIC 9(5) VALUE 0.
           05  WS-COMM-COUNT          PIC 9(5) VALUE 0.
      * Batch control fields
       01  WS-BATCH-CONTROL.
           05  WS-BATCH-ID            PIC X(8) VALUE "PAYBATCH".
           05  WS-BATCH-DATE          PIC X(10) VALUE SPACES.
           05  WS-BATCH-STATUS        PIC X(1) VALUE "I".
               88  BATCH-INIT         VALUE "I".
               88  BATCH-RUNNING      VALUE "R".
               88  BATCH-COMPLETE     VALUE "C".
               88  BATCH-ABORTED      VALUE "A".
           05  WS-BATCH-RETURN-CODE   PIC 9(4) VALUE 0.
               88  BATCH-SUCCESS      VALUE 0.
               88  BATCH-WARNING      VALUE 4.
               88  BATCH-ERROR        VALUE 8.
               88  BATCH-SEVERE       VALUE 12.

       PROCEDURE DIVISION.
      *===============================================================
      * 0000-MAIN: Main control paragraph
      *===============================================================
       0000-MAIN SECTION.
       0000-MAIN-PARA.
           PERFORM 1000-INITIALIZE
           PERFORM 2000-PROCESS-RECORDS
           PERFORM 3000-PRINT-SUMMARY
           PERFORM 9000-TERMINATE
           STOP RUN.

      *===============================================================
      * 1000-INITIALIZE: Open files and set up work areas
      *===============================================================
       1000-INITIALIZE SECTION.
       1000-INIT-PARA.
           MOVE "R" TO WS-BATCH-STATUS
           MOVE FUNCTION CURRENT-DATE
               TO WS-CURRENT-DATE-DATA
           STRING WS-CURRENT-MONTH DELIMITED SIZE
                  "/" DELIMITED SIZE
                  WS-CURRENT-DAY DELIMITED SIZE
                  "/" DELIMITED SIZE
                  WS-CURRENT-YEAR DELIMITED SIZE
               INTO WS-FORMATTED-DATE
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE 1 TO WS-STRING-PTR
           STRING WS-CURRENT-HOUR DELIMITED SIZE
                  ":" DELIMITED SIZE
                  WS-CURRENT-MIN DELIMITED SIZE
                  ":" DELIMITED SIZE
                  WS-CURRENT-SEC DELIMITED SIZE
               INTO WS-FORMATTED-TIME
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE WS-FORMATTED-DATE TO WS-BATCH-DATE
           MOVE WS-FORMATTED-DATE TO WS-HDR-DATE
           MOVE WS-FORMATTED-TIME TO WS-HDR-TIME
           DISPLAY "PAYROLL BATCH STARTED: "
               WS-FORMATTED-DATE " " WS-FORMATTED-TIME
      * Initialize department table
           MOVE "ACCT" TO WS-DEPT-CODE(1)
           MOVE "ACCOUNTING" TO WS-DEPT-NAME(1)
           MOVE "ENGG" TO WS-DEPT-CODE(2)
           MOVE "ENGINEERING" TO WS-DEPT-NAME(2)
           MOVE "SALE" TO WS-DEPT-CODE(3)
           MOVE "SALES" TO WS-DEPT-NAME(3)
           MOVE "MKTG" TO WS-DEPT-CODE(4)
           MOVE "MARKETING" TO WS-DEPT-NAME(4)
           MOVE "HRES" TO WS-DEPT-CODE(5)
           MOVE "HUMAN RESOURCES" TO WS-DEPT-NAME(5)
           MOVE "OPER" TO WS-DEPT-CODE(6)
           MOVE "OPERATIONS" TO WS-DEPT-NAME(6)
           MOVE "LGAL" TO WS-DEPT-CODE(7)
           MOVE "LEGAL" TO WS-DEPT-NAME(7)
           MOVE "ITDP" TO WS-DEPT-CODE(8)
           MOVE "IT DEPARTMENT" TO WS-DEPT-NAME(8)
           MOVE "ADMN" TO WS-DEPT-CODE(9)
           MOVE "ADMINISTRATION" TO WS-DEPT-NAME(9)
           MOVE "EXEC" TO WS-DEPT-CODE(10)
           MOVE "EXECUTIVE" TO WS-DEPT-NAME(10)
      * Initialize counters and accumulators
           INITIALIZE WS-GRAND-TOTALS
           MOVE 0 TO WS-RECORDS-READ
           MOVE 0 TO WS-RECORDS-PROCESSED
           MOVE 0 TO WS-RECORDS-ERROR
           MOVE 0 TO WS-RECORDS-WRITTEN
           MOVE 0 TO WS-REPORT-LINES
           MOVE 0 TO WS-ERROR-LINES
           MOVE 0 TO WS-PAGE-COUNT
           MOVE 0 TO WS-LINE-COUNT
           MOVE 0 TO WS-EOF-FLAG
      * Zero department accumulators
           PERFORM VARYING WS-DEPT-IDX FROM 1 BY 1
               UNTIL WS-DEPT-IDX > 10
               MOVE 0 TO WS-DEPT-EMP-COUNT(WS-DEPT-IDX)
               MOVE 0 TO WS-DEPT-GROSS-TOT(WS-DEPT-IDX)
               MOVE 0 TO WS-DEPT-TAX-TOT(WS-DEPT-IDX)
               MOVE 0 TO WS-DEPT-NET-TOT(WS-DEPT-IDX)
               MOVE 0 TO WS-DEPT-DED-TOT(WS-DEPT-IDX)
               MOVE 0 TO WS-DEPT-HOURS-TOT(WS-DEPT-IDX)
               MOVE 0 TO WS-DEPT-OT-HOURS(WS-DEPT-IDX)
           END-PERFORM
      * Open all files
           OPEN INPUT INPUT-FILE
           IF NOT INPUT-OK
               DISPLAY "ERROR: OPEN INPUT-FILE FAILED: "
                   WS-INPUT-STATUS
               MOVE 12 TO WS-BATCH-RETURN-CODE
               MOVE "A" TO WS-BATCH-STATUS
               STOP RUN
           END-IF
           OPEN OUTPUT REPORT-FILE
           IF NOT REPORT-OK
               DISPLAY "ERROR: OPEN REPORT-FILE FAILED: "
                   WS-REPORT-STATUS
               CLOSE INPUT-FILE
               MOVE 12 TO WS-BATCH-RETURN-CODE
               MOVE "A" TO WS-BATCH-STATUS
               STOP RUN
           END-IF
           OPEN OUTPUT ERROR-FILE
           IF NOT ERROR-OK
               DISPLAY "ERROR: OPEN ERROR-FILE FAILED: "
                   WS-ERROR-STATUS
               CLOSE INPUT-FILE
               CLOSE REPORT-FILE
               MOVE 12 TO WS-BATCH-RETURN-CODE
               MOVE "A" TO WS-BATCH-STATUS
               STOP RUN
           END-IF
      * Write report headers
           ADD 1 TO WS-PAGE-COUNT
           MOVE WS-PAGE-COUNT TO WS-HDR-PAGE
           WRITE REPORT-RECORD FROM WS-HEADER-LINE-1
           WRITE REPORT-RECORD FROM WS-HEADER-LINE-2
           WRITE REPORT-RECORD FROM WS-BLANK-LINE
           WRITE REPORT-RECORD FROM WS-COLUMN-HDR-1
           WRITE REPORT-RECORD FROM WS-COLUMN-HDR-2
           MOVE 5 TO WS-LINE-COUNT
           ADD 5 TO WS-REPORT-LINES
           DISPLAY "FILES OPENED SUCCESSFULLY".

      *===============================================================
      * 2000-PROCESS-RECORDS: Main processing loop
      *===============================================================
       2000-PROCESS-RECORDS SECTION.
       2000-PROCESS-PARA.
           PERFORM UNTIL END-OF-FILE
               READ INPUT-FILE
                   AT END
                       SET END-OF-FILE TO TRUE
                   NOT AT END
                       ADD 1 TO WS-RECORDS-READ
                       PERFORM 2100-VALIDATE-RECORD
                       IF RECORD-IS-VALID
                           PERFORM 2200-CALCULATE-PAY
                           IF CALC-NO-OVERFLOW
                               PERFORM 2300-ACCUMULATE-TOTALS
                               PERFORM 2400-WRITE-REPORT
                               ADD 1 TO WS-RECORDS-PROCESSED
                           ELSE
                               SET ERR-CALC-OVERFLOW TO TRUE
                               MOVE "CALCULATION OVERFLOW"
                                   TO WS-ERROR-MESSAGE
                               PERFORM 2500-WRITE-ERROR
                               ADD 1 TO WS-RECORDS-ERROR
                           END-IF
                       ELSE
                           PERFORM 2500-WRITE-ERROR
                           ADD 1 TO WS-RECORDS-ERROR
                       END-IF
               END-READ
           END-PERFORM.

      *===============================================================
      * 2100-VALIDATE-RECORD: Validate input record fields
      *===============================================================
       2100-VALIDATE-RECORD SECTION.
       2100-VALIDATE-PARA.
           SET RECORD-IS-VALID TO TRUE
           MOVE 0 TO WS-ERROR-CODE
           MOVE SPACES TO WS-ERROR-MESSAGE
      * Validate employee ID (must be non-zero)
           IF IR-EMPLOYEE-ID = 0
               SET RECORD-IS-INVALID TO TRUE
               SET ERR-INVALID-ID TO TRUE
               MOVE "INVALID EMPLOYEE ID (ZERO)"
                   TO WS-ERROR-MESSAGE
           END-IF
      * Validate department code
           IF RECORD-IS-VALID
               MOVE 0 TO WS-DEPT-FOUND
               PERFORM VARYING WS-TABLE-IDX FROM 1 BY 1
                   UNTIL WS-TABLE-IDX > 10
                   OR DEPT-WAS-FOUND
                   IF IR-DEPARTMENT =
                       WS-DEPT-CODE(WS-TABLE-IDX)
                       SET DEPT-WAS-FOUND TO TRUE
                       MOVE WS-TABLE-IDX TO WS-DEPT-SUB
                   END-IF
               END-PERFORM
               IF DEPT-NOT-FOUND
                   SET RECORD-IS-INVALID TO TRUE
                   SET ERR-INVALID-DEPT TO TRUE
                   MOVE "UNKNOWN DEPARTMENT CODE"
                       TO WS-ERROR-MESSAGE
               END-IF
           END-IF
      * Validate pay type
           IF RECORD-IS-VALID
               IF NOT IR-HOURLY
                   AND NOT IR-SALARIED
                   AND NOT IR-COMMISSION
                   SET RECORD-IS-INVALID TO TRUE
                   SET ERR-INVALID-PAY-TYPE TO TRUE
                   MOVE "INVALID PAY TYPE CODE"
                       TO WS-ERROR-MESSAGE
               END-IF
           END-IF
      * Validate hours worked
           IF RECORD-IS-VALID
               IF IR-HOURS-WORKED > WS-MAX-HOURS-ALLOWED
                   SET RECORD-IS-INVALID TO TRUE
                   SET ERR-INVALID-HOURS TO TRUE
                   MOVE "HOURS EXCEED MAXIMUM ALLOWED"
                       TO WS-ERROR-MESSAGE
               END-IF
           END-IF
      * Validate pay rate
           IF RECORD-IS-VALID
               IF IR-HOURLY
                   IF IR-PAY-RATE < WS-MIN-WAGE
                       SET RECORD-IS-INVALID TO TRUE
                       SET ERR-INVALID-RATE TO TRUE
                       MOVE "HOURLY RATE BELOW MINIMUM"
                           TO WS-ERROR-MESSAGE
                   END-IF
               END-IF
               IF IR-PAY-RATE = 0
                   SET RECORD-IS-INVALID TO TRUE
                   SET ERR-INVALID-RATE TO TRUE
                   MOVE "PAY RATE IS ZERO"
                       TO WS-ERROR-MESSAGE
               END-IF
           END-IF.

      *===============================================================
      * 2200-CALCULATE-PAY: Calculate gross, taxes, net pay
      *===============================================================
       2200-CALCULATE-PAY SECTION.
       2200-CALC-PARA.
           MOVE 0 TO WS-GROSS-PAY
           MOVE 0 TO WS-REGULAR-PAY
           MOVE 0 TO WS-OVERTIME-PAY
           MOVE 0 TO WS-OVERTIME-HOURS
           MOVE 0 TO WS-FED-TAX
           MOVE 0 TO WS-STATE-TAX
           MOVE 0 TO WS-FICA-TAX
           MOVE 0 TO WS-TOTAL-TAX
           MOVE 0 TO WS-TOTAL-DEDUCTIONS
           MOVE 0 TO WS-NET-PAY
           MOVE 0 TO WS-COMMISSION-PAY
           SET CALC-NO-OVERFLOW TO TRUE
      * Calculate gross pay based on pay type
           EVALUATE TRUE
               WHEN IR-HOURLY
                   IF IR-HOURS-WORKED >
                       WS-MAX-REGULAR-HOURS
                       MOVE WS-MAX-REGULAR-HOURS
                           TO WS-REGULAR-HOURS
                       COMPUTE WS-OVERTIME-HOURS =
                           IR-HOURS-WORKED -
                           WS-MAX-REGULAR-HOURS
                           ON SIZE ERROR
                               SET CALC-HAS-OVERFLOW
                                   TO TRUE
                       END-COMPUTE
                   ELSE
                       MOVE IR-HOURS-WORKED
                           TO WS-REGULAR-HOURS
                       MOVE 0 TO WS-OVERTIME-HOURS
                   END-IF
                   COMPUTE WS-REGULAR-PAY =
                       WS-REGULAR-HOURS * IR-PAY-RATE
                       ON SIZE ERROR
                           SET CALC-HAS-OVERFLOW TO TRUE
                   END-COMPUTE
                   COMPUTE WS-OVERTIME-PAY =
                       WS-OVERTIME-HOURS * IR-PAY-RATE
                       * WS-OVERTIME-FACTOR
                       ON SIZE ERROR
                           SET CALC-HAS-OVERFLOW TO TRUE
                   END-COMPUTE
                   COMPUTE WS-GROSS-PAY =
                       WS-REGULAR-PAY + WS-OVERTIME-PAY
                       ON SIZE ERROR
                           SET CALC-HAS-OVERFLOW TO TRUE
                   END-COMPUTE
                   ADD 1 TO WS-HOURLY-COUNT
               WHEN IR-SALARIED
                   MOVE IR-PAY-RATE TO WS-GROSS-PAY
                   MOVE IR-HOURS-WORKED
                       TO WS-REGULAR-HOURS
                   MOVE 0 TO WS-OVERTIME-HOURS
                   ADD 1 TO WS-SALARY-COUNT
               WHEN IR-COMMISSION
                   COMPUTE WS-REGULAR-PAY =
                       IR-HOURS-WORKED * IR-PAY-RATE
                       ON SIZE ERROR
                           SET CALC-HAS-OVERFLOW TO TRUE
                   END-COMPUTE
                   COMPUTE WS-COMMISSION-PAY =
                       WS-REGULAR-PAY *
                       WS-COMMISSION-RATE
                       ON SIZE ERROR
                           SET CALC-HAS-OVERFLOW TO TRUE
                   END-COMPUTE
                   COMPUTE WS-GROSS-PAY =
                       WS-REGULAR-PAY + WS-COMMISSION-PAY
                       ON SIZE ERROR
                           SET CALC-HAS-OVERFLOW TO TRUE
                   END-COMPUTE
                   MOVE IR-HOURS-WORKED
                       TO WS-REGULAR-HOURS
                   MOVE 0 TO WS-OVERTIME-HOURS
                   ADD 1 TO WS-COMM-COUNT
               WHEN OTHER
                   MOVE 0 TO WS-GROSS-PAY
                   SET CALC-HAS-OVERFLOW TO TRUE
           END-EVALUATE
      * Skip remaining calcs if overflow occurred
           IF CALC-HAS-OVERFLOW
               EXIT SECTION
           END-IF
      * Calculate federal tax
           COMPUTE WS-FED-TAX =
               WS-GROSS-PAY * WS-FED-TAX-RATE
               ON SIZE ERROR
                   SET CALC-HAS-OVERFLOW TO TRUE
           END-COMPUTE
      * Calculate state tax
           COMPUTE WS-STATE-TAX =
               WS-GROSS-PAY * WS-STATE-TAX-RATE
               ON SIZE ERROR
                   SET CALC-HAS-OVERFLOW TO TRUE
           END-COMPUTE
      * Calculate FICA tax with wage limit check
           IF IR-YTD-GROSS < WS-FICA-WAGE-LIMIT
               COMPUTE WS-FICA-REMAINING =
                   WS-FICA-WAGE-LIMIT - IR-YTD-GROSS
               END-COMPUTE
               IF WS-GROSS-PAY > WS-FICA-REMAINING
                   MOVE WS-FICA-REMAINING
                       TO WS-TAXABLE-GROSS
               ELSE
                   MOVE WS-GROSS-PAY
                       TO WS-TAXABLE-GROSS
               END-IF
               COMPUTE WS-FICA-TAX =
                   WS-TAXABLE-GROSS * WS-FICA-RATE
                   ON SIZE ERROR
                       SET CALC-HAS-OVERFLOW TO TRUE
               END-COMPUTE
           ELSE
               MOVE 0 TO WS-FICA-TAX
           END-IF
      * Total taxes
           COMPUTE WS-TOTAL-TAX =
               WS-FED-TAX + WS-STATE-TAX + WS-FICA-TAX
               ON SIZE ERROR
                   SET CALC-HAS-OVERFLOW TO TRUE
           END-COMPUTE
      * Total deductions (taxes + voluntary)
           COMPUTE WS-TOTAL-DEDUCTIONS =
               WS-TOTAL-TAX +
               IR-HEALTH-DED +
               IR-RETIRE-DED +
               IR-OTHER-DED
               ON SIZE ERROR
                   SET CALC-HAS-OVERFLOW TO TRUE
           END-COMPUTE
      * Net pay
           COMPUTE WS-NET-PAY =
               WS-GROSS-PAY - WS-TOTAL-DEDUCTIONS
               ON SIZE ERROR
                   SET CALC-HAS-OVERFLOW TO TRUE
           END-COMPUTE
      * Check for negative net pay (warning only)
           IF WS-NET-PAY < 0
               SET ERR-NEGATIVE-NET TO TRUE
               MOVE "WARNING: NEGATIVE NET PAY"
                   TO WS-ERROR-MESSAGE
               PERFORM 2500-WRITE-ERROR
               IF WS-BATCH-RETURN-CODE < 4
                   MOVE 4 TO WS-BATCH-RETURN-CODE
               END-IF
           END-IF
      * Track min/max statistics
           IF WS-GROSS-PAY < WS-MIN-GROSS
               MOVE WS-GROSS-PAY TO WS-MIN-GROSS
           END-IF
           IF WS-GROSS-PAY > WS-MAX-GROSS
               MOVE WS-GROSS-PAY TO WS-MAX-GROSS
           END-IF
           IF WS-NET-PAY < WS-MIN-NET
               MOVE WS-NET-PAY TO WS-MIN-NET
           END-IF
           IF WS-NET-PAY > WS-MAX-NET
               MOVE WS-NET-PAY TO WS-MAX-NET
           END-IF.

      *===============================================================
      * 2300-ACCUMULATE-TOTALS: Department and grand totals
      *===============================================================
       2300-ACCUMULATE-TOTALS SECTION.
       2300-ACCUM-PARA.
      * Department totals
           ADD 1 TO WS-DEPT-EMP-COUNT(WS-DEPT-SUB)
           ADD WS-GROSS-PAY
               TO WS-DEPT-GROSS-TOT(WS-DEPT-SUB)
           ADD WS-TOTAL-TAX
               TO WS-DEPT-TAX-TOT(WS-DEPT-SUB)
           ADD WS-NET-PAY
               TO WS-DEPT-NET-TOT(WS-DEPT-SUB)
           ADD WS-TOTAL-DEDUCTIONS
               TO WS-DEPT-DED-TOT(WS-DEPT-SUB)
           ADD IR-HOURS-WORKED
               TO WS-DEPT-HOURS-TOT(WS-DEPT-SUB)
           ADD WS-OVERTIME-HOURS
               TO WS-DEPT-OT-HOURS(WS-DEPT-SUB)
      * Grand totals
           ADD 1 TO WS-GRAND-EMP-COUNT
           ADD WS-GROSS-PAY TO WS-GRAND-GROSS
           ADD WS-TOTAL-TAX TO WS-GRAND-TAX
           ADD WS-NET-PAY TO WS-GRAND-NET
           ADD WS-TOTAL-DEDUCTIONS
               TO WS-GRAND-DEDUCTIONS
           ADD IR-HOURS-WORKED TO WS-GRAND-HOURS
           ADD WS-OVERTIME-HOURS TO WS-GRAND-OT-HOURS
      * Pay type counts for grand totals
           EVALUATE TRUE
               WHEN IR-HOURLY
                   ADD 1 TO WS-GRAND-HOURLY-CT
               WHEN IR-SALARIED
                   ADD 1 TO WS-GRAND-SALARY-CT
               WHEN IR-COMMISSION
                   ADD 1 TO WS-GRAND-COMM-CT
           END-EVALUATE.

      *===============================================================
      * 2400-WRITE-REPORT: Format and write detail line
      *===============================================================
       2400-WRITE-REPORT SECTION.
       2400-WRITE-PARA.
      * Check for page break
           IF WS-LINE-COUNT >= WS-LINES-PER-PAGE
               ADD 1 TO WS-PAGE-COUNT
               MOVE WS-PAGE-COUNT TO WS-HDR-PAGE
               WRITE REPORT-RECORD FROM WS-BLANK-LINE
               WRITE REPORT-RECORD FROM WS-HEADER-LINE-1
               WRITE REPORT-RECORD FROM WS-HEADER-LINE-2
               WRITE REPORT-RECORD FROM WS-BLANK-LINE
               WRITE REPORT-RECORD FROM WS-COLUMN-HDR-1
               WRITE REPORT-RECORD FROM WS-COLUMN-HDR-2
               MOVE 5 TO WS-LINE-COUNT
               ADD 6 TO WS-REPORT-LINES
           END-IF
      * Clean the employee name using INSPECT
           MOVE IR-LAST-NAME TO WS-CLEANED-NAME
           INSPECT WS-CLEANED-NAME
               REPLACING ALL LOW-VALUES BY SPACES
      * Build display name: "LAST, FIRST"
           MOVE SPACES TO WS-DISPLAY-NAME
           MOVE 1 TO WS-STRING-PTR
           STRING WS-CLEANED-NAME DELIMITED SPACES
                  ", " DELIMITED SIZE
                  IR-FIRST-NAME DELIMITED SPACES
               INTO WS-DISPLAY-NAME
               WITH POINTER WS-STRING-PTR
           END-STRING
      * Format detail line
           MOVE IR-EMPLOYEE-ID TO WS-DET-EMP-ID
           MOVE WS-DISPLAY-NAME(1:20) TO WS-DET-NAME
           MOVE IR-DEPARTMENT TO WS-DET-DEPT
           MOVE IR-PAY-TYPE TO WS-DET-TYPE
           MOVE IR-HOURS-WORKED TO WS-DET-HOURS
           MOVE IR-PAY-RATE TO WS-DET-RATE
           MOVE WS-GROSS-PAY TO WS-DET-GROSS
           MOVE WS-TOTAL-TAX TO WS-DET-TAX
           MOVE WS-TOTAL-DEDUCTIONS TO WS-DET-DED
           MOVE WS-NET-PAY TO WS-DET-NET
           WRITE REPORT-RECORD FROM WS-DETAIL-LINE
           ADD 1 TO WS-LINE-COUNT
           ADD 1 TO WS-REPORT-LINES
           ADD 1 TO WS-RECORDS-WRITTEN.

      *===============================================================
      * 2500-WRITE-ERROR: Write error log record
      *===============================================================
       2500-WRITE-ERROR SECTION.
       2500-ERROR-PARA.
           MOVE SPACES TO WS-ERROR-LOG-REC
           MOVE WS-FORMATTED-DATE TO WS-ERR-DATE
           MOVE WS-FORMATTED-TIME TO WS-ERR-TIME
           MOVE IR-EMPLOYEE-ID TO WS-ERR-EMP-ID
           MOVE WS-ERROR-CODE TO WS-ERR-CODE
           MOVE WS-ERROR-MESSAGE TO WS-ERR-MSG
      * Build error field info using STRING
           MOVE SPACES TO WS-ERR-FIELD
           MOVE 1 TO WS-STRING-PTR
           EVALUATE TRUE
               WHEN ERR-INVALID-ID
                   STRING "ID=" DELIMITED SIZE
                          IR-EMPLOYEE-ID DELIMITED SIZE
                       INTO WS-ERR-FIELD
                       WITH POINTER WS-STRING-PTR
                   END-STRING
               WHEN ERR-INVALID-DEPT
                   STRING "DEPT=" DELIMITED SIZE
                          IR-DEPARTMENT DELIMITED SIZE
                       INTO WS-ERR-FIELD
                       WITH POINTER WS-STRING-PTR
                   END-STRING
               WHEN ERR-INVALID-HOURS
                   MOVE IR-HOURS-WORKED TO WS-EDIT-HOURS
                   STRING "HRS=" DELIMITED SIZE
                          WS-EDIT-HOURS DELIMITED SIZE
                       INTO WS-ERR-FIELD
                       WITH POINTER WS-STRING-PTR
                   END-STRING
               WHEN ERR-INVALID-RATE
                   MOVE IR-PAY-RATE TO WS-EDIT-RATE
                   STRING "RATE=" DELIMITED SIZE
                          WS-EDIT-RATE DELIMITED SIZE
                       INTO WS-ERR-FIELD
                       WITH POINTER WS-STRING-PTR
                   END-STRING
               WHEN ERR-INVALID-PAY-TYPE
                   STRING "TYPE=" DELIMITED SIZE
                          IR-PAY-TYPE DELIMITED SIZE
                       INTO WS-ERR-FIELD
                       WITH POINTER WS-STRING-PTR
                   END-STRING
               WHEN ERR-CALC-OVERFLOW
                   MOVE "OVERFLOW" TO WS-ERR-FIELD
               WHEN ERR-NEGATIVE-NET
                   MOVE WS-NET-PAY TO WS-EDIT-NET
                   STRING "NET=" DELIMITED SIZE
                          WS-EDIT-NET DELIMITED SIZE
                       INTO WS-ERR-FIELD
                       WITH POINTER WS-STRING-PTR
                   END-STRING
               WHEN OTHER
                   MOVE "UNKNOWN" TO WS-ERR-FIELD
           END-EVALUATE
           WRITE ERROR-RECORD FROM WS-ERROR-LOG-REC
           ADD 1 TO WS-ERROR-LINES
           IF WS-BATCH-RETURN-CODE < 8
               IF NOT ERR-NEGATIVE-NET
                   MOVE 8 TO WS-BATCH-RETURN-CODE
               END-IF
           END-IF.

      *===============================================================
      * 3000-PRINT-SUMMARY: Print department and grand totals
      *===============================================================
       3000-PRINT-SUMMARY SECTION.
       3000-SUMMARY-PARA.
      * Force new page for summary
           ADD 1 TO WS-PAGE-COUNT
           MOVE WS-PAGE-COUNT TO WS-HDR-PAGE
           WRITE REPORT-RECORD FROM WS-BLANK-LINE
           WRITE REPORT-RECORD FROM WS-HEADER-LINE-1
           WRITE REPORT-RECORD FROM WS-HEADER-LINE-2
           WRITE REPORT-RECORD FROM WS-BLANK-LINE
           ADD 4 TO WS-REPORT-LINES
      * Summary title
           MOVE SPACES TO REPORT-RECORD
           MOVE "DEPARTMENT SUMMARY" TO REPORT-RECORD
           WRITE REPORT-RECORD
           WRITE REPORT-RECORD FROM WS-SEPARATOR-LINE
           ADD 2 TO WS-REPORT-LINES
      * Print department totals using PERFORM VARYING
           PERFORM VARYING WS-DEPT-IDX FROM 1 BY 1
               UNTIL WS-DEPT-IDX > 10
               IF WS-DEPT-EMP-COUNT(WS-DEPT-IDX) > 0
                   MOVE SPACES TO WS-DEPT-SUMMARY-LINE
                   MOVE WS-DEPT-CODE(WS-DEPT-IDX)
                       TO WS-SUM-DEPT-CODE
                   MOVE WS-DEPT-NAME(WS-DEPT-IDX)
                       TO WS-SUM-DEPT-NAME
                   MOVE WS-DEPT-EMP-COUNT(WS-DEPT-IDX)
                       TO WS-SUM-EMP-CT
                   MOVE WS-DEPT-GROSS-TOT(WS-DEPT-IDX)
                       TO WS-SUM-GROSS
                   MOVE WS-DEPT-NET-TOT(WS-DEPT-IDX)
                       TO WS-SUM-NET
                   WRITE REPORT-RECORD
                       FROM WS-DEPT-SUMMARY-LINE
                   ADD 1 TO WS-REPORT-LINES
      * Department detail sub-line
                   MOVE SPACES TO REPORT-RECORD
                   MOVE WS-DEPT-TAX-TOT(WS-DEPT-IDX)
                       TO WS-EDIT-TAX
                   MOVE WS-DEPT-DED-TOT(WS-DEPT-IDX)
                       TO WS-EDIT-DED
                   MOVE WS-DEPT-HOURS-TOT(WS-DEPT-IDX)
                       TO WS-EDIT-HOURS
                   MOVE WS-DEPT-OT-HOURS(WS-DEPT-IDX)
                       TO WS-EDIT-OT-HOURS
                   MOVE 1 TO WS-STRING-PTR
                   STRING
                       "     TAX: " DELIMITED SIZE
                       WS-EDIT-TAX DELIMITED SIZE
                       "  DED: " DELIMITED SIZE
                       WS-EDIT-DED DELIMITED SIZE
                       "  HRS: " DELIMITED SIZE
                       WS-EDIT-HOURS DELIMITED SIZE
                       "  OT: " DELIMITED SIZE
                       WS-EDIT-OT-HOURS DELIMITED SIZE
                       INTO WS-TEMP-STRING
                       WITH POINTER WS-STRING-PTR
                   END-STRING
                   MOVE WS-TEMP-STRING TO REPORT-RECORD
                   WRITE REPORT-RECORD
                   ADD 1 TO WS-REPORT-LINES
                   MOVE SPACES TO WS-TEMP-STRING
               END-IF
           END-PERFORM
      * Grand totals
           WRITE REPORT-RECORD FROM WS-BLANK-LINE
           WRITE REPORT-RECORD FROM WS-SEPARATOR-LINE
           ADD 2 TO WS-REPORT-LINES
           MOVE SPACES TO WS-GRAND-TOTAL-LINE
           MOVE "GRAND TOTALS: " TO WS-GRAND-TOTAL-LINE
           MOVE WS-GRAND-EMP-COUNT TO WS-GT-EMP-CT
           MOVE WS-GRAND-GROSS TO WS-GT-GROSS
           MOVE WS-GRAND-TAX TO WS-GT-TAX
           MOVE WS-GRAND-NET TO WS-GT-NET
           WRITE REPORT-RECORD FROM WS-GRAND-TOTAL-LINE
           ADD 1 TO WS-REPORT-LINES
      * Additional totals detail
           MOVE WS-GRAND-DEDUCTIONS TO WS-EDIT-TOTAL
           MOVE WS-GRAND-HOURS TO WS-EDIT-HOURS
           MOVE 1 TO WS-STRING-PTR
           STRING
               "TOTAL DEDUCTIONS: " DELIMITED SIZE
               WS-EDIT-TOTAL DELIMITED SIZE
               "  TOTAL HOURS: " DELIMITED SIZE
               WS-EDIT-HOURS DELIMITED SIZE
               INTO WS-TEMP-STRING
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE WS-TEMP-STRING TO REPORT-RECORD
           WRITE REPORT-RECORD
           ADD 1 TO WS-REPORT-LINES
           MOVE SPACES TO WS-TEMP-STRING
      * Pay type breakdown
           MOVE WS-GRAND-HOURLY-CT TO WS-EDIT-COUNT
           MOVE 1 TO WS-STRING-PTR
           STRING
               "HOURLY: " DELIMITED SIZE
               WS-EDIT-COUNT DELIMITED SIZE
               INTO WS-TEMP-STRING
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE WS-GRAND-SALARY-CT TO WS-EDIT-COUNT
           STRING
               "  SALARY: " DELIMITED SIZE
               WS-EDIT-COUNT DELIMITED SIZE
               INTO WS-TEMP-STRING
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE WS-GRAND-COMM-CT TO WS-EDIT-COUNT
           STRING
               "  COMMISSION: " DELIMITED SIZE
               WS-EDIT-COUNT DELIMITED SIZE
               INTO WS-TEMP-STRING
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE WS-TEMP-STRING TO REPORT-RECORD
           WRITE REPORT-RECORD
           ADD 1 TO WS-REPORT-LINES
           MOVE SPACES TO WS-TEMP-STRING
      * Average calculations
           IF WS-GRAND-EMP-COUNT > 0
               COMPUTE WS-AVG-GROSS =
                   WS-GRAND-GROSS / WS-GRAND-EMP-COUNT
                   ON SIZE ERROR
                       MOVE 0 TO WS-AVG-GROSS
               END-COMPUTE
               COMPUTE WS-AVG-NET =
                   WS-GRAND-NET / WS-GRAND-EMP-COUNT
                   ON SIZE ERROR
                       MOVE 0 TO WS-AVG-NET
               END-COMPUTE
               MOVE WS-AVG-GROSS TO WS-EDIT-GROSS
               MOVE WS-AVG-NET TO WS-EDIT-NET
               MOVE 1 TO WS-STRING-PTR
               STRING
                   "AVERAGES -- GROSS: " DELIMITED SIZE
                   WS-EDIT-GROSS DELIMITED SIZE
                   "  NET: " DELIMITED SIZE
                   WS-EDIT-NET DELIMITED SIZE
                   INTO WS-TEMP-STRING
                   WITH POINTER WS-STRING-PTR
               END-STRING
               MOVE WS-TEMP-STRING TO REPORT-RECORD
               WRITE REPORT-RECORD
               ADD 1 TO WS-REPORT-LINES
               MOVE SPACES TO WS-TEMP-STRING
           END-IF
      * Processing statistics
           WRITE REPORT-RECORD FROM WS-SEPARATOR-LINE
           MOVE WS-RECORDS-READ TO WS-EDIT-COUNT
           MOVE 1 TO WS-STRING-PTR
           STRING
               "RECORDS READ: " DELIMITED SIZE
               WS-EDIT-COUNT DELIMITED SIZE
               INTO WS-TEMP-STRING
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE WS-RECORDS-PROCESSED TO WS-EDIT-COUNT
           STRING
               "  PROCESSED: " DELIMITED SIZE
               WS-EDIT-COUNT DELIMITED SIZE
               INTO WS-TEMP-STRING
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE WS-RECORDS-ERROR TO WS-EDIT-COUNT
           STRING
               "  ERRORS: " DELIMITED SIZE
               WS-EDIT-COUNT DELIMITED SIZE
               INTO WS-TEMP-STRING
               WITH POINTER WS-STRING-PTR
           END-STRING
           MOVE WS-TEMP-STRING TO REPORT-RECORD
           WRITE REPORT-RECORD
           ADD 2 TO WS-REPORT-LINES
           MOVE SPACES TO WS-TEMP-STRING
      * Batch return code
           MOVE SPACES TO REPORT-RECORD
           EVALUATE TRUE
               WHEN BATCH-SUCCESS
                   MOVE "RC=0 (SUCCESS)"
                       TO REPORT-RECORD
               WHEN BATCH-WARNING
                   MOVE "RC=4 (WARNING)"
                       TO REPORT-RECORD
               WHEN BATCH-ERROR
                   MOVE "RC=8 (ERROR)"
                       TO REPORT-RECORD
               WHEN BATCH-SEVERE
                   MOVE "RC=12 (SEVERE)"
                       TO REPORT-RECORD
           END-EVALUATE
           WRITE REPORT-RECORD
           ADD 1 TO WS-REPORT-LINES
      * Final separator
           WRITE REPORT-RECORD FROM WS-SEPARATOR-LINE
           MOVE SPACES TO REPORT-RECORD
           MOVE "*** END OF REPORT ***" TO REPORT-RECORD
           WRITE REPORT-RECORD
           ADD 2 TO WS-REPORT-LINES.

      *===============================================================
      * 9000-TERMINATE: Close files and display final stats
      *===============================================================
       9000-TERMINATE SECTION.
       9000-TERM-PARA.
      * Close all files
           CLOSE INPUT-FILE
           IF NOT INPUT-OK
               IF NOT INPUT-EOF
                   DISPLAY "WARNING: CLOSE INPUT-FILE: "
                       WS-INPUT-STATUS
               END-IF
           END-IF
           CLOSE REPORT-FILE
           IF NOT REPORT-OK
               DISPLAY "WARNING: CLOSE REPORT-FILE: "
                   WS-REPORT-STATUS
           END-IF
           CLOSE ERROR-FILE
           IF NOT ERROR-OK
               DISPLAY "WARNING: CLOSE ERROR-FILE: "
                   WS-ERROR-STATUS
           END-IF
      * Display final statistics to console
           DISPLAY "========================================="
           DISPLAY "  PAYROLL BATCH PROCESSING COMPLETE"
           DISPLAY "========================================="
           MOVE WS-RECORDS-READ TO WS-EDIT-COUNT
           DISPLAY "RECORDS READ:      " WS-EDIT-COUNT
           MOVE WS-RECORDS-PROCESSED TO WS-EDIT-COUNT
           DISPLAY "RECORDS PROCESSED: " WS-EDIT-COUNT
           MOVE WS-RECORDS-ERROR TO WS-EDIT-COUNT
           DISPLAY "RECORDS IN ERROR:  " WS-EDIT-COUNT
           MOVE WS-RECORDS-WRITTEN TO WS-EDIT-COUNT
           DISPLAY "REPORT LINES:      " WS-EDIT-COUNT
           MOVE WS-ERROR-LINES TO WS-EDIT-COUNT
           DISPLAY "ERROR LOG ENTRIES: " WS-EDIT-COUNT
           MOVE WS-GRAND-GROSS TO WS-EDIT-TOTAL
           DISPLAY "TOTAL GROSS PAY:   " WS-EDIT-TOTAL
           MOVE WS-GRAND-TAX TO WS-EDIT-TOTAL
           DISPLAY "TOTAL TAXES:       " WS-EDIT-TOTAL
           MOVE WS-GRAND-NET TO WS-EDIT-TOTAL
           DISPLAY "TOTAL NET PAY:     " WS-EDIT-TOTAL
           DISPLAY "BATCH ID:     " WS-BATCH-ID
           DISPLAY "BATCH DATE:   " WS-BATCH-DATE
           DISPLAY "RETURN CODE:  " WS-BATCH-RETURN-CODE
           EVALUATE TRUE
               WHEN BATCH-SUCCESS
                   DISPLAY "STATUS: SUCCESSFUL COMPLETION"
               WHEN BATCH-WARNING
                   DISPLAY "STATUS: COMPLETED WITH WARNINGS"
               WHEN BATCH-ERROR
                   DISPLAY "STATUS: COMPLETED WITH ERRORS"
               WHEN BATCH-SEVERE
                   DISPLAY "STATUS: SEVERE ERRORS - ABORTED"
           END-EVALUATE
           MOVE "C" TO WS-BATCH-STATUS
           DISPLAY "=========================================".
