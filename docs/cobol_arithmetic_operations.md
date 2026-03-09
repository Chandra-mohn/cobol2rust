# COBOL Arithmetic Operations: Complete Cross-Dialect Reference

## Purpose

This document catalogs every arithmetic operation, math function, and
computation semantic across all major COBOL dialects. It serves as the
specification for implementing the arithmetic engine in the cobol2rust
transpiler runtime library.

---

## Table of Contents

1. [Arithmetic Verbs](#1-arithmetic-verbs)
   - 1.1 [ADD](#11-add)
   - 1.2 [SUBTRACT](#12-subtract)
   - 1.3 [MULTIPLY](#13-multiply)
   - 1.4 [DIVIDE](#14-divide)
   - 1.5 [COMPUTE](#15-compute)
2. [ROUNDED Phrase and Rounding Modes](#2-rounded-phrase-and-rounding-modes)
3. [ON SIZE ERROR / Overflow Handling](#3-on-size-error--overflow-handling)
4. [Intermediate Result Rules](#4-intermediate-result-rules)
   - 4.1 [IBM ARITH(COMPAT) -- 18-Digit Mode](#41-ibm-arithcompat----18-digit-mode)
   - 4.2 [IBM ARITH(EXTEND) -- 31-Digit Mode](#42-ibm-arithextend----31-digit-mode)
   - 4.3 [GnuCOBOL Intermediate Precision](#43-gnucobol-intermediate-precision)
   - 4.4 [Micro Focus Arithmetic Modes](#44-micro-focus-arithmetic-modes)
5. [Arithmetic Semantics](#5-arithmetic-semantics)
   - 5.1 [Fixed-Point Decimal Model](#51-fixed-point-decimal-model)
   - 5.2 [Left-Truncation (High-Order Truncation)](#52-left-truncation-high-order-truncation)
   - 5.3 [Decimal Alignment](#53-decimal-alignment)
   - 5.4 [Sign Handling in Arithmetic](#54-sign-handling-in-arithmetic)
   - 5.5 [Division Semantics and REMAINDER](#55-division-semantics-and-remainder)
   - 5.6 [Exponentiation Semantics](#56-exponentiation-semantics)
   - 5.7 [CORRESPONDING Arithmetic](#57-corresponding-arithmetic)
   - 5.8 [Arithmetic with GROUP Items](#58-arithmetic-with-group-items)
   - 5.9 [Arithmetic with Edited Fields](#59-arithmetic-with-edited-fields)
   - 5.10 [USAGE Effect on Arithmetic](#510-usage-effect-on-arithmetic)
6. [Intrinsic Math Functions](#6-intrinsic-math-functions)
   - 6.1 [Absolute Value and Rounding](#61-absolute-value-and-rounding)
   - 6.2 [Exponentiation and Roots](#62-exponentiation-and-roots)
   - 6.3 [Logarithmic Functions](#63-logarithmic-functions)
   - 6.4 [Trigonometric Functions](#64-trigonometric-functions)
   - 6.5 [Financial Functions](#65-financial-functions)
   - 6.6 [Statistical Functions](#66-statistical-functions)
   - 6.7 [Numeric Conversion Functions](#67-numeric-conversion-functions)
   - 6.8 [Other Numeric Functions](#68-other-numeric-functions)
7. [Operator Precedence in COMPUTE](#7-operator-precedence-in-compute)
8. [Dialect-Specific Compiler Options](#8-dialect-specific-compiler-options)
   - 8.1 [IBM Enterprise COBOL](#81-ibm-enterprise-cobol)
   - 8.2 [GnuCOBOL](#82-gnucobol)
   - 8.3 [Micro Focus Visual COBOL](#83-micro-focus-visual-cobol)
   - 8.4 [Fujitsu NetCOBOL](#84-fujitsu-netcobol)
   - 8.5 [COBOL 2014 Standard Additions](#85-cobol-2014-standard-additions)
9. [Cross-Dialect Compatibility Matrix](#9-cross-dialect-compatibility-matrix)
10. [Transpiler Implementation Notes](#10-transpiler-implementation-notes)

---

## 1. Arithmetic Verbs

All five arithmetic verbs share common phrases: ROUNDED, ON SIZE ERROR,
NOT ON SIZE ERROR, and END-verb scope terminators. These are documented
once in Sections 2 and 3, then referenced from each verb.

### 1.1 ADD

**Format 1 -- ADD TO**
```cobol
ADD {identifier-1 | literal-1} ...
    TO {identifier-2 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-ADD]
```

Semantics: All operands before TO are summed together, then added to each
identifier after TO. Each receiving identifier is updated independently.

Example:
```cobol
ADD A B C TO D E
*> D = D + A + B + C
*> E = E + A + B + C
```

**Format 2 -- ADD GIVING**
```cobol
ADD {identifier-1 | literal-1} ...
    TO {identifier-2 | literal-2}
    GIVING {identifier-3 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-ADD]
```

Semantics: All operands (including the one after TO) are summed. The result
is stored in each identifier after GIVING. Source operands are unchanged.

Example:
```cobol
ADD A B TO C GIVING D E
*> D = A + B + C
*> E = A + B + C
*> A, B, C are unchanged
```

**Format 3 -- ADD CORRESPONDING**
```cobol
ADD CORRESPONDING identifier-1
    TO identifier-2 [ROUNDED [MODE IS mode]]
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-ADD]
```

Semantics: For each elementary numeric item in identifier-1 that has a
matching name in identifier-2, performs ADD. See Section 5.7 for matching
rules.

Dialect support: All major dialects.

### 1.2 SUBTRACT

**Format 1 -- SUBTRACT FROM**
```cobol
SUBTRACT {identifier-1 | literal-1} ...
    FROM {identifier-2 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-SUBTRACT]
```

Semantics: All operands before FROM are summed, then subtracted from each
identifier after FROM.

Example:
```cobol
SUBTRACT A B FROM C D
*> C = C - (A + B)
*> D = D - (A + B)
```

**Format 2 -- SUBTRACT GIVING**
```cobol
SUBTRACT {identifier-1 | literal-1} ...
    FROM {identifier-2 | literal-2}
    GIVING {identifier-3 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-SUBTRACT]
```

**Format 3 -- SUBTRACT CORRESPONDING**
```cobol
SUBTRACT CORRESPONDING identifier-1
    FROM identifier-2 [ROUNDED [MODE IS mode]]
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-SUBTRACT]
```

### 1.3 MULTIPLY

**Format 1 -- MULTIPLY BY**
```cobol
MULTIPLY {identifier-1 | literal-1}
    BY {identifier-2 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-MULTIPLY]
```

Semantics: identifier-1 is multiplied by each identifier-2. Each
identifier-2 is updated with the product.

Example:
```cobol
MULTIPLY A BY B C
*> B = A * B
*> C = A * C
```

**Format 2 -- MULTIPLY GIVING**
```cobol
MULTIPLY {identifier-1 | literal-1}
    BY {identifier-2 | literal-2}
    GIVING {identifier-3 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-MULTIPLY]
```

Note: MULTIPLY has no CORRESPONDING format.

### 1.4 DIVIDE

DIVIDE has 5 formats -- the most of any arithmetic verb.

**Format 1 -- DIVIDE INTO**
```cobol
DIVIDE {identifier-1 | literal-1}
    INTO {identifier-2 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-DIVIDE]
```

Semantics: Each identifier-2 is divided BY identifier-1 (the divisor).

Example:
```cobol
DIVIDE 2 INTO A B
*> A = A / 2
*> B = B / 2
```

**Format 2 -- DIVIDE INTO GIVING**
```cobol
DIVIDE {identifier-1 | literal-1}
    INTO {identifier-2 | literal-2}
    GIVING {identifier-3 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-DIVIDE]
```

Semantics: identifier-2 / identifier-1 -> identifier-3.

**Format 3 -- DIVIDE BY GIVING**
```cobol
DIVIDE {identifier-1 | literal-1}
    BY {identifier-2 | literal-2}
    GIVING {identifier-3 [ROUNDED [MODE IS mode]]} ...
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-DIVIDE]
```

Semantics: identifier-1 / identifier-2 -> identifier-3.

**Format 4 -- DIVIDE INTO GIVING REMAINDER**
```cobol
DIVIDE {identifier-1 | literal-1}
    INTO {identifier-2 | literal-2}
    GIVING identifier-3 [ROUNDED [MODE IS mode]]
    REMAINDER identifier-4
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-DIVIDE]
```

**Format 5 -- DIVIDE BY GIVING REMAINDER**
```cobol
DIVIDE {identifier-1 | literal-1}
    BY {identifier-2 | literal-2}
    GIVING identifier-3 [ROUNDED [MODE IS mode]]
    REMAINDER identifier-4
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-DIVIDE]
```

CRITICAL: REMAINDER semantics are non-trivial. See Section 5.5.

Note: DIVIDE has no CORRESPONDING format. When REMAINDER is specified,
only one GIVING identifier is allowed.

### 1.5 COMPUTE

```cobol
COMPUTE {identifier-1 [ROUNDED [MODE IS mode]]} ...
    = arithmetic-expression
    [ON SIZE ERROR imperative-statement-1]
    [NOT ON SIZE ERROR imperative-statement-2]
    [END-COMPUTE]
```

Arithmetic expression operators:
```
+    Addition
-    Subtraction (binary) or Negation (unary)
*    Multiplication
/    Division
**   Exponentiation
( )  Grouping
```

Semantics: The expression is evaluated according to operator precedence
(Section 7), then the result is stored in each identifier-1. Multiple
receiving identifiers each get the same computed value, independently
rounded/truncated to their respective PIC definitions.

COMPUTE is the most flexible arithmetic statement. Any operation expressible
with ADD/SUBTRACT/MULTIPLY/DIVIDE can also be written as COMPUTE, plus
it supports exponentiation and complex expressions.

Example:
```cobol
COMPUTE TAX ROUNDED = INCOME * RATE / 100 + SURCHARGE
COMPUTE A B C = X ** 2 + Y ** 2
```

---

## 2. ROUNDED Phrase and Rounding Modes

### Default Behavior (No ROUNDED)

When ROUNDED is not specified, the result is **truncated** toward zero.
Decimal places beyond the receiving field's scale are simply dropped.

```cobol
01 RESULT PIC 9V9.
COMPUTE RESULT = 3.75    *> RESULT = 3.7 (truncated, not 3.8)
COMPUTE RESULT = -3.75   *> RESULT = -3.7 (truncated toward zero)
```

### Basic ROUNDED (No MODE)

When ROUNDED is specified without MODE, the default rounding mode is
**NEAREST-AWAY-FROM-ZERO** (also called "commercial rounding" or
"round half away from zero").

```cobol
01 RESULT PIC 9V9.
COMPUTE RESULT ROUNDED = 3.75    *> RESULT = 3.8
COMPUTE RESULT ROUNDED = 3.74    *> RESULT = 3.7
COMPUTE RESULT ROUNDED = -3.75   *> RESULT = -3.8
```

### Extended Rounding Modes (ROUNDED MODE IS)

IBM Enterprise COBOL v4+ and COBOL 2014 standard define 8 rounding modes:

| Mode | Keyword | Behavior |
|------|---------|----------|
| 1 | AWAY-FROM-ZERO | Always round away from zero (ceiling of absolute value) |
| 2 | NEAREST-AWAY-FROM-ZERO | Round to nearest; ties round away from zero (default ROUNDED) |
| 3 | NEAREST-EVEN | Round to nearest; ties round to even digit ("banker's rounding") |
| 4 | NEAREST-TOWARD-ZERO | Round to nearest; ties round toward zero |
| 5 | TOWARD-GREATER | Round toward positive infinity (ceiling) |
| 6 | TOWARD-LESSER | Round toward negative infinity (floor) |
| 7 | TRUNCATION | Truncate toward zero (same as no ROUNDED) |
| 8 | PROHIBITED | SIZE ERROR if any truncation would occur |

Examples with PIC 9V9 receiving field:

| Value | AWAY-FROM-ZERO | NEAREST-AWAY | NEAREST-EVEN | TOWARD-GREATER | TOWARD-LESSER |
|-------|---------------|--------------|--------------|----------------|---------------|
| 3.75 | 3.8 | 3.8 | 3.8 | 3.8 | 3.7 |
| 3.74 | 3.8 | 3.7 | 3.7 | 3.8 | 3.7 |
| 3.85 | 3.9 | 3.9 | 3.8 | 3.9 | 3.8 |
| 3.65 | 3.7 | 3.7 | 3.6 | 3.7 | 3.6 |
| -3.75 | -3.8 | -3.8 | -3.8 | -3.7 | -3.8 |

### Dialect Support for Extended Rounding

| Dialect | Basic ROUNDED | ROUNDED MODE IS |
|---------|--------------|-----------------|
| COBOL-85 | Yes | No |
| COBOL 2014 | Yes | Yes (all 8 modes) |
| IBM Enterprise COBOL v4+ | Yes | Yes (all 8 modes) |
| GnuCOBOL 3.x | Yes | Yes (all 8 modes) |
| Micro Focus | Yes | No |
| Fujitsu NetCOBOL | Yes | No |

For dialects without MODE IS, only the default NEAREST-AWAY-FROM-ZERO
is available through the ROUNDED keyword.

---

## 3. ON SIZE ERROR / Overflow Handling

### When SIZE ERROR Triggers

SIZE ERROR is raised when the **integer part** of the result exceeds the
receiving field's integer capacity. Specifically:

- The result (after rounding if ROUNDED) has more integer digits than
  the receiving field's PIC can hold.
- Division by zero always triggers SIZE ERROR.
- Exponentiation with invalid operands (0**0, 0**negative) triggers
  SIZE ERROR.

### What Happens When SIZE ERROR Occurs

1. The receiving field is **NOT modified** (retains its previous value).
2. The ON SIZE ERROR imperative statement executes.
3. If no ON SIZE ERROR phrase is present, the behavior depends on the
   dialect:
   - IBM: The result is undefined (implementation behavior).
   - GnuCOBOL: Silent left-truncation by default.
   - Standard: Undefined.

### What Happens Without SIZE ERROR (Silent Truncation)

When ON SIZE ERROR is NOT specified and overflow occurs:
- The result is **left-truncated** (high-order digits are dropped).
- No error, no warning, no exception.
- This is the COBOL default and is a major source of bugs.

```cobol
01 RESULT PIC 9(3).
COMPUTE RESULT = 12345
*> Without ON SIZE ERROR: RESULT = 345 (left-truncated)
*> With ON SIZE ERROR: RESULT unchanged, error handler runs
```

### SIZE ERROR with Multiple Receiving Fields

When multiple identifiers receive the same result, SIZE ERROR is evaluated
independently for each:

```cobol
01 SMALL-FIELD PIC 99.
01 BIG-FIELD   PIC 9(5).
COMPUTE SMALL-FIELD BIG-FIELD = 12345
    ON SIZE ERROR DISPLAY "OVERFLOW"
    NOT ON SIZE ERROR DISPLAY "OK"
END-COMPUTE.
*> SMALL-FIELD: SIZE ERROR (12345 > 99), field unchanged
*> BIG-FIELD: 12345 stored successfully
*> "OVERFLOW" is displayed (any SIZE ERROR triggers it)
```

### SIZE ERROR Detection for ROUNDED MODE PROHIBITED

When ROUNDED MODE IS PROHIBITED is specified, SIZE ERROR is raised if
ANY low-order truncation would occur (not just integer overflow):

```cobol
01 RESULT PIC 9V9.
COMPUTE RESULT ROUNDED MODE IS PROHIBITED = 3.75
*> SIZE ERROR: 3.75 cannot be stored exactly in PIC 9V9
```

---

## 4. Intermediate Result Rules

Intermediate results are the internal precision used during computation
of arithmetic expressions. This is one of the most dialect-dependent
aspects of COBOL arithmetic.

### 4.1 IBM ARITH(COMPAT) -- 18-Digit Mode

The "compatible" mode reproduces OS/VS COBOL and older IBM behavior.

**Rules for simple verbs (ADD, SUBTRACT, MULTIPLY, DIVIDE):**
- Intermediate result precision = max(digits of operands involved)
  plus enough digits to avoid loss.
- Maximum intermediate precision: 30 digits (but literals limited to 18).

**Rules for COMPUTE expressions:**
IBM defines a specific algorithm for intermediate result sizing:

1. For each sub-expression, compute the number of integer places (i)
   and decimal places (d) needed.
2. Addition/Subtraction: i = max(i1, i2) + 1, d = max(d1, d2)
3. Multiplication: i = i1 + i2, d = d1 + d2
4. Division: i = i1 + d2, d = implementation-dependent (large)
5. Exponentiation: If exponent is integer literal, compute exactly.
   Otherwise, use floating-point (64-bit long).

**Key limitation:** Numeric literals cannot exceed 18 digits. Intermediate
results can reach 30 digits but the final store is limited by the
receiving PIC.

### 4.2 IBM ARITH(EXTEND) -- 31-Digit Mode

The "extended" mode provides higher precision (default in IBM Ent. v6+).

- Numeric literals can be up to 31 digits.
- Intermediate fixed-point results up to 31 digits.
- Floating-point exponentiation uses 128-bit (extended precision).
- FUNCTION RANDOM produces a different (higher-quality) sequence.

**Intermediate sizing algorithm is the same conceptually**, but the
upper bound increases from 30 to 31 digits.

### 4.3 GnuCOBOL Intermediate Precision

GnuCOBOL uses the GMP (GNU Multiple Precision) library for intermediate
arithmetic. This means:

- Intermediate results have **arbitrary precision** (limited only by memory).
- No practical digit limit during computation.
- Final result is truncated/rounded to the receiving field's PIC.
- Numeric literals can be up to 38 digits.
- Configurable via: `-fdefault-arithmetic-precision=N`

This is a significant behavioral difference from IBM. A COMPUTE expression
that loses precision on IBM due to intermediate overflow will produce the
correct result on GnuCOBOL.

### 4.4 Micro Focus Arithmetic Modes

Micro Focus provides directives to switch arithmetic behavior:

| Directive | Behavior |
|-----------|----------|
| `ARITHMETIC"NATIVE"` | Native platform arithmetic |
| `ARITHMETIC"OS390COMPAT"` | Emulates IBM ARITH(COMPAT) |
| `ARITHMETIC"OS390EXTEND"` | Emulates IBM ARITH(EXTEND) |
| `ARITHMETIC"STANDARD"` | COBOL standard arithmetic |

When emulating IBM modes, intermediate result sizing follows the
corresponding IBM rules.

---

## 5. Arithmetic Semantics

### 5.1 Fixed-Point Decimal Model

COBOL uses **fixed-point decimal** arithmetic, not floating-point. Every
numeric field has a compile-time-known number of integer digits and
decimal digits defined by its PIC clause.

Key properties:
- **Exact decimal representation**: 0.1 is exactly 0.1, not an
  approximation (unlike IEEE 754 binary float).
- **Compile-time scale**: The position of the decimal point is fixed
  at compile time by the V in the PIC clause.
- **No dynamic precision**: A field cannot hold more digits than its
  PIC allows.
- **Alignment on decimal point**: All storage, comparison, and
  arithmetic operations align on the implied decimal point.

The computation model is:
1. Convert operands to a common intermediate representation.
2. Perform the arithmetic operation with sufficient intermediate precision.
3. Store the result into the receiving field by:
   a. Aligning on the decimal point.
   b. Truncating (or rounding if ROUNDED) excess decimal digits.
   c. Left-truncating excess integer digits (if no SIZE ERROR clause).
   d. Padding with zeros where the receiving field has more positions.

### 5.2 Left-Truncation (High-Order Truncation)

This is the single most dangerous COBOL arithmetic behavior for
transpiler correctness.

When an arithmetic result has more integer digits than the receiving
field, COBOL **silently drops the leftmost (high-order) digits**:

```cobol
01 RESULT PIC 9(3).

COMPUTE RESULT = 12345       *> RESULT = 345
COMPUTE RESULT = 100000      *> RESULT = 000
ADD 1 TO RESULT              *> If RESULT=999, wraps to 000
MULTIPLY 999 BY RESULT       *> If RESULT=999, product=998001, stored as 001
```

This is NOT the same as modular arithmetic (though for unsigned fields
it acts like mod 10^n). For signed fields:

```cobol
01 RESULT PIC S9(3).
COMPUTE RESULT = -12345      *> RESULT = -345
COMPUTE RESULT = 12345       *> RESULT = 345 (sign from the result)
```

**When does left-truncation occur?**
- On any arithmetic verb result (ADD, SUBTRACT, MULTIPLY, DIVIDE, COMPUTE)
- On MOVE of a numeric value to a smaller field
- ONLY when ON SIZE ERROR is NOT specified

**When does it NOT occur?**
- When ON SIZE ERROR IS specified -- the receiving field is unchanged
- During intermediate computations (intermediates have sufficient precision)

### 5.3 Decimal Alignment

COBOL always aligns on the decimal point (V) when storing results.

Example:
```cobol
01 SOURCE   PIC 99V99   VALUE 12.34.
01 TARGET-1 PIC 9V9.
01 TARGET-2 PIC 9(4)V9(4).

MOVE SOURCE TO TARGET-1     *> TARGET-1 = 2.3
*> Integer part: 12 -> 2 (left-truncated to 1 digit)
*> Decimal part: 34 -> 3 (right-truncated to 1 digit)

MOVE SOURCE TO TARGET-2     *> TARGET-2 = 0012.3400
*> Integer part: 12 -> 0012 (left-padded with zeros to 4 digits)
*> Decimal part: 34 -> 3400 (right-padded with zeros to 4 digits)
```

Rules:
- Integer digits: **Left-truncated** if too many, **left-padded with 0** if too few
- Decimal digits: **Right-truncated** if too many (or rounded if ROUNDED),
  **right-padded with 0** if too few
- The decimal point (V) is the fixed alignment anchor

### 5.4 Sign Handling in Arithmetic

**Unsigned receiving fields:**
When a negative result is stored in an unsigned field (PIC 9 without S),
the absolute value is stored. The sign is discarded.

```cobol
01 UNSIGNED-RESULT PIC 9(3).
COMPUTE UNSIGNED-RESULT = -123   *> RESULT = 123 (absolute value)
SUBTRACT 200 FROM UNSIGNED-RESULT
*> UNSIGNED-RESULT was 123, result is -77
*> Stored as 077 (absolute value, left-truncated if needed)
```

**Sign storage options:**
```cobol
01 TRAILING-SIGN   PIC S9(3).          *> Default: sign embedded in last byte
01 LEADING-SIGN    PIC S9(3) SIGN IS LEADING.
01 SEPARATE-TRAIL  PIC S9(3) SIGN IS TRAILING SEPARATE CHARACTER.
01 SEPARATE-LEAD   PIC S9(3) SIGN IS LEADING SEPARATE CHARACTER.
```

SIGN IS ... SEPARATE CHARACTER adds one byte to the field for the sign
(+ or -). This affects storage size but NOT arithmetic behavior.

**IBM NUMPROC compiler option effect on signs:**

| Option | Behavior |
|--------|----------|
| NUMPROC(NOPFD) | Strict: validates sign nibbles, corrects non-preferred signs |
| NUMPROC(PFD) | Preferred: assumes signs are already preferred, no validation |
| NUMPROC(MIG) | Migration: treats unsigned fields as positive even if sign bits present |

### 5.5 Division Semantics and REMAINDER

**Basic division:**
COBOL division produces a quotient whose precision is determined by the
receiving field's PIC. If the exact result has more decimal places than
the receiving field allows, the excess is truncated (or rounded).

```cobol
01 QUOTIENT PIC 9V99.
DIVIDE 10 BY 3 GIVING QUOTIENT
*> Mathematical result: 3.33333...
*> Stored as 3.33 (truncated to PIC 9V99)
```

**REMAINDER computation -- CRITICAL for transpiler:**

The REMAINDER is NOT the mathematical remainder. It is computed as:

```
remainder = dividend - (quotient_as_stored * divisor)
```

Where `quotient_as_stored` is the quotient AFTER truncation/rounding
to the GIVING field's PIC. This means the REMAINDER depends on the
GIVING field's precision.

```cobol
01 QUOT PIC 99.
01 RMDR PIC 99.
DIVIDE 100 BY 7 GIVING QUOT REMAINDER RMDR
*> Mathematical: 100 / 7 = 14.2857...
*> QUOT = 14 (truncated to PIC 99, integer)
*> RMDR = 100 - (14 * 7) = 100 - 98 = 2
```

```cobol
01 QUOT PIC 99V9.
01 RMDR PIC 99V9.
DIVIDE 100 BY 7 GIVING QUOT REMAINDER RMDR
*> QUOT = 14.2 (truncated to PIC 99V9)
*> RMDR = 100 - (14.2 * 7) = 100 - 99.4 = 0.6
```

Note how different GIVING PICs produce different REMAINDERs.

**Division by zero:**
Always triggers SIZE ERROR. If no ON SIZE ERROR clause, the result is
undefined (implementation-dependent).

**Negative operands in division:**
- Sign of quotient follows standard math rules (negative / positive = negative)
- Sign of REMAINDER follows the sign of the dividend

### 5.6 Exponentiation Semantics

The `**` operator in COMPUTE expressions has special rules:

**Integer exponent (literal):**
When the exponent is a non-negative integer literal, the computation uses
fixed-point arithmetic with full intermediate precision.

```cobol
COMPUTE X = A ** 3    *> Fixed-point: A * A * A
```

**Variable or non-integer exponent:**
When the exponent is a variable or a non-integer literal, the computation
switches to **floating-point**:

```cobol
COMPUTE X = A ** B      *> Floating-point: uses f64 (ARITH(COMPAT)) or f128 (ARITH(EXTEND))
COMPUTE X = A ** 2.5    *> Floating-point: non-integer exponent
```

**Special cases:**

| Expression | Result | SIZE ERROR? |
|-----------|--------|-------------|
| 0 ** 0 | Undefined | Yes (SIZE ERROR) |
| 0 ** positive | 0 | No |
| 0 ** negative | Undefined | Yes (SIZE ERROR) |
| positive ** 0 | 1 | No |
| negative ** integer | Computed normally | No |
| negative ** non-integer | Undefined | Yes (SIZE ERROR) |

**Floating-point contamination:**
If ANY operand in a COMPUTE expression is COMP-1, COMP-2, or an
intrinsic function returning float, the ENTIRE expression evaluates
using floating-point arithmetic. The transpiler must detect this at
compile time.

### 5.7 CORRESPONDING Arithmetic

ADD CORRESPONDING and SUBTRACT CORRESPONDING match fields by name
between two GROUP items.

**Matching rules:**
1. Fields are matched by **data-name** (case-insensitive in standard COBOL).
2. Both the source and target field must be **elementary numeric** items.
3. The following are excluded from matching:
   - FILLER items
   - Non-numeric fields (PIC X, PIC A)
   - Group items themselves (only elementary items participate)
   - Fields with REDEFINES
   - Index items
4. The matching ignores level numbers -- only names matter.
5. Qualification is resolved: if ambiguous names exist, they must be
   qualified to participate.

Example:
```cobol
01 GROUP-A.
   05 AMOUNT      PIC 9(5)V99.
   05 TAX         PIC 9(3)V99.
   05 NAME        PIC X(20).     *> Excluded: not numeric
   05 FILLER      PIC 9(3).      *> Excluded: FILLER

01 GROUP-B.
   05 AMOUNT      PIC 9(7)V99.
   05 TAX         PIC 9(5)V99.
   05 DISCOUNT    PIC 9(3)V99.   *> No match in GROUP-A

ADD CORRESPONDING GROUP-A TO GROUP-B
*> Equivalent to:
*>   ADD AMOUNT OF GROUP-A TO AMOUNT OF GROUP-B
*>   ADD TAX OF GROUP-A TO TAX OF GROUP-B
*> DISCOUNT, NAME, and FILLER are not affected
```

### 5.8 Arithmetic with GROUP Items

GROUP items are treated as **alphanumeric** in most contexts. You
CANNOT use a GROUP item directly in arithmetic:

```cobol
01 MY-GROUP.
   05 FIELD-A PIC 9(3).
   05 FIELD-B PIC 9(3).

ADD 1 TO MY-GROUP              *> INVALID: MY-GROUP is alphanumeric
ADD 1 TO FIELD-A OF MY-GROUP   *> Valid: FIELD-A is elementary numeric
```

The exception is ADD/SUBTRACT CORRESPONDING, which operates on the
elementary items within the group (see Section 5.7).

### 5.9 Arithmetic with Edited Fields

**Numeric-edited fields (e.g., PIC Z(3)9.99, PIC $$$,$$9.99CR):**

| As source operand | As receiving operand |
|-------------------|---------------------|
| NOT allowed in arithmetic | NOT allowed in arithmetic |
| Can be used with NUMVAL/NUMVAL-C | Can receive MOVE from numeric |

Numeric-edited fields are for display purposes only. To do arithmetic
on a displayed value, use FUNCTION NUMVAL or FUNCTION NUMVAL-C to
convert it back to a numeric value.

```cobol
01 EDITED-AMT    PIC $$$,$$9.99.
01 WORK-AMT      PIC 9(7)V99.

MOVE WORK-AMT TO EDITED-AMT       *> Valid: numeric to edited
ADD 1 TO EDITED-AMT                *> INVALID
COMPUTE WORK-AMT = FUNCTION NUMVAL-C(EDITED-AMT)  *> Valid
```

**Alphanumeric-edited fields:**
Cannot participate in arithmetic at all.

### 5.10 USAGE Effect on Arithmetic

USAGE affects **storage representation** but NOT arithmetic semantics --
with important exceptions:

**When USAGE does NOT matter:**
- ADD, SUBTRACT, MULTIPLY, DIVIDE verb results are the same regardless
  of whether operands are DISPLAY, COMP-3, or COMP.
- The runtime converts operands to an intermediate representation,
  computes, then converts back.

**When USAGE DOES matter:**

1. **COMP/COMP-4 with TRUNC(STD)**: Binary fields are truncated to PIC
   limits after arithmetic, even though the binary storage could hold
   larger values. TRUNC(BIN) and TRUNC(OPT) change this behavior.

2. **COMP-1 / COMP-2**: These are floating-point. Arithmetic with
   float operands switches the entire expression to floating-point,
   changing precision behavior.

3. **COMP-5 / BINARY**: These use the full native binary range regardless
   of PIC. SIZE ERROR is based on the binary storage limit, not PIC.

4. **Performance**: COMP/COMP-5 arithmetic is faster on most platforms
   because the CPU can operate on binary directly. DISPLAY and COMP-3
   require conversion to/from decimal representation.

---

## 6. Intrinsic Math Functions

All intrinsic functions are invoked with the syntax:
```cobol
FUNCTION function-name(argument-1 [, argument-2 ...])
```

Functions can appear anywhere a numeric literal or identifier can appear
in arithmetic contexts.

### 6.1 Absolute Value and Rounding

**FUNCTION ABS(argument-1)**
- Returns: Absolute value of argument-1.
- Argument: Numeric.
- Result: Numeric (same type category as input).
- Dialects: All (COBOL-85+, IBM, GnuCOBOL, Micro Focus, Fujitsu)

**FUNCTION INTEGER(argument-1)**
- Returns: Greatest integer NOT greater than argument-1 (floor function).
- `FUNCTION INTEGER(3.7)` = 3
- `FUNCTION INTEGER(-3.7)` = -4
- `FUNCTION INTEGER(3.0)` = 3
- Dialects: All (COBOL-85+)

**FUNCTION INTEGER-PART(argument-1)**
- Returns: Integer part with truncation toward zero.
- `FUNCTION INTEGER-PART(3.7)` = 3
- `FUNCTION INTEGER-PART(-3.7)` = -3
- Dialects: All (COBOL-85+)

**Critical difference: INTEGER vs INTEGER-PART:**
```
FUNCTION INTEGER(-3.7) = -4     (floor)
FUNCTION INTEGER-PART(-3.7) = -3   (truncation toward zero)
```

**FUNCTION MOD(argument-1, argument-2)**
- Returns: argument-1 modulo argument-2.
- Formula: `argument-1 - (argument-2 * FUNCTION INTEGER(argument-1 / argument-2))`
- Uses **floor division** (FUNCTION INTEGER).
- Result sign: same as argument-2 (the divisor).
- Dialects: All (COBOL-85+)

**FUNCTION REM(argument-1, argument-2)**
- Returns: Remainder of argument-1 / argument-2.
- Formula: `argument-1 - (argument-2 * FUNCTION INTEGER-PART(argument-1 / argument-2))`
- Uses **truncated division** (FUNCTION INTEGER-PART).
- Result sign: same as argument-1 (the dividend).
- Dialects: All (COBOL-85+)

**MOD vs REM -- Critical Difference:**
```
FUNCTION MOD(-11, 3)  = -11 - (3 * INTEGER(-11/3))
                      = -11 - (3 * INTEGER(-3.667))
                      = -11 - (3 * -4) = -11 + 12 = 1

FUNCTION REM(-11, 3)  = -11 - (3 * INTEGER-PART(-11/3))
                      = -11 - (3 * INTEGER-PART(-3.667))
                      = -11 - (3 * -3) = -11 + 9 = -2
```

### 6.2 Exponentiation and Roots

**FUNCTION SQRT(argument-1)**
- Returns: Square root. Argument must be >= 0.
- Result: Floating-point.
- Dialects: All (COBOL 2002+, IBM, GnuCOBOL, Micro Focus)

**FUNCTION FACTORIAL(argument-1)**
- Returns: argument-1! (factorial).
- Argument: Non-negative integer.
- Max argument: implementation-defined. IBM supports FACTORIAL(29) in
  ARITH(COMPAT), higher in ARITH(EXTEND).
- Dialects: All (COBOL 2002+)

**FUNCTION EXP(argument-1)**
- Returns: e^argument-1.
- Dialects: All (COBOL 2002+)

**FUNCTION EXP10(argument-1)**
- Returns: 10^argument-1.
- Dialects: All (COBOL 2002+)

### 6.3 Logarithmic Functions

**FUNCTION LOG(argument-1)**
- Returns: Natural logarithm (ln). Argument must be > 0.
- Dialects: All (COBOL 2002+)

**FUNCTION LOG10(argument-1)**
- Returns: Common logarithm (log base 10). Argument must be > 0.
- Dialects: All (COBOL 2002+)

**FUNCTION E**
- Returns: Mathematical constant e (~2.71828).
- No arguments.
- Dialects: COBOL 2014, GnuCOBOL 3.x. NOT IBM Enterprise COBOL. NOT Micro Focus.

**FUNCTION PI**
- Returns: Mathematical constant pi (~3.14159).
- No arguments.
- Dialects: COBOL 2014, GnuCOBOL 3.x. NOT IBM Enterprise COBOL. NOT Micro Focus.

### 6.4 Trigonometric Functions

All trigonometric functions operate in **radians**.

| Function | Returns | Domain | Range | Dialects |
|----------|---------|--------|-------|----------|
| FUNCTION SIN(x) | Sine | All reals | [-1, 1] | All (COBOL 2002+) |
| FUNCTION COS(x) | Cosine | All reals | [-1, 1] | All (COBOL 2002+) |
| FUNCTION TAN(x) | Tangent | All reals (except pi/2 + n*pi) | All reals | All (COBOL 2002+) |
| FUNCTION ASIN(x) | Arc sine | [-1, 1] | [-pi/2, pi/2] | All (COBOL 2002+) |
| FUNCTION ACOS(x) | Arc cosine | [-1, 1] | [0, pi] | All (COBOL 2002+) |
| FUNCTION ATAN(x) | Arc tangent | All reals | [-pi/2, pi/2] | All (COBOL 2002+) |

No COBOL dialect provides ATAN2 (two-argument arctangent), SINH, COSH,
TANH, or other hyperbolic functions as intrinsic functions.

### 6.5 Financial Functions

**FUNCTION ANNUITY(rate, periods)**
- Returns: Annuity-immediate payment ratio.
- rate (argument-1): Interest rate per period (>= 0).
- periods (argument-2): Number of periods (integer > 0).
- Formula:
  - If rate = 0: `1 / periods`
  - Otherwise: `rate / (1 - (1 + rate) ** (-periods))`
- Use: `payment = principal * FUNCTION ANNUITY(rate, periods)`
- Dialects: All (COBOL 2002+)

**FUNCTION PRESENT-VALUE(rate, amount-1, amount-2 ...)**
- Returns: Present value of future cash flows at discount rate.
- rate (argument-1): Discount rate per period.
- amount-n: Future amount for period n.
- Formula: `SUM of (amount-k / (1 + rate) ** k)` for k = 1 to n
- Dialects: All (COBOL 2002+)

### 6.6 Statistical Functions

| Function | Returns | Arguments | Dialects |
|----------|---------|-----------|----------|
| MAX(a, b, ...) | Maximum value | 2+ numeric (or alphanumeric) | All (COBOL-85+) |
| MIN(a, b, ...) | Minimum value | 2+ numeric (or alphanumeric) | All (COBOL-85+) |
| ORD-MAX(a, b, ...) | 1-based position of max | 2+ same category | All (COBOL 2002+) |
| ORD-MIN(a, b, ...) | 1-based position of min | 2+ same category | All (COBOL 2002+) |
| SUM(a, b, ...) | Sum of all | 2+ numeric | All (COBOL 2002+) |
| MEAN(a, b, ...) | Arithmetic mean | 2+ numeric | All (COBOL 2002+) |
| MEDIAN(a, b, ...) | Median value | 2+ numeric | All (COBOL 2002+) |
| VARIANCE(a, b, ...) | Population variance | 2+ numeric | All (COBOL 2002+) |
| STANDARD-DEVIATION(a, b, ...) | Population std dev | 2+ numeric | All (COBOL 2002+) |
| RANGE(a, b, ...) | MAX - MIN | 2+ numeric | All (COBOL 2002+) |
| RANDOM([seed]) | Pseudo-random [0, 1) | Optional integer seed | All (COBOL 2002+) |

FUNCTION SUM can operate on array elements with the ALL subscript:
```cobol
FUNCTION SUM(TABLE-ITEM(ALL))   *> Sums all array elements
```

FUNCTION VARIANCE uses **population variance** formula (divides by N,
not N-1). This is NOT sample variance.

FUNCTION RANDOM: First call with a seed initializes the generator.
Subsequent calls without arguments continue the sequence. The sequence
is deterministic for a given seed but implementation-specific.

### 6.7 Numeric Conversion Functions

**FUNCTION NUMVAL(string)**
- Parses simple numeric string -> numeric value.
- Accepts: spaces, optional sign (+/-), digits, optional decimal point.
- Also accepts trailing CR/DB as negative indicators.
- `FUNCTION NUMVAL("  123.45 ")` = 123.45
- `FUNCTION NUMVAL("  123CR")` = -123
- Dialects: All (COBOL 2002+)

**FUNCTION NUMVAL-C(string [, currency])**
- Parses formatted currency string -> numeric value.
- Strips currency symbols, grouping separators (commas), and handles signs.
- `FUNCTION NUMVAL-C("$1,234.56")` = 1234.56
- `FUNCTION NUMVAL-C("$1,234.56CR")` = -1234.56
- currency: Optional currency string (default "$" or SPECIAL-NAMES setting).
- Dialects: All (COBOL 2002+)

**FUNCTION NUMVAL-F(string)**
- Parses floating-point string -> numeric value.
- Format: [sign] digits [.digits] E [sign] digits
- `FUNCTION NUMVAL-F("1.5E2")` = 150
- Dialects: COBOL 2002+, IBM, GnuCOBOL 3.x, Micro Focus

**FUNCTION TEST-NUMVAL(string)** (COBOL 2014)
- Returns 0 if NUMVAL would succeed, or the position of the first
  invalid character.
- Dialects: COBOL 2014, GnuCOBOL 3.x. NOT IBM. NOT Micro Focus.

**FUNCTION TEST-NUMVAL-C(string [, currency])** (COBOL 2014)
- Same validation pattern for NUMVAL-C format.
- Dialects: COBOL 2014, GnuCOBOL 3.x.

**FUNCTION TEST-NUMVAL-F(string)** (COBOL 2014)
- Same validation pattern for NUMVAL-F format.
- Dialects: COBOL 2014, GnuCOBOL 3.x.

### 6.8 Other Numeric Functions

**FUNCTION ORD(character)**
- Returns ordinal position in the collating sequence (1-based).
- `FUNCTION ORD("A")` = 66 (ASCII) or 194 (EBCDIC)
- Dialects: All (COBOL-85+)

**FUNCTION LENGTH(item)**
- Returns length in character positions.
- Dialects: All (COBOL-85+)

**FUNCTION BYTE-LENGTH(item)**
- Returns length in bytes (useful for DBCS/national data).
- Dialects: COBOL 2002+, IBM, GnuCOBOL, Micro Focus

**FUNCTION INTEGER-OF-DATE(YYYYMMDD)**
- Converts Gregorian date to integer day count.
- Dialects: All (COBOL 2002+)

**FUNCTION INTEGER-OF-DAY(YYYYDDD)**
- Converts Julian date to integer day count.
- Dialects: All (COBOL 2002+)

**FUNCTION HIGHEST-ALGEBRAIC(field)** (COBOL 2014)
- Returns maximum value a numeric field's PIC can hold.
- `FUNCTION HIGHEST-ALGEBRAIC(MY-FIELD)` where MY-FIELD is PIC S9(3)V99
  returns 999.99
- Dialects: COBOL 2014, GnuCOBOL 3.x. NOT IBM. NOT Micro Focus.

**FUNCTION LOWEST-ALGEBRAIC(field)** (COBOL 2014)
- Returns minimum value a numeric field's PIC can hold.
- For PIC S9(3)V99 returns -999.99. For PIC 9(3) returns 0.
- Dialects: COBOL 2014, GnuCOBOL 3.x. NOT IBM. NOT Micro Focus.

---

## 7. Operator Precedence in COMPUTE

### Standard Precedence (Highest to Lowest)

| Level | Operator | Associativity | Description |
|-------|----------|---------------|-------------|
| 1 | `( )` | N/A | Parentheses (innermost first) |
| 2 | Unary `+`, Unary `-` | Right-to-left | Sign operators |
| 3 | `**` | Right-to-left | Exponentiation |
| 4 | `*`, `/` | Left-to-right | Multiplication, Division |
| 5 | `+`, `-` | Left-to-right | Addition, Subtraction |

### Exponentiation Associativity

Exponentiation is RIGHT-associative:
```cobol
COMPUTE X = 2 ** 3 ** 2
*> Evaluated as: 2 ** (3 ** 2) = 2 ** 9 = 512
*> NOT as: (2 ** 3) ** 2 = 8 ** 2 = 64
```

### Consecutive Operators

Two consecutive operators require parentheses or unary sign context:
```cobol
COMPUTE X = A * -B        *> Valid: unary minus on B
COMPUTE X = A * (-B)      *> Valid: explicit parentheses
COMPUTE X = A ** -B       *> Valid: negative exponent
```

### Dialect Consistency

Operator precedence rules are **identical across all major dialects**.
No known dialect deviates from the standard precedence rules.

---

## 8. Dialect-Specific Compiler Options

### 8.1 IBM Enterprise COBOL

#### ARITH Compiler Option

| Feature | ARITH(COMPAT) | ARITH(EXTEND) |
|---------|---------------|---------------|
| Max literal size | 18 digits | 31 digits |
| Max intermediate digits | 30 | 31 |
| Float exponentiation | Long (64-bit) | Extended (128-bit) |
| FUNCTION RANDOM precision | Standard | Extended |
| Compatible with OS/VS COBOL | Yes | No |
| Default in v6+ | No | Yes |

#### NUMPROC Compiler Option

Controls sign handling for DISPLAY and COMP-3 fields:

| Option | Behavior |
|--------|----------|
| NUMPROC(NOPFD) | Strict sign validation; invalid signs cause errors |
| NUMPROC(PFD) | Assumes preferred signs valid; faster but less safe |
| NUMPROC(MIG) | Migration mode; treats unsigned as positive |

#### TRUNC Compiler Option

Controls binary field truncation after arithmetic:

| Option | Effect |
|--------|--------|
| TRUNC(STD) | Binary fields truncated to PIC size after arithmetic |
| TRUNC(OPT) | No truncation; binary uses full native range; faster |
| TRUNC(BIN) | Binary uses full native range with explicit PIC checks |

#### IEEE 754 Types (IBM Ent. v6+)

```cobol
01 IEEE-SHORT    USAGE FLOAT-SHORT.        *> IEEE 754 binary32
01 IEEE-LONG     USAGE FLOAT-LONG.         *> IEEE 754 binary64
01 DEC-FLOAT-16  USAGE FLOAT-DECIMAL-16.   *> IEEE 754 decimal64
01 DEC-FLOAT-34  USAGE FLOAT-DECIMAL-34.   *> IEEE 754 decimal128
```

These are distinct from COMP-1/COMP-2, which map to IBM hexadecimal
floating-point on z/OS.

### 8.2 GnuCOBOL

#### Arithmetic Configuration

```
-fdefault-arithmetic-precision=N     Set intermediate digit count
-fbinary-size={1-2-4-8|2-4-8}       Binary field size allocation
-fbinary-truncate                     Truncate binary to PIC size
```

#### Extended Features

- COMP-X: Unsigned native binary (full range regardless of PIC).
- Numeric literals: Up to 38 digits.
- Hex/octal/binary literals: H"FF", O"77", B"1010" in some contexts.
- FUNCTION PI and FUNCTION E supported (COBOL 2014).
- Dialect emulation via `-std=` flag:
  - `-std=ibm` -- IBM Enterprise COBOL compatibility
  - `-std=mf` -- Micro Focus compatibility
  - `-std=cobol85` -- COBOL-85 standard
  - `-std=cobol2014` -- COBOL 2014 standard
  - `-std=default` -- GnuCOBOL default (most permissive)

#### Additional Intrinsic Functions (COBOL 2014)

| Function | Description |
|----------|-------------|
| COMBINED-DATETIME | Combines date and time integers |
| TEST-NUMVAL | Validates NUMVAL input without error |
| TEST-NUMVAL-C | Validates NUMVAL-C input without error |
| TEST-NUMVAL-F | Validates NUMVAL-F input without error |
| HIGHEST-ALGEBRAIC | Max value for numeric field PIC |
| LOWEST-ALGEBRAIC | Min value for numeric field PIC |
| FORMATTED-CURRENT-DATE | Formatted date/time string |
| TEST-FORMATTED-DATETIME | Validates formatted date/time |

### 8.3 Micro Focus Visual COBOL

#### Arithmetic Directives

```
$SET ARITHMETIC"NATIVE"          Native platform arithmetic
$SET ARITHMETIC"OS390COMPAT"     IBM ARITH(COMPAT) emulation
$SET ARITHMETIC"OS390EXTEND"     IBM ARITH(EXTEND) emulation
$SET ARITHMETIC"STANDARD"        COBOL standard arithmetic
```

#### COMP-X Type

Unsigned native binary. Full storage range regardless of PIC:

| PIC Digits | Bytes | Max Value |
|------------|-------|-----------|
| 1-2 | 1 | 255 |
| 3-4 | 2 | 65,535 |
| 5-7 | 3 | 16,777,215 |
| 8-9 | 4 | 4,294,967,295 |
| 10-12 | 5 | 1,099,511,627,775 |
| 13-14 | 6 | 281,474,976,710,655 |
| 15-16 | 7 | 72,057,594,037,927,935 |
| 17-18 | 8 | 18,446,744,073,709,551,615 |

#### COMP-6 (Micro Focus Extension)

Unsigned packed decimal. Like COMP-3 but without sign nibble. Each byte
holds two digits. Storage = CEIL(digits / 2) bytes.

### 8.4 Fujitsu NetCOBOL

- COMP-6: Different semantics than Micro Focus in some versions.
- Extended precision via compiler options.
- Trig functions supported following COBOL 2002 standard.
- Financial functions (ANNUITY, PRESENT-VALUE) supported.
- Max 18-digit numeric literals (COBOL-85 compatible).

### 8.5 COBOL 2014 Standard Additions

**New intrinsic functions:**

| Function | Description |
|----------|-------------|
| BOOLEAN-OF-INTEGER | Integer to boolean conversion |
| INTEGER-OF-BOOLEAN | Boolean to integer conversion |
| COMBINED-DATETIME | Combine date and time integers |
| FORMATTED-DATE | Format a date integer |
| FORMATTED-TIME | Format a time integer |
| FORMATTED-DATETIME | Format combined date-time |
| FORMATTED-CURRENT-DATE | Current date formatted |
| TEST-NUMVAL | Validate numeric string |
| TEST-NUMVAL-C | Validate currency numeric string |
| TEST-NUMVAL-F | Validate float numeric string |
| TEST-FORMATTED-DATETIME | Validate formatted date-time |
| HIGHEST-ALGEBRAIC | Max value for field PIC |
| LOWEST-ALGEBRAIC | Min value for field PIC |
| TRIM | Trim whitespace |

**New data types:**

| Type | Description |
|------|-------------|
| FLOAT-DECIMAL-16 | IEEE 754 decimal64 (16 significant digits) |
| FLOAT-DECIMAL-34 | IEEE 754 decimal128 (34 significant digits) |
| FLOAT-BINARY-32 | IEEE 754 binary32 |
| FLOAT-BINARY-64 | IEEE 754 binary64 |
| FLOAT-BINARY-128 | IEEE 754 binary128 |

**Extended ROUNDED modes:** All 8 modes (Section 2).

**CONSTANT clause:**
```cobol
01 WS-PI CONSTANT 3.14159265358979323846.
```

---

## 9. Cross-Dialect Compatibility Matrix

### 9.1 Arithmetic Verbs

| Feature | COBOL-85 | COBOL 2014 | IBM Ent. 6.x | GnuCOBOL 3.x | Micro Focus | Fujitsu |
|---------|----------|------------|--------------|---------------|-------------|---------|
| ADD (all formats) | Yes | Yes | Yes | Yes | Yes | Yes |
| SUBTRACT (all formats) | Yes | Yes | Yes | Yes | Yes | Yes |
| MULTIPLY (all formats) | Yes | Yes | Yes | Yes | Yes | Yes |
| DIVIDE (all 5 formats) | Yes | Yes | Yes | Yes | Yes | Yes |
| COMPUTE | Yes | Yes | Yes | Yes | Yes | Yes |
| CORRESPONDING (ADD/SUB) | Yes | Yes | Yes | Yes | Yes | Yes |
| ON SIZE ERROR | Yes | Yes | Yes | Yes | Yes | Yes |
| NOT ON SIZE ERROR | Yes | Yes | Yes | Yes | Yes | Yes |
| REMAINDER (DIVIDE) | Yes | Yes | Yes | Yes | Yes | Yes |
| ROUNDED (basic) | Yes | Yes | Yes | Yes | Yes | Yes |
| ROUNDED MODE IS | No | Yes | Yes | Yes | No | No |

### 9.2 Intrinsic Math Functions

| Function | IBM Ent. | GnuCOBOL 3.x | Micro Focus | Fujitsu | COBOL 2014 |
|----------|----------|---------------|-------------|---------|------------|
| ABS | Yes | Yes | Yes | Yes | Yes |
| ACOS | Yes | Yes | Yes | Yes | Yes |
| ANNUITY | Yes | Yes | Yes | Yes | Yes |
| ASIN | Yes | Yes | Yes | Yes | Yes |
| ATAN | Yes | Yes | Yes | Yes | Yes |
| COS | Yes | Yes | Yes | Yes | Yes |
| E | No | Yes | No | No | Yes |
| EXP | Yes | Yes | Yes | Yes | Yes |
| EXP10 | Yes | Yes | Yes | Yes | Yes |
| FACTORIAL | Yes | Yes | Yes | Yes | Yes |
| HIGHEST-ALGEBRAIC | No | Yes | No | No | Yes |
| INTEGER | Yes | Yes | Yes | Yes | Yes |
| INTEGER-PART | Yes | Yes | Yes | Yes | Yes |
| LOG | Yes | Yes | Yes | Yes | Yes |
| LOG10 | Yes | Yes | Yes | Yes | Yes |
| LOWEST-ALGEBRAIC | No | Yes | No | No | Yes |
| MAX | Yes | Yes | Yes | Yes | Yes |
| MEAN | Yes | Yes | Yes | Yes | Yes |
| MEDIAN | Yes | Yes | Yes | Yes | Yes |
| MIN | Yes | Yes | Yes | Yes | Yes |
| MOD | Yes | Yes | Yes | Yes | Yes |
| NUMVAL | Yes | Yes | Yes | Yes | Yes |
| NUMVAL-C | Yes | Yes | Yes | Yes | Yes |
| NUMVAL-F | Yes | Yes | Yes | Yes | Yes |
| ORD | Yes | Yes | Yes | Yes | Yes |
| ORD-MAX | Yes | Yes | Yes | Yes | Yes |
| ORD-MIN | Yes | Yes | Yes | Yes | Yes |
| PI | No | Yes | No | No | Yes |
| PRESENT-VALUE | Yes | Yes | Yes | Yes | Yes |
| RANDOM | Yes | Yes | Yes | Yes | Yes |
| RANGE | Yes | Yes | Yes | Yes | Yes |
| REM | Yes | Yes | Yes | Yes | Yes |
| SIN | Yes | Yes | Yes | Yes | Yes |
| SQRT | Yes | Yes | Yes | Yes | Yes |
| STANDARD-DEVIATION | Yes | Yes | Yes | Yes | Yes |
| SUM | Yes | Yes | Yes | Yes | Yes |
| TAN | Yes | Yes | Yes | Yes | Yes |
| TEST-NUMVAL | No | Yes | No | No | Yes |
| TEST-NUMVAL-C | No | Yes | No | No | Yes |
| TEST-NUMVAL-F | No | Yes | No | No | Yes |
| VARIANCE | Yes | Yes | Yes | Yes | Yes |

### 9.3 Intermediate Result Precision

| Dialect | Max Fixed-Point | Max Literal | Float Exponent | Configurable |
|---------|----------------|-------------|----------------|--------------|
| IBM ARITH(COMPAT) | 30 digits | 18 digits | 64-bit | Via compiler option |
| IBM ARITH(EXTEND) | 31 digits | 31 digits | 128-bit | Via compiler option |
| GnuCOBOL (default) | 36+ digits | 38 digits | 64-bit | Via -f options |
| Micro Focus (native) | 18-19 digits | 18 digits | 64-bit | Via $SET directive |
| Micro Focus (OS390) | 30 or 31 digits | 18 or 31 digits | 64-bit | Via $SET directive |
| Fujitsu NetCOBOL | 18 digits | 18 digits | 64-bit | Limited |
| COBOL-85 standard | Impl-defined | 18 digits | Impl-defined | N/A |
| COBOL 2014 standard | Impl-defined | 31+ digits | Impl-defined | N/A |

---

## 10. Transpiler Implementation Notes

### 10.1 Key Challenges for cobol2rust

1. **Fixed-point intermediate precision**: Rust has no built-in fixed-point
   type. The runtime must implement the IBM intermediate result sizing
   algorithm to reproduce COBOL arithmetic exactly. The `rust_decimal`
   crate (128-bit, 28-digit) covers IBM ARITH(COMPAT) comfortably.

2. **REMAINDER semantics**: COBOL DIVIDE REMAINDER depends on the GIVING
   field's PIC, not just the mathematical division. The transpiler must:
   a. Compute quotient to full precision.
   b. Store quotient (with truncation/rounding to target PIC).
   c. Recompute: `remainder = dividend - (stored_quotient * divisor)`.

3. **MOD vs REM**: Opposite sign conventions. MOD uses floor division
   (FUNCTION INTEGER), REM uses truncated division (FUNCTION INTEGER-PART).
   Both must be implemented correctly.

4. **ROUNDED modes**: Rust's standard library lacks most of these.
   The `rust_decimal` crate supports several that can be mapped:
   - NEAREST-AWAY-FROM-ZERO -> `RoundingStrategy::MidpointAwayFromZero`
   - NEAREST-EVEN -> `RoundingStrategy::BankersRounding`
   - TRUNCATION -> `RoundingStrategy::ToZero`
   - Others require custom implementation.

5. **SIZE ERROR detection**: Must check integer overflow BEFORE storing.
   If SIZE ERROR, destination is unchanged. Requires:
   a. Computing result to full precision.
   b. Checking integer digits against target PIC.
   c. Conditionally storing or executing SIZE ERROR handler.

6. **CORRESPONDING**: Requires compile-time field name matching between
   two structs. The transpiler generates explicit field-by-field
   operations based on matching rules.

7. **Expression tree evaluation**: COMPUTE expressions must respect
   right-associative exponentiation. Intermediate precision must follow
   the configured dialect's sizing algorithm.

8. **Floating-point contamination**: If ANY operand is COMP-1/COMP-2
   or a float-returning intrinsic, the ENTIRE expression uses float.
   Detect at compile time.

9. **TRUNC behavior**: Binary truncation must be configurable:
   - TRUNC(STD): Truncate to PIC after arithmetic.
   - TRUNC(OPT): No truncation.
   - TRUNC(BIN): Full range with explicit checks.

10. **NUMVAL/NUMVAL-C/NUMVAL-F**: Require custom parsers for COBOL-specific
    numeric string formats (CR/DB signs, currency, grouping separators).

### 10.2 Rust Type Mapping for Arithmetic

| COBOL Concept | Recommended Rust Approach |
|---------------|--------------------------|
| Fixed-point intermediate results | `rust_decimal::Decimal` with configurable precision |
| ROUNDED modes | Enum with 8 variants, applied at store time |
| SIZE ERROR | `Result<(), SizeError>` from arithmetic operations |
| REMAINDER | Computed from stored quotient, not mathematical |
| Trig/log/exp intrinsics | `f64` from std, converted back to decimal |
| ANNUITY/PRESENT-VALUE | Custom implementations using `f64` |
| NUMVAL/NUMVAL-C | Custom parsers returning `Decimal` |
| CORRESPONDING | Code generation at transpile time |
| Left-truncation | `value % 10^integer_digits` at store time |
| MOD function | Custom: `a - b * floor(a/b)` |
| REM function | Custom: `a - b * trunc(a/b)` |

### 10.3 Implementation Priority

**Phase 1 (Core arithmetic engine):**
- ADD, SUBTRACT, MULTIPLY, DIVIDE, COMPUTE verbs
- Basic ROUNDED (NEAREST-AWAY-FROM-ZERO)
- ON SIZE ERROR detection
- Left-truncation semantics
- Decimal alignment on store
- Sign handling (signed/unsigned fields)

**Phase 2 (Extended modes and division):**
- All 8 ROUNDED modes
- REMAINDER with correct PIC-dependent semantics
- CORRESPONDING (ADD CORR, SUBTRACT CORR)
- TRUNC(STD/OPT/BIN) configuration
- Intermediate result sizing algorithm (ARITH COMPAT/EXTEND)

**Phase 3 (Intrinsic functions):**
- ABS, INTEGER, INTEGER-PART, MOD, REM
- Trig functions (SIN, COS, TAN, ASIN, ACOS, ATAN)
- LOG, LOG10, EXP, EXP10, SQRT
- Statistical (MAX, MIN, SUM, MEAN, MEDIAN, VARIANCE, STANDARD-DEVIATION)
- Financial (ANNUITY, PRESENT-VALUE)
- FACTORIAL, RANDOM
- NUMVAL, NUMVAL-C, NUMVAL-F

**Phase 4 (Dialect extensions and edge cases):**
- IEEE 754 decimal types (FLOAT-DECIMAL-16, FLOAT-DECIMAL-34)
- GnuCOBOL extended precision (38-digit support)
- Micro Focus COMP-X, COMP-6
- COBOL 2014 functions (TEST-NUMVAL-*, HIGHEST/LOWEST-ALGEBRAIC)
- Exponentiation edge cases (0**0, negative**fractional)
- FUNCTION E and FUNCTION PI constants

---

## Appendix A: Quick Reference -- All Arithmetic Operations

### Verbs (5)
```
ADD ... TO ... [GIVING ...] [CORRESPONDING]
SUBTRACT ... FROM ... [GIVING ...] [CORRESPONDING]
MULTIPLY ... BY ... [GIVING ...]
DIVIDE ... INTO/BY ... [GIVING ...] [REMAINDER ...]
COMPUTE ... = expression
```

### COMPUTE Operators (5)
```
+   Addition
-   Subtraction / Negation
*   Multiplication
/   Division
**  Exponentiation
```

### Intrinsic Math Functions (42)
```
Absolute/Rounding:  ABS, INTEGER, INTEGER-PART, MOD, REM
Exponent/Root:      SQRT, FACTORIAL, EXP, EXP10
Logarithmic:        LOG, LOG10
Constants:          E*, PI*
Trigonometric:      SIN, COS, TAN, ASIN, ACOS, ATAN
Financial:          ANNUITY, PRESENT-VALUE
Statistical:        MAX, MIN, ORD-MAX, ORD-MIN, SUM, MEAN,
                    MEDIAN, VARIANCE, STANDARD-DEVIATION,
                    RANGE, RANDOM
Conversion:         NUMVAL, NUMVAL-C, NUMVAL-F,
                    TEST-NUMVAL*, TEST-NUMVAL-C*, TEST-NUMVAL-F*
Other numeric:      ORD, LENGTH, BYTE-LENGTH,
                    INTEGER-OF-DATE, INTEGER-OF-DAY,
                    HIGHEST-ALGEBRAIC*, LOWEST-ALGEBRAIC*

* = Not available in all dialects (see Section 9.2)
```

### Rounding Modes (8)
```
AWAY-FROM-ZERO, NEAREST-AWAY-FROM-ZERO (default),
NEAREST-EVEN, NEAREST-TOWARD-ZERO,
TOWARD-GREATER, TOWARD-LESSER,
TRUNCATION, PROHIBITED
```
