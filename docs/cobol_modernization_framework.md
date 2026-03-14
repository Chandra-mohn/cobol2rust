# COBOL Modernization Framework (CMF)

A methodical, evidence-based framework for transforming COBOL systems into modern
Rust applications. Two phases, six layers, tooling-first philosophy.

## Core Principles

### Decision Hierarchy (AI-Last)

Every decision at every stage follows this hierarchy. Move to the next level
ONLY when the current level cannot provide a sufficient answer.

```
Level 1: Static Analysis     -- deterministic, provable, repeatable
Level 2: Rule-Based Transform -- pattern matching, codegen rules
Level 3: Heuristic Analysis   -- scoring, classification, graph algorithms
Level 4: Human Domain Expert  -- business knowledge, architectural judgment
Level 5: AI Assistance        -- LAST RESORT, always human-validated
```

### Evidence-Based Decisions

- Every transformation must cite its justification (metric, rule, or expert decision)
- No "this looks right" -- quantify with field usage counts, dependency scores, complexity metrics
- Every phase produces measurable artifacts that feed the next phase

### Behavioral Equivalence Invariant

Phase 1 output MUST produce identical results to the original COBOL for all valid inputs.
Phase 2 output MUST produce equivalent results through different mechanisms (database
queries returning the same data that file reads returned).

---

## Phase 1: Mechanical Rustification

**Goal**: COBOL-equivalent Rust running on Linux/Kubernetes. Progressively more idiomatic
while maintaining strict behavioral equivalence.

### 1.1 Tier 1: Cosmetic Cleanup (Fully Automatable)

Mechanical pattern replacements with zero semantic risk. Can be enabled by default.

| Transformation | Method | Risk |
|---|---|---|
| Literal constant extraction | Codegen pass: collect `"N".parse::<Decimal>()` -> `const` | None |
| Display trait simplification | `impl Display` on types, replace verbose call pattern | None |
| Dead field elimination | Static analysis: find write-only WorkingStorage fields | None |
| Unused import/allow cleanup | Replace blanket `#![allow(...)]` with targeted allows | None |

**Decision level**: Level 1 (static analysis). No human input needed.

### 1.2 Tier 2: Type Promotion (Per-Field Safety Analysis)

Each field is analyzed against safety criteria. Promotion only when ALL criteria pass.

**PicX -> String** (safe when):
- No reference modification
- No REDEFINES overlay on or by this field
- No group-level MOVE treating it as raw bytes
- No byte-level comparison depending on space-padding

**PackedDecimal -> i64** (safe when):
- Scale is 0 (no fractional digits)
- No overflow-dependent behavior
- No REDEFINES over the field
- Precision fits in i64 (up to 18 digits)

**PackedDecimal -> Decimal** (safe when):
- No REDEFINES over the field
- Program doesn't depend on PIC-specific truncation

**Local variable extraction** (safe when):
- Field used within a single paragraph only
- Not passed across PERFORM calls
- Not part of a REDEFINES group
- Not referenced by CALL BY REFERENCE

**Decision level**: Level 1-2 (static analysis + rules). Automatable with safety report.

### 1.3 Tier 3: Structural Transformation

Requires understanding control flow and data access patterns at a program level.

#### 1.3.1 Paragraph Dispatcher Elimination

| Program Pattern | Feasibility | Approach |
|---|---|---|
| No GO TO, no PERFORM THRU | ~40% of programs | Direct sequential calls |
| Simple GO TO (end-of-program only) | ~20% | Case-by-case rewrite |
| Complex GO TO / ALTER / PERFORM THRU | ~40% | Keep dispatcher |

**Decision level**: Level 2-3 (rules + graph analysis).

#### 1.3.2 WorkingStorage Decomposition

Break flat struct into domain structs using:
- Level-01/05 group boundaries (structural hint)
- Field co-access patterns (which fields are always used together)
- Naming conventions (prefix-based grouping)

**Decision level**: Level 3-4 (heuristics + human validation).

#### 1.3.3 Status Flags -> Enum/Result

Level-88 conditions provide value names. Control flow analysis determines
whether `Result`, `enum`, or `bool` is the right Rust type.

**Decision level**: Level 3-4 (heuristics + human judgment).

