# COBOL SORT/MERGE: Complete Specification

## Purpose

This document specifies the complete semantics of COBOL's SORT and MERGE
operations for the cobol2rust transpiler runtime library. SORT/MERGE is a
core batch processing capability -- nearly every COBOL batch program uses
sorting at some point. The SORT verb delegates to a high-performance sort
subsystem (DFSORT, SYNCSORT on IBM) while providing hooks for arbitrary
pre-sort and post-sort processing through INPUT/OUTPUT PROCEDURES.

This spec covers: SD entries, SORT verb (all forms), MERGE verb, RELEASE
and RETURN verbs, INPUT/OUTPUT PROCEDURE semantics, sort keys and collating
sequences, special registers, IBM DFSORT/SYNCSORT internals, and the
complete Rust implementation architecture.

---

## Table of Contents

1. [Fundamental Concepts](#1-fundamental-concepts)
2. [SD (Sort Description)](#2-sd-sort-description)
3. [SORT Verb](#3-sort-verb)
   - 3.1 [Basic Syntax](#31-basic-syntax)
   - 3.2 [SORT USING GIVING (Simple Form)](#32-sort-using-giving-simple-form)
   - 3.3 [SORT with INPUT PROCEDURE](#33-sort-with-input-procedure)
   - 3.4 [SORT with OUTPUT PROCEDURE](#34-sort-with-output-procedure)
   - 3.5 [SORT with Both Procedures](#35-sort-with-both-procedures)
   - 3.6 [SORT with Multiple Input/Output Files](#36-sort-with-multiple-inputoutput-files)
   - 3.7 [SORT Restrictions](#37-sort-restrictions)
4. [MERGE Verb](#4-merge-verb)
5. [RELEASE and RETURN Verbs](#5-release-and-return-verbs)
6. [Sort Keys](#6-sort-keys)
   - 6.1 [Key Specification](#61-key-specification)
   - 6.2 [Multiple Keys](#62-multiple-keys)
   - 6.3 [Key Data Types and Comparison](#63-key-data-types-and-comparison)
   - 6.4 [Collating Sequence](#64-collating-sequence)
7. [Special Registers](#7-special-registers)
8. [IBM DFSORT / SYNCSORT Internals](#8-ibm-dfsort--syncsort-internals)
   - 8.1 [Architecture Overview](#81-architecture-overview)
   - 8.2 [Sort Work Datasets](#82-sort-work-datasets)
   - 8.3 [DFSORT Control Statements](#83-dfsort-control-statements)
   - 8.4 [Performance Tuning](#84-performance-tuning)
   - 8.5 [Memory and I/O Optimization](#85-memory-and-io-optimization)
   - 8.6 [DFSORT Exits (E15/E35)](#86-dfsort-exits-e15e35)
9. [Rust Implementation Architecture](#9-rust-implementation-architecture)
   - 9.1 [Design Overview](#91-design-overview)
   - 9.2 [Sort Key Comparator](#92-sort-key-comparator)
   - 9.3 [In-Memory Sort](#93-in-memory-sort)
   - 9.4 [External Merge Sort](#94-external-merge-sort)
   - 9.5 [INPUT/OUTPUT PROCEDURE Mapping](#95-inputoutput-procedure-mapping)
   - 9.6 [MERGE Implementation](#96-merge-implementation)
   - 9.7 [Trait Hierarchy](#97-trait-hierarchy)
   - 9.8 [Configuration](#98-configuration)
10. [Worked Examples](#10-worked-examples)
11. [Dialect Variations](#11-dialect-variations)

---

## 1. Fundamental Concepts

### 1.1 What SORT Does

SORT takes an unordered set of records, arranges them by one or more key
fields, and produces an ordered output. The process has three phases:

```
Phase 1: INPUT          Phase 2: SORT        Phase 3: OUTPUT
+---------------+      +-----------+        +----------------+
| USING file(s) |----->|           |------->| GIVING file(s) |
|    - or -     |      |  Sort     |        |     - or -     |
| INPUT         |----->|  Engine   |------->| OUTPUT         |
| PROCEDURE     |      | (DFSORT)  |        | PROCEDURE      |
|  (RELEASE)    |      |           |        |  (RETURN)      |
+---------------+      +-----------+        +----------------+
```

### 1.2 What MERGE Does

MERGE takes two or more **already-sorted** files and combines them into a
single sorted output. No full sort is needed -- it performs a k-way merge.

```
Input file 1 (sorted) ---+
Input file 2 (sorted) ---+--> MERGE --> Output file or OUTPUT PROCEDURE
Input file 3 (sorted) ---+
```

### 1.3 The SD Work File

The sort work file (defined by SD) is a **temporary file** that exists only
during the SORT/MERGE operation. It is the staging area where records are
collected, sorted, and then output. The programmer never OPENs, CLOSEs,
READs, or WRITEs to an SD file directly -- only RELEASE and RETURN are used.

### 1.4 SORT vs Application-Level Sort

COBOL SORT is not a simple array sort. It is designed for:
- Datasets that may exceed available memory (external sort)
- Record-oriented data (fixed or variable length records)
- Multi-key sorting with mixed ascending/descending
- Integration with file I/O (USING/GIVING)
- Arbitrary pre/post processing (INPUT/OUTPUT PROCEDURES)
- On IBM: delegation to DFSORT/SYNCSORT for terabyte-scale performance

---

## 2. SD (Sort Description)

The SD entry describes the sort work file, analogous to FD for regular files.

### 2.1 Syntax

```cobol
ENVIRONMENT DIVISION.
INPUT-OUTPUT SECTION.
FILE-CONTROL.
    SELECT SORT-WORK ASSIGN TO SORTWK01.

DATA DIVISION.
FILE SECTION.
SD  SORT-WORK
    [RECORD CONTAINS integer-1 CHARACTERS]
    [RECORD CONTAINS integer-1 TO integer-2 CHARACTERS]
    [RECORD IS VARYING IN SIZE FROM integer-1 TO integer-2 CHARACTERS
        DEPENDING ON data-name-1]
    [DATA RECORDS ARE data-name-2 [data-name-3 ...]]
    .
01  SORT-RECORD.
    05  SR-KEY-1    PIC X(10).
    05  SR-KEY-2    PIC 9(8).
    05  SR-DATA     PIC X(62).
```

### 2.2 SD vs FD Differences

| Aspect | FD (Regular File) | SD (Sort Work File) |
|---|---|---|
| OPEN/CLOSE | Program explicitly opens/closes | System manages automatically |
| READ/WRITE | Program reads/writes | Not allowed (use RELEASE/RETURN) |
| Lifetime | Persistent (exists on disk) | Temporary (exists during SORT only) |
| BLOCK CONTAINS | Specified by programmer | Managed by sort subsystem |
| RECORDING MODE | Specified by programmer | Managed by sort subsystem |
| LABEL RECORDS | May be STANDARD/OMITTED | Always OMITTED (temp file) |
| LINAGE | May be specified | Not applicable |
| VALUE OF | May be specified | Not applicable |
| FILE STATUS | May be specified | Not applicable (use SORT-RETURN) |

### 2.3 Multiple Record Descriptions

Like FD, an SD can have multiple 01-level record descriptions. This is
useful when the sort work file receives records of different types:

```cobol
SD  SORT-WORK.
01  SORT-CUSTOMER-REC.
    05  SC-TYPE   PIC X VALUE "C".
    05  SC-KEY    PIC X(10).
    05  SC-NAME   PIC X(30).
    05  SC-FILLER PIC X(39).
01  SORT-ORDER-REC.
    05  SO-TYPE   PIC X.
    05  SO-KEY    PIC X(10).
    05  SO-AMOUNT PIC S9(7)V99 COMP-3.
    05  SO-FILLER PIC X(60).
```

### 2.4 ASSIGN TO for Sort Work Files

On IBM, sort work files are assigned to SORTWKnn DD statements:

```cobol
SELECT SORT-WORK ASSIGN TO SORTWK01.
```

```jcl
//SORTWK01 DD UNIT=SYSDA,SPACE=(CYL,(10,5))
//SORTWK02 DD UNIT=SYSDA,SPACE=(CYL,(10,5))
//SORTWK03 DD UNIT=SYSDA,SPACE=(CYL,(10,5))
```

DFSORT uses multiple work datasets (SORTWK01-SORTWK32) for parallel
merge operations. More work datasets = more merge parallelism.

In the Rust implementation, sort work files are temporary files managed
by the `tempfile` crate.

---

## 3. SORT Verb

### 3.1 Basic Syntax

```cobol
SORT file-name-1
    ON {ASCENDING | DESCENDING} KEY data-name-1 [data-name-2 ...]
    [ON {ASCENDING | DESCENDING} KEY data-name-3 [data-name-4 ...]]
    [WITH DUPLICATES IN ORDER]
    [COLLATING SEQUENCE IS alphabet-name]
    {USING file-name-2 [file-name-3 ...]
     | INPUT PROCEDURE IS section-name-1 [THROUGH section-name-2]}
    {GIVING file-name-4 [file-name-5 ...]
     | OUTPUT PROCEDURE IS section-name-3 [THROUGH section-name-4]}
```

### 3.2 SORT USING GIVING (Simple Form)

The simplest form reads from one or more input files, sorts, and writes to
one or more output files:

```cobol
SORT SORT-WORK
    ON ASCENDING KEY SR-KEY-1
    ON DESCENDING KEY SR-KEY-2
    USING INPUT-FILE
    GIVING OUTPUT-FILE.
```

**Execution flow**:
1. SORT implicitly OPENs INPUT-FILE for INPUT
2. All records are READ from INPUT-FILE and transferred to SORT-WORK
   (this is a GROUP MOVE -- alphanumeric byte copy semantics)
3. Records are sorted by the specified keys
4. SORT implicitly OPENs OUTPUT-FILE for OUTPUT
5. All sorted records are WRITTEN to OUTPUT-FILE
   (GROUP MOVE semantics)
6. SORT implicitly CLOSEs both files

**Important**: The input and output files must NOT be already open when SORT
executes. SORT manages the OPEN/CLOSE lifecycle.

**Multiple input files**: When multiple USING files are specified, records
from all files are concatenated before sorting:

```cobol
SORT SORT-WORK
    ON ASCENDING KEY SR-KEY
    USING FILE-A FILE-B FILE-C
    GIVING FILE-OUT.
*> Records from A, B, and C are all sorted together
```

**Multiple output files**: When multiple GIVING files are specified, the
sorted output is written to ALL of them (copied to each):

```cobol
SORT SORT-WORK
    ON ASCENDING KEY SR-KEY
    USING FILE-IN
    GIVING FILE-OUT-1 FILE-OUT-2.
*> Both output files receive identical sorted data
```

### 3.3 SORT with INPUT PROCEDURE

INPUT PROCEDURE allows the program to control which records are fed to the
sort, with arbitrary transformation:

```cobol
SORT SORT-WORK
    ON ASCENDING KEY SR-KEY
    INPUT PROCEDURE IS 1000-SELECT-RECORDS
    GIVING OUTPUT-FILE.

1000-SELECT-RECORDS SECTION.
    OPEN INPUT TRANS-FILE.
    PERFORM UNTIL WS-EOF
        READ TRANS-FILE INTO WS-TRANS
            AT END SET WS-EOF TO TRUE
        END-READ
        IF NOT WS-EOF
            IF TR-AMOUNT > 100
                MOVE WS-TRANS TO SORT-RECORD
                RELEASE SORT-RECORD
            END-IF
        END-IF
    END-PERFORM.
    CLOSE TRANS-FILE.
```

**Key behaviors**:
- The INPUT PROCEDURE is a SECTION (or range of sections via THROUGH)
- Inside the procedure, RELEASE sends records to the sort
- The procedure can read from ANY source (files, tables, computed data)
- The procedure can filter, transform, or aggregate before RELEASE
- The procedure runs to completion before the sort phase begins
- After the procedure finishes, all RELEASEd records are sorted

### 3.4 SORT with OUTPUT PROCEDURE

OUTPUT PROCEDURE allows the program to control what happens with sorted
records:

```cobol
SORT SORT-WORK
    ON ASCENDING KEY SR-KEY
    USING INPUT-FILE
    OUTPUT PROCEDURE IS 2000-PROCESS-SORTED.

2000-PROCESS-SORTED SECTION.
    PERFORM UNTIL WS-SORT-EOF
        RETURN SORT-WORK INTO WS-SORTED-REC
            AT END SET WS-SORT-EOF TO TRUE
            NOT AT END
                PERFORM 2100-PROCESS-RECORD
        END-RETURN
    END-PERFORM.
```

**Key behaviors**:
- The OUTPUT PROCEDURE runs AFTER the sort is complete
- RETURN retrieves the next sorted record (like READ for sort files)
- The procedure can filter, transform, aggregate, or write to any destination
- AT END indicates no more sorted records
- The procedure must consume all records (or at least run to completion)

### 3.5 SORT with Both Procedures

Both INPUT and OUTPUT PROCEDURE can be used in the same SORT:

```cobol
SORT SORT-WORK
    ON ASCENDING KEY SR-CUST-ID
    ON ASCENDING KEY SR-DATE
    INPUT PROCEDURE IS 1000-READ-AND-FILTER
    OUTPUT PROCEDURE IS 2000-WRITE-REPORT.
```

**Execution order**:
1. INPUT PROCEDURE runs -- all RELEASEd records are collected
2. Sort engine sorts the collected records
3. OUTPUT PROCEDURE runs -- RETURNs sorted records for processing

### 3.6 SORT with Multiple Input/Output Files

```cobol
*> Multiple USING files (concatenated input)
SORT SORT-WORK
    ON ASCENDING KEY SR-KEY
    USING FILE-1 FILE-2 FILE-3
    GIVING FILE-OUT.

*> Multiple GIVING files (replicated output)
SORT SORT-WORK
    ON ASCENDING KEY SR-KEY
    USING FILE-IN
    GIVING FILE-OUT-A FILE-OUT-B.
```

When USING specifies multiple files:
- Records are read from each file in the order listed
- All files must have record descriptions compatible with the SD record
- The GROUP MOVE from each input FD record to the SD record is implicit

When GIVING specifies multiple files:
- Each output file receives an identical copy of the sorted data
- All files are opened OUTPUT and closed after the sort

### 3.7 SORT Restrictions

| Restriction | Detail |
|---|---|
| No nested SORT | Cannot execute SORT from within INPUT/OUTPUT PROCEDURE |
| No explicit I/O on SD | Cannot OPEN, CLOSE, READ, or WRITE the SD file |
| RELEASE only in INPUT | RELEASE is only valid inside an INPUT PROCEDURE |
| RETURN only in OUTPUT | RETURN is only valid inside an OUTPUT PROCEDURE |
| Files must be closed | USING/GIVING files must not be open when SORT starts |
| Section-based | INPUT/OUTPUT PROCEDURE must reference a SECTION, not a paragraph |
| No GO TO out | Cannot GO TO a target outside the INPUT/OUTPUT PROCEDURE |
| No ALTER | Cannot ALTER a GO TO target within/from a procedure |
| SD lifetime | SD record area is undefined before first RELEASE or after last RETURN |

---

## 4. MERGE Verb

MERGE combines two or more pre-sorted input files into a single sorted output.

### 4.1 Syntax

```cobol
MERGE file-name-1
    ON {ASCENDING | DESCENDING} KEY data-name-1 [data-name-2 ...]
    [ON {ASCENDING | DESCENDING} KEY data-name-3 [data-name-4 ...]]
    [COLLATING SEQUENCE IS alphabet-name]
    USING file-name-2 file-name-3 [file-name-4 ...]
    {GIVING file-name-5 [file-name-6 ...]
     | OUTPUT PROCEDURE IS section-name-1 [THROUGH section-name-2]}
```

### 4.2 MERGE vs SORT Differences

| Aspect | SORT | MERGE |
|---|---|---|
| Input files | Can be unsorted | **Must be pre-sorted** by merge keys |
| INPUT PROCEDURE | Allowed | **Not allowed** |
| OUTPUT PROCEDURE | Allowed | Allowed |
| Minimum input files | 1 (USING) or 0 (INPUT PROC) | **2 or more** (USING required) |
| Sort phase | Full sort of all records | K-way merge (no full sort needed) |
| Performance | O(n log n) | O(n) -- linear in total records |
| Duplicates | WITH DUPLICATES IN ORDER | Duplicates maintain file order |

### 4.3 MERGE Input Requirement

All input files for MERGE must be sorted in the same order as the MERGE
keys specify. If an input file is not properly sorted, the results are
**undefined** (no runtime error -- data will be silently misordered).

```cobol
*> FILE-1 must be sorted by CUST-ID ascending
*> FILE-2 must be sorted by CUST-ID ascending
*> FILE-3 must be sorted by CUST-ID ascending
MERGE SORT-WORK
    ON ASCENDING KEY SR-CUST-ID
    USING FILE-1 FILE-2 FILE-3
    GIVING MERGED-FILE.
```

### 4.4 MERGE Duplicate Handling

When records from different input files have equal keys:
- Records from the **first-listed** USING file appear before records from
  later-listed files
- Within the same input file, original order is preserved

This provides a stable merge across files with predictable ordering.

---

## 5. RELEASE and RETURN Verbs

### 5.1 RELEASE

```cobol
RELEASE record-name-1 [FROM identifier-1]
```

- **record-name-1**: The 01-level record defined under the SD
- **FROM identifier-1**: Optional -- performs an implicit MOVE (GROUP move
  semantics) from the identifier to the sort record before releasing
- Sends one record to the sort engine
- Only valid inside an INPUT PROCEDURE
- Multiple RELEASEs can be executed per INPUT PROCEDURE invocation

```cobol
*> Direct: populate sort record, then release
MOVE WS-DATA TO SORT-RECORD.
RELEASE SORT-RECORD.

*> Using FROM: implicit MOVE + release
RELEASE SORT-RECORD FROM WS-DATA.
```

### 5.2 RETURN

```cobol
RETURN file-name-1 [INTO identifier-1]
    [AT END imperative-statement-1]
    [NOT AT END imperative-statement-2]
[END-RETURN]
```

- **file-name-1**: The SD file (not the record name -- note the difference
  from RELEASE which uses the record name)
- **INTO identifier-1**: Optional -- performs an implicit MOVE from the sort
  record to the identifier after returning
- Retrieves the next sorted record from the sort engine
- AT END: executed when no more sorted records are available
- Only valid inside an OUTPUT PROCEDURE
- Each RETURN retrieves records in sorted order

```cobol
RETURN SORT-WORK INTO WS-SORTED-RECORD
    AT END SET WS-SORT-DONE TO TRUE
    NOT AT END
        ADD 1 TO WS-RECORD-COUNT
END-RETURN.
```

### 5.3 RELEASE/RETURN Naming Asymmetry

Note the asymmetry in COBOL's design:
- **RELEASE** uses the **record name** (01-level under SD)
- **RETURN** uses the **file name** (the SD name)

This is analogous to:
- **WRITE** uses the **record name**
- **READ** uses the **file name**

The transpiler must handle this naming difference when generating Rust code.

---

## 6. Sort Keys

### 6.1 Key Specification

Sort keys are fields defined within the SD's 01-level record description:

```cobol
SD  SORT-WORK.
01  SORT-RECORD.
    05  SR-DEPT    PIC X(4).        *> Key 1
    05  SR-EMPID   PIC 9(8).        *> Key 2
    05  SR-NAME    PIC X(30).
    05  SR-SALARY  PIC S9(7)V99 COMP-3.  *> Key 3

SORT SORT-WORK
    ON ASCENDING KEY SR-DEPT
    ON ASCENDING KEY SR-EMPID
    ON DESCENDING KEY SR-SALARY
    USING ...
    GIVING ...
```

### 6.2 Multiple Keys

Keys are evaluated left to right (primary key first):

1. Compare by SR-DEPT (ascending). If equal:
2. Compare by SR-EMPID (ascending). If still equal:
3. Compare by SR-SALARY (descending)

Each key can independently be ASCENDING or DESCENDING.

Multiple keys can be specified in a single ON clause:

```cobol
ON ASCENDING KEY SR-DEPT SR-EMPID
*> Equivalent to:
ON ASCENDING KEY SR-DEPT
ON ASCENDING KEY SR-EMPID
```

### 6.3 Key Data Types and Comparison

The sort engine must compare keys according to their data type:

| Key Category | Comparison Method |
|---|---|
| Alphanumeric (PIC X) | Byte-by-byte using collating sequence |
| Alphabetic (PIC A) | Byte-by-byte using collating sequence |
| Numeric DISPLAY (PIC 9) | Zoned decimal numeric comparison |
| Numeric COMP/COMP-4 | Binary integer comparison |
| Numeric COMP-3 | Packed decimal numeric comparison |
| Numeric COMP-5 | Native binary comparison |
| Numeric-Edited | Treated as alphanumeric (byte comparison) |
| Group item as key | Treated as alphanumeric (byte comparison) |

**Numeric comparison**: For numeric keys, the sort compares by **numeric
value**, not by byte representation. This means:

- `COMP-3` key with value -5 sorts before +3 (signed comparison)
- `PIC 9(5)` with value 00100 equals 100 (leading zeros insignificant)
- `PIC S9(5) DISPLAY` encodes sign in zone nibble -- sort must decode

**Alphanumeric comparison**: For alphanumeric keys, comparison is byte-by-byte
from left to right using the collating sequence. Shorter keys are padded
with spaces on the right for comparison purposes.

### 6.4 Collating Sequence

The COLLATING SEQUENCE clause overrides the default character comparison
order:

```cobol
ENVIRONMENT DIVISION.
CONFIGURATION SECTION.
OBJECT-COMPUTER. IBM-370
    PROGRAM COLLATING SEQUENCE IS MY-SEQUENCE.
SPECIAL-NAMES.
    ALPHABET MY-SEQUENCE IS EBCDIC.

*> Or specify per SORT:
SORT SORT-WORK
    ON ASCENDING KEY SR-NAME
    COLLATING SEQUENCE IS MY-SEQUENCE
    USING ...
    GIVING ...
```

**Default collating sequence**: The native sequence of the character set.
On IBM mainframes, this is EBCDIC. On ASCII systems, this is ASCII.

**EBCDIC vs ASCII collating differences**:

| Range | ASCII Order | EBCDIC Order |
|---|---|---|
| Space | 0x20 (low) | 0x40 (low) |
| a-z | 0x61-0x7A | 0x81-0xA9 (split: a-i, j-r, s-z) |
| A-Z | 0x41-0x5A | 0xC1-0xE9 (split: A-I, J-R, S-Z) |
| 0-9 | 0x30-0x39 | 0xF0-0xF9 (highest!) |

**Impact**: In EBCDIC, `"9"` > `"Z"` > `"z"`. In ASCII, `"9"` < `"A"` < `"a"`.
A sort that produces correct results in EBCDIC will produce different results
in ASCII for alphanumeric keys. The transpiler must support EBCDIC collating
sequence for migrated programs.

### 6.5 WITH DUPLICATES IN ORDER

```cobol
SORT SORT-WORK
    ON ASCENDING KEY SR-DEPT
    WITH DUPLICATES IN ORDER
    USING INPUT-FILE
    GIVING OUTPUT-FILE.
```

Without this clause, the relative order of records with equal keys is
**undefined** (unstable sort is permitted).

With this clause, records with equal keys maintain their **input order**
(stable sort). This is required when the output order must be predictable
for records with the same key values.

On IBM, DFSORT's EQUALS option controls this behavior. The COBOL compiler
passes the EQUALS parameter to DFSORT when WITH DUPLICATES IN ORDER is
specified.

---

## 7. Special Registers

COBOL provides special registers for monitoring and controlling SORT/MERGE:

### 7.1 SORT-RETURN

```cobol
01  SORT-RETURN  PIC 9(4) COMP.  *> system-defined, not declared
```

Set after each SORT or MERGE statement:
- **0**: Successful completion
- **16**: Sort/merge failed (I/O error, insufficient work space, etc.)

The program should check SORT-RETURN after every SORT/MERGE:

```cobol
SORT SORT-WORK ...
IF SORT-RETURN NOT = 0
    DISPLAY "SORT FAILED, RC=" SORT-RETURN
    STOP RUN
END-IF.
```

### 7.2 SORT-FILE-SIZE

```cobol
01  SORT-FILE-SIZE  PIC 9(18) COMP.  *> system-defined
```

Set by the program BEFORE the SORT to provide an estimate of the number of
records to be sorted. The sort subsystem uses this as a hint for memory and
work space allocation:

```cobol
MOVE 1000000 TO SORT-FILE-SIZE.
SORT SORT-WORK ...
```

### 7.3 SORT-CORE-SIZE

```cobol
01  SORT-CORE-SIZE  PIC 9(8) COMP.  *> system-defined
```

Specifies the amount of main storage (in bytes) available to the sort. On
modern IBM systems, this is largely obsolete -- DFSORT manages its own
memory allocation. Setting to 0 lets the sort subsystem decide.

### 7.4 SORT-MODE-SIZE

```cobol
01  SORT-MODE-SIZE  PIC 9(5) COMP.  *> system-defined
```

Set by the program to indicate the most frequent record size (for variable-
length records). Used as a hint for buffer allocation optimization.

### 7.5 SORT-MESSAGE

```cobol
01  SORT-MESSAGE  PIC X(8).  *> system-defined
```

Contains the DDname of the file to which sort diagnostic messages are written.
Default is `SYSOUT`. Setting it changes the message destination:

```cobol
MOVE "SORTMSG" TO SORT-MESSAGE.
*> Sort messages will go to the SORTMSG DD dataset
```

### 7.6 SORT-CONTROL

```cobol
01  SORT-CONTROL  PIC X(8).  *> system-defined
```

Contains the DDname of a file with sort control statements (DFSORT control
cards). Default is `IGZSRTCD` (IBM) or implementation-defined. Setting it to
spaces disables external control statements.

---

## 8. IBM DFSORT / SYNCSORT Internals

### 8.1 Architecture Overview

On IBM z/OS, COBOL's SORT verb delegates to an external sort product:

- **DFSORT** (Data Facility Sort): IBM's sort utility (most common)
- **SYNCSORT** (now Precisely): Third-party alternative

The COBOL runtime interfaces with DFSORT through:
1. Parameter list passed via COBOL runtime hooks
2. JCL DD statements for work files and control
3. Exit routines (E15, E35) for INPUT/OUTPUT PROCEDURE

```
COBOL Program
    |
    v
COBOL Sort Runtime Interface (IGZSRTCD)
    |
    v
DFSORT Engine
    |
    +-- Sort Phase (tournament sort / polyphase merge)
    |     |
    |     +-- Read input (from USING files or E15 exit)
    |     +-- Build sorted runs in memory
    |     +-- Spill runs to SORTWKnn work files
    |     +-- Multi-pass merge of sorted runs
    |
    +-- Output Phase
          |
          +-- Write to GIVING files or E35 exit
```

### 8.2 Sort Work Datasets

DFSORT uses temporary work datasets for the merge phase:

```jcl
//SORTWK01 DD UNIT=SYSDA,SPACE=(CYL,(50,10))
//SORTWK02 DD UNIT=SYSDA,SPACE=(CYL,(50,10))
//SORTWK03 DD UNIT=SYSDA,SPACE=(CYL,(50,10))
```

**Work dataset allocation rules**:
- Minimum: 2 work datasets (for 2-way merge)
- Recommended: 3-6 work datasets
- Maximum: 32 (SORTWK01-SORTWK32)
- More work datasets enable higher-order merges (faster for large sorts)
- DFSORT can use HIPERSPACE (data spaces) instead of disk work datasets

**Space estimation**:
```
Work space needed = approximately 2 * (total input size)
                    (for a 2-pass sort with 3 work datasets)
```

For very large sorts, DFSORT may use multiple merge passes:
- Pass 1: Build sorted runs
- Pass 2: Merge runs (k-way merge using work datasets)
- Pass 3+: Additional merge passes if runs exceed work dataset capacity

### 8.3 DFSORT Control Statements

While COBOL specifies sort keys via the SORT verb, DFSORT supports its own
control language for additional capabilities:

```
  SORT FIELDS=(1,10,CH,A,11,8,ZD,D)
  INCLUDE COND=(21,5,CH,EQ,C'ACTIV')
  OMIT COND=(30,1,CH,EQ,C'X')
  SUM FIELDS=(41,8,ZD)
  OUTREC FIELDS=(1,20,41,8,C' TOTAL')
```

| Statement | Purpose |
|---|---|
| SORT FIELDS | Define sort keys (alternative to COBOL KEY clause) |
| INCLUDE/OMIT | Filter records (include or exclude by condition) |
| SUM FIELDS | Summarize (aggregate) records with equal keys |
| OUTREC/INREC | Reformat records (add, remove, rearrange fields) |
| OUTFIL | Multiple output files with filtering |
| OPTION | Sort options (EQUALS, DYNALLOC, FILSZ, etc.) |

**COBOL interaction**: When a COBOL program uses SORT with INPUT/OUTPUT
PROCEDURE, DFSORT's INCLUDE/OMIT and SUM are bypassed -- the COBOL
procedures handle filtering and aggregation.

When SORT USING/GIVING is used without procedures, DFSORT control
statements from IGZSRTCD (or SORT-CONTROL file) are applied.

### 8.4 Performance Tuning

**DFSORT performance factors**:

| Factor | Impact | Optimization |
|---|---|---|
| Memory | More memory = fewer merge passes | OPTION MAINSIZE=MAX |
| Work datasets | More = higher-order merge | 3-6 SORTWKnn DDs |
| Key length | Shorter keys = faster comparison | Minimize key size |
| Key position | Earlier keys = faster | Put most selective key first |
| Record length | Shorter = more records in memory | Omit unnecessary fields |
| Blocksize | Larger = fewer I/Os | OPTION DYNALLOC for auto-tuning |
| HIPERSPACE | In-memory runs | Available on z/OS |

**DFSORT capacity**: DFSORT can sort datasets of virtually unlimited size.
Practical limits are disk space and time, not record count. Sorting 1 billion
records is routine on IBM mainframes.

### 8.5 Memory and I/O Optimization

**DFSORT memory management**:
- **Tournament sort**: Build sorted runs using a replacement-selection
  tournament tree. Average run length = 2 * memory / record_size.
- **Blockset sort**: DFSORT's optimized technique that processes data in
  blocks rather than individual records.
- **HIPERSPACE**: Uses data spaces for in-memory sorting when available.

**I/O optimization**:
- **Dynamic block sizes**: DFSORT automatically selects optimal block sizes
- **Overlap**: Overlaps I/O with CPU processing (double buffering)
- **Channel programs**: Uses chained channel programs for sequential I/O
- **Compression**: Can compress work datasets (OPTION COMPRESS)

### 8.6 DFSORT Exits (E15/E35)

DFSORT provides user exits that correspond to COBOL's INPUT/OUTPUT PROCEDURES:

| Exit | Corresponds To | When Called |
|---|---|---|
| **E15** | INPUT PROCEDURE | Before sort -- provides input records |
| **E35** | OUTPUT PROCEDURE | After sort -- receives sorted records |
| E18 | (assembler only) | During merge phase |
| E39 | (assembler only) | After output phase |

**E15 (Input exit)**:
- Called repeatedly by DFSORT
- Each call should return one record (via RELEASE)
- Return code: 12 = more records, 8 = last record, 16 = delete record

**E35 (Output exit)**:
- Called with each sorted record (via RETURN)
- Program processes the record
- Return code: 12 = continue, 8 = stop (no more records wanted), 4 = insert

The COBOL runtime translates INPUT/OUTPUT PROCEDURE into E15/E35 exit calls
transparently. The COBOL programmer never directly interacts with E15/E35.

---

## 9. Rust Implementation Architecture

### 9.1 Design Overview

The SORT/MERGE implementation uses a two-tier approach:

| Tier | Threshold | Algorithm |
|---|---|---|
| **In-memory** | Dataset fits in configurable memory limit | `Vec::sort_by` with custom comparator |
| **External** | Dataset exceeds memory limit | External merge sort with temp files |

Selection is automatic based on `SortConfig::memory_limit` vs estimated
dataset size.

```
+---------------------+
| SORT entry point    |
+---------------------+
    |
    +-- estimate dataset size
    |
    +-- size <= memory_limit?
    |     YES --> In-memory sort (Vec + sort_by)
    |     NO  --> External merge sort
    |
    +-- INPUT PROCEDURE?
    |     YES --> Closure feeds records via Releaser
    |     NO  --> Read from USING files
    |
    +-- [Sort/Merge phase]
    |
    +-- OUTPUT PROCEDURE?
          YES --> Closure consumes records via Returner
          NO  --> Write to GIVING files
```

### 9.2 Sort Key Comparator

The comparator is built from the key specification at sort initialization
time (in the transpiled code, this is compile-time known):

```rust
/// Specification for a single sort key
#[derive(Debug, Clone)]
pub struct SortKey {
    /// Byte offset within the record (0-based)
    pub offset: usize,
    /// Key length in bytes
    pub length: usize,
    /// Sort direction
    pub ascending: bool,
    /// Data type determines comparison method
    pub key_type: SortKeyType,
}

#[derive(Debug, Clone)]
pub enum SortKeyType {
    /// Byte-by-byte comparison using collating sequence
    Alphanumeric,
    /// Zoned decimal (DISPLAY numeric): decode sign and digits
    ZonedDecimal {
        precision: u32,
        scale: u32,
        signed: bool,
    },
    /// Packed decimal (COMP-3): decode BCD nibbles and sign
    PackedDecimal {
        precision: u32,
        scale: u32,
        signed: bool,
    },
    /// Binary integer (COMP, COMP-4, COMP-5)
    Binary {
        size: usize, // 2, 4, or 8 bytes
        signed: bool,
    },
    /// Single-precision float (COMP-1) -- rare as sort key
    Float32,
    /// Double-precision float (COMP-2) -- rare as sort key
    Float64,
}

/// Collating sequence for alphanumeric comparison
#[derive(Debug, Clone)]
pub enum CollatingSequence {
    /// Native platform sequence (ASCII in Rust)
    Native,
    /// EBCDIC collating order (for migrated programs)
    Ebcdic,
    /// Custom collating sequence (256-byte mapping table)
    Custom([u8; 256]),
}

/// Complete sort key specification
#[derive(Debug, Clone)]
pub struct SortKeySpec {
    pub keys: Vec<SortKey>,
    pub collating_sequence: CollatingSequence,
    pub stable: bool, // WITH DUPLICATES IN ORDER
}

impl SortKeySpec {
    /// Build a comparator function from the key specification
    pub fn comparator(&self) -> impl Fn(&[u8], &[u8]) -> Ordering + '_ {
        move |rec_a: &[u8], rec_b: &[u8]| {
            for key in &self.keys {
                let a_slice = &rec_a[key.offset..key.offset + key.length];
                let b_slice = &rec_b[key.offset..key.offset + key.length];

                let cmp = match &key.key_type {
                    SortKeyType::Alphanumeric => {
                        compare_alphanumeric(
                            a_slice,
                            b_slice,
                            &self.collating_sequence,
                        )
                    }
                    SortKeyType::ZonedDecimal { precision, scale, signed } => {
                        compare_zoned_decimal(
                            a_slice, b_slice,
                            *precision, *scale, *signed,
                        )
                    }
                    SortKeyType::PackedDecimal { precision, scale, signed } => {
                        compare_packed_decimal(
                            a_slice, b_slice,
                            *precision, *scale, *signed,
                        )
                    }
                    SortKeyType::Binary { size, signed } => {
                        compare_binary(
                            a_slice, b_slice,
                            *size, *signed,
                        )
                    }
                    SortKeyType::Float32 => {
                        compare_float32(a_slice, b_slice)
                    }
                    SortKeyType::Float64 => {
                        compare_float64(a_slice, b_slice)
                    }
                };

                let cmp = if key.ascending { cmp } else { cmp.reverse() };
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        }
    }
}

/// Compare alphanumeric keys using collating sequence
fn compare_alphanumeric(
    a: &[u8],
    b: &[u8],
    collating: &CollatingSequence,
) -> Ordering {
    match collating {
        CollatingSequence::Native => a.cmp(b),
        CollatingSequence::Ebcdic => {
            // Map each byte through EBCDIC collating table
            for (ab, bb) in a.iter().zip(b.iter()) {
                let mapped_a = EBCDIC_COLLATE_ORDER[*ab as usize];
                let mapped_b = EBCDIC_COLLATE_ORDER[*bb as usize];
                match mapped_a.cmp(&mapped_b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            a.len().cmp(&b.len())
        }
        CollatingSequence::Custom(table) => {
            for (ab, bb) in a.iter().zip(b.iter()) {
                let mapped_a = table[*ab as usize];
                let mapped_b = table[*bb as usize];
                match mapped_a.cmp(&mapped_b) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            a.len().cmp(&b.len())
        }
    }
}

/// Compare packed decimal (COMP-3) keys by numeric value
fn compare_packed_decimal(
    a: &[u8],
    b: &[u8],
    precision: u32,
    scale: u32,
    signed: bool,
) -> Ordering {
    let val_a = decode_packed_decimal(a, precision, scale, signed);
    let val_b = decode_packed_decimal(b, precision, scale, signed);
    val_a.cmp(&val_b)
}

/// Compare binary (COMP) keys
fn compare_binary(
    a: &[u8],
    b: &[u8],
    size: usize,
    signed: bool,
) -> Ordering {
    if signed {
        match size {
            2 => {
                let va = i16::from_be_bytes([a[0], a[1]]);
                let vb = i16::from_be_bytes([b[0], b[1]]);
                va.cmp(&vb)
            }
            4 => {
                let va = i32::from_be_bytes([a[0], a[1], a[2], a[3]]);
                let vb = i32::from_be_bytes([b[0], b[1], b[2], b[3]]);
                va.cmp(&vb)
            }
            8 => {
                let va = i64::from_be_bytes(a[..8].try_into().unwrap());
                let vb = i64::from_be_bytes(b[..8].try_into().unwrap());
                va.cmp(&vb)
            }
            _ => a.cmp(b),
        }
    } else {
        match size {
            2 => {
                let va = u16::from_be_bytes([a[0], a[1]]);
                let vb = u16::from_be_bytes([b[0], b[1]]);
                va.cmp(&vb)
            }
            4 => {
                let va = u32::from_be_bytes([a[0], a[1], a[2], a[3]]);
                let vb = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
                va.cmp(&vb)
            }
            8 => {
                let va = u64::from_be_bytes(a[..8].try_into().unwrap());
                let vb = u64::from_be_bytes(b[..8].try_into().unwrap());
                va.cmp(&vb)
            }
            _ => a.cmp(b),
        }
    }
}
```

### 9.3 In-Memory Sort

For datasets that fit in memory:

```rust
pub struct InMemorySort {
    records: Vec<Vec<u8>>,
    key_spec: SortKeySpec,
}

impl InMemorySort {
    pub fn new(key_spec: SortKeySpec) -> Self {
        Self {
            records: Vec::new(),
            key_spec,
        }
    }

    /// Add a record (RELEASE equivalent)
    pub fn release(&mut self, record: &[u8]) {
        self.records.push(record.to_vec());
    }

    /// Sort all accumulated records
    pub fn sort(&mut self) {
        let cmp = self.key_spec.comparator();
        if self.key_spec.stable {
            self.records.sort_by(|a, b| cmp(a, b));
        } else {
            self.records.sort_unstable_by(|a, b| cmp(a, b));
        }
    }

    /// Iterate over sorted records (RETURN equivalent)
    pub fn iter(&self) -> impl Iterator<Item = &[u8]> {
        self.records.iter().map(|r| r.as_slice())
    }

    /// Estimated memory usage
    pub fn memory_usage(&self) -> usize {
        self.records.iter().map(|r| r.len() + std::mem::size_of::<Vec<u8>>()).sum()
    }
}
```

### 9.4 External Merge Sort

For datasets that exceed the memory limit:

```rust
use std::io::{BufReader, BufWriter, Read, Write, Seek, SeekFrom};
use tempfile::NamedTempFile;

pub struct ExternalMergeSort {
    key_spec: SortKeySpec,
    record_size: usize,  // fixed record size (or max for variable)
    memory_limit: usize, // max bytes for in-memory buffer
    run_files: Vec<NamedTempFile>,
    run_record_counts: Vec<usize>,
}

impl ExternalMergeSort {
    pub fn new(
        key_spec: SortKeySpec,
        record_size: usize,
        memory_limit: usize,
    ) -> Self {
        Self {
            key_spec,
            record_size,
            memory_limit,
            run_files: Vec::new(),
            run_record_counts: Vec::new(),
        }
    }

    /// Phase 1: Build sorted runs from input records
    pub fn build_runs<I>(&mut self, records: I) -> io::Result<()>
    where
        I: Iterator<Item = Vec<u8>>,
    {
        let records_per_buffer = self.memory_limit / self.record_size;
        let mut buffer: Vec<Vec<u8>> = Vec::with_capacity(records_per_buffer);
        let cmp = self.key_spec.comparator();

        for record in records {
            buffer.push(record);
            if buffer.len() >= records_per_buffer {
                // Sort the buffer and write to a temp file
                if self.key_spec.stable {
                    buffer.sort_by(|a, b| cmp(a, b));
                } else {
                    buffer.sort_unstable_by(|a, b| cmp(a, b));
                }
                self.write_run(&buffer)?;
                buffer.clear();
            }
        }

        // Write remaining records
        if !buffer.is_empty() {
            if self.key_spec.stable {
                buffer.sort_by(|a, b| cmp(a, b));
            } else {
                buffer.sort_unstable_by(|a, b| cmp(a, b));
            }
            self.write_run(&buffer)?;
        }

        Ok(())
    }

    /// Write a sorted run to a temporary file
    fn write_run(&mut self, records: &[Vec<u8>]) -> io::Result<()> {
        let mut temp = NamedTempFile::new()?;
        {
            let mut writer = BufWriter::new(&mut temp);
            for record in records {
                // Write 4-byte length prefix + data (variable length support)
                let len = record.len() as u32;
                writer.write_all(&len.to_be_bytes())?;
                writer.write_all(record)?;
            }
            writer.flush()?;
        }
        self.run_record_counts.push(records.len());
        self.run_files.push(temp);
        Ok(())
    }

    /// Phase 2: K-way merge of sorted runs
    pub fn merge_runs(&mut self) -> io::Result<MergeIterator> {
        let mut readers: Vec<RunReader> = Vec::new();

        for run_file in &mut self.run_files {
            run_file.seek(SeekFrom::Start(0))?;
            let reader = BufReader::new(run_file.reopen()?);
            readers.push(RunReader::new(reader)?);
        }

        Ok(MergeIterator::new(readers, self.key_spec.clone()))
    }
}

/// Reads records from a sorted run file
struct RunReader {
    reader: BufReader<std::fs::File>,
    current: Option<Vec<u8>>,
    exhausted: bool,
}

impl RunReader {
    fn new(mut reader: BufReader<std::fs::File>) -> io::Result<Self> {
        let current = Self::read_next_record(&mut reader)?;
        let exhausted = current.is_none();
        Ok(Self {
            reader,
            current,
            exhausted,
        })
    }

    fn read_next_record(
        reader: &mut BufReader<std::fs::File>,
    ) -> io::Result<Option<Vec<u8>>> {
        let mut len_buf = [0u8; 4];
        match reader.read_exact(&mut len_buf) {
            Ok(()) => {
                let len = u32::from_be_bytes(len_buf) as usize;
                let mut data = vec![0u8; len];
                reader.read_exact(&mut data)?;
                Ok(Some(data))
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn advance(&mut self) -> io::Result<()> {
        self.current = Self::read_next_record(&mut self.reader)?;
        if self.current.is_none() {
            self.exhausted = true;
        }
        Ok(())
    }
}

/// K-way merge iterator over sorted runs
pub struct MergeIterator {
    readers: Vec<RunReader>,
    key_spec: SortKeySpec,
}

impl MergeIterator {
    fn new(readers: Vec<RunReader>, key_spec: SortKeySpec) -> Self {
        Self { readers, key_spec }
    }
}

impl Iterator for MergeIterator {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let cmp = self.key_spec.comparator();

        // Find the reader with the smallest current record
        let mut best_idx: Option<usize> = None;
        for (i, reader) in self.readers.iter().enumerate() {
            if reader.exhausted {
                continue;
            }
            match best_idx {
                None => best_idx = Some(i),
                Some(bi) => {
                    let best_rec = self.readers[bi].current.as_ref().unwrap();
                    let this_rec = reader.current.as_ref().unwrap();
                    if cmp(this_rec, best_rec) == Ordering::Less {
                        best_idx = Some(i);
                    }
                }
            }
        }

        match best_idx {
            None => None, // all runs exhausted
            Some(idx) => {
                let record = self.readers[idx].current.clone().unwrap();
                match self.readers[idx].advance() {
                    Ok(()) => Some(Ok(record)),
                    Err(e) => Some(Err(e)),
                }
            }
        }
    }
}
```

### 9.5 INPUT/OUTPUT PROCEDURE Mapping

INPUT/OUTPUT PROCEDURES are COBOL sections that act as callbacks. In Rust,
they map to closures:

```rust
/// Handle for releasing records to the sort (INPUT PROCEDURE)
pub struct Releaser<'a> {
    sort_engine: &'a mut dyn SortEngine,
}

impl<'a> Releaser<'a> {
    /// RELEASE record-name [FROM identifier]
    pub fn release(&mut self, record: &[u8]) -> SortResult<()> {
        self.sort_engine.accept_record(record)
    }
}

/// Handle for returning sorted records (OUTPUT PROCEDURE)
pub struct Returner<'a> {
    sorted_iter: &'a mut dyn Iterator<Item = io::Result<Vec<u8>>>,
    current_record: Option<Vec<u8>>,
}

impl<'a> Returner<'a> {
    /// RETURN file-name [INTO identifier]
    /// Returns None at end (AT END condition)
    pub fn return_next(&mut self) -> SortResult<Option<&[u8]>> {
        match self.sorted_iter.next() {
            Some(Ok(record)) => {
                self.current_record = Some(record);
                Ok(self.current_record.as_deref())
            }
            Some(Err(e)) => Err(SortError::IoError(e)),
            None => Ok(None), // AT END
        }
    }
}

/// Top-level sort entry point
pub struct CobolSort {
    config: SortConfig,
    key_spec: SortKeySpec,
    sd_record_size: usize,
}

impl CobolSort {
    /// SORT ... USING ... GIVING (simple form)
    pub fn sort_using_giving(
        &self,
        input_files: &mut [&mut dyn SeqFileOps],
        output_files: &mut [&mut dyn SeqFileOps],
    ) -> SortResult<SortReturn> {
        // Phase 1: Read all records from input files
        let records = self.read_all_inputs(input_files)?;

        // Phase 2: Sort
        let sorted = self.sort_records(records)?;

        // Phase 3: Write to all output files
        self.write_all_outputs(output_files, sorted)?;

        Ok(SortReturn::Success)
    }

    /// SORT ... INPUT PROCEDURE ... GIVING
    pub fn sort_with_input_procedure<F>(
        &self,
        input_proc: F,
        output_files: &mut [&mut dyn SeqFileOps],
    ) -> SortResult<SortReturn>
    where
        F: FnOnce(&mut Releaser) -> SortResult<()>,
    {
        // Phase 1: Run INPUT PROCEDURE, collect RELEASEd records
        let mut engine = self.create_engine();
        {
            let mut releaser = Releaser {
                sort_engine: &mut engine,
            };
            input_proc(&mut releaser)?;
        }

        // Phase 2: Sort
        let sorted = engine.finish_and_sort()?;

        // Phase 3: Write to output files
        self.write_sorted_to_files(output_files, sorted)?;

        Ok(SortReturn::Success)
    }

    /// SORT ... USING ... OUTPUT PROCEDURE
    pub fn sort_with_output_procedure<F>(
        &self,
        input_files: &mut [&mut dyn SeqFileOps],
        output_proc: F,
    ) -> SortResult<SortReturn>
    where
        F: FnOnce(&mut Returner) -> SortResult<()>,
    {
        // Phase 1: Read all records from input files
        let records = self.read_all_inputs(input_files)?;

        // Phase 2: Sort
        let sorted = self.sort_records(records)?;

        // Phase 3: Run OUTPUT PROCEDURE
        let mut iter = sorted.into_iter().map(Ok);
        let mut returner = Returner {
            sorted_iter: &mut iter,
            current_record: None,
        };
        output_proc(&mut returner)?;

        Ok(SortReturn::Success)
    }

    /// SORT ... INPUT PROCEDURE ... OUTPUT PROCEDURE
    pub fn sort_with_procedures<F1, F2>(
        &self,
        input_proc: F1,
        output_proc: F2,
    ) -> SortResult<SortReturn>
    where
        F1: FnOnce(&mut Releaser) -> SortResult<()>,
        F2: FnOnce(&mut Returner) -> SortResult<()>,
    {
        // Phase 1: Run INPUT PROCEDURE
        let mut engine = self.create_engine();
        {
            let mut releaser = Releaser {
                sort_engine: &mut engine,
            };
            input_proc(&mut releaser)?;
        }

        // Phase 2: Sort
        let sorted = engine.finish_and_sort()?;

        // Phase 3: Run OUTPUT PROCEDURE
        let mut iter = sorted.into_iter().map(Ok);
        let mut returner = Returner {
            sorted_iter: &mut iter,
            current_record: None,
        };
        output_proc(&mut returner)?;

        Ok(SortReturn::Success)
    }

    /// Create the appropriate sort engine based on estimated size
    fn create_engine(&self) -> Box<dyn SortEngine> {
        // Decision: in-memory or external
        // For INPUT PROCEDURE, we don't know the size in advance,
        // so start in-memory and switch to external if needed
        Box::new(AdaptiveSortEngine::new(
            self.key_spec.clone(),
            self.sd_record_size,
            self.config.memory_limit,
        ))
    }
}
```

### 9.6 MERGE Implementation

MERGE is simpler than SORT -- it only performs a k-way merge of pre-sorted
inputs:

```rust
pub struct CobolMerge {
    key_spec: SortKeySpec,
    sd_record_size: usize,
}

impl CobolMerge {
    /// MERGE ... USING ... GIVING
    pub fn merge_using_giving(
        &self,
        input_files: &mut [&mut dyn SeqFileOps],
        output_files: &mut [&mut dyn SeqFileOps],
    ) -> SortResult<SortReturn> {
        assert!(
            input_files.len() >= 2,
            "MERGE requires at least 2 input files"
        );

        // K-way merge: use a min-heap to efficiently select the
        // smallest record across all input files
        let cmp = self.key_spec.comparator();
        let mut heap = BinaryHeap::new();

        // Initialize: read first record from each input file
        for (file_idx, file) in input_files.iter_mut().enumerate() {
            let status = file.read_next();
            if status.is_success() {
                let record = file.record_area().to_vec();
                heap.push(MergeEntry {
                    record,
                    file_idx,
                    cmp: &cmp,
                });
            }
        }

        // Merge loop: extract min, write, refill from same file
        while let Some(entry) = heap.pop() {
            // Write to all output files
            for output in output_files.iter_mut() {
                let area = output.record_area_mut();
                area[..entry.record.len()]
                    .copy_from_slice(&entry.record);
                output.write_record();
            }

            // Read next record from the same input file
            let status = input_files[entry.file_idx].read_next();
            if status.is_success() {
                let record = input_files[entry.file_idx]
                    .record_area()
                    .to_vec();
                heap.push(MergeEntry {
                    record,
                    file_idx: entry.file_idx,
                    cmp: &cmp,
                });
            }
        }

        Ok(SortReturn::Success)
    }

    /// MERGE ... USING ... OUTPUT PROCEDURE
    pub fn merge_with_output_procedure<F>(
        &self,
        input_files: &mut [&mut dyn SeqFileOps],
        output_proc: F,
    ) -> SortResult<SortReturn>
    where
        F: FnOnce(&mut Returner) -> SortResult<()>,
    {
        // Same k-way merge, but feed records to OUTPUT PROCEDURE
        // instead of writing to files
        let merged = self.merge_to_vec(input_files)?;
        let mut iter = merged.into_iter().map(Ok);
        let mut returner = Returner {
            sorted_iter: &mut iter,
            current_record: None,
        };
        output_proc(&mut returner)?;
        Ok(SortReturn::Success)
    }
}
```

### 9.7 Trait Hierarchy

```rust
/// Abstract sort engine (in-memory or external)
trait SortEngine {
    /// Accept a record (from RELEASE or USING file)
    fn accept_record(&mut self, record: &[u8]) -> SortResult<()>;

    /// Sort all accumulated records and return sorted output
    fn finish_and_sort(self: Box<Self>) -> SortResult<Vec<Vec<u8>>>;
}

/// Adaptive engine: starts in-memory, switches to external if needed
pub struct AdaptiveSortEngine {
    key_spec: SortKeySpec,
    record_size: usize,
    memory_limit: usize,
    // Starts as in-memory, may switch to external
    mode: SortMode,
}

enum SortMode {
    InMemory(InMemorySort),
    External(ExternalMergeSort),
}

impl SortEngine for AdaptiveSortEngine {
    fn accept_record(&mut self, record: &[u8]) -> SortResult<()> {
        match &mut self.mode {
            SortMode::InMemory(mem_sort) => {
                mem_sort.release(record);
                // Check if we've exceeded memory limit
                if mem_sort.memory_usage() > self.memory_limit {
                    // Switch to external sort
                    let mut ext = ExternalMergeSort::new(
                        self.key_spec.clone(),
                        self.record_size,
                        self.memory_limit,
                    );
                    // Flush current in-memory records as first run
                    let mut records = std::mem::take(&mut mem_sort.records);
                    ext.build_runs(records.drain(..))?;
                    self.mode = SortMode::External(ext);
                }
                Ok(())
            }
            SortMode::External(ext) => {
                ext.build_runs(std::iter::once(record.to_vec()))?;
                Ok(())
            }
        }
    }

    fn finish_and_sort(self: Box<Self>) -> SortResult<Vec<Vec<u8>>> {
        match self.mode {
            SortMode::InMemory(mut mem_sort) => {
                mem_sort.sort();
                Ok(mem_sort.records)
            }
            SortMode::External(mut ext) => {
                let mut result = Vec::new();
                let iter = ext.merge_runs()?;
                for record in iter {
                    result.push(record?);
                }
                Ok(result)
            }
        }
    }
}
```

### 9.8 Configuration

```rust
#[derive(Debug, Clone)]
pub struct SortConfig {
    /// Maximum memory for in-memory sort (bytes)
    /// Default: 256 MB
    pub memory_limit: usize,

    /// Temporary directory for external sort work files
    /// Default: system temp dir
    pub temp_dir: PathBuf,

    /// Maximum number of work files (merge fan-in)
    /// Default: 16 (higher = fewer merge passes)
    pub max_work_files: usize,

    /// Buffer size for reading/writing work files
    /// Default: 64 KB
    pub io_buffer_size: usize,

    /// Enable EBCDIC collating sequence by default
    pub default_collating: CollatingSequence,
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            memory_limit: 256 * 1024 * 1024, // 256 MB
            temp_dir: std::env::temp_dir(),
            max_work_files: 16,
            io_buffer_size: 64 * 1024,
            default_collating: CollatingSequence::Native,
        }
    }
}

/// Sort operation return code (maps to SORT-RETURN special register)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortReturn {
    Success,   // SORT-RETURN = 0
    Failed,    // SORT-RETURN = 16
}

impl SortReturn {
    pub fn as_cobol_code(&self) -> i32 {
        match self {
            SortReturn::Success => 0,
            SortReturn::Failed => 16,
        }
    }
}

/// Sort-specific errors
#[derive(Debug)]
pub enum SortError {
    IoError(io::Error),
    InsufficientWorkSpace,
    InvalidKeySpecification,
    NestedSort,     // SORT within INPUT/OUTPUT PROCEDURE
    InvalidVerb,    // RELEASE outside INPUT PROC, RETURN outside OUTPUT PROC
}

pub type SortResult<T> = Result<T, SortError>;
```

---

## 10. Worked Examples

### 10.1 Simple SORT USING GIVING

```cobol
*> Sort employees by department (ascending), then by salary (descending)
SD  SORT-FILE.
01  SORT-REC.
    05  SF-EMPID    PIC 9(8).
    05  SF-NAME     PIC X(30).
    05  SF-DEPT     PIC X(4).
    05  SF-SALARY   PIC S9(7)V99 COMP-3.
    05  SF-FILLER   PIC X(53).

SORT SORT-FILE
    ON ASCENDING KEY SF-DEPT
    ON DESCENDING KEY SF-SALARY
    WITH DUPLICATES IN ORDER
    USING EMP-INPUT
    GIVING EMP-SORTED.
```

**Generated Rust sketch**:

```rust
let key_spec = SortKeySpec {
    keys: vec![
        SortKey {
            offset: 38, // SF-DEPT offset
            length: 4,
            ascending: true,
            key_type: SortKeyType::Alphanumeric,
        },
        SortKey {
            offset: 42, // SF-SALARY offset
            length: 5,  // COMP-3 S9(7)V99 = 5 bytes
            ascending: false,
            key_type: SortKeyType::PackedDecimal {
                precision: 9,
                scale: 2,
                signed: true,
            },
        },
    ],
    collating_sequence: CollatingSequence::Native,
    stable: true, // WITH DUPLICATES IN ORDER
};

let sort = CobolSort::new(sort_config, key_spec, 100); // 100-byte records
let result = sort.sort_using_giving(
    &mut [&mut emp_input],
    &mut [&mut emp_sorted],
)?;
sort_return.set(result.as_cobol_code());
```

### 10.2 SORT with INPUT PROCEDURE (Filtering)

```cobol
*> Sort only active employees with salary > 50000
SORT SORT-FILE
    ON ASCENDING KEY SF-DEPT
    INPUT PROCEDURE IS 1000-SELECT
    GIVING ACTIVE-EMP-FILE.

1000-SELECT SECTION.
    OPEN INPUT ALL-EMP-FILE.
    PERFORM UNTIL WS-EOF
        READ ALL-EMP-FILE INTO WS-EMP-REC
            AT END SET WS-EOF TO TRUE
        END-READ
        IF NOT WS-EOF
            IF WS-EMP-STATUS = "A" AND WS-EMP-SALARY > 50000
                MOVE WS-EMP-REC TO SORT-REC
                RELEASE SORT-REC
            END-IF
        END-IF
    END-PERFORM.
    CLOSE ALL-EMP-FILE.
```

**Generated Rust sketch**:

```rust
let result = sort.sort_with_input_procedure(
    |releaser| {
        // 1000-SELECT SECTION
        let status = all_emp_file.open(OpenMode::Input);
        let mut ws_eof = false;

        while !ws_eof {
            let status = all_emp_file.read_next();
            if status.is_end_of_file() {
                ws_eof = true;
            } else if status.is_success() {
                cobol_move(all_emp_file.record_area(), &mut ws_emp_rec);
                if ws_emp_status == b"A" && ws_emp_salary > dec!(50000) {
                    cobol_move(&ws_emp_rec, &mut sort_rec);
                    releaser.release(&sort_rec)?;
                }
            }
        }

        all_emp_file.close();
        Ok(())
    },
    &mut [&mut active_emp_file],
)?;
```

### 10.3 SORT with OUTPUT PROCEDURE (Aggregation)

```cobol
*> Sort by department and produce a summary report
SORT SORT-FILE
    ON ASCENDING KEY SF-DEPT
    USING EMP-FILE
    OUTPUT PROCEDURE IS 2000-SUMMARIZE.

2000-SUMMARIZE SECTION.
    MOVE SPACES TO WS-PREV-DEPT.
    MOVE 0 TO WS-DEPT-TOTAL.
    PERFORM UNTIL WS-SORT-EOF
        RETURN SORT-FILE INTO WS-SORT-REC
            AT END
                IF WS-PREV-DEPT NOT = SPACES
                    PERFORM 2100-PRINT-DEPT-TOTAL
                END-IF
                SET WS-SORT-EOF TO TRUE
            NOT AT END
                IF WS-SORT-DEPT NOT = WS-PREV-DEPT
                    IF WS-PREV-DEPT NOT = SPACES
                        PERFORM 2100-PRINT-DEPT-TOTAL
                    END-IF
                    MOVE WS-SORT-DEPT TO WS-PREV-DEPT
                    MOVE 0 TO WS-DEPT-TOTAL
                END-IF
                ADD WS-SORT-SALARY TO WS-DEPT-TOTAL
        END-RETURN
    END-PERFORM.
```

### 10.4 MERGE of Pre-Sorted Files

```cobol
*> Merge three regional files into one national file
*> Each regional file is already sorted by customer ID
SD  MERGE-WORK.
01  MW-REC.
    05  MW-CUST-ID PIC 9(10).
    05  MW-DATA    PIC X(90).

MERGE MERGE-WORK
    ON ASCENDING KEY MW-CUST-ID
    USING EAST-FILE CENTRAL-FILE WEST-FILE
    GIVING NATIONAL-FILE.
```

**Generated Rust sketch**:

```rust
let key_spec = SortKeySpec {
    keys: vec![SortKey {
        offset: 0,
        length: 10,
        ascending: true,
        key_type: SortKeyType::Alphanumeric,
    }],
    collating_sequence: CollatingSequence::Native,
    stable: true,
};

let merge = CobolMerge::new(key_spec, 100);
let result = merge.merge_using_giving(
    &mut [&mut east_file, &mut central_file, &mut west_file],
    &mut [&mut national_file],
)?;
```

### 10.5 SORT with Both Procedures

```cobol
*> Read transactions, filter by date, sort by amount,
*> write top 100 to report
SORT SORT-FILE
    ON DESCENDING KEY SF-AMOUNT
    INPUT PROCEDURE IS 1000-FILTER-BY-DATE
    OUTPUT PROCEDURE IS 2000-TOP-100.

1000-FILTER-BY-DATE SECTION.
    OPEN INPUT TRANS-FILE.
    PERFORM UNTIL WS-EOF
        READ TRANS-FILE INTO WS-TRANS
            AT END SET WS-EOF TO TRUE
        END-READ
        IF NOT WS-EOF
            IF WS-TRANS-DATE >= WS-START-DATE
               AND WS-TRANS-DATE <= WS-END-DATE
                MOVE WS-TRANS TO SORT-REC
                RELEASE SORT-REC
            END-IF
        END-IF
    END-PERFORM.
    CLOSE TRANS-FILE.

2000-TOP-100 SECTION.
    MOVE 0 TO WS-COUNT.
    PERFORM UNTIL WS-SORT-EOF OR WS-COUNT >= 100
        RETURN SORT-FILE INTO WS-SORTED-TRANS
            AT END SET WS-SORT-EOF TO TRUE
            NOT AT END
                ADD 1 TO WS-COUNT
                PERFORM 2100-WRITE-REPORT-LINE
        END-RETURN
    END-PERFORM.
```

---

## 11. Dialect Variations

### 11.1 IBM Enterprise COBOL

| Feature | Behavior |
|---|---|
| Sort engine | DFSORT or SYNCSORT (external product) |
| Sort work files | SORTWKnn DD statements (1-32) |
| SORT-CONTROL | DD name for DFSORT control statements |
| SORT-MESSAGE | DD name for sort messages |
| SORT-CORE-SIZE | Supported (memory hint) |
| SORT-FILE-SIZE | Supported (record count hint) |
| SORT-MODE-SIZE | Supported (record size hint) |
| Max keys | 256 sort keys |
| Max record size | 32,760 bytes (fixed), 32,756 bytes (variable) |
| WITH DUPLICATES | Maps to DFSORT EQUALS option |
| FASTSRT compiler option | Optimizes SORT USING/GIVING by bypassing COBOL I/O |
| Sort exits | E15 (input), E35 (output), E18, E39 |
| OUTFIL | Multiple output files with filtering (DFSORT) |

**FASTSRT optimization**: When the FASTSRT compiler option is active and
the SORT uses simple USING/GIVING (no procedures), DFSORT reads/writes
the files directly without going through COBOL's I/O routines. This
significantly improves performance for large datasets.

### 11.2 GnuCOBOL

| Feature | Behavior |
|---|---|
| Sort engine | Built-in (qsort-based) or external (shell sort command) |
| Sort work files | Temporary files in system temp directory |
| SORT-CONTROL | Supported (environment variable) |
| SORT-MESSAGE | Limited support |
| Special registers | SORT-RETURN supported; others may be limited |
| Max keys | Unlimited (practical limit) |
| Max record size | Limited by available memory |
| WITH DUPLICATES | Stable sort via merge sort algorithm |
| COLLATING SEQUENCE | Supported including LOCALE |

### 11.3 Micro Focus Visual COBOL

| Feature | Behavior |
|---|---|
| Sort engine | Built-in Micro Focus sort |
| Sort work files | System-managed temporary files |
| SORT-CONTROL | Supported |
| Max keys | 64 sort keys (typical limit) |
| Max record size | Platform-dependent |
| WITH DUPLICATES | Supported (stable sort) |
| File-based sort | Can delegate to OS sort utility |

### 11.4 Transpiler Sort Configuration

```rust
#[derive(Debug, Clone)]
pub struct SortDialectConfig {
    pub dialect: CobolDialect,

    /// Use stable sort even without WITH DUPLICATES
    /// (some programs depend on stable order without specifying it)
    pub always_stable: bool,

    /// Default collating sequence for sort keys
    pub default_collating: CollatingSequence,

    /// Enable FASTSRT-style optimization (direct file I/O bypass)
    pub fast_sort: bool,

    /// Maximum sort memory (0 = automatic)
    pub max_memory: usize,

    /// Temporary directory for sort work files
    pub work_dir: Option<PathBuf>,
}

impl Default for SortDialectConfig {
    fn default() -> Self {
        Self {
            dialect: CobolDialect::IbmEnterprise,
            always_stable: false,
            default_collating: CollatingSequence::Native,
            fast_sort: true,
            max_memory: 0,
            work_dir: None,
        }
    }
}
```

---

## Appendix A: SORT/MERGE Quick Reference

| Verb | Context | Purpose |
|---|---|---|
| SORT | PROCEDURE DIVISION | Sort records by keys |
| MERGE | PROCEDURE DIVISION | Merge pre-sorted files |
| RELEASE | INPUT PROCEDURE only | Send record to sort |
| RETURN | OUTPUT PROCEDURE only | Get next sorted record |

| Special Register | Purpose | Default |
|---|---|---|
| SORT-RETURN | Return code (0 or 16) | 0 |
| SORT-FILE-SIZE | Estimated record count | 0 (auto) |
| SORT-CORE-SIZE | Available memory (bytes) | 0 (auto) |
| SORT-MODE-SIZE | Record size hint | 0 (auto) |
| SORT-MESSAGE | Message output DD name | SYSOUT |
| SORT-CONTROL | Control statement DD name | IGZSRTCD |

---

## Appendix B: DFSORT Control Statement Quick Reference

| Statement | Example | Purpose |
|---|---|---|
| SORT FIELDS | `SORT FIELDS=(1,10,CH,A)` | Sort keys |
| INCLUDE | `INCLUDE COND=(1,1,CH,EQ,C'A')` | Filter records (include) |
| OMIT | `OMIT COND=(1,1,CH,EQ,C'D')` | Filter records (exclude) |
| SUM | `SUM FIELDS=(11,8,ZD)` | Aggregate equal-key records |
| INREC | `INREC FIELDS=(1,20,C' ',21,10)` | Reformat input records |
| OUTREC | `OUTREC FIELDS=(1,20,C' TOTAL')` | Reformat output records |
| OUTFIL | `OUTFIL FNAMES=OUT1,...` | Multiple filtered outputs |
| OPTION | `OPTION EQUALS,DYNALLOC` | Sort options |

---

## Appendix C: Cross-Reference to Other Specifications

| Topic | Document | Section |
|---|---|---|
| File I/O operations | `cobol_file_io_record_processing.md` | Section 5 |
| VSAM file organizations | `cobol_file_io_record_processing.md` | Section 8 |
| MOVE semantics (RELEASE FROM) | `cobol_move_engine_spec.md` | Section 8 |
| Control flow (PERFORM in procedures) | `cobol_control_flow_constructs.md` | Section 4 |
| Numeric type comparison | `cobol_to_rust_datatype_mapping.md` | Sections 2-3 |
| COMP-3 packed decimal format | `cobol_to_rust_datatype_mapping.md` | Section 2.3 |
