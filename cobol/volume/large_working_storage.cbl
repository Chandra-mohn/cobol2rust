      *************************************************************
      * LARGE-WS-TEST.CBL
      * Volume stress test for COBOL-to-Rust transpiler.
      * Tests 500+ working-storage fields including:
      *   - PIC X alphanumeric fields (100)
      *   - PIC 9 numeric fields (100)
      *   - PIC S9 signed numeric fields (50)
      *   - COMP binary fields (30)
      *   - COMP-3 packed decimal fields (30)
      *   - COMP-5 native binary fields (20)
      *   - Numeric edited fields (20)
      *   - Group records with children (25 groups, ~92 children)
      *   - Level 77 standalone items (15)
      *   - PIC X flag fields (25)
      *   Total: 507+ fields
      *************************************************************
       IDENTIFICATION DIVISION.
       PROGRAM-ID. LARGE-WS-TEST.
       AUTHOR. COBOL2RUST-TEST.

       ENVIRONMENT DIVISION.

       DATA DIVISION.
       WORKING-STORAGE SECTION.

      * ---- PIC X ALPHANUMERIC FIELDS (100) ----
       01  WS-ALPHA-001    PIC X(05)
           VALUE "ALP01".
       01  WS-ALPHA-002    PIC X(10)
           VALUE "ALPHA002".
       01  WS-ALPHA-003    PIC X(15)
           VALUE "ALPHA003".
       01  WS-ALPHA-004    PIC X(20)
           VALUE "ALPHA004".
       01  WS-ALPHA-005    PIC X(30)
           VALUE "ALPHA005".
       01  WS-ALPHA-006    PIC X(05)
           VALUE "ALP06".
       01  WS-ALPHA-007    PIC X(10)
           VALUE "ALPHA007".
       01  WS-ALPHA-008    PIC X(15)
           VALUE "ALPHA008".
       01  WS-ALPHA-009    PIC X(20)
           VALUE "ALPHA009".
       01  WS-ALPHA-010    PIC X(30)
           VALUE "ALPHA010".
       01  WS-ALPHA-011    PIC X(05).
       01  WS-ALPHA-012    PIC X(10).
       01  WS-ALPHA-013    PIC X(15).
       01  WS-ALPHA-014    PIC X(20).
       01  WS-ALPHA-015    PIC X(30).
       01  WS-ALPHA-016    PIC X(05).
       01  WS-ALPHA-017    PIC X(10).
       01  WS-ALPHA-018    PIC X(15).
       01  WS-ALPHA-019    PIC X(20).
       01  WS-ALPHA-020    PIC X(30).
       01  WS-ALPHA-021    PIC X(05).
       01  WS-ALPHA-022    PIC X(10).
       01  WS-ALPHA-023    PIC X(15).
       01  WS-ALPHA-024    PIC X(20).
       01  WS-ALPHA-025    PIC X(30).
       01  WS-ALPHA-026    PIC X(05).
       01  WS-ALPHA-027    PIC X(10).
       01  WS-ALPHA-028    PIC X(15).
       01  WS-ALPHA-029    PIC X(20).
       01  WS-ALPHA-030    PIC X(30).
       01  WS-ALPHA-031    PIC X(05).
       01  WS-ALPHA-032    PIC X(10).
       01  WS-ALPHA-033    PIC X(15).
       01  WS-ALPHA-034    PIC X(20).
       01  WS-ALPHA-035    PIC X(30).
       01  WS-ALPHA-036    PIC X(05).
       01  WS-ALPHA-037    PIC X(10).
       01  WS-ALPHA-038    PIC X(15).
       01  WS-ALPHA-039    PIC X(20).
       01  WS-ALPHA-040    PIC X(30).
       01  WS-ALPHA-041    PIC X(05).
       01  WS-ALPHA-042    PIC X(10).
       01  WS-ALPHA-043    PIC X(15).
       01  WS-ALPHA-044    PIC X(20).
       01  WS-ALPHA-045    PIC X(30).
       01  WS-ALPHA-046    PIC X(05).
       01  WS-ALPHA-047    PIC X(10).
       01  WS-ALPHA-048    PIC X(15).
       01  WS-ALPHA-049    PIC X(20).
       01  WS-ALPHA-050    PIC X(30).
       01  WS-ALPHA-051    PIC X(05).
       01  WS-ALPHA-052    PIC X(10).
       01  WS-ALPHA-053    PIC X(15).
       01  WS-ALPHA-054    PIC X(20).
       01  WS-ALPHA-055    PIC X(30).
       01  WS-ALPHA-056    PIC X(05).
       01  WS-ALPHA-057    PIC X(10).
       01  WS-ALPHA-058    PIC X(15).
       01  WS-ALPHA-059    PIC X(20).
       01  WS-ALPHA-060    PIC X(30).
       01  WS-ALPHA-061    PIC X(05).
       01  WS-ALPHA-062    PIC X(10).
       01  WS-ALPHA-063    PIC X(15).
       01  WS-ALPHA-064    PIC X(20).
       01  WS-ALPHA-065    PIC X(30).
       01  WS-ALPHA-066    PIC X(05).
       01  WS-ALPHA-067    PIC X(10).
       01  WS-ALPHA-068    PIC X(15).
       01  WS-ALPHA-069    PIC X(20).
       01  WS-ALPHA-070    PIC X(30).
       01  WS-ALPHA-071    PIC X(05).
       01  WS-ALPHA-072    PIC X(10).
       01  WS-ALPHA-073    PIC X(15).
       01  WS-ALPHA-074    PIC X(20).
       01  WS-ALPHA-075    PIC X(30).
       01  WS-ALPHA-076    PIC X(05).
       01  WS-ALPHA-077    PIC X(10).
       01  WS-ALPHA-078    PIC X(15).
       01  WS-ALPHA-079    PIC X(20).
       01  WS-ALPHA-080    PIC X(30).
       01  WS-ALPHA-081    PIC X(05).
       01  WS-ALPHA-082    PIC X(10).
       01  WS-ALPHA-083    PIC X(15).
       01  WS-ALPHA-084    PIC X(20).
       01  WS-ALPHA-085    PIC X(30).
       01  WS-ALPHA-086    PIC X(05).
       01  WS-ALPHA-087    PIC X(10).
       01  WS-ALPHA-088    PIC X(15).
       01  WS-ALPHA-089    PIC X(20).
       01  WS-ALPHA-090    PIC X(30).
       01  WS-ALPHA-091    PIC X(05).
       01  WS-ALPHA-092    PIC X(10).
       01  WS-ALPHA-093    PIC X(15).
       01  WS-ALPHA-094    PIC X(20).
       01  WS-ALPHA-095    PIC X(30).
       01  WS-ALPHA-096    PIC X(05).
       01  WS-ALPHA-097    PIC X(10).
       01  WS-ALPHA-098    PIC X(15).
       01  WS-ALPHA-099    PIC X(20).
       01  WS-ALPHA-100    PIC X(30).

      * ---- PIC 9 NUMERIC FIELDS (100) ----
       01  WS-NUM-001         PIC 9 VALUE 1.
       01  WS-NUM-002         PIC 9(05) VALUE 200.
       01  WS-NUM-003         PIC 9(09)
           VALUE 30000.
       01  WS-NUM-004         PIC 9(05)V99
           VALUE 40.50.
       01  WS-NUM-005         PIC 9 VALUE 5.
       01  WS-NUM-006         PIC 9(05) VALUE 600.
       01  WS-NUM-007         PIC 9(09)
           VALUE 70000.
       01  WS-NUM-008         PIC 9(05)V99
           VALUE 80.50.
       01  WS-NUM-009         PIC 9.
       01  WS-NUM-010         PIC 9(05).
       01  WS-NUM-011         PIC 9(09).
       01  WS-NUM-012         PIC 9(05)V99.
       01  WS-NUM-013         PIC 9.
       01  WS-NUM-014         PIC 9(05).
       01  WS-NUM-015         PIC 9(09).
       01  WS-NUM-016         PIC 9(05)V99.
       01  WS-NUM-017         PIC 9.
       01  WS-NUM-018         PIC 9(05).
       01  WS-NUM-019         PIC 9(09).
       01  WS-NUM-020         PIC 9(05)V99.
       01  WS-NUM-021         PIC 9.
       01  WS-NUM-022         PIC 9(05).
       01  WS-NUM-023         PIC 9(09).
       01  WS-NUM-024         PIC 9(05)V99.
       01  WS-NUM-025         PIC 9.
       01  WS-NUM-026         PIC 9(05).
       01  WS-NUM-027         PIC 9(09).
       01  WS-NUM-028         PIC 9(05)V99.
       01  WS-NUM-029         PIC 9.
       01  WS-NUM-030         PIC 9(05).
       01  WS-NUM-031         PIC 9(09).
       01  WS-NUM-032         PIC 9(05)V99.
       01  WS-NUM-033         PIC 9.
       01  WS-NUM-034         PIC 9(05).
       01  WS-NUM-035         PIC 9(09).
       01  WS-NUM-036         PIC 9(05)V99.
       01  WS-NUM-037         PIC 9.
       01  WS-NUM-038         PIC 9(05).
       01  WS-NUM-039         PIC 9(09).
       01  WS-NUM-040         PIC 9(05)V99.
       01  WS-NUM-041         PIC 9.
       01  WS-NUM-042         PIC 9(05).
       01  WS-NUM-043         PIC 9(09).
       01  WS-NUM-044         PIC 9(05)V99.
       01  WS-NUM-045         PIC 9.
       01  WS-NUM-046         PIC 9(05).
       01  WS-NUM-047         PIC 9(09).
       01  WS-NUM-048         PIC 9(05)V99.
       01  WS-NUM-049         PIC 9.
       01  WS-NUM-050         PIC 9(05).
       01  WS-NUM-051         PIC 9(09).
       01  WS-NUM-052         PIC 9(05)V99.
       01  WS-NUM-053         PIC 9.
       01  WS-NUM-054         PIC 9(05).
       01  WS-NUM-055         PIC 9(09).
       01  WS-NUM-056         PIC 9(05)V99.
       01  WS-NUM-057         PIC 9.
       01  WS-NUM-058         PIC 9(05).
       01  WS-NUM-059         PIC 9(09).
       01  WS-NUM-060         PIC 9(05)V99.
       01  WS-NUM-061         PIC 9.
       01  WS-NUM-062         PIC 9(05).
       01  WS-NUM-063         PIC 9(09).
       01  WS-NUM-064         PIC 9(05)V99.
       01  WS-NUM-065         PIC 9.
       01  WS-NUM-066         PIC 9(05).
       01  WS-NUM-067         PIC 9(09).
       01  WS-NUM-068         PIC 9(05)V99.
       01  WS-NUM-069         PIC 9.
       01  WS-NUM-070         PIC 9(05).
       01  WS-NUM-071         PIC 9(09).
       01  WS-NUM-072         PIC 9(05)V99.
       01  WS-NUM-073         PIC 9.
       01  WS-NUM-074         PIC 9(05).
       01  WS-NUM-075         PIC 9(09).
       01  WS-NUM-076         PIC 9(05)V99.
       01  WS-NUM-077         PIC 9.
       01  WS-NUM-078         PIC 9(05).
       01  WS-NUM-079         PIC 9(09).
       01  WS-NUM-080         PIC 9(05)V99.
       01  WS-NUM-081         PIC 9.
       01  WS-NUM-082         PIC 9(05).
       01  WS-NUM-083         PIC 9(09).
       01  WS-NUM-084         PIC 9(05)V99.
       01  WS-NUM-085         PIC 9.
       01  WS-NUM-086         PIC 9(05).
       01  WS-NUM-087         PIC 9(09).
       01  WS-NUM-088         PIC 9(05)V99.
       01  WS-NUM-089         PIC 9.
       01  WS-NUM-090         PIC 9(05).
       01  WS-NUM-091         PIC 9(09).
       01  WS-NUM-092         PIC 9(05)V99.
       01  WS-NUM-093         PIC 9.
       01  WS-NUM-094         PIC 9(05).
       01  WS-NUM-095         PIC 9(09).
       01  WS-NUM-096         PIC 9(05)V99.
       01  WS-NUM-097         PIC 9.
       01  WS-NUM-098         PIC 9(05).
       01  WS-NUM-099         PIC 9(09).
       01  WS-NUM-100         PIC 9(05)V99.

      * ---- PIC S9 SIGNED FIELDS (50) ----
       01  WS-SIGN-001        PIC S9(05) VALUE -10.
       01  WS-SIGN-002        PIC S9(09) VALUE -20.
       01  WS-SIGN-003        PIC S9(07)V99 VALUE -30.
       01  WS-SIGN-004        PIC S9(05) VALUE -40.
       01  WS-SIGN-005        PIC S9(09) VALUE -50.
       01  WS-SIGN-006        PIC S9(07)V99.
       01  WS-SIGN-007        PIC S9(05).
       01  WS-SIGN-008        PIC S9(09).
       01  WS-SIGN-009        PIC S9(07)V99.
       01  WS-SIGN-010        PIC S9(05).
       01  WS-SIGN-011        PIC S9(09).
       01  WS-SIGN-012        PIC S9(07)V99.
       01  WS-SIGN-013        PIC S9(05).
       01  WS-SIGN-014        PIC S9(09).
       01  WS-SIGN-015        PIC S9(07)V99.
       01  WS-SIGN-016        PIC S9(05).
       01  WS-SIGN-017        PIC S9(09).
       01  WS-SIGN-018        PIC S9(07)V99.
       01  WS-SIGN-019        PIC S9(05).
       01  WS-SIGN-020        PIC S9(09).
       01  WS-SIGN-021        PIC S9(07)V99.
       01  WS-SIGN-022        PIC S9(05).
       01  WS-SIGN-023        PIC S9(09).
       01  WS-SIGN-024        PIC S9(07)V99.
       01  WS-SIGN-025        PIC S9(05).
       01  WS-SIGN-026        PIC S9(09).
       01  WS-SIGN-027        PIC S9(07)V99.
       01  WS-SIGN-028        PIC S9(05).
       01  WS-SIGN-029        PIC S9(09).
       01  WS-SIGN-030        PIC S9(07)V99.
       01  WS-SIGN-031        PIC S9(05).
       01  WS-SIGN-032        PIC S9(09).
       01  WS-SIGN-033        PIC S9(07)V99.
       01  WS-SIGN-034        PIC S9(05).
       01  WS-SIGN-035        PIC S9(09).
       01  WS-SIGN-036        PIC S9(07)V99.
       01  WS-SIGN-037        PIC S9(05).
       01  WS-SIGN-038        PIC S9(09).
       01  WS-SIGN-039        PIC S9(07)V99.
       01  WS-SIGN-040        PIC S9(05).
       01  WS-SIGN-041        PIC S9(09).
       01  WS-SIGN-042        PIC S9(07)V99.
       01  WS-SIGN-043        PIC S9(05).
       01  WS-SIGN-044        PIC S9(09).
       01  WS-SIGN-045        PIC S9(07)V99.
       01  WS-SIGN-046        PIC S9(05).
       01  WS-SIGN-047        PIC S9(09).
       01  WS-SIGN-048        PIC S9(07)V99.
       01  WS-SIGN-049        PIC S9(05).
       01  WS-SIGN-050        PIC S9(09).

      * ---- COMP BINARY FIELDS (30) ----
       01  WS-COMP-001        PIC S9(04) COMP
           VALUE 100.
       01  WS-COMP-002        PIC S9(09) COMP
           VALUE 200.
       01  WS-COMP-003        PIC S9(18) COMP
           VALUE 300.
       01  WS-COMP-004        PIC S9(04) COMP
           VALUE 400.
       01  WS-COMP-005        PIC S9(09) COMP
           VALUE 500.
       01  WS-COMP-006        PIC S9(18) COMP.
       01  WS-COMP-007        PIC S9(04) COMP.
       01  WS-COMP-008        PIC S9(09) COMP.
       01  WS-COMP-009        PIC S9(18) COMP.
       01  WS-COMP-010        PIC S9(04) COMP.
       01  WS-COMP-011        PIC S9(09) COMP.
       01  WS-COMP-012        PIC S9(18) COMP.
       01  WS-COMP-013        PIC S9(04) COMP.
       01  WS-COMP-014        PIC S9(09) COMP.
       01  WS-COMP-015        PIC S9(18) COMP.
       01  WS-COMP-016        PIC S9(04) COMP.
       01  WS-COMP-017        PIC S9(09) COMP.
       01  WS-COMP-018        PIC S9(18) COMP.
       01  WS-COMP-019        PIC S9(04) COMP.
       01  WS-COMP-020        PIC S9(09) COMP.
       01  WS-COMP-021        PIC S9(18) COMP.
       01  WS-COMP-022        PIC S9(04) COMP.
       01  WS-COMP-023        PIC S9(09) COMP.
       01  WS-COMP-024        PIC S9(18) COMP.
       01  WS-COMP-025        PIC S9(04) COMP.
       01  WS-COMP-026        PIC S9(09) COMP.
       01  WS-COMP-027        PIC S9(18) COMP.
       01  WS-COMP-028        PIC S9(04) COMP.
       01  WS-COMP-029        PIC S9(09) COMP.
       01  WS-COMP-030        PIC S9(18) COMP.

      * ---- COMP-3 PACKED DECIMAL FIELDS (30) ----
       01  WS-COMP3-001       PIC S9(05) COMP-3
           VALUE 50.
       01  WS-COMP3-002       PIC S9(09)V99 COMP-3
           VALUE 100.
       01  WS-COMP3-003       PIC S9(15) COMP-3
           VALUE 150.
       01  WS-COMP3-004       PIC S9(05) COMP-3
           VALUE 200.
       01  WS-COMP3-005       PIC S9(09)V99 COMP-3
           VALUE 250.
       01  WS-COMP3-006       PIC S9(15) COMP-3.
       01  WS-COMP3-007       PIC S9(05) COMP-3.
       01  WS-COMP3-008       PIC S9(09)V99 COMP-3.
       01  WS-COMP3-009       PIC S9(15) COMP-3.
       01  WS-COMP3-010       PIC S9(05) COMP-3.
       01  WS-COMP3-011       PIC S9(09)V99 COMP-3.
       01  WS-COMP3-012       PIC S9(15) COMP-3.
       01  WS-COMP3-013       PIC S9(05) COMP-3.
       01  WS-COMP3-014       PIC S9(09)V99 COMP-3.
       01  WS-COMP3-015       PIC S9(15) COMP-3.
       01  WS-COMP3-016       PIC S9(05) COMP-3.
       01  WS-COMP3-017       PIC S9(09)V99 COMP-3.
       01  WS-COMP3-018       PIC S9(15) COMP-3.
       01  WS-COMP3-019       PIC S9(05) COMP-3.
       01  WS-COMP3-020       PIC S9(09)V99 COMP-3.
       01  WS-COMP3-021       PIC S9(15) COMP-3.
       01  WS-COMP3-022       PIC S9(05) COMP-3.
       01  WS-COMP3-023       PIC S9(09)V99 COMP-3.
       01  WS-COMP3-024       PIC S9(15) COMP-3.
       01  WS-COMP3-025       PIC S9(05) COMP-3.
       01  WS-COMP3-026       PIC S9(09)V99 COMP-3.
       01  WS-COMP3-027       PIC S9(15) COMP-3.
       01  WS-COMP3-028       PIC S9(05) COMP-3.
       01  WS-COMP3-029       PIC S9(09)V99 COMP-3.
       01  WS-COMP3-030       PIC S9(15) COMP-3.

      * ---- COMP-5 NATIVE BINARY FIELDS (20) ----
       01  WS-COMP5-001      PIC S9(04) COMP-5
           VALUE 25.
       01  WS-COMP5-002      PIC S9(09) COMP-5
           VALUE 50.
       01  WS-COMP5-003      PIC S9(04) COMP-5
           VALUE 75.
       01  WS-COMP5-004      PIC S9(09) COMP-5.
       01  WS-COMP5-005      PIC S9(04) COMP-5.
       01  WS-COMP5-006      PIC S9(09) COMP-5.
       01  WS-COMP5-007      PIC S9(04) COMP-5.
       01  WS-COMP5-008      PIC S9(09) COMP-5.
       01  WS-COMP5-009      PIC S9(04) COMP-5.
       01  WS-COMP5-010      PIC S9(09) COMP-5.
       01  WS-COMP5-011      PIC S9(04) COMP-5.
       01  WS-COMP5-012      PIC S9(09) COMP-5.
       01  WS-COMP5-013      PIC S9(04) COMP-5.
       01  WS-COMP5-014      PIC S9(09) COMP-5.
       01  WS-COMP5-015      PIC S9(04) COMP-5.
       01  WS-COMP5-016      PIC S9(09) COMP-5.
       01  WS-COMP5-017      PIC S9(04) COMP-5.
       01  WS-COMP5-018      PIC S9(09) COMP-5.
       01  WS-COMP5-019      PIC S9(04) COMP-5.
       01  WS-COMP5-020      PIC S9(09) COMP-5.

      * ---- NUMERIC EDITED FIELDS (20) ----
       01  WS-EDITED-001     PIC ZZ,ZZ9.99.
       01  WS-EDITED-002     PIC $$$,$$9.99.
       01  WS-EDITED-003     PIC -(5)9.99.
       01  WS-EDITED-004     PIC ZZ9.99-.
       01  WS-EDITED-005     PIC ZZ,ZZ9.99.
       01  WS-EDITED-006     PIC $$$,$$9.99.
       01  WS-EDITED-007     PIC -(5)9.99.
       01  WS-EDITED-008     PIC ZZ9.99-.
       01  WS-EDITED-009     PIC ZZ,ZZ9.99.
       01  WS-EDITED-010     PIC $$$,$$9.99.
       01  WS-EDITED-011     PIC -(5)9.99.
       01  WS-EDITED-012     PIC ZZ9.99-.
       01  WS-EDITED-013     PIC ZZ,ZZ9.99.
       01  WS-EDITED-014     PIC $$$,$$9.99.
       01  WS-EDITED-015     PIC -(5)9.99.
       01  WS-EDITED-016     PIC ZZ9.99-.
       01  WS-EDITED-017     PIC ZZ,ZZ9.99.
       01  WS-EDITED-018     PIC $$$,$$9.99.
       01  WS-EDITED-019     PIC -(5)9.99.
       01  WS-EDITED-020     PIC ZZ9.99-.

      * ---- GROUP RECORDS WITH CHILDREN (25 groups) ----
      * Group 01 - 3 children
       01  WS-GROUP-01.
           05  WS-G01-FLD-01   PIC X(11)
               VALUE "GROUP01".
           05  WS-G01-FLD-02   PIC 9(07).
           05  WS-G01-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 02 - 3 children
       01  WS-GROUP-02.
           05  WS-G02-FLD-01   PIC X(12)
               VALUE "GROUP02".
           05  WS-G02-FLD-02   PIC 9(07).
           05  WS-G02-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 03 - 4 children
       01  WS-GROUP-03.
           05  WS-G03-FLD-01   PIC X(13)
               VALUE "GROUP03".
           05  WS-G03-FLD-02   PIC 9(07).
           05  WS-G03-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G03-FLD-04   PIC X(20).
      * Group 04 - 3 children
       01  WS-GROUP-04.
           05  WS-G04-FLD-01   PIC X(14).
           05  WS-G04-FLD-02   PIC 9(07).
           05  WS-G04-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 05 - 5 children
       01  WS-GROUP-05.
           05  WS-G05-FLD-01   PIC X(15).
           05  WS-G05-FLD-02   PIC 9(07).
           05  WS-G05-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G05-FLD-04   PIC X(20).
           05  WS-G05-FLD-05   PIC 9(09).
      * Group 06 - 4 children
       01  WS-GROUP-06.
           05  WS-G06-FLD-01   PIC X(16).
           05  WS-G06-FLD-02   PIC 9(07).
           05  WS-G06-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G06-FLD-04   PIC X(20).
      * Group 07 - 3 children
       01  WS-GROUP-07.
           05  WS-G07-FLD-01   PIC X(17).
           05  WS-G07-FLD-02   PIC 9(07).
           05  WS-G07-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 08 - 3 children
       01  WS-GROUP-08.
           05  WS-G08-FLD-01   PIC X(18).
           05  WS-G08-FLD-02   PIC 9(07).
           05  WS-G08-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 09 - 4 children
       01  WS-GROUP-09.
           05  WS-G09-FLD-01   PIC X(19).
           05  WS-G09-FLD-02   PIC 9(07).
           05  WS-G09-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G09-FLD-04   PIC X(20).
      * Group 10 - 5 children
       01  WS-GROUP-10.
           05  WS-G10-FLD-01   PIC X(10).
           05  WS-G10-FLD-02   PIC 9(07).
           05  WS-G10-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G10-FLD-04   PIC X(20).
           05  WS-G10-FLD-05   PIC 9(09).
      * Group 11 - 3 children
       01  WS-GROUP-11.
           05  WS-G11-FLD-01   PIC X(11).
           05  WS-G11-FLD-02   PIC 9(07).
           05  WS-G11-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 12 - 4 children
       01  WS-GROUP-12.
           05  WS-G12-FLD-01   PIC X(12).
           05  WS-G12-FLD-02   PIC 9(07).
           05  WS-G12-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G12-FLD-04   PIC X(20).
      * Group 13 - 3 children
       01  WS-GROUP-13.
           05  WS-G13-FLD-01   PIC X(13).
           05  WS-G13-FLD-02   PIC 9(07).
           05  WS-G13-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 14 - 3 children
       01  WS-GROUP-14.
           05  WS-G14-FLD-01   PIC X(14).
           05  WS-G14-FLD-02   PIC 9(07).
           05  WS-G14-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 15 - 5 children
       01  WS-GROUP-15.
           05  WS-G15-FLD-01   PIC X(15).
           05  WS-G15-FLD-02   PIC 9(07).
           05  WS-G15-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G15-FLD-04   PIC X(20).
           05  WS-G15-FLD-05   PIC 9(09).
      * Group 16 - 3 children
       01  WS-GROUP-16.
           05  WS-G16-FLD-01   PIC X(16).
           05  WS-G16-FLD-02   PIC 9(07).
           05  WS-G16-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 17 - 3 children
       01  WS-GROUP-17.
           05  WS-G17-FLD-01   PIC X(17).
           05  WS-G17-FLD-02   PIC 9(07).
           05  WS-G17-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 18 - 4 children
       01  WS-GROUP-18.
           05  WS-G18-FLD-01   PIC X(18).
           05  WS-G18-FLD-02   PIC 9(07).
           05  WS-G18-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G18-FLD-04   PIC X(20).
      * Group 19 - 3 children
       01  WS-GROUP-19.
           05  WS-G19-FLD-01   PIC X(19).
           05  WS-G19-FLD-02   PIC 9(07).
           05  WS-G19-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 20 - 5 children
       01  WS-GROUP-20.
           05  WS-G20-FLD-01   PIC X(10).
           05  WS-G20-FLD-02   PIC 9(07).
           05  WS-G20-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G20-FLD-04   PIC X(20).
           05  WS-G20-FLD-05   PIC 9(09).
      * Group 21 - 4 children
       01  WS-GROUP-21.
           05  WS-G21-FLD-01   PIC X(11).
           05  WS-G21-FLD-02   PIC 9(07).
           05  WS-G21-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G21-FLD-04   PIC X(20).
      * Group 22 - 3 children
       01  WS-GROUP-22.
           05  WS-G22-FLD-01   PIC X(12).
           05  WS-G22-FLD-02   PIC 9(07).
           05  WS-G22-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 23 - 3 children
       01  WS-GROUP-23.
           05  WS-G23-FLD-01   PIC X(13).
           05  WS-G23-FLD-02   PIC 9(07).
           05  WS-G23-FLD-03   PIC S9(05)V99
               COMP-3.
      * Group 24 - 4 children
       01  WS-GROUP-24.
           05  WS-G24-FLD-01   PIC X(14).
           05  WS-G24-FLD-02   PIC 9(07).
           05  WS-G24-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G24-FLD-04   PIC X(20).
      * Group 25 - 5 children
       01  WS-GROUP-25.
           05  WS-G25-FLD-01   PIC X(15).
           05  WS-G25-FLD-02   PIC 9(07).
           05  WS-G25-FLD-03   PIC S9(05)V99
               COMP-3.
           05  WS-G25-FLD-04   PIC X(20).
           05  WS-G25-FLD-05   PIC 9(09).

      * ---- LEVEL 77 STANDALONE ITEMS (15) ----
       77  WS-STANDALONE-01  PIC X(25)
           VALUE "STAND-01-VALUE".
       77  WS-STANDALONE-02  PIC 9(09)
           VALUE 2000.
       77  WS-STANDALONE-03  PIC S9(07)V99 COMP-3.
       77  WS-STANDALONE-04  PIC X(25).
       77  WS-STANDALONE-05  PIC 9(09).
       77  WS-STANDALONE-06  PIC S9(07)V99 COMP-3.
       77  WS-STANDALONE-07  PIC X(25).
       77  WS-STANDALONE-08  PIC 9(09).
       77  WS-STANDALONE-09  PIC S9(07)V99 COMP-3.
       77  WS-STANDALONE-10  PIC X(25).
       77  WS-STANDALONE-11  PIC 9(09).
       77  WS-STANDALONE-12  PIC S9(07)V99 COMP-3.
       77  WS-STANDALONE-13  PIC X(25).
       77  WS-STANDALONE-14  PIC 9(09).
       77  WS-STANDALONE-15  PIC S9(07)V99 COMP-3.

      * ---- FLAG FIELDS (25) ----
       01  WS-FLAG-001       PIC X(01) VALUE "N".
       01  WS-FLAG-002       PIC X(01) VALUE "Y".
       01  WS-FLAG-003       PIC X(01) VALUE "N".
       01  WS-FLAG-004       PIC X(01) VALUE "Y".
       01  WS-FLAG-005       PIC X(01) VALUE "N".
       01  WS-FLAG-006       PIC X(01).
       01  WS-FLAG-007       PIC X(01).
       01  WS-FLAG-008       PIC X(01).
       01  WS-FLAG-009       PIC X(01).
       01  WS-FLAG-010       PIC X(01).
       01  WS-FLAG-011       PIC X(01).
       01  WS-FLAG-012       PIC X(01).
       01  WS-FLAG-013       PIC X(01).
       01  WS-FLAG-014       PIC X(01).
       01  WS-FLAG-015       PIC X(01).
       01  WS-FLAG-016       PIC X(01).
       01  WS-FLAG-017       PIC X(01).
       01  WS-FLAG-018       PIC X(01).
       01  WS-FLAG-019       PIC X(01).
       01  WS-FLAG-020       PIC X(01).
       01  WS-FLAG-021       PIC X(01).
       01  WS-FLAG-022       PIC X(01).
       01  WS-FLAG-023       PIC X(01).
       01  WS-FLAG-024       PIC X(01).
       01  WS-FLAG-025       PIC X(01).

      * ---- PROCEDURE DIVISION ----
       PROCEDURE DIVISION.
       MAIN-PARAGRAPH.

      * MOVE values to approximately 50 fields
           MOVE "TESTVAL001" TO WS-ALPHA-001.
           MOVE "TESTVAL002" TO WS-ALPHA-002.
           MOVE "TESTVAL003" TO WS-ALPHA-003.
           MOVE "TESTVAL004" TO WS-ALPHA-004.
           MOVE "TESTVAL005" TO WS-ALPHA-005.
           MOVE "TESTVAL006" TO WS-ALPHA-006.
           MOVE "TESTVAL007" TO WS-ALPHA-007.
           MOVE "TESTVAL008" TO WS-ALPHA-008.
           MOVE "TESTVAL009" TO WS-ALPHA-009.
           MOVE "TESTVAL010" TO WS-ALPHA-010.
           MOVE "TESTVAL011" TO WS-ALPHA-011.
           MOVE "TESTVAL012" TO WS-ALPHA-012.
           MOVE "TESTVAL013" TO WS-ALPHA-013.
           MOVE "TESTVAL014" TO WS-ALPHA-014.
           MOVE "TESTVAL015" TO WS-ALPHA-015.
           MOVE "TESTVAL016" TO WS-ALPHA-016.
           MOVE "TESTVAL017" TO WS-ALPHA-017.
           MOVE "TESTVAL018" TO WS-ALPHA-018.
           MOVE "TESTVAL019" TO WS-ALPHA-019.
           MOVE "TESTVAL020" TO WS-ALPHA-020.
           MOVE 111 TO WS-NUM-001.
           MOVE 222 TO WS-NUM-002.
           MOVE 333 TO WS-NUM-003.
           MOVE 444 TO WS-NUM-004.
           MOVE 555 TO WS-NUM-005.
           MOVE 666 TO WS-NUM-006.
           MOVE 777 TO WS-NUM-007.
           MOVE 888 TO WS-NUM-008.
           MOVE 999 TO WS-NUM-009.
           MOVE 1010 TO WS-NUM-010.
           MOVE 1111 TO WS-NUM-011.
           MOVE 1212 TO WS-NUM-012.
           MOVE 1313 TO WS-NUM-013.
           MOVE 1414 TO WS-NUM-014.
           MOVE 1515 TO WS-NUM-015.
           MOVE -50 TO WS-SIGN-001.
           MOVE -100 TO WS-SIGN-002.
           MOVE -150 TO WS-SIGN-003.
           MOVE -200 TO WS-SIGN-004.
           MOVE -250 TO WS-SIGN-005.
           MOVE -300 TO WS-SIGN-006.
           MOVE -350 TO WS-SIGN-007.
           MOVE -400 TO WS-SIGN-008.
           MOVE -450 TO WS-SIGN-009.
           MOVE -500 TO WS-SIGN-010.
           MOVE 1000 TO WS-COMP-001.
           MOVE 2000 TO WS-COMP-002.
           MOVE 3000 TO WS-COMP-003.
           MOVE 4000 TO WS-COMP-004.
           MOVE 5000 TO WS-COMP-005.

      * COMPUTE operations
           COMPUTE WS-NUM-001 =
               WS-NUM-002 + WS-NUM-003.
           COMPUTE WS-SIGN-001 =
               WS-SIGN-002 * WS-SIGN-003.
           COMPUTE WS-COMP-001 =
               WS-COMP-002 + WS-COMP-003
               - 100.
           COMPUTE WS-COMP3-001 =
               WS-COMP3-002 + WS-COMP3-003.
           COMPUTE WS-COMP5-001 =
               WS-COMP5-002 + WS-COMP5-003.

      * DISPLAY about 20 fields
           DISPLAY "--- ALPHANUMERIC FIELDS ---".
           DISPLAY "ALPHA-001: " WS-ALPHA-001.
           DISPLAY "ALPHA-002: " WS-ALPHA-002.
           DISPLAY "ALPHA-003: " WS-ALPHA-003.
           DISPLAY "ALPHA-004: " WS-ALPHA-004.
           DISPLAY "ALPHA-005: " WS-ALPHA-005.
           DISPLAY "--- NUMERIC FIELDS ---".
           DISPLAY "NUM-001: " WS-NUM-001.
           DISPLAY "NUM-002: " WS-NUM-002.
           DISPLAY "NUM-003: " WS-NUM-003.
           DISPLAY "NUM-004: " WS-NUM-004.
           DISPLAY "NUM-005: " WS-NUM-005.
           DISPLAY "--- SIGNED FIELDS ---".
           DISPLAY "SIGN-001: " WS-SIGN-001.
           DISPLAY "SIGN-002: " WS-SIGN-002.
           DISPLAY "SIGN-003: " WS-SIGN-003.
           DISPLAY "--- COMP FIELDS ---".
           DISPLAY "COMP-001: " WS-COMP-001.
           DISPLAY "COMP-002: " WS-COMP-002.
           DISPLAY "COMP-003: " WS-COMP-003.
           DISPLAY "--- COMP-3 FIELDS ---".
           DISPLAY "COMP3-001: " WS-COMP3-001.
           DISPLAY "--- GROUP FIELDS ---".
           DISPLAY "G01-FLD-01: " WS-G01-FLD-01.
           DISPLAY "G02-FLD-01: " WS-G02-FLD-01.
           DISPLAY "--- STANDALONE ---".
           DISPLAY "STANDALONE-01: "
               WS-STANDALONE-01.

           STOP RUN.
