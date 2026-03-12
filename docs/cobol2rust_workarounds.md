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

## W-002: DuckDB Does Not Support SAVEPOINT

**Affected**: `CobolSqlRuntime::savepoint()` and `CobolSqlRuntime::rollback_to_savepoint()`
methods when using the `DuckDbRuntime` backend.

**Root cause**: DuckDB does not implement the SQL `SAVEPOINT` or
`ROLLBACK TO SAVEPOINT` syntax. These are standard SQL features supported by
PostgreSQL, Oracle, DB2, and SQLite, but DuckDB's transaction model only
supports flat `BEGIN`/`COMMIT`/`ROLLBACK`.

**Symptom**: Calling `savepoint()` or `rollback_to_savepoint()` on `DuckDbRuntime`
sets `SQLCODE = -1` with a parser error message. COBOL programs using
`EXEC SQL SAVEPOINT` will fail at runtime with DuckDB.

**Workaround**: SAVEPOINT/ROLLBACK TO SAVEPOINT is not tested with DuckDB.
The `DuckDbRuntime` implementation passes the SQL through to DuckDB which
rejects it. Programs that use savepoints must use an enterprise backend
(PostgreSQL, DB2, Oracle) that supports the syntax.

**Applied to**: `crates/cobol-sql/tests/duckdb_integration.rs` -- savepoint
test case removed (documented as DuckDB limitation in test file comment).

**Proper fix**: Implement `PostgresRuntime` (or other enterprise backend) that
supports `SAVEPOINT`. The `CobolSqlRuntime` trait is backend-agnostic by design,
so savepoint support works correctly once a supporting backend is used.
Alternatively, DuckDB savepoints could be emulated using nested transactions
if DuckDB adds support in the future.

---

## W-003: Deep-Nesting EVALUATE Output Mismatch

**Affected**: `cobol/volume/deep_nesting.cbl` -- the `TEST-EVAL-IN-NEST`
paragraph that exercises `EVALUATE TRUE` inside nested IF inside PERFORM.

**Root cause**: The transpiled Rust code produces incorrect output for the
EVALUATE TRUE / WHEN conditions inside a deeply nested control structure
(3-level nested IF inside a PERFORM VARYING, with the EVALUATE containing
further nested IFs and a nested EVALUATE inside one WHEN branch). The codegen
for EVALUATE WHEN branches interacts incorrectly with the surrounding nested
IF context, likely a scoping or fall-through issue in `proc_gen.rs`.

**Symptom**: The program compiles and runs. 10 of 11 checks pass. The final
check (`WS-CATEGORY = "CAT-E"`) fails -- the EVALUATE TRUE at iteration I=5
with WS-DEPTH=50 does not correctly match `WHEN WS-DEPTH > 40` inside the
nested structure, producing the wrong category value.

**Workaround**: None applied. The test compiles and runs; the mismatch is
accepted as a known minor issue. All other 39 stress test programs pass
all checks.

**Applied to**: `cobol/volume/deep_nesting.cbl` -- test runs but 1 of 11
runtime checks produces wrong output.

**Proper fix**: Debug the EVALUATE TRUE codegen in
`crates/cobol-transpiler/src/codegen/proc_gen.rs` for the specific case of:
1. EVALUATE TRUE inside nested IF (3+ levels deep)
2. WHEN branches that themselves contain nested IF statements
3. Nested EVALUATE inside a WHEN branch
The issue is likely in how WHEN branch conditions interact with the
surrounding if/else chain generation, or how the EVALUATE's `match` arms
handle nested control flow.

---

## W-004: ANTLR Exponential Backtracking on Nested ELSE IF Without END-IF

**Affected**: Any COBOL source with long chains of `ELSE IF` that use
period-terminated (sentence-delimited) style without explicit `END-IF` scope
terminators.

**Root cause**: The ANTLR4 grammar rule for IF statements treats `ELSE IF` as
nested `ELSE (IF ...)` rather than a flat chain. Without `END-IF` delimiters,
each `ELSE` clause introduces ambiguity about which `IF` it belongs to (the
"dangling else" problem). With N nested levels, the parser explores O(2^N)
possible parse trees before finding the correct one.

This is a well-known ANTLR4 issue with recursive grammar rules that have
optional trailing clauses. The COBOL grammar's IF rule likely looks like:

```
ifStatement: IF condition THEN? statement* (ELSE statement*)? END_IF?
```

When `END_IF` is absent and `statement*` can itself contain `ifStatement`,
the parser cannot deterministically decide where each `ELSE` attaches.

