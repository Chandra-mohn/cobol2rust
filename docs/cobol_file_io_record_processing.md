# COBOL File I/O & Record Processing: Complete Specification

## Purpose

This document specifies the complete semantics of COBOL file I/O operations,
record processing, and IBM VSAM internals for the cobol2rust transpiler
runtime library. File I/O is one of COBOL's primary domains -- the language
was designed around record-oriented batch processing. Getting the file
abstraction right is essential for faithful migration.

This spec covers: all four file organizations (Sequential, Line Sequential,
Indexed, Relative), SELECT/ASSIGN, FD entries, all I/O verbs, file status
codes, DECLARATIVES error handling, deep VSAM internals (KSDS/ESDS/RRDS/LDS),
record buffer management, variable-length records, LINAGE print control,
CODE-SET encoding, and the complete Rust implementation architecture.

**Not covered here**: SORT/MERGE operations (see future `cobol_sort_merge_spec.md`).

---

## Table of Contents

1. [Fundamental Concepts](#1-fundamental-concepts)
2. [File Organization Types](#2-file-organization-types)
   - 2.1 [Sequential (QSAM)](#21-sequential-qsam)
   - 2.2 [Line Sequential](#22-line-sequential)
   - 2.3 [Indexed (VSAM KSDS)](#23-indexed-vsam-ksds)
   - 2.4 [Relative (VSAM RRDS)](#24-relative-vsam-rrds)
3. [File Control (SELECT/ASSIGN)](#3-file-control-selectassign)
4. [File Description (FD)](#4-file-description-fd)
5. [I/O Verbs](#5-io-verbs)
6. [File Status Codes](#6-file-status-codes)
7. [Error Handling (DECLARATIVES)](#7-error-handling-declaratives)
8. [IBM VSAM Deep Internals](#8-ibm-vsam-deep-internals)
   - 8.1 [VSAM Architecture Overview](#81-vsam-architecture-overview)
   - 8.2 [KSDS (Key-Sequenced Data Set)](#82-ksds-key-sequenced-data-set)
   - 8.3 [ESDS (Entry-Sequenced Data Set)](#83-esds-entry-sequenced-data-set)
   - 8.4 [RRDS (Relative Record Data Set)](#84-rrds-relative-record-data-set)
   - 8.5 [LDS (Linear Data Set)](#85-lds-linear-data-set)
   - 8.6 [Control Intervals and Control Areas](#86-control-intervals-and-control-areas)
   - 8.7 [VSAM Catalogs](#87-vsam-catalogs)
   - 8.8 [Alternate Indexes (AIX)](#88-alternate-indexes-aix)
   - 8.9 [IDCAMS Utility Equivalents](#89-idcams-utility-equivalents)
   - 8.10 [VSAM Performance Concepts](#810-vsam-performance-concepts)
9. [Record Layout and Buffer Management](#9-record-layout-and-buffer-management)
10. [Variable-Length Records](#10-variable-length-records)
11. [LINAGE and Print File Control](#11-linage-and-print-file-control)
12. [CODE-SET and Encoding](#12-code-set-and-encoding)
13. [Rust Implementation Architecture](#13-rust-implementation-architecture)
14. [Worked Examples](#14-worked-examples)
15. [Dialect Variations](#15-dialect-variations)

---

## 1. Fundamental Concepts

### 1.1 Record-Oriented vs Stream-Oriented

COBOL files are **record-oriented**. Every I/O operation reads or writes a
complete record. There is no concept of reading partial records, seeking to
byte offsets within records, or streaming bytes. This is fundamentally
different from Unix/C-style byte-stream I/O.

```
COBOL model:                    Unix model:
+--------+--------+--------+   +----------------------------+
|Record 1|Record 2|Record 3|   |byte stream..................|
+--------+--------+--------+   +----------------------------+
   READ returns one record        read() returns N bytes
```

### 1.2 The FD Record Area

Each file has exactly one **record area** (buffer) defined by the FD entry.
A READ operation populates this buffer with the next (or keyed) record. A
WRITE sends this buffer's contents to the file. Only one record exists in
the buffer at a time -- reading a new record overwrites the previous one.

```cobol
FD  CUSTOMER-FILE.
01  CUSTOMER-RECORD.
    05  CUST-ID     PIC 9(8).
    05  CUST-NAME   PIC X(30).
    05  CUST-BAL    PIC S9(7)V99 COMP-3.
```

After `READ CUSTOMER-FILE`, the fields `CUST-ID`, `CUST-NAME`, `CUST-BAL`
contain the data from the record just read.

### 1.3 File Processing Paradigm

The typical COBOL file processing pattern:

```
OPEN file(s)
PERFORM UNTIL end-condition
    READ file
    AT END SET end-flag TO TRUE
    NOT AT END
        process record
        WRITE output-record
    END-READ
END-PERFORM
CLOSE file(s)
```

### 1.4 Relationship to JCL (IBM)

On IBM mainframes, COBOL programs do not reference physical file paths
directly. The `ASSIGN TO` clause names a **ddname** (Data Definition name),
which is resolved to a physical dataset by JCL (Job Control Language) at
execution time:

```cobol
SELECT CUSTOMER-FILE ASSIGN TO CUSTFILE.
```
```jcl
//CUSTFILE DD DSN=PROD.CUSTOMER.MASTER,DISP=SHR
```

In the Rust runtime, this indirection maps to a configuration layer that
resolves logical names to file paths or database connections.

---

## 2. File Organization Types

### 2.1 Sequential (QSAM)

**IBM implementation**: QSAM (Queued Sequential Access Method)

Records are stored in the order they are written. Access is strictly
sequential -- you must read from beginning to end. There is no random access,
no key-based retrieval, and no ability to update records in place.

**Characteristics**:
- Records are stored in physical order of WRITE operations
- READ returns the next record in sequence
- No REWRITE or DELETE operations (read-only once written)
- WRITE always appends (unless the file is opened OUTPUT, which replaces)
- Most efficient for batch processing (sequential scans)
- Supports fixed-length and variable-length records
- Can be stored on tape or disk

**Access modes**: SEQUENTIAL only

**Recording modes**:
- **F (Fixed)**: All records are the same length
- **V (Variable)**: Each record has a 4-byte RDW (Record Descriptor Word)
  prefix containing the record length
- **U (Undefined)**: No record structure -- blocks are treated as units
  (used for load modules, not typical COBOL data)
- **S (Spanned)**: Variable-length records that can span multiple blocks
  (when a record is larger than the block size)

**COBOL syntax**:

```cobol
ENVIRONMENT DIVISION.
INPUT-OUTPUT SECTION.
FILE-CONTROL.
    SELECT TRANS-FILE
        ASSIGN TO TRANSIN
        ORGANIZATION IS SEQUENTIAL
        FILE STATUS IS WS-TRANS-STATUS.

DATA DIVISION.
FILE SECTION.
FD  TRANS-FILE
    RECORDING MODE IS F
    RECORD CONTAINS 80 CHARACTERS
    BLOCK CONTAINS 0 RECORDS.
01  TRANS-RECORD.
    05  TRANS-TYPE  PIC X.
    05  TRANS-ACCT  PIC 9(10).
    05  TRANS-AMT   PIC S9(7)V99 COMP-3.
    05  FILLER      PIC X(64).
```

**Valid operations**:

| OPEN Mode | READ | WRITE | REWRITE | DELETE | START |
|---|---|---|---|---|---|
| INPUT | Yes | No | No | No | No |
| OUTPUT | No | Yes | No | No | No |
| I-O | Yes | No | Yes(1) | No | No |
| EXTEND | No | Yes | No | No | No |

(1) REWRITE for sequential files replaces the record most recently READ.
The replacement record must be the same length (for fixed-length files).

### 2.2 Line Sequential

**Not in ANSI standard** -- universal extension (IBM, GnuCOBOL, Micro Focus)

Records are terminated by platform-specific line delimiters (LF on Unix,
CR+LF on Windows). This is the bridge between COBOL's record model and
text files.

**Characteristics**:
- Each record is a text line
- Line delimiters are NOT part of the record data
- Trailing spaces may be stripped on WRITE (implementation-dependent)
- Records are variable-length (determined by delimiter position)
- No REWRITE or DELETE

**Differences from Sequential**:
- No RDW -- length determined by delimiter
- No BLOCK CONTAINS (blocking is meaningless for text files)
- No RECORDING MODE (always text-mode)
- Trailing spaces handling varies by implementation

```cobol
SELECT TEXT-FILE
    ASSIGN TO TEXTOUT
    ORGANIZATION IS LINE SEQUENTIAL
    FILE STATUS IS WS-TEXT-STATUS.

FD  TEXT-FILE.
01  TEXT-RECORD PIC X(132).
```

### 2.3 Indexed (VSAM KSDS)

**IBM implementation**: VSAM KSDS (Key-Sequenced Data Set)

Records are stored and retrieved by key values. Each record has a primary key
that uniquely identifies it. Optional alternate keys provide additional access
paths. Supports sequential, random, and dynamic (mixed) access.

**Characteristics**:
- Every record has a primary key (unique, embedded in the record)
- Optional alternate keys (unique or with duplicates)
- Supports all three access modes: SEQUENTIAL, RANDOM, DYNAMIC
- Supports all I/O operations: READ, WRITE, REWRITE, DELETE, START
- B-tree index structure for efficient key lookup
- Records can be fixed or variable length

**Access modes**:

| Access Mode | READ | WRITE | REWRITE | DELETE | START |
|---|---|---|---|---|---|
| SEQUENTIAL | Next record | In key sequence | After READ | After READ | Yes |
| RANDOM | By key value | By key value | By key value | By key value | No |
| DYNAMIC | Both | Both | Both | Both | Yes |

**COBOL syntax**:

```cobol
SELECT CUSTOMER-FILE
    ASSIGN TO CUSTMAST
    ORGANIZATION IS INDEXED
    ACCESS MODE IS DYNAMIC
    RECORD KEY IS CUST-ID
    ALTERNATE RECORD KEY IS CUST-NAME WITH DUPLICATES
    ALTERNATE RECORD KEY IS CUST-SSN
    FILE STATUS IS WS-CUST-STATUS.

FD  CUSTOMER-FILE
    RECORD CONTAINS 200 CHARACTERS.
01  CUSTOMER-RECORD.
    05  CUST-ID     PIC 9(8).
    05  CUST-NAME   PIC X(30).
    05  CUST-SSN    PIC 9(9).
    05  CUST-ADDR   PIC X(50).
    05  CUST-BAL    PIC S9(9)V99 COMP-3.
    05  FILLER      PIC X(97).
```

**Key concepts**:

- **RECORD KEY**: Primary key, must be unique, embedded within the record
  at a fixed offset. The field named in RECORD KEY must be a subordinate
  of the FD's 01-level record description.

- **ALTERNATE RECORD KEY**: Secondary access paths. Can be WITH DUPLICATES
  (non-unique) or without (unique). Each alternate key creates a separate
  index structure.

- **START**: Positions the file for sequential reading from a specified key
  value. Supports relational operators:
  ```cobol
  START CUSTOMER-FILE KEY >= CUST-ID
  START CUSTOMER-FILE KEY = CUST-NAME
  ```

- **DYNAMIC access**: Allows mixing sequential and random access in the same
  OPEN session. `READ CUSTOMER-FILE` reads sequentially;
  `READ CUSTOMER-FILE KEY IS CUST-ID` reads randomly.

### 2.4 Relative (VSAM RRDS)

**IBM implementation**: VSAM RRDS (Relative Record Data Set)

Records are accessed by their relative record number (position/slot number).
Record 1 is the first slot, record 2 is the second, etc. Slots can be
empty (deleted records leave empty slots).

**Characteristics**:
- Each record is identified by its relative record number (1-based)
- Fixed-size slots -- all records occupy the same space
- Empty slots are tracked (deleted records)
- Supports sequential, random, and dynamic access
- Simpler than indexed (no key management) but limited use cases

**COBOL syntax**:

```cobol
SELECT SLOT-FILE
    ASSIGN TO SLOTDATA
    ORGANIZATION IS RELATIVE
    ACCESS MODE IS DYNAMIC
    RELATIVE KEY IS WS-SLOT-NUMBER
    FILE STATUS IS WS-SLOT-STATUS.

WORKING-STORAGE SECTION.
01  WS-SLOT-NUMBER PIC 9(8).

FD  SLOT-FILE
    RECORD CONTAINS 100 CHARACTERS.
01  SLOT-RECORD.
    05  SLOT-DATA  PIC X(100).
```

**RELATIVE KEY**: A working-storage field (NOT part of the record) that
holds the record number for random access. On READ, it returns the record
number of the record just read. On WRITE/REWRITE/DELETE, it specifies which
slot to target.

**Access pattern**:

| Access Mode | READ | WRITE | REWRITE | DELETE |
|---|---|---|---|---|
| SEQUENTIAL | Next non-empty slot | Next empty slot | After READ | After READ |
| RANDOM | By slot number | By slot number | By slot number | By slot number |
| DYNAMIC | Both | Both | Both | Both |

---

## 3. File Control (SELECT/ASSIGN)

The FILE-CONTROL paragraph establishes the mapping between logical file names
in the COBOL program and their physical characteristics.

### 3.1 Complete SELECT Syntax

```cobol
SELECT [OPTIONAL] file-name
    ASSIGN TO assignment-name
    [ORGANIZATION IS {SEQUENTIAL | LINE SEQUENTIAL | INDEXED | RELATIVE}]
    [ACCESS MODE IS {SEQUENTIAL | RANDOM | DYNAMIC}]
    [RECORD KEY IS data-name-1]
    [ALTERNATE RECORD KEY IS data-name-2 [WITH DUPLICATES]]
    [RELATIVE KEY IS data-name-3]
    [FILE STATUS IS data-name-4]
    [LOCK MODE IS {MANUAL | AUTOMATIC} [WITH LOCK ON {RECORD | MULTIPLE RECORDS}]]
    [PASSWORD IS data-name-5]
    .
```

### 3.2 OPTIONAL Keyword

The OPTIONAL keyword applies to INPUT and I-O files. It means the file need
not exist when opened. If the file does not exist:
- OPEN INPUT: succeeds, first READ returns AT END immediately
- OPEN I-O: succeeds, creates the file
- Without OPTIONAL: OPEN fails with FILE STATUS 35

### 3.3 ASSIGN TO

The assignment name is implementation-dependent:

| Dialect | ASSIGN TO | Meaning |
|---|---|---|
| IBM | `ASSIGN TO ddname` | JCL DD name |
| IBM | `ASSIGN TO ddname-suffix` | Explicit QSAM/VSAM reference |
| GnuCOBOL | `ASSIGN TO "filename.dat"` | Literal file path |
| GnuCOBOL | `ASSIGN TO WS-FILENAME` | Variable file path |
| Micro Focus | `ASSIGN TO "filename"` | File path |

For the transpiler, all ASSIGN TO values are mapped through a configuration
layer to resolve to actual file system paths or database connections.

### 3.4 Organization and Access Mode Combinations

| Organization | Sequential | Random | Dynamic |
|---|---|---|---|
| SEQUENTIAL | Yes (default) | No | No |
| LINE SEQUENTIAL | Yes (default) | No | No |
| INDEXED | Yes | Yes | Yes |
| RELATIVE | Yes | Yes | Yes |

### 3.5 File Status

FILE STATUS specifies a 2-byte alphanumeric field in working storage that
is set after every I/O operation. The program can inspect this field to
determine the outcome of the operation.

```cobol
01  WS-FILE-STATUS PIC XX.
    88  WS-FS-OK        VALUE "00".
    88  WS-FS-EOF       VALUE "10".
    88  WS-FS-NOT-FOUND VALUE "23".
```

See Section 6 for the complete code table.

### 3.6 Locking (Multi-User Access)

IBM VSAM supports record-level and file-level locking:

```cobol
LOCK MODE IS MANUAL WITH LOCK ON RECORD
```

- **MANUAL**: Program explicitly locks/unlocks records
- **AUTOMATIC**: System locks records on READ, unlocks on next I/O
- **WITH LOCK ON RECORD**: Record-level granularity
- **WITH LOCK ON MULTIPLE RECORDS**: Multiple records locked simultaneously

In the Rust implementation, locking maps to file-level or record-level
mutexes, or database-level locking for the indexed file backend.

---

## 4. File Description (FD)

The FD entry describes the physical characteristics of a file.

### 4.1 Complete FD Syntax

```cobol
FD  file-name
    [IS EXTERNAL | IS GLOBAL]
    [RECORD CONTAINS integer-1 CHARACTERS]
    [RECORD CONTAINS integer-1 TO integer-2 CHARACTERS]
    [RECORD IS VARYING IN SIZE FROM integer-1 TO integer-2 CHARACTERS
        DEPENDING ON data-name-1]
    [BLOCK CONTAINS {integer-1 | integer-1 TO integer-2}
        {CHARACTERS | RECORDS}]
    [RECORDING MODE IS {F | V | U | S}]
    [LABEL RECORDS ARE {STANDARD | OMITTED}]
    [VALUE OF implementor-name IS {literal | data-name}]
    [DATA RECORDS ARE data-name-1 [data-name-2 ...]]
    [LINAGE IS {integer | data-name} LINES
        [WITH FOOTING AT {integer | data-name}]
        [LINES AT TOP {integer | data-name}]
        [LINES AT BOTTOM {integer | data-name}]]
    [CODE-SET IS alphabet-name]
    .
```

### 4.2 RECORD CONTAINS

Specifies the record length:

- **Fixed**: `RECORD CONTAINS 80 CHARACTERS` -- all records are 80 bytes
- **Variable range**: `RECORD CONTAINS 50 TO 200 CHARACTERS` -- records
  range from 50 to 200 bytes
- **Varying**: `RECORD IS VARYING IN SIZE FROM 50 TO 200 DEPENDING ON
  WS-REC-LEN` -- the field WS-REC-LEN holds the current record length

If RECORD CONTAINS is omitted, the record size is determined by the 01-level
record description(s) under the FD.

### 4.3 RECORDING MODE (IBM)

| Mode | Record Type | Structure |
|---|---|---|
| **F** | Fixed | All records same length. Block = N records. |
| **V** | Variable | Each record has 4-byte RDW. Block has 4-byte BDW. |
| **U** | Undefined | No record structure. Block is the I/O unit. |
| **S** | Spanned | Variable records can span blocks. Segment descriptors. |

If omitted, IBM determines the mode from the record description:
- Single 01-level with no OCCURS DEPENDING ON -> F
- Multiple 01-levels or OCCURS DEPENDING ON -> V

### 4.4 BLOCK CONTAINS

Specifies physical blocking:

- `BLOCK CONTAINS 10 RECORDS` -- 10 records per physical block
- `BLOCK CONTAINS 800 CHARACTERS` -- 800-byte physical blocks
- `BLOCK CONTAINS 0 RECORDS` -- system-determined blocking (IBM)

Blocking is a physical I/O optimization that is transparent to the COBOL
program's logic. The runtime handles block buffering internally. In the Rust
implementation, blocking maps to buffer sizes in BufReader/BufWriter.

### 4.5 LABEL RECORDS and DATA RECORDS

**Obsolete but still present in legacy code.** These clauses are:
- `LABEL RECORDS ARE STANDARD` -- file has standard system labels
- `LABEL RECORDS ARE OMITTED` -- no labels (tape files)
- `DATA RECORDS ARE record-1, record-2` -- names the 01-levels (documentary)

The transpiler should accept and ignore these clauses.

### 4.6 Multiple Record Descriptions

A single FD can have multiple 01-level record descriptions. These are
**implicit REDEFINES** -- they all share the same record buffer:

```cobol
FD  MIXED-FILE.
01  HEADER-RECORD.
    05  REC-TYPE    PIC X     VALUE "H".
    05  FILE-DATE   PIC 9(8).
    05  FILLER      PIC X(71).
01  DETAIL-RECORD.
    05  REC-TYPE    PIC X.
    05  CUST-ID     PIC 9(8).
    05  AMOUNT      PIC S9(7)V99 COMP-3.
    05  FILLER      PIC X(66).
01  TRAILER-RECORD.
    05  REC-TYPE    PIC X     VALUE "T".
    05  RECORD-COUNT PIC 9(8).
    05  TOTAL-AMT   PIC S9(11)V99 COMP-3.
    05  FILLER      PIC X(63).
```

After a READ, the program inspects `REC-TYPE` (which exists at the same
offset in all three descriptions) to determine which record layout to use.
All three 01-levels overlay the same 80-byte buffer.

---

## 5. I/O Verbs

### 5.1 OPEN

```cobol
OPEN {INPUT | OUTPUT | I-O | EXTEND} file-name-1 [file-name-2 ...]
```

**Modes**:

| Mode | Purpose | File must exist? | Creates file? |
|---|---|---|---|
| INPUT | Read only | Yes (unless OPTIONAL) | No |
| OUTPUT | Write only (creates/replaces) | No | Yes |
| I-O | Read and update | Yes (unless OPTIONAL) | No (IBM: depends) |
| EXTEND | Append (write only at end) | Yes (unless OPTIONAL) | No |

**Rules**:
- A file can be opened in only one mode at a time
- Opening an already-open file -> FILE STATUS 41
- OUTPUT mode destroys existing content (creates from scratch)
- EXTEND positions to end of file for writing
- I-O mode allows READ, REWRITE, DELETE but not WRITE (for sequential files)
- I-O mode allows all operations for indexed and relative files

**IBM VSAM considerations**:
- OPEN OUTPUT for VSAM KSDS creates an empty cluster
- OPEN EXTEND for VSAM KSDS adds records in key sequence
- OPEN I-O for VSAM requires the cluster to be defined (IDCAMS DEFINE)
- SHAREOPTIONS control concurrent access from multiple programs

### 5.2 CLOSE

```cobol
CLOSE file-name-1 [WITH LOCK | WITH NO REWIND | REEL/UNIT [FOR REMOVAL]]
```

**Options**:
- No option: normal close, file can be re-opened
- `WITH LOCK`: close and prevent re-opening in this execution
- `WITH NO REWIND`: leave tape positioned (tape only, ignore for disk)
- `REEL/UNIT`: close current volume of multi-volume file (IBM tape)

**Rules**:
- CLOSE on a file that isn't open -> FILE STATUS 42
- After CLOSE, the record area is undefined
- Resources (buffers, locks) are released

### 5.3 READ

#### 5.3.1 Sequential READ

```cobol
READ file-name [NEXT | PREVIOUS] [INTO identifier-1]
    [AT END imperative-statement-1]
    [NOT AT END imperative-statement-2]
[END-READ]
```

- Reads the next (or previous) record in sequence
- For sequential files: reads the next physical record
- For indexed files (sequential mode): reads the next record in key order
- For relative files (sequential mode): reads the next non-empty slot
- `INTO identifier-1`: implicit MOVE of record area to working-storage field
- `AT END`: executed when no more records are available
- `NEXT`: explicit sequential read (required for DYNAMIC access mode)
- `PREVIOUS`: IBM extension for indexed files -- reads in reverse key order

#### 5.3.2 Random READ

```cobol
READ file-name [INTO identifier-1]
    KEY IS data-name-1
    [INVALID KEY imperative-statement-1]
    [NOT INVALID KEY imperative-statement-2]
[END-READ]
```

- Reads a specific record by key value (indexed) or slot number (relative)
- The key field must be populated before the READ
- For indexed files: `KEY IS` specifies which key (primary or alternate) to use
- For relative files: `RELATIVE KEY` field holds the slot number
- `INVALID KEY`: executed when the key is not found (FILE STATUS 23)

#### 5.3.3 READ with LOCK

```cobol
READ file-name [WITH LOCK | WITH NO LOCK | WITH KEPT LOCK]
```

IBM extension for multi-user VSAM access:
- `WITH LOCK`: lock the record (prevent other programs from updating)
- `WITH NO LOCK`: read without locking
- `WITH KEPT LOCK`: keep existing locks, add new lock

### 5.4 WRITE

```cobol
WRITE record-name [FROM identifier-1]
    [BEFORE | AFTER ADVANCING {integer | data-name | PAGE | mnemonic} LINES]
    [AT END-OF-PAGE imperative-statement-1]
    [NOT AT END-OF-PAGE imperative-statement-2]
    [INVALID KEY imperative-statement-3]
    [NOT INVALID KEY imperative-statement-4]
[END-WRITE]
```

**Important**: WRITE uses the **record name** (01-level), not the file name.

**Operation by organization**:

| Organization | Sequential Mode | Random Mode |
|---|---|---|
| Sequential | Appends to file | N/A |
| Indexed | Writes in key order | Writes by key value |
| Relative | Writes to next slot | Writes to slot number |

**FROM identifier**: Performs an implicit MOVE from identifier to the record
area before writing. Equivalent to: `MOVE id TO record; WRITE record`.

**ADVANCING** (print files):
- `BEFORE ADVANCING 2 LINES`: write, then advance 2 lines
- `AFTER ADVANCING PAGE`: advance to new page, then write
- `AFTER ADVANCING 0 LINES`: overprint (no vertical movement)
- `AFTER ADVANCING mnemonic-name`: channel skip (IBM carriage control)

**INVALID KEY**: For indexed files, triggered when:
- Primary key already exists (duplicate)
- Alternate unique key already exists (duplicate)
- Record is out of sequence (sequential write to indexed file)

### 5.5 REWRITE

```cobol
REWRITE record-name [FROM identifier-1]
    [INVALID KEY imperative-statement-1]
    [NOT INVALID KEY imperative-statement-2]
[END-REWRITE]
```

Updates an existing record in place.

**Rules by organization**:

| Organization | Sequential Access | Random Access |
|---|---|---|
| Sequential | After READ, same length | N/A |
| Indexed | After READ, key must match | By key value |
| Relative | After READ | By slot number |

**Critical rules**:
- Sequential files: REWRITE must follow a successful READ. The record being
  rewritten must be the same length as the record just read (for F mode).
- Indexed files (sequential): must follow a successful READ. The primary key
  must not be changed. Alternate keys can be changed.
- Indexed files (random): the primary key in the record area identifies which
  record to update. No prior READ required.
- Cannot change the primary key of an indexed file record via REWRITE.

### 5.6 DELETE

```cobol
DELETE file-name RECORD
    [INVALID KEY imperative-statement-1]
    [NOT INVALID KEY imperative-statement-2]
[END-DELETE]
```

Removes a record from the file.

**Rules**:
- **Not available for sequential files**
- Indexed files (sequential): deletes the record most recently READ
- Indexed files (random): deletes by primary key value in the record area
- Relative files (sequential): deletes the record most recently READ
- Relative files (random): deletes the slot specified by RELATIVE KEY

**After DELETE**: The record slot is marked as deleted. For indexed files,
the key is removed from all indexes. For relative files, the slot is marked
empty.

### 5.7 START

```cobol
START file-name KEY IS {= | > | >= | < | <= | NOT < | NOT >} data-name
    [INVALID KEY imperative-statement-1]
    [NOT INVALID KEY imperative-statement-2]
[END-START]
```

Positions the file for subsequent sequential READ operations. Only valid for
indexed and relative files with sequential or dynamic access.

**Operations**:
- `KEY IS = data-name`: position at exact match
- `KEY IS > data-name`: position at first record with key greater than value
- `KEY IS >= data-name`: position at first record with key greater than or
  equal to value
- `KEY IS < data-name`: position at last record with key less than value
- `KEY IS <= data-name`: position at last record with key less than or equal

**After START**: The next sequential READ returns the positioned record.
START does NOT read a record -- it only positions.

**Generic key START (IBM)**: If data-name is shorter than the full key,
START uses a partial key match (prefix matching):

```cobol
01  WS-PARTIAL-KEY PIC X(3) VALUE "ABC".
START CUSTOMER-FILE KEY >= WS-PARTIAL-KEY.
*> Positions to first record where first 3 bytes of key >= "ABC"
```

---

## 6. File Status Codes

The FILE STATUS field is a 2-byte alphanumeric field. The first byte is the
status class; the second byte provides detail.

### 6.1 Complete File Status Table

| Code | Class | Meaning | Triggered By |
|---|---|---|---|
| **00** | Success | Operation successful | All verbs |
| **02** | Success | Duplicate key detected (READ) or alternate key exists | READ, WRITE, REWRITE |
| **04** | Success | Record length mismatch (read shorter/longer) | READ |
| **05** | Success | OPEN on OPTIONAL file that doesn't exist (created) | OPEN |
| **07** | Success | CLOSE with NO REWIND/REEL on non-tape | CLOSE |
| **10** | End | AT END: no more records | READ |
| **14** | End | Relative READ: relative key too large for file | READ |
| **21** | Key error | Sequence error: key out of order for sequential write | WRITE |
| **22** | Key error | Duplicate primary key | WRITE, REWRITE |
| **23** | Key error | Record not found | READ, START, DELETE |
| **24** | Boundary | Boundary violation: disk full or KSDS full | WRITE |
| **30** | Permanent | Permanent I/O error | All verbs |
| **34** | Boundary | Boundary violation: sequential write beyond limits | WRITE |
| **35** | Open error | File not found (OPEN INPUT without OPTIONAL) | OPEN |
| **37** | Open error | OPEN mode conflict (permission denied) | OPEN |
| **38** | Open error | File closed WITH LOCK, cannot re-open | OPEN |
| **39** | Open error | File attributes conflict with FD | OPEN |
| **41** | Logic error | File already OPEN | OPEN |
| **42** | Logic error | File not OPEN | CLOSE |
| **43** | Logic error | READ without prior READ/START (sequential delete) | DELETE, REWRITE |
| **44** | Logic error | REWRITE: record size changed (fixed-length file) | REWRITE |
| **46** | Logic error | Sequential READ without positioning (no valid next) | READ |
| **47** | Logic error | READ on file not open INPUT or I-O | READ |
| **48** | Logic error | WRITE on file not open OUTPUT, I-O, or EXTEND | WRITE |
| **49** | Logic error | DELETE/REWRITE on file not open I-O | DELETE, REWRITE |

### 6.2 IBM-Specific Extended Status

IBM provides an extended FILE STATUS via a VSAM return code / reason code
pair. When FILE STATUS is "9x", the program can examine a VSAM feedback
area for detailed error information:

| Code | Meaning |
|---|---|
| **90** | General VSAM logic error |
| **91** | VSAM password failure |
| **92** | Logic error (resource not available) |
| **93** | VSAM resource not available |
| **95** | Invalid or incomplete VSAM file information |
| **96** | No DD statement for file |
| **97** | OPEN successful, file integrity verified |

For IBM files, the VSAM return code (register 15) and reason code (feedback
field) provide additional diagnostics beyond the 2-byte FILE STATUS.

---

## 7. Error Handling (DECLARATIVES)

DECLARATIVES provide an error-handling mechanism for I/O operations:

```cobol
PROCEDURE DIVISION.
DECLARATIVES.
INPUT-ERROR SECTION.
    USE AFTER STANDARD ERROR PROCEDURE ON INPUT.
INPUT-ERROR-PARA.
    DISPLAY "I/O ERROR ON INPUT FILE, STATUS: " WS-STATUS.
    STOP RUN.

CUST-ERROR SECTION.
    USE AFTER STANDARD ERROR PROCEDURE ON CUSTOMER-FILE.
CUST-ERROR-PARA.
    DISPLAY "CUSTOMER FILE ERROR, STATUS: " WS-CUST-STATUS.
    PERFORM ERROR-RECOVERY.
END DECLARATIVES.
```

### 7.1 USE Statement Scope

| USE Statement | Scope |
|---|---|
| `USE AFTER ERROR ON file-name` | Specific file only |
| `USE AFTER ERROR ON INPUT` | All files opened INPUT |
| `USE AFTER ERROR ON OUTPUT` | All files opened OUTPUT |
| `USE AFTER ERROR ON I-O` | All files opened I-O |
| `USE AFTER ERROR ON EXTEND` | All files opened EXTEND |

### 7.2 Execution Flow

1. I/O operation occurs
2. System detects an error
3. FILE STATUS is set
4. If a matching USE procedure exists, it executes
5. Control returns to the statement following the I/O verb
6. AT END / INVALID KEY phrases execute (if applicable)

**Important**: The USE procedure executes BEFORE AT END/INVALID KEY phrases.
This allows error recovery before the conditional logic runs.

### 7.3 Rust Mapping

USE procedures map to registered error callbacks:

```rust
// Registration
file_handle.on_error(|status, file_info| {
    eprintln!("I/O error on {}: status {}", file_info.name, status);
    // Return action: continue, abort, retry
    ErrorAction::Continue
});
```

---

## 8. IBM VSAM Deep Internals

### 8.1 VSAM Architecture Overview

VSAM (Virtual Storage Access Method) is IBM's primary disk file management
system for z/OS. It provides four dataset organizations and a catalog system
for managing them.

```
VSAM Architecture:

+------------------+
|   COBOL Program  |
+------------------+
        |
+------------------+
|   VSAM Macros    |  (GET, PUT, POINT, ERASE, ENDREQ)
+------------------+
        |
+------------------+
|   VSAM Buffers   |  (LSR / NSR / GSR buffer pools)
+------------------+
        |
+------------------+
|   Control        |  CI = Control Interval (physical I/O unit)
|   Intervals      |  CA = Control Area (group of CIs)
+------------------+
        |
+------------------+
|   Physical Disk  |  (DASD extents)
+------------------+
```

### 8.2 KSDS (Key-Sequenced Data Set)

KSDS is the most common VSAM type and maps to COBOL's INDEXED organization.

**Structure**:
- **Data component**: stores the actual records in key sequence
- **Index component**: B+ tree index mapping keys to record positions
- Records are stored in ascending primary key order
- New records are inserted in key sequence (CI/CA splits if necessary)

**Index levels**:
```
Sequence Set (lowest level) -> points to CIs in data component
              |
         Index Set (middle levels) -> points to sequence set entries
              |
         High-Level Index -> root of the B+ tree
```

**Key characteristics**:
- Primary key: unique, fixed position within record, fixed length
- Key is stored both in the data record and in the index component
- Alternate keys: separate VSAM clusters (PATH + AIX)
- Supports free space (FREESPACE parameter) for future inserts
- CI/CA splits accommodate growth

**Record addressing**:
- **RBA (Relative Byte Address)**: physical position in the data component
- Not directly exposed to COBOL programs
- Used internally by VSAM for index pointers

**Insert process**:
1. Locate the CI where the record should be inserted (by key)
2. If the CI has free space: shift existing records, insert
3. If the CI is full: **CI split** -- move half the records to a new CI
4. If the CA has no free CIs: **CA split** -- move half the CIs to a new CA
5. Update index entries to reflect the new positions

**Delete process**:
1. Locate the record by key
2. Mark the record as deleted (reclaim space within the CI)
3. Update indexes
4. Space is reused by future inserts

### 8.3 ESDS (Entry-Sequenced Data Set)

ESDS maps to COBOL's SEQUENTIAL organization when stored in VSAM format.

**Characteristics**:
- Records are stored in the order they are written (chronological)
- No primary key (records identified by RBA)
- No deletion of records
- No insertion between existing records
- New records always appended at the end
- Can have alternate indexes (AIX) for keyed access
- Commonly used for log files, audit trails, journal records

**COBOL usage**:
```cobol
SELECT LOG-FILE
    ASSIGN TO LOGDATA
    ORGANIZATION IS SEQUENTIAL   *> VSAM ESDS behind the scenes
    ACCESS MODE IS SEQUENTIAL
    FILE STATUS IS WS-LOG-STATUS.
```

The programmer may not even know the file is VSAM ESDS -- the JCL/IDCAMS
definition determines the physical organization.

**RBA access (assembler only)**: ESDS records can be accessed by RBA using
VSAM macros, but this is not available from COBOL. From COBOL, ESDS access
is always sequential or via alternate index.

### 8.4 RRDS (Relative Record Data Set)

RRDS maps to COBOL's RELATIVE organization.

**Two subtypes**:

| Type | Record Format | Slot Size |
|---|---|---|
| Fixed-length RRDS | All records same size | record size + control info |
| Variable-length RRDS (IBM extension) | Records vary in size | max record size + control info |

**Characteristics**:
- Records addressed by relative record number (1-based)
- Fixed-size slots in the data component
- Empty slots are tracked with control byte flags
- No index component (slot number = offset calculation)
- Fast random access: record position = (slot_number - 1) * slot_size
- Supports sequential access (skips empty slots)

**Slot structure**:
```
+---+--------------------+
|Ctl|   Record Data      |   Ctl = control byte (empty/occupied)
+---+--------------------+
     \___slot_size_____/
```

### 8.5 LDS (Linear Data Set)

LDS is a VSAM dataset with no record structure -- it is a contiguous byte
stream. Not directly used from COBOL programs.

**Characteristics**:
- No record boundaries
- No CI/CA structure (from the application's perspective)
- Accessed via Data-In-Virtual (DIV) or memory-mapped I/O
- Used for: DB2 tablespaces, IMS databases, custom applications
- Not relevant to COBOL file I/O (included for completeness)

### 8.6 Control Intervals and Control Areas

**Control Interval (CI)**: The unit of physical I/O. A CI is a fixed-size
block of data that VSAM reads/writes as a single physical operation.

```
CI structure:
+--------+--------+-----+--------+------+------+
|Record 1|Record 2| ... |Record N| Free |CIDF  |
+--------+--------+-----+--------+------+------+
                                         |RDFs  |
                                         +------+
```

| Component | Description |
|---|---|
| Records | The actual data records (variable or fixed length) |
| Free Space | Unused space for future insertions |
| RDF | Record Definition Field (4 bytes per record): record length and flags |
| CIDF | CI Definition Field (4 bytes): free space offset and length |

**CI sizes**: 512, 1024, 2048, 4096, 8192, 12288, 16384, 20480, 24576,
28672, 32768 bytes. Default is typically 4096.

**Control Area (CA)**: A group of CIs. CAs are the unit of allocation and
space management.

```
CA structure:
+------+------+------+------+-----+------+------+
| CI 1 | CI 2 | CI 3 | CI 4 | ... | CI N | Free |
+------+------+------+------+-----+------+------+
```

**FREESPACE parameter**: Controls how much free space VSAM leaves for
future inserts:
- `FREESPACE(ci_pct ca_pct)` -- e.g., `FREESPACE(20 10)`
- `ci_pct`: percentage of each CI reserved as free space (for insert within CI)
- `ca_pct`: percentage of CIs in each CA left empty (for CI splits)

### 8.7 VSAM Catalogs

VSAM uses catalogs to manage dataset metadata:

**ICF Catalog (Integrated Catalog Facility)**:
- Contains metadata about all VSAM datasets and SMS-managed non-VSAM datasets
- **Master Catalog**: system-wide, contains high-level qualifiers
- **User Catalogs**: owned by users/applications, contain dataset entries

**Catalog entries for a KSDS**:
```
Cluster entry (logical name)
  |
  +-- Data component entry (physical data file)
  |     - CI size, CA size, record size, key position, key length
  |     - Space allocation, FREESPACE settings
  |     - Statistics (record count, CI/CA split counts)
  |
  +-- Index component entry (B+ tree index)
        - Index CI size, index levels
        - Key compression settings
```

**For the transpiler**: The catalog metadata maps to a configuration file
that defines the logical-to-physical mapping and file attributes.

### 8.8 Alternate Indexes (AIX)

An alternate index provides an additional access path to a base cluster
(KSDS or ESDS) via a different key field.

**Structure**:
- AIX is itself a VSAM KSDS
- Records contain: alternate key -> primary key(s) mapping
- A PATH connects the AIX to the base cluster

```
Base Cluster (KSDS):
  Primary Key: CUST-ID (9(8))
  Data: { CUST-ID, CUST-NAME, CUST-SSN, ... }

AIX 1: By CUST-NAME (with duplicates)
  Key: CUST-NAME -> [CUST-ID-1, CUST-ID-2, ...]

AIX 2: By CUST-SSN (unique)
  Key: CUST-SSN -> CUST-ID

PATH 1: Connects AIX 1 to Base Cluster
PATH 2: Connects AIX 2 to Base Cluster
```

**COBOL mapping**:
```cobol
ALTERNATE RECORD KEY IS CUST-NAME WITH DUPLICATES.
ALTERNATE RECORD KEY IS CUST-SSN.
```

**UPGRADE SET**: When the base cluster is updated (WRITE, REWRITE, DELETE),
all AIXes in the UPGRADE SET are automatically updated. AIXes not in the
UPGRADE SET must be rebuilt manually (IDCAMS BLDINDEX).

**Duplicate key handling**: For non-unique alternate keys, the AIX record
stores all primary keys that share the alternate key value. Sequential
READ through an AIX with duplicates returns records in primary key order
within each duplicate group.

### 8.9 IDCAMS Utility Equivalents

IDCAMS is the IBM utility for managing VSAM datasets. The transpiler needs
equivalents for these operations:

| IDCAMS Command | Purpose | Rust Equivalent |
|---|---|---|
| `DEFINE CLUSTER` | Create a VSAM dataset | Create file + metadata |
| `DEFINE AIX` | Create alternate index | Create secondary index |
| `DEFINE PATH` | Connect AIX to base | Register AIX with file handle |
| `DELETE` | Remove a VSAM dataset | Delete file + metadata |
| `REPRO` | Copy records between datasets | Bulk copy utility |
| `BLDINDEX` | Build/rebuild AIX | Rebuild secondary index |
| `LISTCAT` | Display catalog entries | Dump file metadata |
| `ALTER` | Modify dataset attributes | Modify metadata |
| `PRINT` | Display dataset contents | Dump/export utility |
| `VERIFY` | Fix improperly closed dataset | Integrity check |

```rust
// IDCAMS DEFINE CLUSTER equivalent
pub struct VsamClusterDef {
    pub name: String,
    pub organization: VsamOrg,     // KSDS, ESDS, RRDS
    pub record_size: RecordSize,   // average and max
    pub key_def: Option<KeyDef>,   // position, length (KSDS)
    pub ci_size: u32,              // control interval size
    pub freespace: (u8, u8),       // (ci_pct, ca_pct)
    pub shareoptions: (u8, u8),    // (cross-region, cross-system)
    pub alternate_indexes: Vec<AixDef>,
}

pub struct KeyDef {
    pub offset: u32,    // key position within record (0-based)
    pub length: u32,    // key length in bytes
}

pub struct AixDef {
    pub name: String,
    pub key_def: KeyDef,
    pub unique: bool,
    pub upgrade: bool,  // part of UPGRADE SET?
}
```

### 8.10 VSAM Performance Concepts

Understanding these is critical for generating efficient Rust code:

**Buffer management**:
- **NSR (Non-Shared Resources)**: Each file gets its own buffers. Default mode.
  Simple but uses more memory when multiple files are open.
- **LSR (Local Shared Resources)**: Buffer pool shared across files in the same
  address space. Better memory utilization for many small files.
- **GSR (Global Shared Resources)**: System-wide shared buffer pool.

**BUFND/BUFNI (Buffer tuning)**:
- BUFND: number of data component buffers
- BUFNI: number of index component buffers
- More buffers = fewer physical I/Os but more memory usage
- Rule of thumb: BUFNI = index levels + 1, BUFND = at least 2

**Key compression (KSDS index)**:
- Front-end compression: removes common prefix bytes between adjacent keys
- Rear-end compression: removes trailing characters
- Reduces index size -> fewer index I/Os -> better performance

**CI/CA split impact**:
- CI splits: moderate overhead, creates adjacent free space
- CA splits: expensive, may cause dataset fragmentation
- Proper FREESPACE settings minimize splits during batch inserts
- REORGANIZE (REPRO out + REPRO in) rebuilds optimal layout

**Rust performance mapping**:
- NSR mode -> dedicated BufReader/BufWriter per file
- LSR mode -> shared buffer pool (useful for indexed file backends)
- Key compression -> handled by SQLite B-tree internally
- Split avoidance -> SQLite page splits are analogous, auto-managed

---

## 9. Record Layout and Buffer Management

### 9.1 FD Record Area Semantics

The FD record area is a single fixed-size buffer in memory. Key behaviors:

1. **One record at a time**: Only the most recently READ record is available.
   Reading a new record overwrites the previous one.

2. **Shared buffer**: If multiple 01-levels are defined under one FD, they
   all overlay the same buffer (implicit REDEFINES).

3. **Undefined after OPEN/CLOSE**: The record area contents are undefined
   after OPEN (before first READ) and after CLOSE.

4. **Undefined after unsuccessful READ**: If READ returns AT END or
   INVALID KEY, the record area contents are undefined.

5. **WRITE uses current buffer**: WRITE sends whatever is currently in the
   record area. The program must populate the buffer before WRITE.

### 9.2 READ INTO / WRITE FROM

These are convenience forms that combine I/O with MOVE:

```cobol
READ file-name INTO ws-record.
*> Equivalent to:
*>   READ file-name.
*>   MOVE fd-record TO ws-record.   (GROUP move semantics)

WRITE fd-record FROM ws-record.
*> Equivalent to:
*>   MOVE ws-record TO fd-record.   (GROUP move semantics)
*>   WRITE fd-record.
```

**Why INTO/FROM is common practice**: Since the FD record area is volatile
(overwritten on every READ), programs typically READ INTO a working-storage
copy for processing. This also allows processing the previous record while
reading the next one.

### 9.3 Multiple Record Descriptions

When an FD has multiple 01-levels:

```cobol
FD  TRANSACTION-FILE.
01  TRANS-HEADER.
    05  TH-TYPE    PIC X VALUE "H".
    05  TH-DATE    PIC 9(8).
    05  TH-FILLER  PIC X(71).
01  TRANS-DETAIL.
    05  TD-TYPE    PIC X.
    05  TD-ACCT    PIC 9(10).
    05  TD-AMOUNT  PIC S9(7)V99 COMP-3.
    05  TD-FILLER  PIC X(63).
```

In Rust, this maps to a raw byte buffer with typed overlay views:

```rust
pub struct TransactionFileRecord {
    raw: [u8; 80],  // fixed-size buffer
}

impl TransactionFileRecord {
    pub fn as_header(&self) -> &TransHeader {
        // Safe: same size, same alignment
        unsafe { &*(self.raw.as_ptr() as *const TransHeader) }
    }

    pub fn as_detail(&self) -> &TransDetail {
        unsafe { &*(self.raw.as_ptr() as *const TransDetail) }
    }

    pub fn record_type(&self) -> u8 {
        self.raw[0]  // first byte determines record type
    }
}
```

### 9.4 Record Buffer Lifetime

```
OPEN INPUT file-name
    record area: undefined
    |
READ file-name
    record area: contains record 1
    |
READ file-name
    record area: contains record 2 (record 1 is lost!)
    |
READ file-name INTO ws-copy
    record area: contains record 3
    ws-copy: contains copy of record 3 (safe from next READ)
    |
CLOSE file-name
    record area: undefined
```

---

## 10. Variable-Length Records

### 10.1 IBM Recording Mode V

Variable-length records are prefixed with a 4-byte **Record Descriptor Word
(RDW)** that contains the record length:

```
RDW structure (4 bytes):
+------+------+------+------+
| LL   | LL   | BB   | BB   |
+------+------+------+------+
  MSB    LSB    (reserved, usually 0x0000)

LL = record length INCLUDING the 4-byte RDW itself
     Minimum: 4 (empty record = just RDW)
     Maximum: 32760
```

Example: A record with 76 bytes of data has RDW = `0x0050 0x0000`
(0x0050 = 80 decimal = 76 data bytes + 4 RDW bytes).

### 10.2 Block Descriptor Word (BDW)

For blocked variable-length records, each physical block is prefixed with a
4-byte **Block Descriptor Word (BDW)**:

```
Block structure:
+------+------+------+------+------+------+------+------+------+
| BDW  |   RDW 1 + Data 1   |   RDW 2 + Data 2   |  ...  |
+------+------+------+------+------+------+------+------+------+

BDW:
+------+------+------+------+
| LL   | LL   | 00   | 00   |
+------+------+------+------+
LL = block length INCLUDING the 4-byte BDW
```

### 10.3 Spanned Records (Recording Mode S)

When a record is longer than the CI or block size, it is split into segments:

```
Segment structure:
+------+------+------+------+------+------+
| SDW  |           Segment Data            |
+------+------+------+------+------+------+

SDW (Segment Descriptor Word):
+------+------+------+------+
| LL   | LL   | SS   | 00   |
+------+------+------+------+
LL = segment length including SDW
SS = segment control code:
     00 = complete record (no spanning)
     01 = first segment
     10 = last segment
     11 = middle segment
```

### 10.4 COBOL Variable-Length Record Handling

```cobol
FD  VAR-FILE
    RECORDING MODE IS V
    RECORD CONTAINS 50 TO 200 CHARACTERS.
01  VAR-RECORD.
    05  VR-LENGTH   PIC S9(4) COMP.  *> not the RDW, but data field
    05  VR-DATA     PIC X(196).

*> Or with OCCURS DEPENDING ON:
FD  VAR-FILE
    RECORD IS VARYING IN SIZE FROM 10 TO 500
        DEPENDING ON WS-REC-LEN.
01  VAR-RECORD.
    05  VR-HEADER  PIC X(10).
    05  VR-ITEMS   PIC X(10) OCCURS 1 TO 49 DEPENDING ON WS-ITEM-COUNT.
```

**COBOL programs do NOT see the RDW.** The RDW is added by the I/O subsystem
on WRITE and stripped on READ. The COBOL program deals only with the data
portion. The RECORD IS VARYING DEPENDING ON field tracks the logical record
length.

### 10.5 Rust Implementation for Variable-Length Records

```rust
pub struct VariableLengthRecord {
    /// Maximum record size
    max_size: usize,
    /// Current record data (up to max_size)
    data: Vec<u8>,
}

impl VariableLengthRecord {
    pub fn current_length(&self) -> usize {
        self.data.len()
    }

    pub fn set_length(&mut self, len: usize) {
        assert!(len <= self.max_size);
        self.data.resize(len, 0);
    }
}

/// Reading a variable-length record from a Mode V file
fn read_variable_record<R: Read>(
    reader: &mut R,
    max_record_size: usize,
) -> io::Result<Vec<u8>> {
    // Read 4-byte RDW
    let mut rdw = [0u8; 4];
    reader.read_exact(&mut rdw)?;

    // Extract record length (big-endian u16, includes RDW)
    let total_length = u16::from_be_bytes([rdw[0], rdw[1]]) as usize;
    let data_length = total_length - 4; // subtract RDW size

    if data_length > max_record_size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Record length {} exceeds max {}", data_length, max_record_size),
        ));
    }

    // Read the data portion
    let mut data = vec![0u8; data_length];
    reader.read_exact(&mut data)?;
    Ok(data)
}

/// Writing a variable-length record to a Mode V file
fn write_variable_record<W: Write>(
    writer: &mut W,
    data: &[u8],
) -> io::Result<()> {
    let total_length = (data.len() + 4) as u16;
    let rdw = [
        (total_length >> 8) as u8,  // MSB
        (total_length & 0xFF) as u8, // LSB
        0x00,                        // reserved
        0x00,                        // reserved
    ];
    writer.write_all(&rdw)?;
    writer.write_all(data)?;
    Ok(())
}
```

---

## 11. LINAGE and Print File Control

### 11.1 LINAGE Clause

The LINAGE clause defines a logical page structure for print files:

```cobol
FD  PRINT-FILE
    LINAGE IS 60 LINES
        WITH FOOTING AT 55
        LINES AT TOP 3
        LINES AT BOTTOM 3.
01  PRINT-LINE PIC X(132).
```

```
Physical page layout:
+----------------------------------+
|  LINES AT TOP (3 blank lines)    |
+----------------------------------+  <- LINAGE-COUNTER = 1
|                                  |
|  Page Body (60 lines)            |
|                                  |
+----------------------------------+  <- FOOTING AT 55 (5 lines for footer)
|  Footer area (lines 55-60)       |
+----------------------------------+
|  LINES AT BOTTOM (3 blank lines) |
+----------------------------------+
Total physical lines: 3 + 60 + 3 = 66
```

### 11.2 LINAGE-COUNTER

A special register automatically maintained by the runtime:

- Set to 1 when the file is OPENed
- Incremented by WRITE ADVANCING
- Reset to 1 when a new page begins (WRITE AFTER ADVANCING PAGE)
- Available for inspection: `IF LINAGE-COUNTER > 50`

### 11.3 WRITE ADVANCING

Controls vertical positioning on print files:

```cobol
WRITE PRINT-LINE AFTER ADVANCING 2 LINES.
*> Advance 2 lines, then write

WRITE PRINT-LINE BEFORE ADVANCING 1 LINE.
*> Write, then advance 1 line

WRITE PRINT-LINE AFTER ADVANCING PAGE.
*> Advance to top of new page, then write

WRITE PRINT-LINE AFTER ADVANCING 0 LINES.
*> Overprint: write on current line (no advance)
```

**IBM channel skips**:

```cobol
WRITE PRINT-LINE AFTER ADVANCING C01.
*> Skip to channel 1 (top of form) -- IBM mainframe carriage control
```

Channels 1-12 are defined by the FCB (Forms Control Buffer) on IBM printers.
Channel 1 = top of form, Channel 12 = typically bottom of form.

### 11.4 END-OF-PAGE Condition

```cobol
WRITE PRINT-LINE FROM WS-DETAIL
    AFTER ADVANCING 1 LINE
    AT END-OF-PAGE
        PERFORM PRINT-FOOTER
        WRITE PRINT-LINE FROM WS-HEADER AFTER ADVANCING PAGE
    END-WRITE.
```

END-OF-PAGE is triggered when LINAGE-COUNTER reaches or passes the FOOTING
value. It does NOT automatically advance to a new page -- the program must
explicitly WRITE AFTER ADVANCING PAGE.

### 11.5 Rust Print File Implementation

```rust
pub struct PrintFile {
    writer: BufWriter<File>,
    linage: LinageSpec,
    linage_counter: u32,
    current_line: u32,  // physical line on page
}

pub struct LinageSpec {
    pub body_lines: u32,      // LINAGE IS n LINES
    pub footing_at: u32,      // WITH FOOTING AT n (0 = no footing)
    pub top_lines: u32,       // LINES AT TOP n
    pub bottom_lines: u32,    // LINES AT BOTTOM n
}

impl LinageSpec {
    pub fn total_page_lines(&self) -> u32 {
        self.top_lines + self.body_lines + self.bottom_lines
    }
}

impl PrintFile {
    pub fn write_advancing(
        &mut self,
        record: &[u8],
        advance: Advance,
    ) -> Result<bool, CobolFileError> {
        let mut end_of_page = false;

        match advance {
            Advance::AfterLines(n) => {
                // Advance n lines, then write
                self.emit_newlines(n)?;
                self.linage_counter += n;
                self.write_line(record)?;
            }
            Advance::BeforeLines(n) => {
                // Write, then advance n lines
                self.write_line(record)?;
                self.emit_newlines(n)?;
                self.linage_counter += n;
            }
            Advance::AfterPage => {
                // Advance to top of new page, then write
                self.eject_page()?;
                self.linage_counter = 1;
                self.write_line(record)?;
            }
            Advance::BeforePage => {
                self.write_line(record)?;
                self.eject_page()?;
                self.linage_counter = 1;
            }
        }

        // Check end-of-page condition
        if self.linage.footing_at > 0
            && self.linage_counter >= self.linage.footing_at
        {
            end_of_page = true;
        }

        Ok(end_of_page)
    }

    fn eject_page(&mut self) -> io::Result<()> {
        // Emit remaining body lines
        let remaining = self.linage.body_lines
            .saturating_sub(self.linage_counter);
        self.emit_newlines(remaining)?;
        // Emit bottom margin
        self.emit_newlines(self.linage.bottom_lines)?;
        // Emit top margin of new page
        self.emit_newlines(self.linage.top_lines)?;
        Ok(())
    }
}
```

---

## 12. CODE-SET and Encoding

### 12.1 CODE-SET Clause

```cobol
FD  DATA-FILE
    CODE-SET IS EBCDIC-ALPHABET.

SPECIAL-NAMES.
    ALPHABET EBCDIC-ALPHABET IS EBCDIC.
```

The CODE-SET clause specifies the character encoding of the physical file.
This triggers automatic encoding conversion on READ and WRITE.

### 12.2 Encoding Conversion Strategy

Per the architectural decision in `cobol_to_rust_datatype_mapping.md`:
**ASCII internally, EBCDIC conversion at I/O boundaries only.**

```
READ:   File (EBCDIC) -> [conversion] -> Record area (ASCII internal)
WRITE:  Record area (ASCII internal) -> [conversion] -> File (EBCDIC)
```

### 12.3 Conversion Scope

Encoding conversion applies to **alphanumeric data only**:
- PIC X, PIC A fields: converted character by character
- PIC 9 DISPLAY (zoned decimal): zone nibbles differ between EBCDIC and ASCII
- COMP, COMP-3: binary data, NOT converted (pure numeric bytes)
- GROUP items: converted byte by byte (same as alphanumeric)

### 12.4 EBCDIC Code Pages

| Code Page | Description | Usage |
|---|---|---|
| CP037 | US/Canada EBCDIC | Most common for US mainframes |
| CP1140 | US/Canada EBCDIC with Euro | Modern IBM mainframes |
| CP500 | International Latin-1 EBCDIC | International systems |
| CP1047 | Open Systems Latin-1 EBCDIC | Unix System Services on z/OS |
| CP290 | Japanese Katakana EBCDIC | Japanese systems |

### 12.5 Conversion Impact on Collating Sequence

EBCDIC and ASCII have different collating sequences:

| Character Group | ASCII Order | EBCDIC Order |
|---|---|---|
| Digits (0-9) | Before uppercase | After uppercase and lowercase |
| Uppercase (A-Z) | Before lowercase | Between lowercase and digits |
| Lowercase (a-z) | After uppercase | Before uppercase |
| Special chars | Various positions | Various positions |

**Implication**: SORT operations and comparisons that work correctly in
EBCDIC may produce different results in ASCII. The runtime must provide
an EBCDIC collating sequence option for comparisons when processing
EBCDIC-origin data.

### 12.6 Rust Encoding Module

```rust
pub struct EbcdicConverter {
    to_ascii: [u8; 256],   // EBCDIC byte -> ASCII byte
    to_ebcdic: [u8; 256],  // ASCII byte -> EBCDIC byte
}

impl EbcdicConverter {
    /// Create converter for a specific code page
    pub fn new(code_page: CodePage) -> Self {
        match code_page {
            CodePage::Cp037 => Self::cp037(),
            CodePage::Cp1140 => Self::cp1140(),
            CodePage::Cp500 => Self::cp500(),
            CodePage::Cp1047 => Self::cp1047(),
        }
    }

    /// Convert a byte buffer from EBCDIC to ASCII (in-place)
    pub fn to_ascii_inplace(&self, buf: &mut [u8]) {
        for b in buf.iter_mut() {
            *b = self.to_ascii[*b as usize];
        }
    }

    /// Convert a byte buffer from ASCII to EBCDIC (in-place)
    pub fn to_ebcdic_inplace(&self, buf: &mut [u8]) {
        for b in buf.iter_mut() {
            *b = self.to_ebcdic[*b as usize];
        }
    }

    /// Convert zoned decimal from EBCDIC to ASCII
    /// Zone nibble: EBCDIC F -> ASCII 3, sign C/D -> context-dependent
    pub fn convert_zoned_decimal(
        &self,
        buf: &mut [u8],
        is_signed: bool,
    ) {
        // Zoned decimal in EBCDIC: zone=0xF, digit=0-9
        // Zoned decimal in ASCII:  zone=0x3, digit=0-9
        // Last byte sign: C=positive, D=negative, F=unsigned
        let len = buf.len();
        for b in buf[..len - 1].iter_mut() {
            // Convert zone nibble: 0xFn -> 0x3n
            let digit = *b & 0x0F;
            *b = 0x30 | digit;
        }
        if is_signed {
            // Last byte retains sign in zone nibble
            let digit = buf[len - 1] & 0x0F;
            let sign = buf[len - 1] & 0xF0;
            buf[len - 1] = match sign {
                0xC0 => 0x30 | digit, // positive (or use encoding convention)
                0xD0 => 0x70 | digit, // negative (convention: 0x7n for negative)
                0xF0 => 0x30 | digit, // unsigned
                _ => 0x30 | digit,    // default to unsigned
            };
        } else {
            let digit = buf[len - 1] & 0x0F;
            buf[len - 1] = 0x30 | digit;
        }
    }
}
```

---

## 13. Rust Implementation Architecture

### 13.1 Trait Hierarchy

```
CobolFile (root trait)
  |
  +-- SeqFile (sequential files)
  |     +-- LineSeqFile (line sequential)
  |     +-- QsamFile (IBM QSAM / standard sequential)
  |
  +-- IndexedFile (indexed / VSAM KSDS)
  |
  +-- RelativeFile (relative / VSAM RRDS)
  |
  +-- PrintFile (LINAGE-controlled print files)
```

### 13.2 Core Trait

```rust
/// Root trait for all COBOL file types
pub trait CobolFile {
    /// Open the file in the specified mode
    fn open(&mut self, mode: OpenMode) -> FileStatus;

    /// Close the file
    fn close(&mut self) -> FileStatus;

    /// Get the current file status
    fn file_status(&self) -> FileStatus;

    /// Get the file organization
    fn organization(&self) -> FileOrganization;

    /// Get the record area as a byte slice
    fn record_area(&self) -> &[u8];

    /// Get the record area as a mutable byte slice
    fn record_area_mut(&mut self) -> &mut [u8];

    /// Current record length (for variable-length records)
    fn current_record_length(&self) -> usize;
}

/// Sequential file operations
pub trait SeqFileOps: CobolFile {
    /// Read the next record (sequential)
    fn read_next(&mut self) -> FileStatus;

    /// Write a record
    fn write_record(&mut self) -> FileStatus;

    /// Rewrite the most recently read record (I-O mode only)
    fn rewrite_record(&mut self) -> FileStatus;
}

/// Indexed file operations
pub trait IndexedFileOps: CobolFile {
    /// Read the next record (sequential by current key)
    fn read_next(&mut self) -> FileStatus;

    /// Read the previous record (IBM extension)
    fn read_previous(&mut self) -> FileStatus;

    /// Read by key value (random)
    fn read_key(&mut self, key_name: &str) -> FileStatus;

    /// Write a new record
    fn write_record(&mut self) -> FileStatus;

    /// Rewrite (update) a record
    fn rewrite_record(&mut self) -> FileStatus;

    /// Delete a record
    fn delete_record(&mut self) -> FileStatus;

    /// Position for sequential reading
    fn start(&mut self, key_name: &str, op: StartOp) -> FileStatus;
}

/// Relative file operations
pub trait RelativeFileOps: CobolFile {
    fn read_next(&mut self) -> FileStatus;
    fn read_slot(&mut self, slot: u64) -> FileStatus;
    fn write_record(&mut self) -> FileStatus;
    fn write_slot(&mut self, slot: u64) -> FileStatus;
    fn rewrite_record(&mut self) -> FileStatus;
    fn delete_record(&mut self) -> FileStatus;
    fn delete_slot(&mut self, slot: u64) -> FileStatus;
    fn start(&mut self, slot: u64, op: StartOp) -> FileStatus;
}

/// Print file operations
pub trait PrintFileOps: CobolFile {
    fn write_advancing(
        &mut self,
        advance: Advance,
    ) -> (FileStatus, bool); // (status, end_of_page)

    fn linage_counter(&self) -> u32;
}
```

### 13.3 Supporting Types

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenMode {
    Input,
    Output,
    Io,
    Extend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOrganization {
    Sequential,
    LineSequential,
    Indexed,
    Relative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Sequential,
    Random,
    Dynamic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingMode {
    Fixed,
    Variable,
    Undefined,
    Spanned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartOp {
    Equal,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Advance {
    BeforeLines(u32),
    AfterLines(u32),
    BeforePage,
    AfterPage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStatus {
    pub code: [u8; 2],  // e.g., b"00", b"10", b"23"
}

impl FileStatus {
    pub const OK: Self = Self { code: [b'0', b'0'] };
    pub const EOF: Self = Self { code: [b'1', b'0'] };
    pub const DUPLICATE_KEY: Self = Self { code: [b'0', b'2'] };
    pub const SEQUENCE_ERROR: Self = Self { code: [b'2', b'1'] };
    pub const DUPLICATE_PRIMARY: Self = Self { code: [b'2', b'2'] };
    pub const NOT_FOUND: Self = Self { code: [b'2', b'3'] };
    pub const BOUNDARY: Self = Self { code: [b'2', b'4'] };
    pub const PERMANENT_ERROR: Self = Self { code: [b'3', b'0'] };
    pub const FILE_NOT_FOUND: Self = Self { code: [b'3', b'5'] };
    pub const ALREADY_OPEN: Self = Self { code: [b'4', b'1'] };
    pub const NOT_OPEN: Self = Self { code: [b'4', b'2'] };
    pub const NO_PRIOR_READ: Self = Self { code: [b'4', b'3'] };

    pub fn is_success(&self) -> bool {
        self.code[0] == b'0'
    }

    pub fn is_end_of_file(&self) -> bool {
        self.code == [b'1', b'0']
    }

    /// Set the corresponding working-storage FILE STATUS field
    pub fn store_to(&self, ws_field: &mut [u8; 2]) {
        ws_field[0] = self.code[0];
        ws_field[1] = self.code[1];
    }
}
```

### 13.4 Sequential File Implementation

```rust
pub struct SequentialFile {
    // Configuration
    config: SeqFileConfig,
    // State
    state: FileState,
    // I/O
    reader: Option<BufReader<File>>,
    writer: Option<BufWriter<File>>,
    // Record area
    record_buf: Vec<u8>,
    current_record_len: usize,
    // Status
    status: FileStatus,
    // Encoding
    converter: Option<EbcdicConverter>,
    // Error callback (DECLARATIVES)
    on_error: Option<Box<dyn Fn(FileStatus) -> ErrorAction>>,
}

pub struct SeqFileConfig {
    pub file_path: PathBuf,        // resolved from ASSIGN TO
    pub record_size: RecordSize,    // Fixed(n) or Variable(min, max)
    pub recording_mode: RecordingMode,
    pub block_size: usize,         // for BufReader/BufWriter capacity
    pub optional: bool,            // OPTIONAL keyword
    pub code_set: Option<CodePage>,
}

#[derive(Debug)]
pub enum RecordSize {
    Fixed(usize),
    Variable { min: usize, max: usize },
}

impl SeqFileOps for SequentialFile {
    fn read_next(&mut self) -> FileStatus {
        let reader = match &mut self.reader {
            Some(r) => r,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        match self.config.recording_mode {
            RecordingMode::Fixed => {
                let size = match &self.config.record_size {
                    RecordSize::Fixed(n) => *n,
                    _ => unreachable!(),
                };
                match reader.read_exact(&mut self.record_buf[..size]) {
                    Ok(()) => {
                        self.current_record_len = size;
                        if let Some(conv) = &self.converter {
                            conv.to_ascii_inplace(
                                &mut self.record_buf[..size]
                            );
                        }
                        self.status = FileStatus::OK;
                    }
                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                        self.status = FileStatus::EOF;
                    }
                    Err(_) => {
                        self.status = FileStatus::PERMANENT_ERROR;
                    }
                }
            }
            RecordingMode::Variable => {
                match read_variable_record(reader, self.record_buf.len()) {
                    Ok(data) => {
                        self.record_buf[..data.len()].copy_from_slice(&data);
                        self.current_record_len = data.len();
                        if let Some(conv) = &self.converter {
                            conv.to_ascii_inplace(
                                &mut self.record_buf[..data.len()]
                            );
                        }
                        self.status = FileStatus::OK;
                    }
                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                        self.status = FileStatus::EOF;
                    }
                    Err(_) => {
                        self.status = FileStatus::PERMANENT_ERROR;
                    }
                }
            }
            _ => {
                self.status = FileStatus::PERMANENT_ERROR;
            }
        }

        // Invoke error handler if applicable
        if !self.status.is_success() && !self.status.is_end_of_file() {
            if let Some(handler) = &self.on_error {
                handler(self.status);
            }
        }

        self.status
    }

    fn write_record(&mut self) -> FileStatus {
        let writer = match &mut self.writer {
            Some(w) => w,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        let len = self.current_record_len;
        let mut write_buf = self.record_buf[..len].to_vec();

        // Convert encoding if needed
        if let Some(conv) = &self.converter {
            conv.to_ebcdic_inplace(&mut write_buf);
        }

        match self.config.recording_mode {
            RecordingMode::Fixed => {
                match writer.write_all(&write_buf) {
                    Ok(()) => self.status = FileStatus::OK,
                    Err(_) => self.status = FileStatus::PERMANENT_ERROR,
                }
            }
            RecordingMode::Variable => {
                match write_variable_record(writer, &write_buf) {
                    Ok(()) => self.status = FileStatus::OK,
                    Err(_) => self.status = FileStatus::PERMANENT_ERROR,
                }
            }
            _ => {
                self.status = FileStatus::PERMANENT_ERROR;
            }
        }

        self.status
    }

    fn rewrite_record(&mut self) -> FileStatus {
        // Sequential REWRITE: replace the last-read record
        // Only valid for fixed-length records in I-O mode
        // Requires seeking back by one record length
        // Implementation depends on recording mode
        todo!("Sequential REWRITE implementation")
    }
}
```

### 13.5 Indexed File Backend (SQLite)

**Recommendation**: Use SQLite as the primary backend for indexed files.
SQLite provides B-tree indexing, ACID transactions, and concurrent access --
all semantics needed to emulate VSAM KSDS.

```rust
pub struct IndexedFile {
    config: IndexedFileConfig,
    state: FileState,
    db: Option<rusqlite::Connection>,
    record_buf: Vec<u8>,
    current_record_len: usize,
    status: FileStatus,
    // Sequential read cursor
    cursor_position: Option<Vec<u8>>, // last key read
    cursor_direction: CursorDirection,
    converter: Option<EbcdicConverter>,
    on_error: Option<Box<dyn Fn(FileStatus) -> ErrorAction>>,
}

pub struct IndexedFileConfig {
    pub db_path: PathBuf,          // SQLite database file
    pub record_size: RecordSize,
    pub primary_key: KeySpec,
    pub alternate_keys: Vec<AltKeySpec>,
    pub optional: bool,
    pub code_set: Option<CodePage>,
}

pub struct KeySpec {
    pub name: String,       // COBOL field name
    pub offset: usize,      // byte offset within record
    pub length: usize,      // key length in bytes
}

pub struct AltKeySpec {
    pub name: String,
    pub offset: usize,
    pub length: usize,
    pub allow_duplicates: bool,
}

impl IndexedFile {
    /// Create the SQLite schema for this indexed file
    fn create_schema(&self, db: &rusqlite::Connection) -> rusqlite::Result<()> {
        // Main data table
        db.execute(
            "CREATE TABLE IF NOT EXISTS records (
                primary_key BLOB NOT NULL PRIMARY KEY,
                record_data BLOB NOT NULL
            )",
            [],
        )?;

        // Alternate key indexes
        for (i, ak) in self.config.alternate_keys.iter().enumerate() {
            let unique = if ak.allow_duplicates { "" } else { "UNIQUE" };
            db.execute(
                &format!(
                    "CREATE TABLE IF NOT EXISTS alt_key_{i} (
                        alt_key BLOB NOT NULL,
                        primary_key BLOB NOT NULL,
                        {unique} CONSTRAINT uq_{i} UNIQUE (alt_key, primary_key)
                    )"
                ),
                [],
            )?;
            db.execute(
                &format!(
                    "CREATE INDEX IF NOT EXISTS idx_alt_{i}
                     ON alt_key_{i} (alt_key)"
                ),
                [],
            )?;
        }

        Ok(())
    }

    /// Extract the primary key from the record buffer
    fn extract_primary_key(&self) -> Vec<u8> {
        let k = &self.config.primary_key;
        self.record_buf[k.offset..k.offset + k.length].to_vec()
    }

    /// Extract an alternate key from the record buffer
    fn extract_alt_key(&self, idx: usize) -> Vec<u8> {
        let k = &self.config.alternate_keys[idx];
        self.record_buf[k.offset..k.offset + k.length].to_vec()
    }
}

impl IndexedFileOps for IndexedFile {
    fn read_key(&mut self, key_name: &str) -> FileStatus {
        let db = match &self.db {
            Some(db) => db,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        if key_name == self.config.primary_key.name {
            // Read by primary key
            let key = self.extract_primary_key();
            match db.query_row(
                "SELECT record_data FROM records WHERE primary_key = ?1",
                [&key],
                |row| row.get::<_, Vec<u8>>(0),
            ) {
                Ok(data) => {
                    self.record_buf[..data.len()].copy_from_slice(&data);
                    self.current_record_len = data.len();
                    self.cursor_position = Some(key);
                    self.status = FileStatus::OK;
                }
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    self.status = FileStatus::NOT_FOUND;
                }
                Err(_) => {
                    self.status = FileStatus::PERMANENT_ERROR;
                }
            }
        } else {
            // Read by alternate key
            let ak_idx = self.config.alternate_keys
                .iter()
                .position(|ak| ak.name == key_name)
                .expect("Unknown key name");
            let alt_key = self.extract_alt_key(ak_idx);
            match db.query_row(
                &format!(
                    "SELECT r.record_data FROM records r
                     JOIN alt_key_{ak_idx} a ON r.primary_key = a.primary_key
                     WHERE a.alt_key = ?1
                     ORDER BY a.primary_key LIMIT 1"
                ),
                [&alt_key],
                |row| row.get::<_, Vec<u8>>(0),
            ) {
                Ok(data) => {
                    self.record_buf[..data.len()].copy_from_slice(&data);
                    self.current_record_len = data.len();
                    self.status = FileStatus::OK;
                }
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    self.status = FileStatus::NOT_FOUND;
                }
                Err(_) => {
                    self.status = FileStatus::PERMANENT_ERROR;
                }
            }
        }

        self.status
    }

    fn write_record(&mut self) -> FileStatus {
        let db = match &self.db {
            Some(db) => db,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        let pk = self.extract_primary_key();
        let data = self.record_buf[..self.current_record_len].to_vec();

        // Begin transaction for atomic insert across all tables
        let tx = match db.unchecked_transaction() {
            Ok(tx) => tx,
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
                return self.status;
            }
        };

        // Insert into main table
        match tx.execute(
            "INSERT INTO records (primary_key, record_data) VALUES (?1, ?2)",
            rusqlite::params![pk, data],
        ) {
            Ok(_) => {}
            Err(rusqlite::Error::SqliteFailure(err, _))
                if err.code == rusqlite::ErrorCode::ConstraintViolation =>
            {
                self.status = FileStatus::DUPLICATE_PRIMARY;
                return self.status;
            }
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
                return self.status;
            }
        }

        // Insert alternate key entries
        for (i, ak) in self.config.alternate_keys.iter().enumerate() {
            let alt_key = self.extract_alt_key(i);
            match tx.execute(
                &format!(
                    "INSERT INTO alt_key_{i} (alt_key, primary_key)
                     VALUES (?1, ?2)"
                ),
                rusqlite::params![alt_key, pk],
            ) {
                Ok(_) => {}
                Err(rusqlite::Error::SqliteFailure(err, _))
                    if err.code == rusqlite::ErrorCode::ConstraintViolation
                        && !ak.allow_duplicates =>
                {
                    // Unique alternate key violation
                    self.status = FileStatus::DUPLICATE_PRIMARY; // STATUS 22
                    return self.status;
                }
                Err(_) => {
                    self.status = FileStatus::PERMANENT_ERROR;
                    return self.status;
                }
            }
        }

        match tx.commit() {
            Ok(_) => self.status = FileStatus::OK,
            Err(_) => self.status = FileStatus::PERMANENT_ERROR,
        }

        self.status
    }

    fn read_next(&mut self) -> FileStatus {
        // Sequential read: get next record after cursor position
        let db = match &self.db {
            Some(db) => db,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        let query = match &self.cursor_position {
            Some(last_key) => {
                db.query_row(
                    "SELECT primary_key, record_data FROM records
                     WHERE primary_key > ?1
                     ORDER BY primary_key LIMIT 1",
                    [last_key],
                    |row| Ok((
                        row.get::<_, Vec<u8>>(0)?,
                        row.get::<_, Vec<u8>>(1)?,
                    )),
                )
            }
            None => {
                // First read: get the first record
                db.query_row(
                    "SELECT primary_key, record_data FROM records
                     ORDER BY primary_key LIMIT 1",
                    [],
                    |row| Ok((
                        row.get::<_, Vec<u8>>(0)?,
                        row.get::<_, Vec<u8>>(1)?,
                    )),
                )
            }
        };

        match query {
            Ok((pk, data)) => {
                self.record_buf[..data.len()].copy_from_slice(&data);
                self.current_record_len = data.len();
                self.cursor_position = Some(pk);
                self.status = FileStatus::OK;
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                self.status = FileStatus::EOF;
            }
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
            }
        }

        self.status
    }

    fn read_previous(&mut self) -> FileStatus {
        // IBM extension: reverse sequential read
        let db = match &self.db {
            Some(db) => db,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        let query = match &self.cursor_position {
            Some(last_key) => {
                db.query_row(
                    "SELECT primary_key, record_data FROM records
                     WHERE primary_key < ?1
                     ORDER BY primary_key DESC LIMIT 1",
                    [last_key],
                    |row| Ok((
                        row.get::<_, Vec<u8>>(0)?,
                        row.get::<_, Vec<u8>>(1)?,
                    )),
                )
            }
            None => {
                db.query_row(
                    "SELECT primary_key, record_data FROM records
                     ORDER BY primary_key DESC LIMIT 1",
                    [],
                    |row| Ok((
                        row.get::<_, Vec<u8>>(0)?,
                        row.get::<_, Vec<u8>>(1)?,
                    )),
                )
            }
        };

        match query {
            Ok((pk, data)) => {
                self.record_buf[..data.len()].copy_from_slice(&data);
                self.current_record_len = data.len();
                self.cursor_position = Some(pk);
                self.status = FileStatus::OK;
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                self.status = FileStatus::EOF;
            }
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
            }
        }

        self.status
    }

    fn rewrite_record(&mut self) -> FileStatus {
        let db = match &self.db {
            Some(db) => db,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        let pk = self.extract_primary_key();
        let data = self.record_buf[..self.current_record_len].to_vec();

        let tx = match db.unchecked_transaction() {
            Ok(tx) => tx,
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
                return self.status;
            }
        };

        // Delete old alternate key entries
        for i in 0..self.config.alternate_keys.len() {
            let _ = tx.execute(
                &format!("DELETE FROM alt_key_{i} WHERE primary_key = ?1"),
                [&pk],
            );
        }

        // Update main record
        match tx.execute(
            "UPDATE records SET record_data = ?1 WHERE primary_key = ?2",
            rusqlite::params![data, pk],
        ) {
            Ok(0) => {
                self.status = FileStatus::NOT_FOUND;
                return self.status;
            }
            Ok(_) => {}
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
                return self.status;
            }
        }

        // Re-insert alternate key entries with new values
        for (i, ak) in self.config.alternate_keys.iter().enumerate() {
            let alt_key = self.extract_alt_key(i);
            match tx.execute(
                &format!(
                    "INSERT INTO alt_key_{i} (alt_key, primary_key)
                     VALUES (?1, ?2)"
                ),
                rusqlite::params![alt_key, pk],
            ) {
                Ok(_) => {}
                Err(rusqlite::Error::SqliteFailure(err, _))
                    if err.code == rusqlite::ErrorCode::ConstraintViolation
                        && !ak.allow_duplicates =>
                {
                    self.status = FileStatus::DUPLICATE_PRIMARY;
                    return self.status;
                }
                Err(_) => {
                    self.status = FileStatus::PERMANENT_ERROR;
                    return self.status;
                }
            }
        }

        match tx.commit() {
            Ok(_) => self.status = FileStatus::OK,
            Err(_) => self.status = FileStatus::PERMANENT_ERROR,
        }

        self.status
    }

    fn delete_record(&mut self) -> FileStatus {
        let db = match &self.db {
            Some(db) => db,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        let pk = self.extract_primary_key();

        let tx = match db.unchecked_transaction() {
            Ok(tx) => tx,
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
                return self.status;
            }
        };

        // Delete alternate key entries
        for i in 0..self.config.alternate_keys.len() {
            let _ = tx.execute(
                &format!("DELETE FROM alt_key_{i} WHERE primary_key = ?1"),
                [&pk],
            );
        }

        // Delete main record
        match tx.execute(
            "DELETE FROM records WHERE primary_key = ?1",
            [&pk],
        ) {
            Ok(0) => {
                self.status = FileStatus::NOT_FOUND;
                return self.status;
            }
            Ok(_) => {}
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
                return self.status;
            }
        }

        match tx.commit() {
            Ok(_) => self.status = FileStatus::OK,
            Err(_) => self.status = FileStatus::PERMANENT_ERROR,
        }

        self.status
    }

    fn start(&mut self, key_name: &str, op: StartOp) -> FileStatus {
        let db = match &self.db {
            Some(db) => db,
            None => {
                self.status = FileStatus::NOT_OPEN;
                return self.status;
            }
        };

        let key_bytes = if key_name == self.config.primary_key.name {
            self.extract_primary_key()
        } else {
            let idx = self.config.alternate_keys
                .iter()
                .position(|ak| ak.name == key_name)
                .expect("Unknown key name");
            self.extract_alt_key(idx)
        };

        let (sql_op, order) = match op {
            StartOp::Equal => ("=", "ASC"),
            StartOp::GreaterThan => (">", "ASC"),
            StartOp::GreaterOrEqual => (">=", "ASC"),
            StartOp::LessThan => ("<", "DESC"),
            StartOp::LessOrEqual => ("<=", "DESC"),
        };

        let result = db.query_row(
            &format!(
                "SELECT primary_key FROM records
                 WHERE primary_key {sql_op} ?1
                 ORDER BY primary_key {order} LIMIT 1"
            ),
            [&key_bytes],
            |row| row.get::<_, Vec<u8>>(0),
        );

        match result {
            Ok(pk) => {
                // Position cursor just BEFORE this record
                // so next READ NEXT returns it
                self.cursor_position = Some(decrement_key(&pk));
                self.status = FileStatus::OK;
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                self.status = FileStatus::NOT_FOUND;
            }
            Err(_) => {
                self.status = FileStatus::PERMANENT_ERROR;
            }
        }

        self.status
    }
}
```

### 13.6 Relative File Implementation

```rust
pub struct RelativeFile {
    config: RelativeFileConfig,
    state: FileState,
    file: Option<File>,
    record_buf: Vec<u8>,
    status: FileStatus,
    current_slot: u64,  // for sequential access
    relative_key: u64,  // mirrors WS RELATIVE KEY field
    converter: Option<EbcdicConverter>,
}

pub struct RelativeFileConfig {
    pub file_path: PathBuf,
    pub slot_size: usize,  // record size + 1 byte control flag
    pub record_size: usize,
    pub optional: bool,
}

impl RelativeFile {
    /// Calculate file offset for a given slot number (1-based)
    fn slot_offset(&self, slot: u64) -> u64 {
        (slot - 1) * self.config.slot_size as u64
    }

    /// Read the control byte for a slot (0x00 = empty, 0xFF = occupied)
    fn is_slot_occupied(&mut self, slot: u64) -> io::Result<bool> {
        let file = self.file.as_mut().unwrap();
        file.seek(SeekFrom::Start(self.slot_offset(slot)))?;
        let mut control = [0u8; 1];
        file.read_exact(&mut control)?;
        Ok(control[0] == 0xFF)
    }
}
```

### 13.7 File Name Resolution

The transpiler needs a configuration layer to resolve COBOL ASSIGN TO names
to actual file system paths:

```rust
pub struct FileResolver {
    /// Maps COBOL ddname/assign-name to file path
    assignments: HashMap<String, FileAssignment>,
}

pub struct FileAssignment {
    pub path: PathBuf,
    pub organization: FileOrganization,
    pub record_size: RecordSize,
    /// For indexed files: SQLite database path
    pub index_db_path: Option<PathBuf>,
    /// Encoding of the physical file
    pub code_page: Option<CodePage>,
}

impl FileResolver {
    /// Load assignments from a configuration file
    /// (analogous to JCL DD statements)
    pub fn from_config(path: &Path) -> io::Result<Self> {
        // TOML or JSON config file:
        // [files.CUSTFILE]
        // path = "/data/customer.dat"
        // organization = "indexed"
        // record_size = 200
        // index_db = "/data/customer.idx.sqlite"
        // code_page = "cp037"
        todo!()
    }

    /// Resolve a COBOL assignment name to a file assignment
    pub fn resolve(&self, assign_name: &str) -> Option<&FileAssignment> {
        self.assignments.get(assign_name)
    }
}
```

### 13.8 Transpiler-Generated I/O Code Pattern

The transpiler generates Rust code that follows this pattern for every
COBOL I/O statement:

```rust
// COBOL:
//   READ CUSTOMER-FILE INTO WS-CUSTOMER
//       AT END SET WS-EOF TO TRUE
//       NOT AT END PERFORM PROCESS-CUSTOMER
//   END-READ.

// Generated Rust:
{
    let status = customer_file.read_next();
    ws_cust_status.store_status(&status);

    // Invoke DECLARATIVES handler if registered
    if !status.is_success() && !status.is_end_of_file() {
        if let Some(handler) = &customer_file_error_handler {
            handler(status);
        }
    }

    if status.is_end_of_file() {
        // AT END
        ws_eof = true;
    } else if status.is_success() {
        // NOT AT END
        // Implicit MOVE: READ INTO (group move semantics)
        cobol_move(
            customer_file.record_area(),
            &mut ws_customer,
        );
        process_customer(&mut ctx);
    }
}
```

---

## 14. Worked Examples

### 14.1 Sequential File Processing (Fixed-Length)

```cobol
IDENTIFICATION DIVISION.
PROGRAM-ID. SEQREAD.

ENVIRONMENT DIVISION.
INPUT-OUTPUT SECTION.
FILE-CONTROL.
    SELECT TRANS-FILE ASSIGN TO TRANSIN
        ORGANIZATION IS SEQUENTIAL
        FILE STATUS IS WS-TRANS-STATUS.

DATA DIVISION.
FILE SECTION.
FD  TRANS-FILE
    RECORDING MODE IS F
    RECORD CONTAINS 80 CHARACTERS.
01  TRANS-RECORD.
    05  TR-TYPE    PIC X.
    05  TR-ACCT    PIC 9(10).
    05  TR-AMOUNT  PIC S9(7)V99 COMP-3.
    05  TR-FILLER  PIC X(64).

WORKING-STORAGE SECTION.
01  WS-TRANS-STATUS PIC XX.
01  WS-EOF          PIC 9 VALUE 0.
    88  EOF-REACHED  VALUE 1.
01  WS-TOTAL        PIC S9(11)V99 COMP-3 VALUE 0.

PROCEDURE DIVISION.
    OPEN INPUT TRANS-FILE.
    PERFORM UNTIL EOF-REACHED
        READ TRANS-FILE
            AT END SET EOF-REACHED TO TRUE
            NOT AT END
                ADD TR-AMOUNT TO WS-TOTAL
        END-READ
    END-PERFORM.
    CLOSE TRANS-FILE.
    DISPLAY "TOTAL: " WS-TOTAL.
    STOP RUN.
```

**Generated Rust sketch**:

```rust
fn main() {
    let resolver = FileResolver::from_config("files.toml").unwrap();
    let mut trans_file = SequentialFile::new(
        resolver.resolve("TRANSIN").unwrap(),
    );
    let mut ws_trans_status = [b'0', b'0'];
    let mut ws_eof = false;
    let mut ws_total = Decimal::ZERO;

    let status = trans_file.open(OpenMode::Input);
    ws_trans_status = status.code;

    while !ws_eof {
        let status = trans_file.read_next();
        ws_trans_status = status.code;

        if status.is_end_of_file() {
            ws_eof = true;
        } else if status.is_success() {
            let tr_amount = PackedDecimal::from_bytes(
                &trans_file.record_area()[11..16],
                7, 2, true,
            );
            ws_total += tr_amount.to_decimal();
        }
    }

    trans_file.close();
    println!("TOTAL: {}", ws_total);
}
```

### 14.2 Indexed File CRUD (Dynamic Access)

```cobol
ENVIRONMENT DIVISION.
INPUT-OUTPUT SECTION.
FILE-CONTROL.
    SELECT CUSTOMER-FILE ASSIGN TO CUSTMAST
        ORGANIZATION IS INDEXED
        ACCESS MODE IS DYNAMIC
        RECORD KEY IS CUST-ID
        ALTERNATE RECORD KEY IS CUST-NAME WITH DUPLICATES
        FILE STATUS IS WS-CUST-STATUS.

*> Random read by primary key
MOVE 12345678 TO CUST-ID.
READ CUSTOMER-FILE
    INVALID KEY DISPLAY "NOT FOUND"
    NOT INVALID KEY DISPLAY CUST-NAME
END-READ.

*> Update the record
MOVE "NEW NAME" TO CUST-NAME.
REWRITE CUSTOMER-RECORD
    INVALID KEY DISPLAY "REWRITE FAILED"
END-REWRITE.

*> Sequential scan from a starting point
MOVE "SMITH" TO CUST-NAME.
START CUSTOMER-FILE KEY >= CUST-NAME
    INVALID KEY DISPLAY "NO MATCH"
END-START.
PERFORM UNTIL WS-CUST-STATUS NOT = "00"
    READ CUSTOMER-FILE NEXT
        AT END CONTINUE
        NOT AT END DISPLAY CUST-ID " " CUST-NAME
    END-READ
END-PERFORM.

*> Delete a record
MOVE 12345678 TO CUST-ID.
DELETE CUSTOMER-FILE
    INVALID KEY DISPLAY "DELETE FAILED"
END-DELETE.
```

### 14.3 Variable-Length Record Processing

```cobol
FD  VAR-FILE
    RECORDING MODE IS V
    RECORD IS VARYING IN SIZE FROM 20 TO 500
        DEPENDING ON WS-REC-LEN.
01  VAR-RECORD.
    05  VR-CODE     PIC X(4).
    05  VR-COUNT    PIC 9(4) COMP.
    05  VR-ITEMS.
        10  VR-ITEM PIC X(10) OCCURS 1 TO 49
            DEPENDING ON VR-COUNT.
```

The record length varies based on VR-COUNT:
- VR-COUNT = 1: 20 bytes (4 + 2 + 10*1 + padding)
- VR-COUNT = 49: 500 bytes (4 + 2 + 10*49 + padding)

On READ, the runtime reads the RDW to determine actual record length, then
populates VR-RECORD. The DEPENDING ON field (WS-REC-LEN) is set to the
actual length read.

On WRITE, the runtime uses WS-REC-LEN to determine how many bytes to write
(plus the RDW prefix).

### 14.4 Print File with LINAGE

```cobol
FD  REPORT-FILE
    LINAGE IS 60 LINES
        WITH FOOTING AT 55
        LINES AT TOP 3
        LINES AT BOTTOM 3.
01  REPORT-LINE PIC X(132).

PROCEDURE DIVISION.
    OPEN OUTPUT REPORT-FILE.
    PERFORM WRITE-HEADER.
    PERFORM PROCESS-DATA.
    CLOSE REPORT-FILE.

WRITE-HEADER.
    MOVE HEADER-LINE TO REPORT-LINE.
    WRITE REPORT-LINE AFTER ADVANCING PAGE.

WRITE-DETAIL.
    MOVE DETAIL-LINE TO REPORT-LINE.
    WRITE REPORT-LINE AFTER ADVANCING 1 LINE
        AT END-OF-PAGE
            PERFORM WRITE-FOOTER
            PERFORM WRITE-HEADER
        END-WRITE.

WRITE-FOOTER.
    MOVE FOOTER-LINE TO REPORT-LINE.
    WRITE REPORT-LINE AFTER ADVANCING 1 LINE.
```

---

## 15. Dialect Variations

### 15.1 IBM Enterprise COBOL

| Feature | Behavior |
|---|---|
| File backend | VSAM (KSDS, ESDS, RRDS, LDS) and QSAM |
| ASSIGN TO | JCL DD name (no file path) |
| RECORDING MODE | F, V, U, S (all supported) |
| READ PREVIOUS | Supported for VSAM KSDS |
| LOCK MODE | Record-level and file-level |
| LINE SEQUENTIAL | Not directly supported (use USS) |
| VSAM SHAREOPTIONS | Cross-region and cross-system sharing |
| Extended FILE STATUS | 2-byte + VSAM return/reason codes |
| PASSWORD clause | Legacy VSAM password protection |
| SAME RECORD AREA | Share buffer between multiple files |

### 15.2 GnuCOBOL

| Feature | Behavior |
|---|---|
| File backend | Native file system (Berkeley DB for ISAM optional) |
| ASSIGN TO | File path (literal or variable) |
| RECORDING MODE | F and V (no U or S) |
| READ PREVIOUS | Supported |
| LOCK MODE | Advisory file locking (fcntl) |
| LINE SEQUENTIAL | Fully supported |
| ORGANIZATION | SEQUENTIAL, LINE SEQUENTIAL, INDEXED, RELATIVE |
| ISAM backend | Configurable: Berkeley DB, VBISAM, etc. |
| Dynamic file name | ASSIGN TO WS-VARIABLE-NAME |

### 15.3 Micro Focus Visual COBOL

| Feature | Behavior |
|---|---|
| File backend | Native FS + Micro Focus File Handler |
| ASSIGN TO | File path or environment variable |
| RECORDING MODE | F and V |
| LOCK MODE | Record-level locking |
| LINE SEQUENTIAL | Fully supported |
| ORGANIZATION | All four types + BINARY SEQUENTIAL |
| File compression | Built-in compression support |
| File encryption | Optional encryption layer |

### 15.4 Transpiler Dialect Configuration

```rust
pub struct FileIoConfig {
    pub dialect: CobolDialect,
    /// How to resolve ASSIGN TO names
    pub resolver_mode: ResolverMode,
    /// Default encoding for files without CODE-SET
    pub default_encoding: Encoding,
    /// VSAM emulation backend for indexed files
    pub indexed_backend: IndexedBackend,
    /// Enable READ PREVIOUS (IBM extension)
    pub allow_read_previous: bool,
    /// Enable record-level locking
    pub enable_locking: bool,
    /// How to handle BLOCK CONTAINS
    pub block_handling: BlockHandling,
}

#[derive(Debug)]
pub enum ResolverMode {
    /// JCL-style: resolve via config file (IBM mode)
    ConfigFile(PathBuf),
    /// Direct: ASSIGN TO is the file path (GnuCOBOL mode)
    DirectPath,
    /// Environment variable: ASSIGN TO names env vars
    EnvironmentVariable,
}

#[derive(Debug)]
pub enum IndexedBackend {
    Sqlite,         // Recommended: SQLite via rusqlite
    BTreeMap,       // In-memory only (testing)
    Redb,           // Rust-native embedded DB
}

#[derive(Debug)]
pub enum BlockHandling {
    /// Map to BufReader/BufWriter capacity
    BufferSize,
    /// Ignore (let OS handle buffering)
    Ignore,
}
```

---

## Appendix A: COBOL I/O Verb Quick Reference

| Verb | Sequential | Indexed | Relative | Print |
|---|---|---|---|---|
| OPEN INPUT | Yes | Yes | Yes | No |
| OPEN OUTPUT | Yes | Yes | Yes | Yes |
| OPEN I-O | Yes | Yes | Yes | No |
| OPEN EXTEND | Yes | Yes | No | Yes |
| CLOSE | Yes | Yes | Yes | Yes |
| READ (seq) | Yes | Yes | Yes | No |
| READ (random) | No | Yes | Yes | No |
| WRITE | Yes | Yes | Yes | Yes(1) |
| REWRITE | Yes(2) | Yes | Yes | No |
| DELETE | No | Yes | Yes | No |
| START | No | Yes | Yes | No |

(1) Print files use WRITE with ADVANCING
(2) Sequential REWRITE requires prior READ, same record length

---

## Appendix B: File Organization Decision Tree

```
What kind of access does the program need?

Sequential only (batch processing)?
  |
  +-- Text data with line breaks? --> LINE SEQUENTIAL
  |
  +-- Binary/fixed records? --> SEQUENTIAL (QSAM)
  |
  +-- Append to existing file? --> SEQUENTIAL with EXTEND

Random access by unique key?
  |
  +-- Single key sufficient? --> INDEXED (KSDS) with RANDOM access
  |
  +-- Multiple access paths? --> INDEXED with alternate keys
  |
  +-- Mix of sequential + random? --> INDEXED with DYNAMIC access

Access by record number (slot)?
  |
  +-- Fixed-size records, slot-based? --> RELATIVE (RRDS)

High-volume log/audit trail?
  |
  +-- Append only, optional keyed access? --> SEQUENTIAL (ESDS) + AIX
```

---

## Appendix C: Cross-Reference to Other Specifications

| Topic | Document | Section |
|---|---|---|
| Record layout (GROUP, REDEFINES) | `cobol_to_rust_datatype_mapping.md` | Sections 5.1, 5.2 |
| MOVE semantics (READ INTO) | `cobol_move_engine_spec.md` | Section 8.2 |
| I/O control flow (AT END, INVALID KEY) | `cobol_control_flow_constructs.md` | Section 12 |
| SORT and MERGE operations | (future: `cobol_sort_merge_spec.md`) | -- |
| EBCDIC encoding tables | `cobol_to_rust_datatype_mapping.md` | Encoding section |
| Arithmetic operations | `cobol_arithmetic_operations.md` | -- |

---

## Appendix D: Additional Crate Dependencies for File I/O

| Crate | Purpose | Required? |
|---|---|---|
| `rusqlite` | SQLite backend for indexed files | Yes (indexed files) |
| `tempfile` | Temporary files for SORT scratch | Yes (SORT operations) |
| `toml` or `serde_json` | File resolver configuration | Yes |
| `encoding_rs` | Additional encoding support | Optional |
| `memmap2` | Memory-mapped I/O (LDS emulation) | Optional (Phase 3) |
| `redb` | Alternative indexed file backend | Optional |