### 1.4 Monster File Decomposition

For files exceeding 100K lines (and especially the 500K-1M line cases), the
transpiled Rust file is unusable as-is. Decomposition is mandatory.

#### Step 1: Dependency Graph Extraction (Level 1 -- Static Analysis)

Extract from the COBOL AST:

```
PARAGRAPH CALL GRAPH:
  For each paragraph P:
    performs(P)  = set of paragraphs P calls via PERFORM
    goto_targets(P) = set of paragraphs P jumps to via GO TO
    sections(P) = containing SECTION

DATA DEPENDENCY GRAPH:
  For each paragraph P:
    reads(P)  = set of data items P reads
    writes(P) = set of data items P writes

FILE USAGE GRAPH:
  For each paragraph P:
    opens(P)  = files opened
    reads_file(P)  = files read from
    writes_file(P) = files written to
    closes(P) = files closed
```

Output: Three graphs as adjacency lists + JSON for tooling consumption.

#### Step 2: Strongly Connected Components (Level 1 -- Graph Algorithm)

Compute SCCs on the paragraph call graph (Tarjan's algorithm).

- Each SCC = a set of paragraphs that call each other (cycles from GO TO)
- SCCs are irreducible units -- they MUST stay together
- The condensation graph (DAG of SCCs) reveals the module structure

#### Step 3: Graph Partitioning (Level 3 -- Heuristic Algorithm)

Partition the condensation DAG into modules that maximize:

```
Quality(partition) = sum(cohesion(module)) / sum(coupling(module_pair))

Where:
  cohesion(M)       = |data items shared within M| / |total data items in M|
  coupling(M1, M2)  = |data items shared between M1 and M2| / |total data items|
```

Algorithms (in order of simplicity):
1. **SECTION-based**: Use existing COBOL SECTION boundaries as initial partition
2. **Prefix-based**: Group paragraphs by naming prefix (INIT-, PROCESS-, REPORT-)
3. **Community detection**: Louvain algorithm on the bipartite (paragraph, field) graph
4. **Spectral clustering**: For cases where naming and sections are misleading

Start with (1), score it, try (2) if score is poor, escalate to (3)/(4) only if needed.

#### Step 4: Interface Extraction (Level 2 -- Rule-Based)

For each module boundary:
- Identify fields that cross the boundary (read in one module, written in another)
- Those fields become the module's interface struct
- Internal-only fields become module-private
- PERFORM calls across modules become function calls with explicit parameters

#### Step 5: Code Generation (Level 2 -- Codegen)

Generate Rust module structure:
```
src/
  mod.rs              -- program entry, shared state structs
  initialization.rs   -- INIT-* paragraphs
  processing.rs       -- PROCESS-* paragraphs
  reporting.rs        -- REPORT-* paragraphs
  error_handling.rs   -- ERROR-* paragraphs
  shared_data.rs      -- cross-module data structs
```

**Target for procedure code**: Each module 500-2000 lines of logic. No module
exceeds 5000 lines of procedure code.

**Data definitions are exempt**: A copybook with 8000 attributes generates a large
struct. This struct MUST remain intact in Phase 1 -- it IS the record layout, and
splitting it would break REDEFINES overlays, group-level MOVEs, and byte-offset
calculations. The struct may be large (thousands of lines) but it is purely
declarative -- no logic, no complexity, just field definitions.

Data struct decomposition happens in Phase 2 when the data model is redesigned
for database storage. In Phase 1, the monolithic struct is correct and necessary.

### 1.5 Kubernetes Deployment Model

| COBOL Pattern | K8s Equivalent |
|---|---|
| Batch program (JCL) | K8s Job or CronJob |
| Online program (CICS) | K8s Deployment + Service |
| File I/O (DD statements) | PersistentVolumeClaim or S3/MinIO |
| SORT utility | In-process (cobol-sort crate) or external sort |
| DB2 access (EXEC SQL) | Connection pool to PostgreSQL/DuckDB |
| MQ access | NATS, Kafka, or RabbitMQ client |

### 1.6 Phase 1 Validation

**Regression suite**: Stress tests (currently 46/47 passing) extended per program.

**Equivalence testing**:
- Same input files -> same output files (byte-for-byte)
- Same DISPLAY output sequence
- Same file status codes
- Same RETURN-CODE / SQLCODE values

**Decomposition validation**:
- Decomposed version produces identical output to monolithic version
- All cross-module calls resolve correctly
- No data races from shared state

---

## Phase 2: Data Model Modernization

**Goal**: Transform COBOL file-based data models into modern database schemas.
This is an ARCHITECTURE transformation, not a code transformation.

### 2.1 The Problem Space

A typical COBOL data model (e.g., cardholder master):

```
Characteristics:
- Single flat file, 32KB fixed-width records
- 8000 attributes in one record
- COMP-3 (packed decimal) for monetary/numeric fields
- COMP (binary) for counters/indices
- Multi-level REDEFINES (union types via byte reinterpretation)
- OCCURS (embedded arrays)
- Programs read/modify/rewrite entire records
- No explicit relationships -- implied by matching key fields
```

Target:
```
- Relational tables or document collections
- Normalized or strategically denormalized
- Native types (DECIMAL, INTEGER, VARCHAR, TIMESTAMP)
- Explicit relationships (foreign keys, references)
- Set-oriented access (SQL queries, aggregation pipelines)
```

### 2.1.1 Reference Scale (Credit Card Domain)

Based on real-world calibration from a production cardholder master file:

```
Source:  1 flat file, 32KB records, 8000 attributes
Target: ~70 logical entities (domain expert estimate)

Expected distribution:
  5-10 core entities      300-500 fields each   (cardholder, account, product, pricing)
  20-30 medium entities    50-150 fields each   (billing, rewards, disputes, fraud, auth)
  30-40 small entities     10-50 fields each    (status codes, audit, flags, lookups)

Average: ~114 attributes per entity
```

This distribution is typical of mature financial systems where 30 years of
feature accretion packs ever more attributes into a single master record
rather than adding new files (because adding files required JCL changes,
VSAM catalog updates, and operations team coordination -- so developers
added fields to existing records instead).

**Tooling implications**:
- Co-access clustering must handle 8000-node, 70-cluster problems (tractable)
- Domain experts already know the ~70 entities -- tooling CONFIRMS, DISCOVERS
  what they've forgotten, and QUANTIFIES what they intuit
- The hardest entities are the core 5-10 with 300-500 fields: these have the
  most REDEFINES, the deepest OCCURS nesting, and the most cross-program usage
- Small entities (lookup codes, flags) are the easiest wins -- migrate first

### 2.2 Discovery Layer (Automated -- Level 1-2)

All outputs are generated by tooling from COBOL source. No human input.

#### 2.2.1 Record Anatomy

Parse the copybook/DATA DIVISION to produce:

```
FIELD CATALOG:
  For each field:
    - name, level, offset, length, type (PIC clause)
    - storage format (DISPLAY, COMP, COMP-3, COMP-5)
    - parent group, REDEFINES target, OCCURS count
    - OCCURS DEPENDING ON reference (if variable)
    - Level-88 condition values

RECORD MAP:
  Byte-by-byte layout showing field boundaries, overlaps (REDEFINES),
  and array regions (OCCURS). Visual output for human review.

REDEFINES CATALOG:
  For each REDEFINES chain:
    - Base field and all variants
    - Byte range shared
    - Type interpretation per variant
    - Candidate discriminator field (detected from IF/EVALUATE patterns)

OCCURS CATALOG:
  For each OCCURS:
    - Fixed count or DEPENDING ON field
    - Element structure (nested fields)
    - Maximum and typical usage (from runtime data if available)
```

#### 2.2.2 Field Usage Matrix

Cross-reference every field against every program in the codebase:

```
USAGE MATRIX (programs x fields):
  For each (program, field) pair:
    - R  = program reads this field
    - W  = program writes this field
    - RW = both
    - .  = not used

  Derived metrics per field:
    - read_count:  number of programs that read it
    - write_count: number of programs that write it
    - total_count: read_count + write_count
    - classification: CORE (>50% programs), COMMON (10-50%), NICHE (<10%), DEAD (0%)
```

#### 2.2.3 Co-Access Clustering

Fields that are always accessed together likely belong to the same entity.

```
CO-ACCESS MATRIX (fields x fields):
  Jaccard similarity of access patterns:
    similarity(F1, F2) = |programs_using_both| / |programs_using_either|

  Cluster using hierarchical clustering with similarity threshold.
  Output: candidate entity groups with cohesion scores.
```

#### 2.2.4 Dead Field Detection

```
DEAD FIELDS:
  Fields with total_count = 0 across ALL programs.

  Caution: verify against:
  - JCL/batch scripts that reference fields by offset (no symbolic name)
  - COPY members used by external systems
  - Fields populated by CICS MAP definitions

  Classification:
  - CONFIRMED DEAD: no reference anywhere, no external system dependency
  - SUSPECTED DEAD: no program reference, but in a COPY used externally
  - RETAINED: explicitly marked as required by domain expert
```

### 2.3 Classification Layer (Level 2-3, Human Validated)

#### 2.3.1 Field Classification Taxonomy

| Category | Definition | Detection Method | Modernization Path |
|---|---|---|---|
| **Identity** | Primary/alternate keys | INDEXED BY, SEARCH, key in READ/WRITE | Primary key / unique constraint |
| **Reference** | Lookup/code values | Read-only, Level-88 conditions | Lookup table or enum |
| **Transactional** | Frequently changing state | High write_count, in COMPUTE/ADD/SUBTRACT | Column in main table |
| **Derived** | Computable from other fields | Write-only, preceded by COMPUTE | View or computed column |
| **Temporal** | Date/time fields | Naming pattern (*-DATE, *-TIME), PIC 9(8) | DATE/TIMESTAMP column |
| **Monetary** | Currency amounts | COMP-3 with V99, in arithmetic operations | DECIMAL(p,s) column |
| **Counter** | Accumulation fields | In ADD/SUBTRACT, COMP/COMP-3, integer PIC | INTEGER/BIGINT column |
| **Flag** | Boolean/status indicators | PIC X or PIC 9, Level-88 conditions | BOOLEAN or ENUM |
| **Text** | Descriptive/name fields | PIC X(n) with large n, in STRING/DISPLAY | VARCHAR(n) |
| **Blob** | Opaque data regions | Large PIC X(n), no individual field access | BYTEA/BLOB |
| **Dead** | Never accessed | total_count = 0 | Remove (after validation) |

**Detection**: Levels 1-3 (static analysis + rules + heuristics).
**Validation**: Level 4 (human domain expert confirms classifications).

#### 2.3.2 Entity Boundary Detection

Candidate entity boundaries come from multiple signals:

```
Signal 1: COBOL Level Structure (weight: 0.3)
  Level-01 groups are natural entity candidates.
  Level-05 sub-groups may be sub-entities.

Signal 2: Naming Conventions (weight: 0.2)
  Common prefix = likely same entity.
  e.g., CUST-NAME, CUST-ADDR, CUST-PHONE -> Customer entity

Signal 3: Co-Access Clustering (weight: 0.3)
  Fields always used together = same entity.
  High Jaccard similarity cluster = entity candidate.

Signal 4: REDEFINES Boundaries (weight: 0.2)
  REDEFINES chain = variant type, a single logical entity
  with multiple physical representations.

Combined score:
  entity_score(group) = sum(signal_weight * signal_score)
  Threshold: 0.6 for automatic entity proposal
  Below 0.6: flagged for human review
```

**Detection**: Level 3 (heuristic scoring).
**Validation**: Level 4 (human reviews all proposed entities).

### 2.4 REDEFINES Resolution Patterns

REDEFINES is the single hardest problem in data model modernization. Each pattern
requires a different resolution strategy.

#### Pattern A: Discriminated Union (Most Common)

```cobol
05 RECORD-TYPE       PIC X.
05 RECORD-DATA       PIC X(100).
05 CREDIT-DATA REDEFINES RECORD-DATA.
   10 CREDIT-LIMIT   PIC S9(9)V99 COMP-3.
05 DEBIT-DATA REDEFINES RECORD-DATA.
   10 DAILY-LIMIT    PIC S9(7)V99 COMP-3.
```

Detection: IF/EVALUATE on discriminator field determines which variant is active.

Resolution options (human decides):
| Option | When to Use | Trade-off |
|---|---|---|
| Separate tables per variant | Variants have very different structures | More tables, JOINs needed |
| Single table, nullable columns | Variants share most fields | NULLs, wider rows |
| Table + JSON variant column | Variant data is rarely queried | Flexible but less queryable |
| Inheritance (PostgreSQL) | Clean type hierarchy | DB-specific feature |

#### Pattern B: Format Conversion (Same Data, Different View)

```cobol
05 AMOUNT-DISPLAY    PIC Z(7)9.99.
05 AMOUNT-COMP REDEFINES AMOUNT-DISPLAY PIC S9(9)V99 COMP-3.
```

Detection: Same logical value, different physical representation.

Resolution: Single column with the appropriate modern type. The REDEFINES disappears.

#### Pattern C: Memory Reuse (Unrelated Fields Sharing Space)

```cobol
05 TEMP-AREA         PIC X(200).
05 ERROR-MESSAGE REDEFINES TEMP-AREA PIC X(200).
05 SORT-KEY REDEFINES TEMP-AREA PIC X(200).
```

Detection: No discriminator, fields used in different code paths, never simultaneously.

Resolution: Separate columns (or local variables if scope is narrow enough).

#### Pattern D: Byte-Level Manipulation

```cobol
05 PACKED-AMOUNT     PIC S9(7)V99 COMP-3.
05 PACKED-BYTES REDEFINES PACKED-AMOUNT PIC X(5).
```

Detection: One variant is a raw byte view of another.

Resolution: Single column. The byte-level view is a serialization concern, not a data model concern.

### 2.5 Logical Data Model (Database-Agnostic)

The entity model is defined independent of any target database. This allows the
same analysis to produce RDBMS schemas, document collections, or hybrid targets.

#### 2.5.1 Entity Model Definition (LDM)

Each entity in the Logical Data Model contains:

```
Entity:
  name:           string          -- business name (e.g., "Cardholder")
  source_fields:  [FieldRef]      -- which COBOL fields map here
  attributes:     [Attribute]     -- target attributes (type-neutral)
  identity:       [Attribute]     -- primary key / document ID
  relationships:  [Relationship]  -- links to other entities
  variant_of:     EntityRef?      -- if from REDEFINES (parent entity + discriminator)
  constraints:    [Constraint]    -- business rules (from Level-88, EVALUATE patterns)
  source_offset:  (start, end)    -- byte range in original record

Attribute:
  name:           string          -- target field name
  logical_type:   LogicalType     -- database-neutral type
  source_field:   FieldRef        -- COBOL field it came from
  nullable:       bool            -- can this be absent?
  default:        Value?          -- default value (from VALUE clause)
  classification: FieldClass      -- identity|reference|transactional|derived|...

LogicalType (database-neutral):
  Integer(precision)              -- whole numbers
  Decimal(precision, scale)       -- exact decimal (monetary, etc.)
  Float(bits)                     -- approximate (COMP-1/COMP-2)
  Text(max_length)                -- variable-length string
  FixedText(length)               -- fixed-width (preserves COBOL semantics if needed)
  Boolean                         -- true/false (from Level-88 with 2 values)
  Enum(values)                    -- from Level-88 with >2 values
  Date(format)                    -- date with format hint
  Time(format)                    -- time with format hint
  Timestamp(format)               -- combined date+time
  Binary(length)                  -- opaque bytes
  Array(element_type, max_count)  -- from OCCURS (fixed)
  VarArray(element_type, count_ref) -- from OCCURS DEPENDING ON
  Variant(discriminator, options) -- from REDEFINES

Relationship:
  target:         EntityRef       -- related entity
  cardinality:    1:1 | 1:N | N:M
  source:         string          -- why this relationship exists
                                  -- (shared key, OCCURS, REDEFINES, domain rule)
```

#### 2.5.2 Target Generators

From a single LDM, generate target-specific artifacts:

**RDBMS Generator (PostgreSQL, MySQL, Oracle)**:
```
Entity         -> CREATE TABLE
Attribute      -> Column with SQL type
  Integer      -> INTEGER / BIGINT
  Decimal      -> DECIMAL(p,s) / NUMERIC(p,s)
  Text         -> VARCHAR(n) / TEXT
  Boolean      -> BOOLEAN
  Enum         -> ENUM type or lookup table
  Date/Time    -> DATE / TIME / TIMESTAMP
  Binary       -> BYTEA / BLOB
  Array        -> Child table with FK (normalized)
  VarArray     -> Child table with FK
  Variant      -> Strategy per REDEFINES pattern (see 2.4)
Relationship   -> FOREIGN KEY constraint
Constraint     -> CHECK constraint or trigger
```

**Document DB Generator (MongoDB, DynamoDB)**:
```
Entity         -> Collection
Attribute      -> Field in document
  Integer      -> NumberInt / NumberLong
  Decimal      -> Decimal128
  Text         -> String
  Boolean      -> Boolean
  Enum         -> String (with validation)
  Date/Time    -> ISODate
  Binary       -> BinData
  Array        -> Embedded array (denormalized)
  VarArray     -> Embedded array
  Variant      -> Embedded sub-document (natural fit for document DBs)
Relationship   -> Embedded document (1:1, 1:few) or reference (1:N, N:M)
Constraint     -> JSON Schema validation
```

**Hybrid Generator**:
Some entities map better to relational (highly normalized reference data,
entities with complex joins), others to document (entities with deep nesting,
variants, or arrays). The LDM supports per-entity target annotation:

```
Entity annotations:
  target: rdbms              -- force relational
  target: document           -- force document
  target: auto               -- generator picks based on structure
                              -- (REDEFINES/OCCURS-heavy -> document,
                              --  flat/reference -> relational)
```

#### 2.5.3 DAL Generation (Database-Agnostic Interface)

The generated Data Access Layer uses a trait/interface pattern so application
code is independent of the target database:

```rust
// Generated trait -- application code uses this
pub trait CardholderRepository {
    fn find_by_key(&self, card_number: &str) -> Result<Cardholder>;
    fn update(&self, record: &Cardholder) -> Result<()>;
    fn insert(&self, record: &Cardholder) -> Result<()>;
    fn delete(&self, card_number: &str) -> Result<()>;
    fn find_by_criteria(&self, criteria: &CardholderQuery) -> Result<Vec<Cardholder>>;
}

// Generated implementation -- swappable
pub struct PostgresCardholderRepo { pool: PgPool }
pub struct MongoCardholderRepo { collection: Collection<Cardholder> }

impl CardholderRepository for PostgresCardholderRepo { ... }
impl CardholderRepository for MongoCardholderRepo { ... }
```

This allows:
- Running the same application against different databases
- Migration testing (compare Postgres vs Mongo results)
- Gradual migration (some entities on RDBMS, others on document DB)

### 2.6 OCCURS Resolution Patterns

| COBOL Pattern | Detection | Target Model |
|---|---|---|
| Fixed OCCURS, few elements | `OCCURS n` with n <= 10 | Flattened columns (PHONE-1, PHONE-2, ...) |
| Fixed OCCURS, many elements | `OCCURS n` with n > 10 | Child table with foreign key |
| Variable OCCURS (DEPENDING ON) | `OCCURS ... DEPENDING ON` | Child table (row count = DEPENDING ON value) |
| Nested OCCURS | OCCURS within OCCURS | Child table with composite key |
| OCCURS with REDEFINES | Array elements have variants | Child table with type discriminator |

**Decision**: Level 3-4 (heuristics propose, human validates).

### 2.7 Type Mapping Rules (Fully Automatable -- Level 2)

| COBOL Type | Condition | SQL Type | Notes |
|---|---|---|---|
| PIC 9(n) COMP-3 | n <= 9, scale 0 | INTEGER | |
| PIC 9(n) COMP-3 | n <= 18, scale 0 | BIGINT | |
| PIC S9(p)V9(s) COMP-3 | any | DECIMAL(p+s, s) | Exact mapping |
| PIC 9(n) COMP / COMP-5 | n <= 4 | SMALLINT | |
| PIC 9(n) COMP / COMP-5 | n <= 9 | INTEGER | |
| PIC 9(n) COMP / COMP-5 | n <= 18 | BIGINT | |
| COMP-1 | | REAL | Single-precision float |
| COMP-2 | | DOUBLE PRECISION | Double-precision float |
| PIC X(n) | n <= 255 | VARCHAR(n) | |
| PIC X(n) | n > 255 | TEXT | |
| PIC 9(8) (date pattern) | naming: *-DATE | DATE | Requires format analysis |
| PIC 9(6) (time pattern) | naming: *-TIME | TIME | Requires format analysis |
| Level-88 with 2 values | e.g., Y/N | BOOLEAN | |
| Level-88 with >2 values | | ENUM or lookup table | |

### 2.8 Migration Execution

#### 2.7.1 Data Migration Pipeline

```
Phase A: Extract
  - Read COBOL flat file using generated record reader
  - Record reader understands exact byte layout (from copybook)
  - Handles COMP-3 unpacking, COMP byte-swapping, EBCDIC-to-UTF8
  - Outputs: NDJSON with one object per record, all fields named

Phase B: Transform
  - Apply entity decomposition (split monolithic record into entities)
  - Apply type conversions (COMP-3 -> decimal string, PIC X -> trimmed string)
  - Resolve REDEFINES (read discriminator, select active variant)
  - Expand OCCURS (generate child records with parent key)
  - Apply business rules (default values, constraint enforcement)
  - Outputs: NDJSON per target entity

Phase C: Load
  - Bulk insert into target database (COPY for PostgreSQL, mongoimport for MongoDB)
  - Apply constraints and indices after load
  - Generate row counts and checksums per table

Phase D: Reconcile
  - Source record count == sum of target entity record counts
  - Key field values match between source and target
  - Monetary field sums match (account for precision differences)
  - Sample-based deep comparison (random N records, all fields)
```

**Implementation**: cobol2rust tooling generates the Extract and Transform code
from the copybook + entity mapping. Load and Reconcile are standard database operations.

#### 2.7.2 Dual-Run Strategy

During transition, both old (COBOL/file) and new (Rust/database) systems run
in parallel on the same inputs:

```
Input -> COBOL system -> Output A (files)
      -> Rust system  -> Output B (database -> exported files)

Compare: Output A == Output B?

Phase 1: Shadow mode (Rust runs, output discarded, diffs logged)
Phase 2: Parallel mode (both outputs used, diffs are incidents)
Phase 3: Cutover (COBOL system retired, Rust is primary)
```

Duration of each phase depends on diff rate trending to zero.

### 2.9 Code Transformation (File I/O -> Database)

Once the data model is in a database, the Rust code must change:

```
COBOL pattern:           Rust Phase 1:              Rust Phase 2:
OPEN INPUT FILE    ->    file.open()?          ->   let conn = pool.get()?
READ FILE INTO REC ->    file.read(&mut rec)?  ->   SELECT * FROM entity WHERE key = ?
MOVE X TO REC-FIELD ->   rec.field = x         ->   UPDATE entity SET field = ? WHERE key = ?
WRITE REC          ->    file.write(&rec)?     ->   INSERT INTO entity (...) VALUES (...)
CLOSE FILE         ->    file.close()?         ->   conn.release()
```

This transformation is per-file-definition, not per-program. A data access layer (DAL)
is generated per entity:

```rust
// Generated DAL for cardholder entity
pub struct Cardholder {
    pub card_number: String,
    pub customer_name: String,
    pub credit_limit: Decimal,
    // ... mapped fields
}

impl CardholderRepository {
    pub fn find_by_key(&self, card_number: &str) -> Result<Cardholder>;
    pub fn update(&self, record: &Cardholder) -> Result<()>;
    pub fn insert(&self, record: &Cardholder) -> Result<()>;
}
```

Programs are updated to use the DAL instead of file I/O. This is a mechanical
replacement of file operations with repository calls -- automatable at Level 2.

---

## Tooling Requirements

### What cobol2rust Must Provide

| Capability | Phase | Status |
|---|---|---|
| COBOL parser (AST) | Both | Done |
| Copybook parser (field catalog) | Both | Done |
| Transpiler (COBOL -> Rust) | Phase 1 | Done |
| Dependency graph extraction | Phase 1 | Planned |
| Graph partitioning / module decomposition | Phase 1 | Planned |
| Field usage matrix generation | Phase 2 | Planned |
| Co-access clustering | Phase 2 | Planned |
| REDEFINES analysis (discriminator detection) | Phase 2 | Planned |
| Type mapping engine | Phase 2 | Planned |
| DDL generation (from entity mapping) | Phase 2 | Planned |
| Record reader generation (for data migration) | Phase 2 | Planned |
| DAL generation (repository pattern) | Phase 2 | Planned |
| Reconciliation report generation | Phase 2 | Planned |

### What External Tools Are Needed

| Tool | Purpose |
|---|---|
| PostgreSQL / MongoDB | Target database |
| DuckDB | Analysis database (NDJSON queries, field usage analysis) |
| Graphviz / D3.js | Dependency graph visualization |
| K8s / Helm | Deployment orchestration |

---

## Risk Framework

### Risk Classification

| Risk Level | Definition | Mitigation |
|---|---|---|
| **None** | Transformation is provably equivalent | Automated validation |
| **Low** | Transformation has clear safety criteria, all checkable | Safety analysis + test |
| **Medium** | Transformation requires per-field/per-program analysis | Analysis report + human review |
| **High** | Transformation changes architecture, requires domain knowledge | Dual-run + reconciliation |
| **Critical** | Transformation affects data integrity or financial calculations | Full parallel run, field-level reconciliation, sign-off |

### Decision Gates

Before proceeding to the next stage, ALL criteria must pass:

```
Gate 1 (Phase 1 -> Tier 2): All stress tests pass, corpus transpile >95% success
Gate 2 (Tier 2 -> Tier 3): Type promotions validated, no behavioral change detected
Gate 3 (Phase 1 -> Phase 2): Phase 1 system running in production equivalent
Gate 4 (Phase 2 design -> Phase 2 execution): Entity model reviewed by domain expert
Gate 5 (Phase 2 execution -> cutover): Dual-run diff rate < 0.001% for 30 days
```

---

## AI Usage Policy

### When AI Is Permitted

AI may be used as an ADVISORY tool (Level 5) only when Levels 1-4 are insufficient:

| Task | AI Role | Human Role |
|---|---|---|
| Entity naming from field patterns | Suggest names | Approve/reject each name |
| Business rule extraction from nested IF/EVALUATE | Identify candidate rules | Validate against domain knowledge |
| Documentation generation | Draft descriptions | Review and correct |
| Ambiguous normalization decisions | Present options with trade-offs | Make final decision |

### When AI Is NOT Permitted

- Data migration correctness decisions (field mapping, type conversion)
- Schema design sign-off (human domain expert only)
- Production cutover decisions (human operational decision)
- Financial calculation equivalence validation (deterministic tooling only)

### AI Output Validation Rule

Every AI-generated artifact MUST be:
1. Reviewed by a human before use
2. Validated against deterministic tooling output where possible
3. Marked as "AI-assisted" in documentation for traceability

---

## Document Inventory

This framework produces the following artifacts at each stage:

| Stage | Artifact | Generator |
|---|---|---|
| Discovery | Field Catalog (JSON) | cobol2rust tooling |
| Discovery | Record Map (visual) | cobol2rust tooling |
| Discovery | REDEFINES Catalog | cobol2rust tooling |
| Discovery | Field Usage Matrix | cobol2rust tooling |
| Discovery | Co-Access Clusters | cobol2rust tooling |
| Discovery | Dependency Graphs | cobol2rust tooling |
| Classification | Classification Matrix | Tooling + human review |
| Classification | Entity Boundary Proposals | Tooling (heuristic) |
| Classification | Dead Field Report | Tooling + human validation |
| Strategy | Modernization Strategy Document | Human (informed by tooling) |
| Design | Target Entity Model (ERD) | Human |
| Design | Type Mapping Table | Tooling (auto-generated) |
| Design | REDEFINES Resolution Decisions | Human |
| Design | DDL Scripts | Tooling (from entity model) |
| Execution | Data Migration Pipeline | Tooling (generated) |
| Execution | DAL Code (repository pattern) | Tooling (generated) |
| Validation | Reconciliation Report | Tooling (automated) |
| Validation | Dual-Run Diff Report | Tooling (automated) |
| Validation | Sign-Off Checklist | Human |