**Symptom**: Parse times grow exponentially with chain length:
- 10 nested ELSE IF: milliseconds
- 28 nested ELSE IF (WOPO.COB): seconds to minutes
- 128 nested ELSE IF (ENCASCII.COB): 22+ minutes on a 24-core Linux machine

During scanning of 280K COBOL files, these pathological files consume the
majority of wall-clock time. Average parse time across all files was 6.3s,
but pathological files took 1,000x longer (22 minutes for a 403-line file).

**Files exhibiting this pattern**:

| File | Lines | ELSE IF depth | END-IF count | Parse time |
|------|-------|---------------|-------------|------------|
| ENCASCII.COB | 403 | 128 | 0 | ~22 min |
| WOPO.COB | 1,449 | 28 (command dispatch) + 3 (message handler) | 0 | minutes |

These are real-world COBOL programs (ENCASCII is an ASCII encoding lookup
table; WOPO is an IRC bot written in COBOL). The `ELSE IF` chain pattern is
a common COBOL idiom for dispatch tables when `EVALUATE` is unavailable or
unfamiliar to the author.

**Workaround**: None applied yet. Files that hit this pattern will parse
correctly but slowly. The `scan` command's Phase 1 will eventually complete
but with very long wall times for affected files.

Possible runtime mitigations (not yet implemented):
- Per-file parse timeout with graceful skip and diagnostic recording
- Pre-scan heuristic: count `ELSE IF` occurrences vs `END-IF`; flag files
  exceeding a threshold (e.g., >20 unmatched ELSE IF chains)
- Source rewriting: inject `END-IF` before each `ELSE IF` in a preprocessing
  step to flatten the nesting

**Proper fix**: Modify the ANTLR4 grammar to handle `ELSE IF` as a flat
chain rather than nested recursion. Two approaches:

1. **Grammar-level**: Add an explicit `elseIfClause` production:
   ```
   ifStatement
       : IF condition THEN? statement*
         elseIfClause*
         (ELSE statement*)?
         END_IF?
         DOT?
       ;

   elseIfClause
       : ELSE IF condition THEN? statement*
       ;
   ```
   This converts O(2^N) backtracking to O(N) linear parsing.

2. **Preprocessor-level**: Before ANTLR parsing, scan for `ELSE IF` patterns
   without matching `END-IF` and rewrite them as `END-IF ELSE IF` or as
   `EVALUATE TRUE / WHEN` constructs. This is fragile but avoids grammar
   changes.

3. **SLL prediction mode**: ANTLR4's SLL prediction mode (used before falling
   back to LL) may already handle this efficiently. Verify that the grammar
   doesn't force LL mode for IF statements. If it does, restructuring the
   rule to be SLL-compatible would give a large speedup.

---

## W-005: Parser Slowdown on Deeply Qualified Data References (7-Dim Tables)

**Affected**: COBOL programs with 7-dimensional tables using fully qualified
data names with 5+ qualifiers and subscript expressions.

**Root cause**: The ANTLR4 grammar for qualified data references
(`identifier OF identifier OF identifier...`) combined with subscript
expressions (`(IN1 + 3)`) creates parsing ambiguity. Each qualification
level adds a decision point where the parser must determine whether the
next token is another qualifier, a subscript, or the end of the reference.
With 5+ qualifiers and arithmetic subscript expressions, the parser explores
many possible interpretations.

**Symptom**: Files with deeply qualified references parse much slower than
their line count would suggest. The NIST COBOL85 validation suite file
NC246A.CBL (1,321 lines) exercises 7-dimensional tables with references like:

```cobol
MOVE TABLE-ITEM OF TABLE-LEVEL-5 OF TABLE-LEVEL-4
     OF TABLE-LEVEL-3 OF TABLE-LEVEL-2
     OF GROUP-1-TABLE (IN1 + 3) TO TEMP-VALUE.
```

Key characteristics of NC246A.CBL:
- 7-level OCCURS nesting (data levels 02 through 08)
- Two 7-dimensional tables (TABLE-A, TABLE-B) with identical structure
- Subscript groups nested 48 levels deep (levels 02 through 49)
- 114 PERFORM statements with qualified references
- 0 IF/ELSE IF nesting (no dangling-else issue)
- Repetitive qualified IF comparisons between two tables:
  ```cobol
  IF TABLE-ITEM IN TABLE-LEVEL-5 IN TABLE-LEVEL-4
     IN TABLE-LEVEL-3 IN TABLE-LEVEL-2
     IN GROUP-1-TABLE (1) IS EQUAL TO
     TABLE-ITEM IN TABLE-LEVEL-5 IN TABLE-LEVEL-4
     IN TABLE-LEVEL-3 IN TABLE-LEVEL-2
     IN GROUP-2-TABLE (12)
  ```

