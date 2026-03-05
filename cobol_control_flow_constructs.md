# COBOL Control Flow Constructs -- Complete Reference

**Scope**: All conditionals, branching, looping, program structure, and control flow constructs across IBM Enterprise COBOL, GnuCOBOL, and Micro Focus dialects.

**Purpose**: Transpiler specification reference for the COBOL-to-Rust code generation engine.

**Companion Documents**:
- `cobol_to_rust_datatype_mapping.md` -- Data type mapping specification
- `cobol_arithmetic_operations.md` -- Arithmetic operations reference
- `cobol_migration_target_analysis.md` -- Strategic analysis

---

## Table of Contents

1. [IF / ELSE / END-IF](#1-if--else--end-if)
2. [EVALUATE Statement](#2-evaluate-statement)
3. [Condition Types](#3-condition-types)
4. [PERFORM Statement](#4-perform-statement)
5. [PERFORM Stack Semantics](#5-perform-stack-semantics)
6. [GO TO and ALTER](#6-go-to-and-alter)
7. [EXIT Variants](#7-exit-variants)
8. [STOP RUN / GOBACK](#8-stop-run--goback)
9. [CONTINUE vs NEXT SENTENCE](#9-continue-vs-next-sentence)
10. [Program Structure as Control Flow](#10-program-structure-as-control-flow)
11. [CALL Statement](#11-call-statement)
12. [File I/O Control Flow](#12-file-io-control-flow)
13. [STRING / UNSTRING Control Flow](#13-string--unstring-control-flow)
14. [SORT / MERGE Control Flow](#14-sort--merge-control-flow)
15. [DECLARATIVES Section](#15-declaratives-section)
16. [Dialect-Specific Extensions](#16-dialect-specific-extensions)
17. [Rust Mapping Strategy](#17-rust-mapping-strategy)
18. [Appendices](#18-appendices)

---

## 1. IF / ELSE / END-IF

### 1.1 Complete Syntax

```
IF condition-1
   {statement-1} ...
[ELSE
   {statement-2} ...]
{END-IF | .}
```

### 1.2 Scope Terminators

Two scope termination styles exist:

**Explicit scope terminator (modern)**:
```cobol
IF WS-A = 1
   MOVE 10 TO WS-B
ELSE
   MOVE 20 TO WS-B
END-IF.
```

**Period-delimited (legacy)**:
```cobol
IF WS-A = 1
   MOVE 10 TO WS-B
ELSE
   MOVE 20 TO WS-B.
```

The period terminates ALL open IF/ELSE scopes. This creates nesting hazards in legacy code.

### 1.3 Nested IF

```cobol
IF WS-TYPE = "A"
   IF WS-STATUS = "ACTIVE"
      PERFORM PROCESS-ACTIVE-A
   ELSE
      PERFORM PROCESS-INACTIVE-A
   END-IF
ELSE
   IF WS-TYPE = "B"
      PERFORM PROCESS-B
   END-IF
END-IF.
```

### 1.4 Legacy Period-Delimited Nesting Hazard

```cobol
IF WS-A = 1
   IF WS-B = 2
      MOVE 10 TO WS-C
   ELSE
      MOVE 20 TO WS-C.
```

The period terminates BOTH IF statements. The ELSE belongs to the inner IF. After the period, neither IF is active. This is a common source of bugs in legacy code -- the programmer may have intended the ELSE for the outer IF.

**Transpiler requirement**: The parser must correctly associate ELSE with the nearest open IF, and handle mixed period-delimited and END-IF termination within the same program.

### 1.5 Rust Mapping

```rust
if ws_a == 1 {
    ws_b = 10;
} else {
    ws_b = 20;
}
```

Direct 1:1 mapping. No special handling required beyond correct scope resolution during parsing.

---

## 2. EVALUATE Statement

EVALUATE is COBOL's multi-way branching construct, analogous to Rust's `match` but significantly more powerful.

### 2.1 Basic Syntax

```
EVALUATE {identifier-1 | literal-1 | expression-1 | TRUE | FALSE}
   [ALSO {identifier-2 | literal-2 | expression-2 | TRUE | FALSE}] ...

   {WHEN {ANY | condition-1 | TRUE | FALSE |
          [NOT] {identifier-3 | literal-3 | arithmetic-expression-1}
          [THRU | THROUGH {identifier-4 | literal-4 | arithmetic-expression-2}]}
      [ALSO {ANY | condition-2 | TRUE | FALSE |
             [NOT] {identifier-5 | literal-5 | arithmetic-expression-3}
             [THRU | THROUGH {identifier-6 | literal-6 | arithmetic-expression-4}]}] ...
      {statement-1} ...} ...

   [WHEN OTHER
      {statement-2} ...]

END-EVALUATE
```

### 2.2 EVALUATE Identifier (Switch/Match)

```cobol
EVALUATE WS-FUNCTION-CODE
   WHEN "A"    PERFORM ADD-RECORD
   WHEN "U"    PERFORM UPDATE-RECORD
   WHEN "D"    PERFORM DELETE-RECORD
   WHEN "I"    PERFORM INQUIRY
   WHEN OTHER  DISPLAY "INVALID FUNCTION"
END-EVALUATE.
```

**Rust Mapping**:
```rust
match ws_function_code.as_str() {
    "A" => add_record(),
    "U" => update_record(),
    "D" => delete_record(),
    "I" => inquiry(),
    _   => println!("INVALID FUNCTION"),
}
```

### 2.3 EVALUATE TRUE (Condition-Based)

```cobol
EVALUATE TRUE
   WHEN WS-AGE < 18     PERFORM PROCESS-MINOR
   WHEN WS-AGE < 65     PERFORM PROCESS-ADULT
   WHEN WS-AGE >= 65    PERFORM PROCESS-SENIOR
   WHEN OTHER            PERFORM PROCESS-ERROR
END-EVALUATE.
```

Each WHEN phrase is a condition tested for TRUE. The first matching WHEN executes.

**Rust Mapping**:
```rust
if ws_age < 18 {
    process_minor();
} else if ws_age < 65 {
    process_adult();
} else if ws_age >= 65 {
    process_senior();
} else {
    process_error();
}
```

### 2.4 EVALUATE FALSE

```cobol
EVALUATE FALSE
   WHEN WS-VALID-RECORD  PERFORM ERROR-HANDLING
   WHEN WS-FILE-OPEN     PERFORM OPEN-FILE
END-EVALUATE.
```

Each WHEN condition is tested for FALSE. Useful for checking which condition is NOT met.

### 2.5 EVALUATE with THRU (Range Matching)

```cobol
EVALUATE WS-SCORE
   WHEN 90 THRU 100   MOVE "A" TO WS-GRADE
   WHEN 80 THRU 89    MOVE "B" TO WS-GRADE
   WHEN 70 THRU 79    MOVE "C" TO WS-GRADE
   WHEN 60 THRU 69    MOVE "D" TO WS-GRADE
   WHEN 0  THRU 59    MOVE "F" TO WS-GRADE
   WHEN OTHER          MOVE "?" TO WS-GRADE
END-EVALUATE.
```

**Rust Mapping**:
```rust
let grade = match ws_score.to_i32() {
    90..=100 => "A",
    80..=89  => "B",
    70..=79  => "C",
    60..=69  => "D",
    0..=59   => "F",
    _        => "?",
};
```

### 2.6 EVALUATE with ALSO (Multi-Subject)

```cobol
EVALUATE WS-REGION ALSO WS-PRODUCT-TYPE
   WHEN "NORTH" ALSO "WIDGET"    PERFORM NORTH-WIDGET
   WHEN "NORTH" ALSO "GADGET"    PERFORM NORTH-GADGET
   WHEN "SOUTH" ALSO ANY         PERFORM SOUTH-ALL
   WHEN ANY     ALSO "SPECIAL"   PERFORM ALL-SPECIAL
   WHEN OTHER                    PERFORM DEFAULT-PROCESS
END-EVALUATE.
```

Each WHEN tests multiple subjects simultaneously. ALL subjects must match for the WHEN to fire. `ANY` matches any value for that subject position.

**Rust Mapping**:
```rust
match (ws_region.as_str(), ws_product_type.as_str()) {
    ("NORTH", "WIDGET")  => north_widget(),
    ("NORTH", "GADGET")  => north_gadget(),
    ("SOUTH", _)         => south_all(),
    (_, "SPECIAL")       => all_special(),
    _                    => default_process(),
}
```

### 2.7 EVALUATE TRUE ALSO TRUE

```cobol
EVALUATE TRUE ALSO TRUE
   WHEN WS-AGE > 65   ALSO WS-INCOME < 20000
      PERFORM LOW-INCOME-SENIOR
   WHEN WS-AGE > 65   ALSO WS-INCOME >= 20000
      PERFORM REGULAR-SENIOR
   WHEN WS-AGE <= 65  ALSO ANY
      PERFORM NON-SENIOR
END-EVALUATE.
```

**Rust Mapping**:
```rust
match (ws_age > 65, ws_income < 20000) {
    (true, true)  => low_income_senior(),
    (true, false) => regular_senior(),
    (false, _)    => non_senior(),
}
```

### 2.8 Multiple Values in a Single WHEN

```cobol
EVALUATE WS-DAY-CODE
   WHEN 1
   WHEN 7
      PERFORM WEEKEND-PROCESS
   WHEN 2 THRU 6
      PERFORM WEEKDAY-PROCESS
END-EVALUATE.
```

Multiple consecutive WHEN clauses (before the first statement) act as OR -- any match triggers the associated statements.

**Rust Mapping**:
```rust
match ws_day_code.to_i32() {
    1 | 7    => weekend_process(),
    2..=6    => weekday_process(),
    _        => {},
}
```

### 2.9 Semantics

- First matching WHEN executes; subsequent WHENs are skipped (no fall-through)
- WHEN OTHER is optional; if absent and nothing matches, control passes to after END-EVALUATE
- EVALUATE TRUE/FALSE subjects: each WHEN phrase is a condition, not a value
- THRU ranges are inclusive on both endpoints
- ALSO requires ALL subject positions to match
- ANY matches any value for its subject position

---

## 3. Condition Types

### 3.1 Relation Conditions

```cobol
IF identifier-1 {IS | IS NOT}
   {EQUAL TO | = | GREATER THAN | > | LESS THAN | <
    | GREATER THAN OR EQUAL TO | >= | NOT <
    | LESS THAN OR EQUAL TO | <= | NOT >}
   {identifier-2 | literal-1 | arithmetic-expression-1}
```

**Comparison semantics**:

| Left Type       | Right Type      | Comparison Used            |
|-----------------|-----------------|----------------------------|
| Numeric         | Numeric         | Algebraic (value-based)    |
| Alphanumeric    | Alphanumeric    | Character-by-character (collating sequence) |
| Alphanumeric    | Numeric literal | Numeric literal treated as alphanumeric |
| Group           | Any             | Alphanumeric (raw bytes)   |

**Group comparison**: When comparing GROUP items, the comparison is always alphanumeric regardless of the subordinate field types. The raw byte representations are compared character-by-character using the program's collating sequence.

```cobol
01 WS-GROUP-1.
   05 WS-NUM PIC 9(3) VALUE 42.
   05 WS-TEXT PIC X(5) VALUE "HELLO".
01 WS-GROUP-2.
   05 WS-NUM PIC 9(3) VALUE 42.
   05 WS-TEXT PIC X(5) VALUE "HELLO".

IF WS-GROUP-1 = WS-GROUP-2
```

This compares the raw 8-byte representation, not field-by-field.

### 3.2 Class Conditions

```cobol
IF identifier-1 IS [NOT]
   {NUMERIC | ALPHABETIC | ALPHABETIC-LOWER | ALPHABETIC-UPPER
    | class-name-1 | DBCS | KANJI}
```

```cobol
IF WS-INPUT IS NUMERIC
   COMPUTE WS-VALUE = FUNCTION NUMVAL(WS-INPUT)
END-IF.
```

**Semantics by data type**:

| Data Type       | NUMERIC means                                     |
|-----------------|---------------------------------------------------|
| PIC 9 (DISPLAY) | All characters are 0-9, sign is valid             |
| PIC X           | All characters are 0-9 (no sign allowed)          |
| COMP-3          | All nibbles are 0-9, sign nibble is C/D/F         |
| COMP/COMP-4/5   | Always NUMERIC (binary has no invalid state)       |
| Group           | Treated as alphanumeric -- all bytes must be 0-9   |

### 3.3 Sign Conditions

```cobol
IF identifier-1 IS [NOT] {POSITIVE | NEGATIVE | ZERO}
```

```cobol
IF WS-BALANCE IS NEGATIVE
   PERFORM OVERDRAFT-HANDLER
END-IF.
```

**Semantics**: POSITIVE means > 0 (not >= 0). ZERO means = 0. NEGATIVE means < 0.

### 3.4 Condition-Name Conditions (88-Level)

```cobol
01 WS-FILE-STATUS PIC XX.
   88 WS-SUCCESS       VALUE "00".
   88 WS-EOF           VALUE "10".
   88 WS-NOT-FOUND     VALUE "23".
   88 WS-VALID-STATUS  VALUE "00" "10".
   88 WS-RANGE-CHECK   VALUE "20" THRU "29".

IF WS-SUCCESS
   PERFORM PROCESS-RECORD
END-IF.

IF WS-RANGE-CHECK
   PERFORM HANDLE-RANGE-ERROR
END-IF.
```

Testing an 88-level condition-name checks whether the parent field contains one of the defined values.

**SET TRUE/FALSE**:
```cobol
SET WS-EOF TO TRUE.     *> MOVE "10" TO WS-FILE-STATUS
SET WS-EOF TO FALSE.    *> Requires FALSE value: 88 WS-EOF VALUE "10" FALSE "00"
```

### 3.5 Combined Conditions

```cobol
IF condition-1 {AND | OR} condition-2

IF WS-AGE >= 18 AND WS-AGE <= 65
   PERFORM ELIGIBLE-PROCESS
END-IF.

IF NOT (WS-STATUS = "A" OR WS-STATUS = "B")
   PERFORM INVALID-STATUS
END-IF.
```

**Operator precedence** (highest to lowest):
1. Parentheses
2. NOT
3. AND
4. OR

### 3.6 Abbreviated Combined Conditions

COBOL allows omitting repeated subjects and operators:

```cobol
*> Full form:
IF WS-CODE = "A" OR WS-CODE = "B" OR WS-CODE = "C"

*> Abbreviated (omit subject):
IF WS-CODE = "A" OR "B" OR "C"

*> Full form:
IF WS-X > 1 AND WS-X < 10

*> Abbreviated (omit subject and operator):
IF WS-X > 1 AND < 10
```

**Expansion rules**:
- The implied subject is the last explicitly stated subject
- The implied operator is the last explicitly stated relational operator
- NOT distributes: `IF NOT (A = 1 OR 2)` becomes `IF NOT (A = 1) AND NOT (A = 2)`

**Transpiler requirement**: The parser must fully expand abbreviated conditions during AST construction. The expansion algorithm must correctly handle implied subjects, implied operators, and NOT distribution. This is one of the trickier parsing challenges.

### 3.7 Non-Short-Circuit Evaluation

**IBM Enterprise COBOL**: Does NOT guarantee short-circuit evaluation. Both sides of AND/OR may be evaluated even when the first operand determines the result.

```cobol
*> DANGEROUS on IBM:
IF WS-INDEX > 0 AND WS-TABLE(WS-INDEX) = "X"
```

If WS-INDEX = 0, `WS-TABLE(WS-INDEX)` may still be evaluated, causing an out-of-bounds reference.

**GnuCOBOL**: Configurable via `COB_SHORT_CIRCUIT` runtime setting or compiler flag. Default behavior may vary by version.

**Micro Focus**: Mode-dependent. Some compilation modes support short-circuit.

**Transpiler critical note**: Since Rust always short-circuits `&&` and `||`, the transpiler must ensure migrated code does not depend on both sides being evaluated (side effects in conditions). For IBM source code where non-short-circuit matters, evaluate all conditions into temporaries first:

```rust
// IBM-safe: evaluate both conditions first
let cond1 = ws_index > 0;
let cond2 = ws_table.get(ws_index) == "X"; // evaluated regardless
if cond1 && cond2 {
    // ...
}
```

---

## 4. PERFORM Statement

PERFORM is COBOL's primary mechanism for both subroutine calls and looping. It has two fundamental forms: **out-of-line** (call a paragraph/section) and **inline** (loop with embedded body).

### 4.1 Out-of-Line PERFORM (Subroutine Call)

#### Format 1: Simple PERFORM

```
PERFORM procedure-name-1 [THRU procedure-name-2]
```

```cobol
PERFORM 2000-VALIDATE-INPUT.
PERFORM 3000-PROCESS THRU 3000-EXIT.
```

Executes the named paragraph(s), then returns control to the statement after PERFORM. When THRU is specified, all paragraphs from procedure-name-1 through procedure-name-2 are executed sequentially.

#### Format 2: PERFORM TIMES

```
PERFORM procedure-name-1 [THRU procedure-name-2]
   {identifier-1 | integer-1} TIMES
```

```cobol
PERFORM 2000-PROCESS 5 TIMES.
PERFORM 2000-PROCESS WS-COUNT TIMES.
PERFORM 2000-PROCESS 0 TIMES.     *> Legal -- body never executes
```

#### Format 3: PERFORM UNTIL

```
PERFORM procedure-name-1 [THRU procedure-name-2]
   [WITH TEST {BEFORE | AFTER}]
   UNTIL condition-1
```

```cobol
PERFORM 2000-READ-RECORD
   UNTIL WS-EOF = "Y".

PERFORM 2000-PROCESS
   WITH TEST AFTER
   UNTIL WS-DONE.
```

#### Format 4: PERFORM VARYING

```
PERFORM procedure-name-1 [THRU procedure-name-2]
   [WITH TEST {BEFORE | AFTER}]
   VARYING identifier-1 FROM {identifier-2 | literal-1}
      BY {identifier-3 | literal-2}
      UNTIL condition-1
   [AFTER identifier-4 FROM {identifier-5 | literal-3}
      BY {identifier-6 | literal-4}
      UNTIL condition-2] ...
```

```cobol
PERFORM 2000-PROCESS
   VARYING WS-I FROM 1 BY 1
   UNTIL WS-I > 100.

PERFORM 2000-PROCESS-CELL
   VARYING WS-ROW FROM 1 BY 1 UNTIL WS-ROW > 10
   AFTER WS-COL FROM 1 BY 1 UNTIL WS-COL > 20.
```

### 4.2 Inline PERFORM (Loop with Embedded Body)

All formats above can be used inline (with the body between PERFORM and END-PERFORM instead of a paragraph name).

#### Inline UNTIL

```cobol
PERFORM UNTIL WS-EOF = "Y"
   READ INPUT-FILE INTO WS-RECORD
      AT END SET WS-EOF TO TRUE
   END-READ
   IF NOT WS-EOF
      ADD 1 TO WS-COUNT
      PERFORM 2000-PROCESS-RECORD
   END-IF
END-PERFORM.
```

#### Inline VARYING

```cobol
PERFORM VARYING WS-I FROM 1 BY 1
   UNTIL WS-I > WS-TABLE-SIZE
   IF WS-TABLE-ENTRY(WS-I) = WS-SEARCH-KEY
      MOVE WS-I TO WS-FOUND-INDEX
      EXIT PERFORM
   END-IF
END-PERFORM.
```

#### Inline TIMES

```cobol
PERFORM 3 TIMES
   DISPLAY "RETRY ATTEMPT"
   PERFORM CONNECT-TO-SERVER
   IF WS-CONNECTED
      EXIT PERFORM
   END-IF
END-PERFORM.
```

### 4.3 TEST BEFORE vs TEST AFTER

**WITH TEST BEFORE** (default):

```
1. Set loop variable = FROM value
2. TEST: Is UNTIL condition true? If yes, exit
3. Execute body
4. Add BY value to loop variable
5. Go to step 2
```

**WITH TEST AFTER**:

```
1. Set loop variable = FROM value
2. Execute body
3. Add BY value to loop variable
4. TEST: Is UNTIL condition true? If yes, exit
5. Go to step 2
```

TEST AFTER guarantees at least one execution (like do-while).

**Value of loop variable after completion**:

TEST BEFORE, FROM 1 BY 1 UNTIL > 10:
- Body executes 10 times (i=1 through i=10)
- Final value: 11 (first value that made condition true)

TEST AFTER, FROM 1 BY 1 UNTIL > 10:
- Body executes 10 times (i=1 through i=10)
- Final value: 11

But consider UNTIL >= 1:
- TEST BEFORE: Body executes 0 times. Final value: 1
- TEST AFTER: Body executes 1 time. Final value: 2

**Many COBOL programs depend on the final loop variable value.** The transpiler must preserve this behavior exactly.

### 4.4 PERFORM VARYING AFTER (Nested Loops)

```cobol
PERFORM PARA-A
   VARYING WS-I FROM 1 BY 1 UNTIL WS-I > 3
   AFTER WS-J FROM 1 BY 1 UNTIL WS-J > 4
```

Creates nested loops. AFTER is the inner loop. Algorithm:

```
1. Set WS-I = 1 (outer FROM)
2. Set WS-J = 1 (inner FROM)
3. Test WS-I > 3 -- if true, exit entirely
4. Test WS-J > 4 -- if true, go to step 8
5. Execute PARA-A
6. Add 1 to WS-J
7. Go to step 4
8. Add 1 to WS-I
9. Set WS-J = 1 (reset inner to FROM)
10. Go to step 3
```

The inner variable resets to its FROM value each time the outer increments.

After completion: WS-I = 4, WS-J = 1 (inner was reset when outer incremented past limit).

### 4.5 Modifying Loop Variables Inside the Body

The COBOL standard does NOT prohibit this:

```cobol
PERFORM PARA-A
   VARYING WS-I FROM 1 BY 1 UNTIL WS-I > 10.

PARA-A.
   IF WS-I = 5
      MOVE 8 TO WS-I
   END-IF.
   DISPLAY WS-I.
```

After MOVE 8, the BY increment still applies: WS-I becomes 9 (8 + 1). The loop continues from 9.

**The loop variable is a regular data item, not a protected iterator.** The BY increment always applies to the current value after the body executes.

### 4.6 Floating-Point Loop Variables

```cobol
01 WS-F COMP-2.
PERFORM PARA-A VARYING WS-F FROM 0.0 BY 0.1 UNTIL WS-F >= 1.0
```

Standard floating-point precision issues apply. COBOL programs often use COMP-3 (packed decimal) for loop variables to get exact decimal arithmetic. **The transpiler must use rust_decimal for decimal-type loop variables, not f64.**

### 4.7 Rust Mapping

| PERFORM Format           | Rust Mapping               |
|--------------------------|----------------------------|
| PERFORM paragraph        | `function_call()`          |
| PERFORM THRU             | Sequential function calls or merged function |
| PERFORM n TIMES          | `for _ in 0..n { }`       |
| PERFORM UNTIL (BEFORE)   | `while !condition { }`     |
| PERFORM UNTIL (AFTER)    | `loop { ...; if condition { break; } }` |
| PERFORM VARYING (BEFORE) | `let mut i = from; while !cond { ...; i += by; }` |
| PERFORM VARYING (AFTER)  | `let mut i = from; loop { ...; i += by; if cond { break; } }` |
| Inline PERFORM           | Same patterns, body inline |

---

## 5. PERFORM Stack Semantics

This section covers the deep behavioral semantics of COBOL's PERFORM mechanism that are critical for correct transpilation.

### 5.1 The PERFORM Stack Model

COBOL maintains an implicit PERFORM return stack. When `PERFORM PARA-A` executes:
1. The system pushes a return entry: {return_to: next_statement, range_end: PARA-A}
2. Control transfers to PARA-A
3. When PARA-A completes (reaches the next paragraph header), the stack is popped
4. Control returns to the statement after PERFORM

For PERFORM THRU:
1. Push: {return_to: next_statement, range_end: PARA-C} (the THRU target)
2. Control transfers to PARA-A
3. Execution flows through PARA-A, PARA-B, PARA-C (fall-through)
4. When PARA-C completes, the stack is popped
5. Control returns

### 5.2 Overlapping PERFORM Ranges

```cobol
PERFORM A THRU C.

A.
   PERFORM B THRU D.    *> Overlaps with A THRU C!
B.
   DISPLAY "B".
C.
   DISPLAY "C".
D.
   DISPLAY "D".
```

The ranges A-C and B-D overlap. This is **undefined behavior** per the COBOL standard. IBM Enterprise COBOL uses a LIFO stack model where the most recent PERFORM return fires first, but the exact behavior with overlapping ranges is implementation-defined. The transpiler should detect this pattern and warn.

### 5.3 GO TO Within PERFORM Range

```cobol
PERFORM 1000-PROCESS THRU 1000-EXIT.

1000-PROCESS.
   IF WS-SKIP
      GO TO 1000-EXIT        *> Still within range -- OK
   END-IF.
   PERFORM 2000-VALIDATE.
1000-EXIT.
   EXIT.
```

GO TO within the PERFORM range is legal. When 1000-EXIT completes, the PERFORM return fires normally. This pattern (skipping paragraphs within a THRU range) is common and idiomatic.

### 5.4 GO TO Outside PERFORM Range

```cobol
PERFORM 1000-PROCESS THRU 1000-EXIT.
DISPLAY "AFTER PERFORM".
STOP RUN.

1000-PROCESS.
   IF WS-FATAL-ERROR
      GO TO 9999-ABORT        *> OUTSIDE the PERFORM range!
   END-IF.
1000-EXIT.
   EXIT.

9999-ABORT.
   DISPLAY "FATAL ERROR".
   STOP RUN.
```

A GO TO that transfers outside the PERFORM range terminates the PERFORM. Control does NOT return to "AFTER PERFORM". The standard says behavior is undefined, but most implementations simply transfer control and abandon the PERFORM stack entry.

**Transpiler note**: This pattern is common in error handling. Convert to `Result`-based error propagation in Rust.

### 5.5 Recursive and Re-entrant PERFORM

Standard COBOL (pre-2002) does not support recursive PERFORM. If PARA-A contains `PERFORM PARA-A`, the behavior is undefined (typically stack corruption because WORKING-STORAGE is statically allocated).

COBOL 2002+ added the RECURSIVE attribute for programs, but recursive PERFORM of paragraphs within a single program remains problematic across implementations.

**Transpiler note**: Detect recursive PERFORM patterns and either reject or convert to actual Rust recursion with stack-allocated state.

---

## 6. GO TO and ALTER

### 6.1 Simple GO TO

```
GO TO procedure-name-1
```

```cobol
GO TO 9999-ABORT.
```

Unconditional transfer of control. No return. If within a PERFORM range, the PERFORM is terminated (see Section 5.4).

**Rust Mapping**: No direct equivalent. Strategies:
1. **Restructuring** (preferred): Convert GO TO patterns to structured control flow (if-else, loop, break, continue)
2. **State machine**: For complex GO TO patterns:

```rust
enum State { Para1000, Para2000, Para3000, Exit }
let mut state = State::Para1000;
loop {
    match state {
        State::Para1000 => {
            // paragraph body
            state = State::Para3000; // GO TO 3000
        }
        State::Para2000 => { /* ... */ }
        State::Para3000 => { /* ... */ }
        State::Exit => break,
    }
}
```

### 6.2 GO TO DEPENDING ON (Computed GO TO)

```
GO TO procedure-name-1 [procedure-name-2 ...]
   DEPENDING ON identifier-1
```

```cobol
GO TO 1000-ADD
      2000-UPDATE
      3000-DELETE
      4000-INQUIRY
   DEPENDING ON WS-FUNCTION-CODE.
DISPLAY "INDEX OUT OF RANGE".
```

**Semantics**:
- 1-based indexing: identifier = 1 -> first target, 2 -> second, etc.
- Out of range (< 1 or > count of targets): No transfer occurs, execution falls through to the next statement
- All dialects follow this behavior

**Rust Mapping**:
```rust
match ws_function_code.to_i64() {
    1 => { /* GO TO 1000-ADD */ }
    2 => { /* GO TO 2000-UPDATE */ }
    3 => { /* GO TO 3000-DELETE */ }
    4 => { /* GO TO 4000-INQUIRY */ }
    _ => { /* fall through */ }
}
```

### 6.3 ALTER Statement (Dynamic GO TO Modification)

```
ALTER procedure-name-1 TO [PROCEED TO] procedure-name-2
```

```cobol
1000-DISPATCH.
   GO TO 1000-DISPATCH-TARGET.

*> Later:
ALTER 1000-DISPATCH TO PROCEED TO 2000-SPECIAL-CASE.
```

**Semantics**: ALTER changes the target of a GO TO statement at runtime. The paragraph named must contain a single GO TO statement. After ALTER, that GO TO transfers to the new target.

**Status**: Obsolete in COBOL-85, deleted from COBOL-2002. All modern dialects still accept it for backward compatibility.

**Why ALTER exists**: Provided dynamic dispatch in early COBOL (COBOL-60/68) when structured programming constructs were limited.

**Why never use it**:
- Makes control flow unanalyzable at compile time
- Any paragraph with an alterable GO TO could go anywhere
- Multiple ALTER statements for the same GO TO create nearly untraceable state machines
- Every style guide explicitly bans it

**Rust Mapping**:
```rust
let mut dispatch_target: fn() = default_target;
dispatch_target = special_case;  // ALTER equivalent
dispatch_target();               // GO TO equivalent

// Or with state enum:
let mut alter_targets: HashMap<&str, ParagraphId> = HashMap::new();
alter_targets.insert("1000-DISPATCH", ParagraphId::DefaultTarget);
// ALTER equivalent:
alter_targets.insert("1000-DISPATCH", ParagraphId::SpecialCase);
```

---

## 7. EXIT Variants

### 7.1 EXIT (Plain)

```
EXIT.
```

A no-op statement. Used as the sole statement in an exit paragraph to provide a target for PERFORM THRU:

```cobol
PERFORM 1000-PROCESS THRU 1000-EXIT.

1000-PROCESS.
   * ... processing logic ...
1000-EXIT.
   EXIT.
```

This pattern is idiomatic COBOL. The EXIT paragraph serves as a clean endpoint for the PERFORM range.

**Rust Mapping**: No code emitted.

### 7.2 EXIT PERFORM (Break)

```
EXIT PERFORM
```

```cobol
PERFORM UNTIL WS-EOF = "Y"
   READ INPUT-FILE INTO WS-RECORD
      AT END
         EXIT PERFORM
   END-READ
   PERFORM PROCESS-RECORD
END-PERFORM.
```

Transfers control to the statement after END-PERFORM. Exactly equivalent to Rust's `break`.

**Scope**: Exits the innermost enclosing inline PERFORM only. There is no way to EXIT PERFORM from an outer loop inside an inner inline PERFORM.

**Validity**: Only valid inside inline PERFORM (with END-PERFORM). NOT valid inside out-of-line PERFORM paragraphs.

### 7.3 EXIT PERFORM CYCLE (Continue)

```
EXIT PERFORM CYCLE
```

```cobol
PERFORM VARYING WS-I FROM 1 BY 1 UNTIL WS-I > 100
   IF WS-RECORD(WS-I) = SPACES
      EXIT PERFORM CYCLE
   END-IF
   PERFORM PROCESS-ITEM
END-PERFORM.
```

Skips the rest of the current iteration and returns to the increment/test. Exactly equivalent to Rust's `continue`.

**Same scope and validity rules as EXIT PERFORM.**

### 7.4 EXIT PARAGRAPH

```
EXIT PARAGRAPH
```

```cobol
1000-PROCESS.
   IF WS-SKIP-FLAG = "Y"
      EXIT PARAGRAPH
   END-IF.
   MOVE WS-INPUT TO WS-OUTPUT.
   PERFORM 2000-VALIDATE.
```

Transfers control to the end of the current paragraph. The next paragraph begins executing (if in a PERFORM THRU range or during sequential execution).

**Dialect support**: IBM Enterprise COBOL, GnuCOBOL, Micro Focus. Not in COBOL-85 standard. Added to COBOL 2002.

**Rust Mapping**: Early return from paragraph function, or restructured using if-else.

### 7.5 EXIT SECTION

```
EXIT SECTION
```

Transfers control to the end of the current section.

**Same dialect support as EXIT PARAGRAPH.**

### 7.6 EXIT PROGRAM

```
EXIT PROGRAM [WITH {STATUS | NORMAL STATUS | ERROR STATUS} identifier-1]
```

Returns control from a called subprogram. If used in a main program, it is a no-op.

**Rust Mapping**: `return` from function.

### 7.7 EXIT FUNCTION (COBOL 2014)

```cobol
FUNCTION-ID. CALCULATE-TAX.
   ...
   IF INCOME < 0
      MOVE 0 TO RETURN-VALUE
      EXIT FUNCTION
   END-IF.
   COMPUTE RETURN-VALUE = INCOME * TAX-RATE.
END FUNCTION CALCULATE-TAX.
```

**Rust Mapping**: Direct `return` from function.

---

## 8. STOP RUN / GOBACK

### 8.1 STOP RUN

```
STOP RUN [WITH {STATUS | NORMAL STATUS | ERROR STATUS} identifier-1]
```

```cobol
STOP RUN.
STOP RUN WITH STATUS WS-RETURN-CODE.
STOP RUN WITH NORMAL STATUS 0.
STOP RUN WITH ERROR STATUS 8.
```

**Semantics**: Terminates the entire run unit (main program AND all called subprograms). All files are closed. Resources are released. In a subprogram, STOP RUN terminates the ENTIRE run unit, not just the subprogram.

**Rust Mapping**: `std::process::exit(return_code);`

### 8.2 GOBACK

```
GOBACK
```

**Semantics**:
- In a **main program**: Acts like STOP RUN
- In a **subprogram**: Returns control to the calling program (like EXIT PROGRAM)

**Dialect support**: Not in COBOL-85 standard but supported by all major dialects. Added to COBOL 2002.

**Rust Mapping**: `return` from function, or `std::process::exit()` from main.

### 8.3 STOP Literal (Obsolete)

```
STOP literal-1
```

```cobol
STOP "MOUNT TAPE ON DRIVE 3".
```

Displays the literal on the operator console and halts. The operator can resume execution. From the batch-processing era. Obsolete in COBOL-85. Still accepted by most compilers.

---

## 9. CONTINUE vs NEXT SENTENCE

This distinction is **critical** for correct transpilation and a major source of bugs in legacy COBOL code.

### 9.1 CONTINUE

```
CONTINUE
```

A no-op. Does nothing. Execution continues with the next statement in sequence. Used as a placeholder where a statement is syntactically required:

```cobol
IF VALID-RECORD
   CONTINUE
ELSE
   PERFORM ERROR-HANDLING
END-IF.

EVALUATE WS-CODE
   WHEN "A"   CONTINUE
   WHEN "B"   PERFORM PROCESS-B
END-EVALUATE.
```

**Dialect support**: COBOL-85 standard. All dialects.

**Rust Mapping**: Empty block `{}` or no code at all.

### 9.2 NEXT SENTENCE

```
NEXT SENTENCE
```

Transfers control to the **first statement after the next period (`.`)** in the source code. This is a source-level concept tied to the physical period character.

### 9.3 The Critical Difference

```cobol
IF WS-A = 1
   IF WS-B = 2
      NEXT SENTENCE
   ELSE
      MOVE 10 TO WS-C
   END-IF
   MOVE 20 TO WS-D
END-IF.
MOVE 30 TO WS-E.
```

With **NEXT SENTENCE**: If WS-A = 1 and WS-B = 2, control jumps to `MOVE 30 TO WS-E` (past the period after END-IF). `MOVE 20 TO WS-D` is **SKIPPED**.

With **CONTINUE** (if substituted): If WS-A = 1 and WS-B = 2, control continues to `MOVE 20 TO WS-D` (next statement within current scope).

**NEXT SENTENCE breaks out of ALL enclosing IF statements up to the period. CONTINUE stays within the current scope.**

### 9.4 Why NEXT SENTENCE Is Dangerous

NEXT SENTENCE is a legacy construct from before END-IF existed (when all IFs were terminated by periods). It should never be used in new code. CONTINUE should always be used instead.

### 9.5 Transpiler Strategy

NEXT SENTENCE requires the transpiler to track sentence boundaries (periods). Options:

1. **Labeled blocks** (recommended):
```rust
'sentence: {
    if ws_a == 1 {
        if ws_b == 2 {
            break 'sentence;  // NEXT SENTENCE
        } else {
            ws_c = 10;
        }
        ws_d = 20;
    }
}
ws_e = 30;
```

2. **Pre-processing pass**: Convert NEXT SENTENCE to CONTINUE with appropriate restructuring before code generation.

3. **GO TO target resolution**: Resolve NEXT SENTENCE to a specific paragraph/statement and replace with a GO TO during normalization.

---

## 10. Program Structure as Control Flow

### 10.1 Sequential Execution (Fall-Through)

Without explicit control flow statements, COBOL executes paragraphs sequentially from top to bottom:

```cobol
PROCEDURE DIVISION.
MAIN-LOGIC.
   MOVE 1 TO WS-A.
NEXT-STEP.
   MOVE 2 TO WS-B.
FINAL-STEP.
   STOP RUN.
```

Execution flows: MAIN-LOGIC -> NEXT-STEP -> FINAL-STEP -> stops.

This is fundamentally different from languages where functions are independent. In COBOL, **paragraphs fall through to the next paragraph** unless explicitly redirected. A paragraph is both a callable unit (via PERFORM) and a sequential label in the flow.

### 10.2 What Constitutes the "End" of a Paragraph

A paragraph ends at:
1. The next paragraph header (a name followed by a period)
2. The next section header
3. The end of the Procedure Division

Determined purely by the **physical layout** of the source code, not by any explicit END marker.

### 10.3 Sections vs Paragraphs

```cobol
PROCEDURE DIVISION.
SECTION-1 SECTION.
PARA-1A.
   DISPLAY "1A".
PARA-1B.
   DISPLAY "1B".
SECTION-2 SECTION.
PARA-2A.
   DISPLAY "2A".
```

- `PERFORM SECTION-1` executes ALL paragraphs within SECTION-1 (PARA-1A and PARA-1B)
- `PERFORM PARA-1A` executes only PARA-1A (up to the next paragraph header)
- `PERFORM PARA-1A THRU PARA-1B` executes PARA-1A and PARA-1B

### 10.4 PERFORM THRU and Fall-Through Interaction

```cobol
PERFORM 1000-INIT THRU 1000-EXIT.

1000-INIT.
   MOVE 0 TO WS-COUNT.
1000-VALIDATE.
   IF WS-INPUT = SPACES
      GO TO 1000-EXIT
   END-IF.
1000-PROCESS.
   ADD 1 TO WS-COUNT.
1000-EXIT.
   EXIT.
```

The GO TO 1000-EXIT stays within the PERFORM range, so after 1000-EXIT completes, control returns to the caller. This is legal and idiomatic.

### 10.5 Rust Mapping Strategy

Convert paragraphs to functions. For PERFORM THRU ranges: either merge paragraphs into a single function or call each paragraph function sequentially. For complex GO TO patterns, use state machines.

```rust
// Simple paragraph -> function
fn para_1000_process(ctx: &mut ProgramState) {
    // paragraph body
}

// PERFORM THRU -> sequential calls or merged function
fn perform_1000_init_thru_exit(ctx: &mut ProgramState) {
    // 1000-INIT body
    // 1000-VALIDATE body
    // 1000-PROCESS body
    // 1000-EXIT body
}
```

---

## 11. CALL Statement

### 11.1 Complete Syntax

```
CALL {identifier-1 | literal-1}
   [USING {[BY REFERENCE] | BY CONTENT | BY VALUE}
      {identifier-2 | ADDRESS OF identifier-3 |
       OMITTED | literal-2} ...] ...
   [RETURNING identifier-4]
   [ON OVERFLOW imperative-statement-1]
   [ON EXCEPTION imperative-statement-2]
   [NOT ON EXCEPTION imperative-statement-3]
[END-CALL]
```

### 11.2 Static vs Dynamic CALL

**Static CALL** (literal program name):
```cobol
CALL "SUBPROG" USING WS-DATA.
```
Target resolved at compile/link time. Fixed address.

**Dynamic CALL** (identifier):
```cobol
MOVE "SUBPROG" TO WS-PROG-NAME.
CALL WS-PROG-NAME USING WS-DATA.
```
Target loaded at runtime. Runtime searches for the program module.

**Transpiler note**: Static calls -> direct Rust function calls. Dynamic calls -> runtime dispatch (HashMap of function pointers or trait objects).

### 11.3 Parameter Passing Modes

| Mode          | COBOL Syntax         | Semantics                              | Rust Mapping          |
|---------------|----------------------|----------------------------------------|-----------------------|
| BY REFERENCE  | `USING BY REFERENCE` | Shared memory; callee can modify       | `&mut T`              |
| BY CONTENT    | `USING BY CONTENT`   | Copy passed; callee cannot affect caller | pass clone          |
| BY VALUE      | `USING BY VALUE`     | Native value; for C interop            | pass by value         |
| OMITTED       | `USING OMITTED`      | Null pointer for that position         | `Option<&mut T>` = None |

BY REFERENCE is the default when no qualifier is specified.

**Aliasing concern**:
```cobol
CALL "SUB" USING BY REFERENCE WS-A BY REFERENCE WS-A.
```
Both parameters point to the same memory. The called program sees two aliased parameters. This is legal COBOL but creates challenges for Rust's borrow checker.

### 11.4 ON EXCEPTION / ON OVERFLOW

```cobol
CALL WS-PROGRAM-NAME USING WS-DATA
   ON EXCEPTION
      DISPLAY "PROGRAM NOT FOUND: " WS-PROGRAM-NAME
      MOVE 99 TO WS-RETURN-CODE
   NOT ON EXCEPTION
      MOVE RETURN-CODE TO WS-RETURN-CODE
END-CALL.
```

**ON EXCEPTION** (synonym: ON OVERFLOW): Handles dynamic CALL failure (target program not found/loadable). Does NOT handle exceptions within the called program.

**Rust Mapping**:
```rust
match call_program(&program_name, &mut ws_data) {
    Ok(return_code) => { ws_return_code = return_code; }
    Err(_) => {
        println!("PROGRAM NOT FOUND: {}", program_name);
        ws_return_code = 99;
    }
}
```

### 11.5 CANCEL Statement

```
CANCEL {identifier-1 | literal-1} ...
```

Releases the called program's memory. Next CALL reloads and reinitializes it. Without CANCEL, a subprogram's WORKING-STORAGE persists between CALLs (retains state like a module with static variables).

**Rust Mapping**: Reset the subprogram's state struct to initial values.

### 11.6 INITIAL Keyword

```cobol
PROGRAM-ID. SUBPROG IS INITIAL.
```

A program marked INITIAL is restored to its initial state every time it is called. All WORKING-STORAGE items reset to their VALUE clauses (or binary zeros).

Without INITIAL, state persists between calls.

**Transpiler note**: Non-INITIAL programs -> structs with persistent state. INITIAL programs -> reset all fields on each invocation.

### 11.7 Recursive CALL

Standard COBOL (pre-2002) does NOT support recursion. A program cannot CALL itself.

COBOL 2002+ added:
```cobol
PROGRAM-ID. RECURSE IS RECURSIVE.
```

- **IBM Enterprise COBOL**: Requires RECURSIVE attribute AND `THREAD` compiler option
- **GnuCOBOL**: Supports recursion with RECURSIVE attribute

Without RECURSIVE, attempting recursion produces undefined behavior (typically runtime abend or stack corruption).

**Transpiler note**: Check for RECURSIVE attribute. Without it, either reject recursive patterns or allocate WORKING-STORAGE on the stack for RECURSIVE programs.

---

## 12. File I/O Control Flow

File I/O statements have built-in conditional branching via AT END, INVALID KEY, and ON EXCEPTION phrases.

### 12.1 READ Statement

**Sequential**:
```
READ file-name [INTO identifier-1]
   [AT END imperative-statement-1]
   [NOT AT END imperative-statement-2]
[END-READ]
```

**Keyed (indexed/relative)**:
```
READ file-name [INTO identifier-1]
   [KEY IS data-name-1]
   [INVALID KEY imperative-statement-1]
   [NOT INVALID KEY imperative-statement-2]
[END-READ]
```

```cobol
READ INPUT-FILE INTO WS-RECORD
   AT END
      SET WS-EOF TO TRUE
   NOT AT END
      ADD 1 TO WS-RECORD-COUNT
      PERFORM 2000-PROCESS-RECORD
END-READ.
```

### 12.2 WRITE Statement

```
WRITE record-name [FROM identifier-1]
   [INVALID KEY imperative-statement-1]
   [NOT INVALID KEY imperative-statement-2]
[END-WRITE]
```

For print files:
```
WRITE record-name [FROM identifier-1]
   [{BEFORE | AFTER} ADVANCING {identifier-2 | integer-1} {LINES | LINE}
    | {BEFORE | AFTER} ADVANCING {PAGE | mnemonic-name}]
   [AT {END-OF-PAGE | EOP} imperative-statement-1]
   [NOT AT {END-OF-PAGE | EOP} imperative-statement-2]
[END-WRITE]
```

### 12.3 DELETE Statement

```
DELETE file-name
   [INVALID KEY imperative-statement-1]
   [NOT INVALID KEY imperative-statement-2]
[END-DELETE]
```

### 12.4 REWRITE Statement

```
REWRITE record-name [FROM identifier-1]
   [INVALID KEY imperative-statement-1]
   [NOT INVALID KEY imperative-statement-2]
[END-REWRITE]
```

### 12.5 START Statement

```
START file-name KEY IS {= | > | NOT < | >= | < | <= | NOT >} data-name-1
   [INVALID KEY imperative-statement-1]
   [NOT INVALID KEY imperative-statement-2]
[END-START]
```

### 12.6 RETURN Statement (SORT/MERGE)

```
RETURN sort-file INTO identifier-1
   AT END imperative-statement-1
   [NOT AT END imperative-statement-2]
[END-RETURN]
```

### 12.7 FILE STATUS

The file status variable (defined in SELECT clause) is always set after every I/O operation, regardless of whether AT END/INVALID KEY phrases are coded.

| Status | Meaning           |
|--------|-------------------|
| "00"   | Success           |
| "10"   | At end (EOF)      |
| "22"   | Duplicate key     |
| "23"   | Not found         |
| "30"   | Permanent error   |
| "35"   | File not found    |
| "41"   | Already open      |
| "42"   | Not open          |

### 12.8 Rust Mapping Strategy

```rust
// General pattern using Result
match file.read_next(&mut record) {
    Ok(()) => { /* NOT AT END / NOT INVALID KEY */ }
    Err(FileError::AtEnd) => { /* AT END */ }
    Err(FileError::InvalidKey) => { /* INVALID KEY */ }
    Err(e) => { /* Other errors -- check FILE STATUS */ }
}
```

---

## 13. STRING / UNSTRING Control Flow

### 13.1 STRING Statement

```
STRING {identifier-1 | literal-1} ...
   DELIMITED BY {identifier-2 | literal-2 | SIZE}
   ...
   INTO identifier-3
   [WITH POINTER identifier-4]
   [ON OVERFLOW imperative-statement-1]
   [NOT ON OVERFLOW imperative-statement-2]
[END-STRING]
```

**ON OVERFLOW**: Triggers when the receiving field is too small (pointer exceeds field length).

### 13.2 UNSTRING Statement

```
UNSTRING identifier-1
   DELIMITED BY [ALL] {identifier-2 | literal-1}
      [OR [ALL] {identifier-3 | literal-2}] ...
   INTO {identifier-4 [DELIMITER IN identifier-5]
         [COUNT IN identifier-6]} ...
   [WITH POINTER identifier-7]
   [TALLYING IN identifier-8]
   [ON OVERFLOW imperative-statement-1]
   [NOT ON OVERFLOW imperative-statement-2]
[END-UNSTRING]
```

**ON OVERFLOW for UNSTRING**: Triggers when all INTO identifiers are filled and data remains, or the POINTER value exceeds the source field length.

### 13.3 Rust Mapping

```rust
match cobol_unstring(&source, &delimiters, &mut targets, &mut ptr, &mut tally) {
    Ok(()) => { /* NOT ON OVERFLOW */ }
    Err(StringOverflow) => { /* ON OVERFLOW */ }
}
```

---

## 14. SORT / MERGE Control Flow

### 14.1 SORT with INPUT/OUTPUT PROCEDURE

```cobol
SORT SORT-FILE
   ON ASCENDING KEY SORT-KEY
   INPUT PROCEDURE IS INPUT-SECTION
   OUTPUT PROCEDURE IS OUTPUT-SECTION.
```

### 14.2 Control Flow Semantics

1. The SORT statement begins execution
2. The INPUT PROCEDURE section is invoked as if by an implicit PERFORM
3. Inside INPUT PROCEDURE, RELEASE sends records to the sort utility
4. When INPUT PROCEDURE ends, the sort utility sorts all released records
5. The OUTPUT PROCEDURE section is invoked as if by an implicit PERFORM
6. Inside OUTPUT PROCEDURE, RETURN retrieves sorted records one at a time
7. When OUTPUT PROCEDURE ends, the SORT statement is complete

**Key constraints**:
- RELEASE is only valid inside INPUT PROCEDURE
- RETURN is only valid inside OUTPUT PROCEDURE
- You CANNOT use GO TO to jump out of an INPUT/OUTPUT PROCEDURE
- You CAN PERFORM other paragraphs from within INPUT/OUTPUT PROCEDURE
- The sort utility acts as a coroutine-like intermediary

### 14.3 INPUT PROCEDURE Example

```cobol
INPUT-SECTION SECTION.
INPUT-PARA.
   OPEN INPUT INPUT-FILE.
   PERFORM UNTIL WS-EOF = "Y"
      READ INPUT-FILE INTO WS-RECORD
         AT END SET WS-EOF TO TRUE
      END-READ
      IF WS-EOF NOT = "Y"
         IF WS-RECORD-TYPE = "A"
            MOVE WS-RECORD TO SORT-RECORD
            RELEASE SORT-RECORD
         END-IF
      END-IF
   END-PERFORM.
   CLOSE INPUT-FILE.
```

### 14.4 OUTPUT PROCEDURE Example

```cobol
OUTPUT-SECTION SECTION.
OUTPUT-PARA.
   OPEN OUTPUT OUTPUT-FILE.
   PERFORM UNTIL WS-SORT-EOF = "Y"
      RETURN SORT-FILE INTO WS-RECORD
         AT END SET WS-SORT-EOF TO TRUE
      END-RETURN
      IF WS-SORT-EOF NOT = "Y"
         WRITE OUTPUT-RECORD FROM WS-RECORD
      END-IF
   END-PERFORM.
   CLOSE OUTPUT-FILE.
```

### 14.5 USING/GIVING (Simple Alternative)

```cobol
SORT SORT-FILE
   ON ASCENDING KEY SORT-KEY
   USING INPUT-FILE
   GIVING OUTPUT-FILE.
```

Runtime handles all I/O. No INPUT/OUTPUT PROCEDURE needed.

### 14.6 MERGE

```cobol
MERGE SORT-FILE
   ON ASCENDING KEY SORT-KEY
   USING FILE-1 FILE-2 FILE-3
   OUTPUT PROCEDURE IS OUTPUT-SECTION.
```

MERGE combines multiple sorted files. Does NOT support INPUT PROCEDURE (input files must already be sorted).

### 14.7 Rust Mapping

```rust
// SORT with INPUT/OUTPUT PROCEDURE
let mut records: Vec<SortRecord> = Vec::new();

// INPUT PROCEDURE: fill the vec
input_procedure(&mut records, &input_file);

// SORT
records.sort_by_key(|r| r.sort_key.clone());

// OUTPUT PROCEDURE: process sorted records
output_procedure(&records, &output_file);
```

For large datasets, use iterators and external sort libraries.

---

## 15. DECLARATIVES Section

### 15.1 Structure

```cobol
PROCEDURE DIVISION.
DECLARATIVES.

INPUT-ERROR SECTION.
   USE AFTER STANDARD ERROR PROCEDURE ON INPUT-FILE.
INPUT-ERROR-PARA.
   DISPLAY "Error reading INPUT-FILE".
   DISPLAY "File status: " WS-FILE-STATUS.

OUTPUT-ERROR SECTION.
   USE AFTER STANDARD ERROR PROCEDURE ON OUTPUT-FILE.
OUTPUT-ERROR-PARA.
   DISPLAY "Error writing OUTPUT-FILE".
   STOP RUN.

END DECLARATIVES.

MAIN-SECTION SECTION.
   ... normal code ...
```

### 15.2 Semantics

Declaratives define exception handlers invoked **implicitly** by the runtime when specified conditions occur. They are NOT called explicitly by the programmer.

**USE AFTER ERROR/EXCEPTION PROCEDURE ON file-name**: Invoked automatically when an I/O error occurs on the specified file, but ONLY if the I/O statement does NOT have its own AT END / INVALID KEY / ON EXCEPTION phrase.

**USE BEFORE REPORTING identifier**: Invoked before a REPORT WRITER report group is generated. Allows modification of data before printing.

### 15.3 Scope Constraints

- Declaratives must appear at the beginning of the Procedure Division
- Each declarative section must begin with a USE statement
- Code in declaratives cannot reference non-declarative sections (no PERFORM of regular paragraphs)
- Regular code cannot PERFORM declarative sections
- A declarative can PERFORM other paragraphs within the same declarative section

### 15.4 COBOL 2002+ Exception Conditions

Predefined exception conditions (EC- prefix):

| Condition              | Meaning                    |
|------------------------|----------------------------|
| EC-SIZE-OVERFLOW       | Arithmetic overflow         |
| EC-SIZE-ZERO-DIVIDE    | Division by zero            |
| EC-PROGRAM-NOT-FOUND   | CALL target not found       |
| EC-I-O                 | I/O errors (with sub-conditions) |
| EC-DATA                | Data errors                 |
| EC-RANGE               | Subscript/index out of range |

### 15.5 Rust Mapping

Declaratives map to error handling patterns:
- USE AFTER ERROR -> `Result<>` error handling or registered error callback
- USE BEFORE REPORTING -> Pre-processing hook

---

## 16. Dialect-Specific Extensions

### 16.1 IBM Enterprise COBOL

#### XML PARSE

```cobol
XML PARSE identifier-1
   PROCESSING PROCEDURE IS procedure-name-1 [THRU procedure-name-2]
   [ON EXCEPTION imperative-statement-1]
   [NOT ON EXCEPTION imperative-statement-2]
[END-XML]
```

The processing procedure is invoked as a callback for each XML event. Special registers XML-EVENT, XML-TEXT, XML-NTEXT, XML-CODE are set before each invocation. Setting XML-CODE to -1 stops parsing early.

#### JSON GENERATE / JSON PARSE (IBM COBOL V6.2+)

```cobol
JSON GENERATE identifier-1 FROM identifier-2
   [COUNT IN identifier-3]
   [ON EXCEPTION imperative-statement-1]
   [NOT ON EXCEPTION imperative-statement-2]
[END-JSON]

JSON PARSE identifier-1 INTO identifier-2
   [WITH DETAIL]
   [ON EXCEPTION imperative-statement-1]
   [NOT ON EXCEPTION imperative-statement-2]
[END-JSON]
```

#### EXEC SQL Control Flow

```cobol
EXEC SQL
   SELECT CUST_NAME INTO :WS-CUST-NAME
   FROM CUSTOMERS WHERE CUST_ID = :WS-CUST-ID
END-EXEC.

IF SQLCODE = 0
   PERFORM PROCESS-CUSTOMER
ELSE IF SQLCODE = 100
   DISPLAY "NOT FOUND"
ELSE
   DISPLAY "SQL ERROR: " SQLCODE
END-IF.
```

SQLCODE: 0 = success, 100 = not found, negative = error.

#### EXEC CICS Control Flow

```cobol
EXEC CICS READ
   FILE("CUSTFILE") INTO(WS-CUSTOMER)
   RIDFLD(WS-CUST-ID) RESP(WS-RESP) RESP2(WS-RESP2)
END-EXEC.

EVALUATE WS-RESP
   WHEN DFHRESP(NORMAL)    PERFORM PROCESS-CUSTOMER
   WHEN DFHRESP(NOTFND)    PERFORM CUSTOMER-NOT-FOUND
   WHEN OTHER               PERFORM CICS-ERROR
END-EVALUATE.
```

### 16.2 GnuCOBOL Extensions

#### Compiler Directives (Compile-Time Conditionals)

```cobol
>>IF P-DIALECT = "IBM"
   MOVE 1 TO WS-MODE.
>>ELSE
   MOVE 2 TO WS-MODE.
>>END-IF

>>EVALUATE TRUE
>>WHEN P-OS = "LINUX"
   * Linux-specific code
>>WHEN P-OS = "WINDOWS"
   * Windows-specific code
>>END-EVALUATE
```

Not runtime control flow -- compile-time conditionals (like C preprocessor #if). The transpiler must handle them for correct source parsing.

#### RAISE and RESUME (Exception Handling)

GnuCOBOL 3.x supports COBOL 2002 exception handling:

```cobol
RAISE EXCEPTION EC-SIZE-OVERFLOW.
```

#### Extended ACCEPT

```cobol
ACCEPT WS-INPUT FROM CONSOLE
   ON EXCEPTION    PERFORM HANDLE-ACCEPT-ERROR
   NOT ON EXCEPTION PERFORM PROCESS-INPUT
END-ACCEPT.
```

### 16.3 Micro Focus Extensions

#### TRY / CATCH (Managed COBOL for .NET/JVM)

```cobol
TRY
   CALL "RISKY-OPERATION" USING WS-DATA
CATCH exception-object
   DISPLAY "Error: " exception-object
FINALLY
   PERFORM CLEANUP
END-TRY.
```

Not standard COBOL. Only for .NET/JVM targets. Not relevant for mainframe COBOL migration.

#### OO Extensions

Micro Focus supports delegates, events, classes, methods, and interfaces for .NET/JVM interop.

### 16.4 COBOL 2002/2014 Standard Additions

#### RAISE Statement

```
RAISE [EXCEPTION] exception-name-1
RAISE [EXCEPTION] identifier-1
```

#### RESUME Statement

```
RESUME [AT] [NEXT STATEMENT]
RESUME [AT] procedure-name-1
```

Used within an exception handler to specify where execution resumes.

#### User-Defined Functions (COBOL 2002)

```cobol
FUNCTION-ID. MY-MAX.
PROCEDURE DIVISION USING BY VALUE NUM-1 NUM-2
   RETURNING MAX-VAL.
   IF NUM-1 > NUM-2
      MOVE NUM-1 TO MAX-VAL
   ELSE
      MOVE NUM-2 TO MAX-VAL
   END-IF.
END FUNCTION MY-MAX.
```

---

## 17. Rust Mapping Strategy

### 17.1 Control Flow Mapping Matrix

| COBOL Construct                | Primary Rust Mapping                  | Alternative                  |
|--------------------------------|---------------------------------------|------------------------------|
| IF / ELSE / END-IF            | `if / else`                           | --                           |
| EVALUATE identifier           | `match`                               | if-else chain                |
| EVALUATE TRUE                 | if-else chain                         | `match` on bool tuple        |
| EVALUATE with ALSO            | `match` on tuple                      | if-else chain                |
| EVALUATE with THRU            | `match` with range patterns           | if-else with range checks    |
| PERFORM paragraph             | function call                         | --                           |
| PERFORM THRU                  | sequential function calls             | merged function              |
| PERFORM TIMES                 | `for _ in 0..n`                       | --                           |
| PERFORM UNTIL (TEST BEFORE)   | `while !cond`                         | --                           |
| PERFORM UNTIL (TEST AFTER)    | `loop { ...; if cond { break; } }`    | do-while pattern             |
| PERFORM VARYING (BEFORE)      | `while` loop with manual increment    | --                           |
| PERFORM VARYING (AFTER)       | `loop` with manual increment + break  | --                           |
| PERFORM VARYING AFTER         | nested loops                          | --                           |
| EXIT PERFORM                  | `break`                               | --                           |
| EXIT PERFORM CYCLE            | `continue`                            | --                           |
| EXIT PARAGRAPH                | `return` from paragraph fn            | if-else restructure          |
| EXIT SECTION                  | `return` from section fn              | --                           |
| EXIT PROGRAM                  | `return`                              | --                           |
| GO TO                         | restructure to if/else/loop           | state machine                |
| GO TO DEPENDING ON            | `match`                               | --                           |
| ALTER                         | function pointer / enum state         | HashMap dispatch             |
| STOP RUN                      | `std::process::exit()`                | return from main             |
| GOBACK                        | `return`                              | `std::process::exit()`       |
| CONTINUE                      | `{}` (empty block)                    | no-op                        |
| NEXT SENTENCE                 | labeled block `break 'sentence`       | restructure                  |
| CALL BY REFERENCE             | fn call with `&mut`                   | --                           |
| CALL BY CONTENT               | fn call with `.clone()`               | --                           |
| CALL BY VALUE                 | fn call with value                    | --                           |
| ON EXCEPTION / AT END         | `match` on `Result`                   | if-else on status            |
| 88-level condition             | predicate method                      | enum variant check           |
| Class condition               | predicate method                      | --                           |
| Abbreviated conditions        | expanded boolean expression           | --                           |
| DECLARATIVES USE AFTER ERROR  | error callback / Result handler       | --                           |
| RAISE / RESUME                | `Result::Err` / error propagation     | panic / catch_unwind         |
| SORT INPUT/OUTPUT PROCEDURE   | Vec collect -> sort -> iterate        | external sort library        |

### 17.2 Key Transpiler Challenges

1. **GO TO elimination**: Most difficult transformation. Requires control flow graph analysis to convert unstructured GO TO patterns to structured Rust code. The "relooper" algorithm (from Emscripten) can be adapted.

2. **PERFORM THRU with GO TO**: The interaction between PERFORM ranges and GO TO (especially GO TO outside the range) creates complex control flow that may require state machines.

3. **NEXT SENTENCE**: Requires tracking sentence boundaries and converting to structured flow (labeled blocks or restructured code).

4. **Paragraph fall-through**: COBOL's default sequential execution through paragraphs has no Rust equivalent. Must be made explicit in generated code.

5. **88-level conditions with SET**: SET condition-name TO TRUE/FALSE modifies the parent field. The transpiler must track the mapping from condition-names to their parent fields and values.

6. **Abbreviated combined conditions**: Must be fully expanded during parsing. The expansion algorithm must correctly handle implied subjects, implied operators, and NOT distribution.

7. **PERFORM stack**: COBOL maintains a PERFORM return stack. Recursive or overlapping PERFORMs cause stack issues. The transpiler should detect these patterns and warn.

8. **ALTER statement**: Requires dynamic dispatch in Rust. Rare but must be handled for legacy code.

9. **DECLARATIVES**: Automatic error handlers require a callback or event-driven architecture in Rust.

10. **Non-short-circuit evaluation**: IBM COBOL evaluates both sides of AND/OR. Rust always short-circuits. The transpiler must pre-evaluate conditions into temporaries when side effects or safety guards are involved.

11. **Mixed scope terminators**: Legacy code mixing period-delimited and END-IF/END-EVALUATE scope terminators in the same program.

12. **CALL aliasing**: Passing the same variable BY REFERENCE to multiple parameters violates Rust's borrow rules. Requires unsafe code or copying.

### 17.3 Recommended Transpilation Pipeline

```
1. PARSE
   Build AST preserving all control flow constructs.
   Resolve sentence boundaries (period locations).

2. EXPAND
   Expand abbreviated conditions.
   Resolve 88-level conditions to explicit comparisons.

3. NORMALIZE
   Convert NEXT SENTENCE to structured form (labeled blocks or GO TO).
   Convert period-delimited IF to END-IF form.
   Standardize scope terminators.

4. ANALYZE
   Build control flow graph (CFG).
   Identify PERFORM ranges, GO TO targets, paragraph call graph.
   Detect overlapping PERFORMs and recursive patterns.
   Map paragraph/section fall-through chains.

5. STRUCTURE
   Apply GO TO elimination algorithms (relooper/Stackifier).
   Convert unstructured flow to structured constructs.
   Merge simple PERFORM THRU ranges into single functions.

6. GENERATE
   Emit Rust code using the mapping matrix (Section 17.1).
   Handle parameter passing modes (BY REF/CONTENT/VALUE).
   Implement PERFORM VARYING with correct final-value semantics.
   Generate non-short-circuit condition evaluation for IBM source.

7. VERIFY
   Ensure generated Rust compiles.
   Run control flow equivalence tests against COBOL test cases.
```

### 17.4 Architecture Recommendation: State Machine Core

For programs with complex GO TO patterns, use a state machine as the core execution model:

```rust
enum ParagraphId {
    ParaA,
    ParaB,
    ParaC,
    // ...
}

struct PerformStackEntry {
    return_to: ParagraphId,
    range_end: ParagraphId,
}

let mut perform_stack: Vec<PerformStackEntry> = Vec::new();
let mut current = ParagraphId::ParaA;

loop {
    match current {
        ParagraphId::ParaA => {
            // paragraph A code
            current = ParagraphId::ParaB; // fall-through
        }
        ParagraphId::ParaB => {
            // paragraph B code
            current = check_perform_return(
                &mut perform_stack,
                ParagraphId::ParaB,
                ParagraphId::ParaC,  // natural next
            );
        }
        // ...
    }
}

fn check_perform_return(
    stack: &mut Vec<PerformStackEntry>,
    current_para: ParagraphId,
    natural_next: ParagraphId,
) -> ParagraphId {
    if let Some(entry) = stack.last() {
        if entry.range_end == current_para {
            let entry = stack.pop().unwrap();
            return entry.return_to;
        }
    }
    natural_next
}
```

For well-structured programs (no GO TO, no overlapping PERFORMs), the simpler function-per-paragraph approach is preferred.

---

## 18. Appendices

### Appendix A: Condition Evaluation Order

COBOL evaluates conditions left-to-right, but short-circuit behavior is implementation-defined:

| Dialect              | Short-Circuit? | Notes                                    |
|----------------------|----------------|------------------------------------------|
| IBM Enterprise COBOL | No             | Both sides always evaluated              |
| GnuCOBOL            | Configurable   | COB_SHORT_CIRCUIT runtime setting        |
| Micro Focus          | Mode-dependent | Some compilation modes support it        |

**Transpiler implication**: Since Rust always short-circuits `&&` and `||`, the transpiler must ensure migrated code does not depend on both-sides evaluation (side effects, safety guards against null/zero).

### Appendix B: Collating Sequence Impact on Conditions

The collating sequence affects all alphanumeric comparisons, class conditions (ALPHABETIC), and EVALUATE THRU ranges with alphanumeric values.

| Encoding | Character Order              | Space Sorts |
|----------|------------------------------|-------------|
| EBCDIC   | lowercase < uppercase < digits | Low (x'40') |
| ASCII    | digits < uppercase < lowercase | Low (x'20') |

This can be overridden by PROGRAM COLLATING SEQUENCE in OBJECT-COMPUTER or ALPHABET clause in SPECIAL-NAMES.

**Transpiler must either**:
1. Use EBCDIC collating sequence for IBM-sourced code
2. Convert to ASCII-equivalent comparisons
3. Provide a configurable collating sequence in the runtime

### Appendix C: CORRESPONDING Phrase

CORRESPONDING is used with ADD, SUBTRACT, and MOVE but there is NO corresponding facility for IF or EVALUATE. Each field must be compared individually.

### Appendix D: Implementation Priority

**Phase 1 -- Core Control Flow**:
- IF / ELSE / END-IF
- EVALUATE (all formats)
- PERFORM (all formats, inline and out-of-line)
- EXIT PERFORM / EXIT PERFORM CYCLE
- CONTINUE
- CALL (BY REFERENCE, static)
- File I/O conditional phrases (AT END, INVALID KEY)
- Condition types (relation, sign, 88-level)

**Phase 2 -- Advanced Control Flow**:
- GO TO elimination (simple patterns)
- NEXT SENTENCE resolution
- PERFORM THRU with GO TO interaction
- CALL (BY CONTENT, BY VALUE, dynamic)
- STRING / UNSTRING overflow handling
- SORT / MERGE with INPUT/OUTPUT PROCEDURE
- DECLARATIVES
- Abbreviated combined conditions expansion

**Phase 3 -- Legacy and Dialect-Specific**:
- ALTER statement
- Complex GO TO patterns (state machine generation)
- Overlapping PERFORM detection
- EXEC SQL / EXEC CICS (IBM-specific)
- XML PARSE / JSON PARSE (IBM-specific)
- Compiler directives (GnuCOBOL >>IF)
- RAISE / RESUME (COBOL 2002+)
- TRY / CATCH (Micro Focus managed COBOL)

### Appendix E: Dialect Comparison Matrix

```
Feature                | IBM Enterprise     | GnuCOBOL          | Micro Focus
-----------------------|--------------------|--------------------|-------------------
Platform               | z/OS (mainframe)   | UNIX/Linux/Windows | Cross-platform
Native encoding        | EBCDIC             | ASCII              | Platform-dependent
Short-circuit eval     | No                 | Configurable       | Mode-dependent
Overlapping PERFORM    | Defined (LIFO)     | Implementation-def | Similar to IBM
RECURSIVE support      | With THREAD opt    | With attribute     | With attribute
ALTER support          | Yes (deprecated)   | Yes                | Yes (deprecated)
EXIT PERFORM           | Yes                | Yes                | Yes
EXIT PARAGRAPH         | Yes                | Yes                | Yes
EXEC SQL               | DB2 precompiler    | ESQL/OC            | Multiple DBs
OO extensions          | Limited            | Limited            | Full
Managed runtime        | No                 | No                 | JVM/.NET optional
SCREEN SECTION         | No                 | Yes                | Yes
XML/JSON PARSE         | Yes (native)       | Limited            | Yes
CALL to C/system       | Via LE conventions | Native (compiles C)| Via calling conv.
```

---

*Document version: 1.0*
*Target: cobol2rust transpiler control flow code generation engine*
*Primary dialect: IBM Enterprise COBOL*
*Secondary dialects: GnuCOBOL, Micro Focus*
