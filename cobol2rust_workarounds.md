# cobol2rust Known Workarounds

Temporary workarounds for COBOL source or codegen limitations.
Each entry documents the root cause, the workaround applied, and the proper fix needed.

---

## W-001: ANTLR Reserved Word in Identifiers

**Affected**: Any COBOL variable containing a reserved keyword as a suffix
(e.g., `WS-ROUNDED`, `WS-GIVING`, `WS-REMAINDER`)

**Root cause**: The ANTLR4 lexer tokenizes `ROUNDED`, `GIVING`, `REMAINDER`, etc.
as keyword tokens. When these appear as part of a hyphenated identifier like
`WS-ROUNDED`, the lexer splits it into `WS-` (identifier fragment) + `ROUNDED`
(keyword token), producing an invalid parse.

**Symptom**: Generated Rust code contains truncated field names like `ws_` (empty
suffix) or `ws_1` (if the keyword is followed by a digit).

**Workaround**: Rename the COBOL variable to avoid the keyword suffix.
- `WS-ROUNDED` -> `WS-RNDVAL`
- `WS-REMAINDER` -> `WS-REMVAL`
- `WS-GIVING` -> `WS-GIVVAL`

**Applied to**: `cobol/language/statements/arithmetic_verbs.cbl`
- `WS-ROUNDED` renamed to `WS-RNDVAL` (all 7 occurrences)

**Proper fix**: Modify the ANTLR4 `cobolWord` lexer/parser rule to allow reserved
words as segments of hyphenated identifiers. The grammar needs a rule like:
```
cobolWord: IDENTIFIER | IDENTIFIER MINUSCHAR reservedWord | ...
```
Or alternatively, handle this in a pre-processing step that escapes keyword
collisions before lexing.

**Keywords known to cause issues**: ROUNDED, REMAINDER, GIVING, SIZE, ERROR,
PROCEDURE, SECTION, DIVISION (any keyword that can appear as part of a
data name in real COBOL programs).

---

## W-002: (Template for future workarounds)

**Affected**:

**Root cause**:

**Symptom**:

**Workaround**:

**Applied to**:

**Proper fix**:

---

## Index of Affected Files

| File | Workaround | Date |
|------|-----------|------|
| cobol/language/statements/arithmetic_verbs.cbl | W-001: WS-ROUNDED -> WS-RNDVAL | 2026-03-08 |