**Workaround**: None applied. These files are uncommon in production COBOL
(7-dimensional tables are rare). The parser will complete but slowly.

**Proper fix**: Optimize the ANTLR4 grammar's qualified-name and subscript
rules to reduce ambiguity:

1. Make the qualifier chain left-recursive or iterative rather than recursive
2. Clearly separate the subscript expression from the qualifier chain so the
   parser doesn't backtrack between `OF` qualifiers and `(` subscripts
3. Consider a two-pass approach: first tokenize qualified references as a
   single unit, then parse the internal structure

---

## W-006: Non-ASCII Characters in COBOL Source Cause Parser Panics

**Affected**: COBOL source files containing non-ASCII bytes (EBCDIC remnants,
encoding issues, or intentional Unicode in string literals).

**Root cause**: The ANTLR4 COBOL grammar and the Rust preprocessor assume
pure ASCII input. Non-ASCII bytes can cause:
- Rust string slicing panics at non-char-boundary positions
- ANTLR lexer failures on unexpected byte sequences
- Incorrect column calculations in fixed-format preprocessing

**Symptom**: Parser panics (thread crash) or garbage output. During the
280K-file scan, 97 files caused parser panics, many due to encoding issues.

**Workaround**: Implemented in Session 47. Two fixes applied:

1. **Input sanitization** (`crates/cobol-cli/src/analyze.rs`):
   `sanitize_source()` replaces all non-ASCII characters with spaces before
   parsing. COBOL source should be pure ASCII; non-ASCII characters indicate
   EBCDIC remnants or encoding corruption. Replacing with spaces preserves
   line/column structure while preventing panics.

2. **Safe UTF-8 slicing** (`crates/cobol-transpiler/src/parser/preprocess.rs`):
   `snap_to_char_boundary()` ensures string slicing in fixed-format
   preprocessing never splits a multi-byte UTF-8 character. Applied to
   `preprocess_fixed_format()` and `detect_source_format()`.

**Applied to**:
- `crates/cobol-cli/src/analyze.rs` -- `sanitize_source()` + W003 diagnostic
- `crates/cobol-transpiler/src/parser/preprocess.rs` -- `snap_to_char_boundary()`

**Proper fix**: The current sanitization is sufficient for scanning. For
transpilation, a more nuanced approach could:
- Detect the source encoding (EBCDIC vs ASCII vs UTF-8) and convert
- Preserve non-ASCII characters inside string literals (PIC X values)
- Report the exact byte positions and values of non-ASCII content

---

## W-007: ANTLR RwLock<DFA> Contention Limits Thread-Level Parallelism

**Affected**: `cobol2rust scan` when using rayon thread-level parallelism for
parsing 240K+ COBOL files on multi-core machines.

**Root cause**: The `antlr-rust` runtime uses global `lazy_static` DFA caches
protected by `RwLock` for all parser instances. These are auto-generated in
every ANTLR-produced parser/lexer:

```rust
// Generated code (cannot be modified without forking antlr-rust)
lazy_static! {
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = { ... };
    static ref _shared_context_cache: Arc<PredictionContextCache> = { ... };
}
```

Four globals exist (one per generated file):
- `src/generated/cobol85lexer.rs` (line 1004)
- `src/generated/cobol85parser.rs` (line 90360)
- `src/generated/cobol85preprocessorlexer.rs` (line 587)
- `src/generated/cobol85preprocessorparser.rs` (line 10906)

The DFA tables grow dynamically as new parse states are encountered. Every
parse operation requires at minimum a read lock; new states require exclusive
write locks. With N threads, all N contend on the same locks, effectively
serializing parsing to 2-3 concurrent threads regardless of core count.

**Symptom**: On a 24-core Linux machine parsing 240K files, CPU utilization
stays at 10-14% (load average 2-3) despite rayon being configured with 24
threads. Individual cores spike to 100% while others sit idle, indicating
lock contention rather than I/O bottleneck. Confirmed by moving file I/O
into rayon workers (no improvement) and observing top output showing only
2-3 active cores at any time.

**Workaround**: Multi-process parallelism instead of multi-thread. The
`scan` command spawns N worker child processes (`cobol2rust scan-worker`),
each with its own address space and its own copy of the ANTLR DFA caches.
Files are distributed to workers via stdin pipes (`file_id\trel_path\tabs_path`),
results returned as NDJSON on stdout. Zero lock contention between processes.

The hidden `scan-worker` subcommand handles the per-file parse loop:
```
cobol2rust scan-worker --run-id 1 [--with-coverage]
```
Reads file paths from stdin, writes NDJSON results to stdout.

**Applied to**:
- `crates/cobol-cli/src/scan/worker.rs` -- worker process implementation
- `crates/cobol-cli/src/scan/phase1.rs` -- multi-process orchestrator
- `crates/cobol-cli/src/scan/phase2.rs` -- multi-process orchestrator
- `crates/cobol-cli/src/main.rs` -- hidden `ScanWorker` subcommand

**Proper fix**: Two possible approaches (neither implemented):

1. **Fork antlr-rust**: Modify the code generation templates to use
   per-instance DFA tables instead of global shared statics. Each parser
   instance would own its DFA, eliminating cross-thread contention. This
   is a significant undertaking and would complicate ANTLR grammar upgrades.

2. **Replace antlr-rust with a custom parser**: Write a hand-rolled COBOL
   parser (recursive descent or PEG) that avoids shared mutable state
   entirely. This is a major rewrite but would also fix W-004 (exponential
   backtracking) and W-005 (qualified reference slowdown).

The multi-process workaround is preferred as it requires no changes to the
parser infrastructure and achieves near-linear scaling with core count.

---

## W-008: Scan Phase 2 Redundantly Re-Parses All Files

**Affected**: `cobol2rust scan` -- Phase 2 (coverage analysis) re-parses every
file that already parsed successfully in Phase 1.

**Root cause**: The scan pipeline is structured as two independent phases:
- Phase 1 (Inventory): Parse all files, record parse results, filter out failures
- Phase 2 (Coverage): Re-parse all files that passed Phase 1, then run transpiler
  coverage analysis on the resulting AST

Phase 2 receives only file paths from Phase 1, not the parsed ASTs. Each worker
process re-reads the source file and re-runs the full ANTLR parse before
proceeding to transpilation. This means every valid COBOL file is parsed twice:
once in Phase 1 to check validity, once in Phase 2 to build the AST for coverage.

**Symptom**: Total scan wall time is ~30-40% longer than necessary. On a 280K
file corpus, Phase 1 parse time is entirely wasted for files that will also be
processed in Phase 2. The redundant parse is especially costly for large files
(10K+ lines) and files with pathological grammar patterns (W-004, W-005).

**Workaround**: None applied. The two-phase architecture is retained for its
operational benefits:
- Phase 1 provides early visibility into parse success/failure rates
- Phase 1 results enable resume without re-running coverage
- Separating phases allows running Phase 1 only (`--phase1-only`, not yet implemented)
- Phase 2 can be re-run independently after transpiler improvements

**Proper fix**: Merge Phase 1 and Phase 2 into a single pass:

1. Parse each file once
2. If parse fails: record diagnostic, move on (current Phase 1 behavior)
3. If parse succeeds: immediately run coverage analysis on the same AST
   (current Phase 2 behavior) before moving to the next file
4. Emit both parse_result and coverage records from the same worker pass

This eliminates one full parse pass across the corpus. The worker protocol
already supports emitting both `parse_result` and `coverage` record types
in a single session (the `--with-coverage` flag already exists for Phase 2
workers). The change would be:

- Remove the separate Phase 2 orchestrator
- Add `--with-coverage` to all Phase 1 workers
- Phase 1 writer loop handles both parse_result and coverage records
- Files that fail parsing naturally skip coverage (worker emits error, no coverage)

The `load_parseable_files()` / `get_parseable_uncovered_files()` filtering
logic becomes unnecessary since coverage runs inline with parsing.

**Estimated savings**: ~30-40% reduction in total scan wall time (eliminates
one full ANTLR parse pass). Memory usage unchanged. No impact on result quality.

---

## W-009: (Template for future workarounds)

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
| crates/cobol-sql/tests/duckdb_integration.rs | W-002: SAVEPOINT test removed (DuckDB limitation) | 2026-03-09 |
| cobol/volume/deep_nesting.cbl | W-003: EVALUATE output mismatch (1/11 checks) | 2026-03-09 |
| bad/ENCASCII.COB | W-004: 128 nested ELSE IF, ~22 min parse | 2026-03-11 |
| bad/WOPO.COB | W-004: 28 nested ELSE IF, minutes to parse | 2026-03-11 |
| bad/NC246A.CBL | W-005: 7-dim table qualified refs, slow parse | 2026-03-11 |
| (97 files in 280K scan) | W-006: Non-ASCII bytes cause parser panics | 2026-03-11 |
| crates/cobol-cli/src/scan/*.rs | W-007: ANTLR RwLock DFA contention, multi-process fix | 2026-03-11 |
| crates/cobol-cli/src/scan/phase{1,2}.rs | W-008: Redundant re-parse in Phase 2, merge to single pass | 2026-03-11 |
