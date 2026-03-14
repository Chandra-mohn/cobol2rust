# cobol2rust + nexflow Integration Model

## The Core Insight

> **nexflow DSL is the design surface where legacy meets future.**

cobol2rust Phase 2 performs comprehensive analysis of legacy COBOL and expresses
everything it finds as nexflow DSL. The DSL is human-readable, concise, and
precise -- enabling domain experts to validate legacy behavior AND evolve the
architecture toward modern patterns, all in one artifact.

## The End-to-End Flow

```
COBOL source (e.g., 1M-line monolith)
    |
    v
cobol2rust Phase 1 --> "cobol-in-rust" --> PRODUCTION (mainframe exit)
    |
    v
cobol2rust Phase 2 ANALYSIS
    |  (parser, AST, dependency graphs, SCC detection,
    |   data affinity clustering, rule extraction, LDM)
    |
    v
cobol2rust Phase 2 DSL GENERATION (comprehensive extraction)
    |
    |  .schema  -- 70 entity definitions (from LDM + entity boundaries)
    |  .rules   -- business rules (from EVALUATE/IF chain extraction)
    |  .xform   -- data transforms (from COMPUTE/arithmetic extraction)
    |  .proc    -- ~400 processes (from paragraph clustering + SCC)
    |  .infra   -- infrastructure bindings (from file/resource analysis)
    |
    v
nexflow DSL = THE DESIGN SURFACE
    |
    |  SME validates: "is this what the legacy system does?"
    |  SME evolves:   "this is what the modern system should do"
    |  Architect:     "split this process, add this stream, use this DB"
    |
    v
APPROVED nexflow DSL (modernization specification)
    |
    v
nexflow compiler --> target code --> PRODUCTION (modern system)
    |
    +-- Rust code (batch services, compute-intensive processing)
    +-- Java/Flink (stream processing)
    +-- Kafka schemas (event backbone)
    +-- Database DDL (PostgreSQL, MongoDB)
```

## Why Two Outputs in Phase 2

Phase 1 Rust contains only legacy behavior. It is backward-looking -- a faithful
reproduction of what COBOL did. It cannot express architectural evolution.

The nexflow DSL captures legacy behavior AND enables forward-looking changes:
- Monolith splitting into modular processes
- Batch-to-streaming migration where appropriate
- New business rules the business wanted but couldn't implement in COBOL
- Modern integration patterns (events, APIs, async processing)
- Multi-database routing (relational, document, streaming)

The DSL is where the SME's domain knowledge combines with the architect's
technical vision. Neither Rust code nor COBOL source provides this design surface.

## What cobol2rust Phase 2 Extracts into nexflow DSL

### .schema (Entity Definitions from LDM)

cobol2rust's analysis produces the Logical Data Model (LDM) -- ~70 entities
from the 8000-attribute cardholder master. Each entity becomes a `.schema` file.

```
Source: 8000-field COBOL copybook
Analysis: co-access clustering, REDEFINES resolution, naming patterns
Output: 70 .schema files

Example -- Cardholder.schema:

schema Cardholder {
    card_number:     string      @key
    customer_name:   string
    credit_limit:    decimal(9,2)
    current_balance: decimal(9,2)
    card_status:     string      @enum["ACTIVE", "CLOSED", "SUSPENDED", "LOST"]
    issue_date:      date
    expiry_date:     date
}
```

This is what the SME validates. 10 lines vs 200 lines of COBOL copybook
definitions with COMP-3, REDEFINES, and OCCURS.

**Process-type independent**: Schema definitions are the same whether the
consuming process is batch, streaming, or microservice.

### .rules (Business Rules from EVALUATE/IF Extraction)

cobol2rust extracts decision logic from nested EVALUATE/IF chains and
expresses them as nexflow decision tables.

```
Source: 500-line nested IF/EVALUATE in COBOL paragraph
Analysis: condition extraction, threshold detection, action mapping
Output: .rules file

Example -- credit_limit_rules.rules:

ruleset CreditLimitDecision {
    rule standard_approval {
        when application.income > 50000
        and  application.credit_score > 700
        and  application.existing_debt_ratio < 0.4
        then approve with_limit income * 0.3
        priority high
    }

    rule conditional_approval {
        when application.credit_score between 600 and 700
        then approve with_limit 5000
        with review_required
        priority medium
    }

    rule decline {
        when application.credit_score < 600
        or   application.existing_debt_ratio > 0.6
        then decline reason "CREDIT_RISK"
        priority low
    }
}
```

An SME reads this in 30 seconds and says "yes, these are our credit rules"
or "no, the threshold changed to 650 last quarter." They cannot do this with
200 lines of nested COBOL IF statements or 150 lines of Rust match arms.

**Process-type independent**: Rules are the same whether evaluated in a batch
job, a stream processor, or a REST API handler.

### .xform (Data Transforms from COMPUTE/Arithmetic Extraction)

cobol2rust extracts field-level transformations from COMPUTE, MOVE, ADD,
SUBTRACT, MULTIPLY, DIVIDE operations.

```
Source: COBOL COMPUTE and arithmetic statements
Analysis: expression extraction, field dependency mapping
Output: .xform file

Example -- interest_calculation.xform:

transform InterestCalculation {
    input:  Account
    output: Account

    map daily_rate    = account.annual_rate / 365
    map daily_interest = account.current_balance * daily_rate
    map accrued_interest = account.accrued_interest + daily_interest
    map current_balance  = account.current_balance + daily_interest
        when account.interest_method == "COMPOUND"
}
```

**Process-type independent**: Transform logic is identical whether applied
in batch, streaming, or request-response context.

### .proc (Processes from Paragraph Clustering)

cobol2rust's SCC detection and paragraph clustering identifies discrete
processes within the monolith. Each cluster becomes a `.proc` file.

```
Source: 1M-line COBOL with ~3000 paragraphs
Analysis: call graph SCCs, data affinity, SECTION boundaries, naming patterns
Output: ~400 .proc files organized by domain

Example extraction:

1 monolith -->  monetary/           (40-50 processes)
                  payment_processing.proc
                  balance_calculation.proc
                  interest_accrual.proc
                  fee_assessment.proc
                  payment_reversal.proc
                  ...

                non_monetary/       (300-400 processes)
                  address_change.proc
                  card_replacement.proc
                  limit_adjustment.proc
                  name_change.proc
                  ...

                eod/                (20-30 processes)
                  daily_settlement.proc
                  aging_calculation.proc
                  statement_generation.proc
                  ...
```

**NOTE -- nexflow .proc extension needed**: The current nexflow `.proc` grammar
supports stream processing only. COBOL legacy processes include batch, request-
response, and scheduled patterns that do not map to streaming. The `.proc` DSL
needs extension to support multiple process types. See "nexflow Grammar
Evolution" section below.

### .infra (Infrastructure from File/Resource Analysis)

cobol2rust extracts file definitions (FD entries), database connections
(EXEC SQL), and external system references (CALL).

```
Source: COBOL FD entries, EXEC SQL CONNECT, CALL statements, JCL DD references
Analysis: resource inventory, I/O pattern detection
Output: .infra file (legacy bindings, to be evolved by DevOps)

Example -- legacy_bindings.infra:

environment legacy {
    files {
        cardholder_master {
            path: "/data/CARDHOLD.DAT"
            format: fixed_width
            record_length: 32768
        }
        transaction_log {
            path: "/data/TRANLOG.DAT"
            format: fixed_width
            record_length: 512
        }
    }
    databases {
        mainframe_db2 {
            type: db2
            dsn: "PROD.CARDDB"
        }
    }
}
```

The architect evolves this to modern infrastructure:

```
environment production {
    kafka {
        brokers: "kafka-prod:9092"
        security_protocol: SASL_SSL
    }
    streams {
        payment_events { topic: "prod.payments.inbound" }
        card_events { topic: "prod.cards.events" }
    }
    databases {
        account_db {
            type: postgresql
            host: "pg-prod.internal"
            database: "accounts"
        }
        card_profiles {
            type: mongodb
            uri: "mongodb://mongo-prod.internal/cards"
        }
    }
}
```

## nexflow Grammar Evolution (Design Decisions Needed)

The COBOL modernization use case surfaces requirements that the current
nexflow grammar does not cover. These are design decisions for the nexflow
project, documented here for traceability.

### .proc Process Type Extension

Current state: `.proc` supports stream processing (Flink sources, operators, sinks).

Required for COBOL modernization:

| Process Type | COBOL Origin | Example |
|---|---|---|
| `stream` | Real-time event processing (current nexflow) | Authorization, fraud detection |
| `batch` | Sequential file processing, batch jobs | EOD settlement, statement gen |
| `service` | Request-response (CICS transactions) | Account inquiry, balance check |
| `scheduled` | Timed batch execution (JCL scheduling) | Monthly billing, interest calc |

Possible grammar extension (conceptual, not final):

```
proc DailySettlement {
    type: batch                          // <-- NEW: process type
    schedule: daily at 23:00             // <-- NEW: for scheduled type

    from transaction_log
    apply SettlementRules rules
    transform SettlementCalculation
    update Account.balance
    emit to settlement_report
}

proc AccountInquiry {
    type: service                        // <-- NEW: request-response
    endpoint: /api/v1/accounts/{id}      // <-- NEW: for service type

    from request
    read Account where account_id == request.id
    apply AccountVisibility rules
    transform AccountResponse
    respond with AccountSummary
}
```

### File Source/Sink Concept

Current state: nexflow sources and sinks are Kafka topics and external services.

Required: file-based sources and sinks for batch processes that read/write
files (common in COBOL modernization, especially during transition period).

This is a nexflow design decision. Options:
1. Add `file` as a source/sink type in `.proc` grammar
2. Abstract files as named resources in `.infra` and reference by name
3. Treat file I/O as a special case of batch processing

### .schema Layer -- No Changes Needed

Entity definitions, types, enums, constraints -- all process-type agnostic.
No grammar changes required for COBOL modernization.

### .rules Layer -- No Changes Needed

Decision tables, validation rules, business logic -- all process-type agnostic.
No grammar changes required for COBOL modernization.

### .xform Layer -- No Changes Needed

Field mappings, calculations, data transformations -- all process-type agnostic.
No grammar changes required for COBOL modernization.

### .infra Layer -- Minor Extension

Need to support file-based resources and legacy database types (DB2) in
addition to Kafka, MongoDB, PostgreSQL. Primarily additive.

## The Compression Effect

The value of expressing legacy behavior in nexflow DSL:

| Artifact | COBOL Lines | Rust Lines | nexflow DSL Lines | Compression |
|---|---|---|---|---|
| 1 entity (100 fields) | ~400 | ~200 | ~30 | 13x |
| 1 business rule set | ~200 | ~150 | ~20 | 10x |
| 1 transform | ~50 | ~40 | ~5 | 10x |
| 1 process | ~660 | ~500 | ~15 | 44x |
| Full monolith (1M lines) | 1,000,000 | ~800,000 | ~5,000-10,000 | 100-200x |

A domain expert can review 5,000-10,000 lines of nexflow DSL in days.
They cannot review 800,000 lines of Rust in any reasonable timeframe.

## Integration Architecture Summary

```
cobol2rust:
  - Phase 1: COBOL -> Rust emulation (mainframe exit)
  - Phase 2 analysis: COBOL -> LDM, rules, transforms, process boundaries
  - Phase 2 DSL generation: analysis -> comprehensive nexflow DSL

nexflow:
  - Design surface: SMEs validate + evolve the DSL
  - Compiler: approved DSL -> production code (multi-target)

The products:
  cobol2rust UNDERSTANDS the legacy (deep COBOL analysis)
  nexflow SPECIFIES the future (human-readable, evolvable DSL)
  Together: automated legacy comprehension -> human-guided modernization
            -> deterministic code generation
```

## Phase 1 -> Phase 2 Continuity: #[cobol(...)] Attributes

Phase 1 and Phase 2 are ONE product, not two. Phase 2 operates on the Phase 1
Rust output -- it does not go back to the COBOL source.

The challenge: Phase 1 Rust loses structural information from COBOL (level
hierarchy, REDEFINES relationships, SECTION boundaries, Level-88 names, COPY
member origins). Phase 2 needs this information for nexflow DSL generation.

The solution: Phase 1 codegen embeds COBOL structural metadata as Rust
`#[cobol(...)]` attributes directly in the generated code. No sidecar metadata
files. No comments. The Rust source is self-describing.

### Attribute Specification

**On structs (record/copybook level):**

```rust
#[cobol(copy = "CARDHOLD", record_len = 32768)]
pub struct CardRecord {
    ...
}
```

| Attribute | Source | Purpose |
|---|---|---|
| `copy` | COPY member name | Identifies which copybook defined this record |
| `record_len` | Sum of field lengths | Total record width in bytes |

**On fields (data item level):**

```rust
#[cobol(level = 5, pic = "S9(9)V99", comp3, offset = 16)]
pub credit_limit: PackedDecimal,

#[cobol(level = 5, redefines = "card_data", discriminator = "card_type")]
pub credit_data: CreditData,

#[cobol(level = 5, level88 = "ACTIVE:A,CLOSED:C,SUSPENDED:S,LOST:L")]
pub card_status: PicX,

#[cobol(level = 5, occurs = 5)]
pub phone_numbers: [PicX; 5],

#[cobol(level = 5, occurs_depending = "phone_count", max_occurs = 10)]
pub phone_numbers: [PicX; 10],
```

| Attribute | Source | Purpose |
|---|---|---|
| `level` | COBOL level number (01-49, 77, 88) | Preserves hierarchy for entity detection |
| `pic` | PIC clause | Original precision, scale, format |
| `comp3` / `comp` / `comp5` | USAGE clause | Storage format |
| `offset` | Calculated byte offset | Position in record layout |
| `redefines` | REDEFINES clause | Overlay relationship |
| `discriminator` | Detected from IF/EVALUATE | Which field selects the active variant |
| `level88` | Level-88 conditions | Named values (business vocabulary) |
| `occurs` | OCCURS clause | Fixed array count |
| `occurs_depending` | OCCURS DEPENDING ON | Variable array with count field |

**On functions (paragraph level):**

```rust
#[cobol(section = "MONETARY", performs = "VALIDATE-AMOUNT,APPLY-PAYMENT",
        reads = "card_number,credit_limit", writes = "current_balance")]
fn process_payment(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    // ...
}
```

| Attribute | Source | Purpose |
|---|---|---|
| `section` | COBOL SECTION name | Process grouping for .proc generation |
| `performs` | PERFORM targets | Call graph for process boundary detection |
| `reads` | Static analysis: fields read | Data dependency for entity boundary detection |
| `writes` | Static analysis: fields written | Data dependency for entity boundary detection |

**On modules (COPY/SECTION level):**

```rust
#[cobol(section = "MONETARY-PROCESSING")]
mod monetary_processing {
    // paragraphs in this section
}
```

### Runtime Impact: Zero

The `#[cobol(...)]` attributes are inert at compile time. rustc ignores
unknown attributes on items (with `#[allow(unused_attributes)]`), or they
can be defined as no-op proc macros that expand to nothing. Zero runtime
overhead. Zero binary size impact.

### Phase 2 Reads Attributes via `syn`

Phase 2 tooling uses the `syn` crate (standard Rust source parser, 300M+
downloads) to parse the generated Rust and extract all `#[cobol(...)]`
attributes:

```rust
use syn::{parse_file, Item, Attribute};

let source = std::fs::read_to_string("src/main.rs")?;
let ast = parse_file(&source)?;

for item in &ast.items {
    match item {
        Item::Struct(s) => {
            // Extract struct-level #[cobol(...)] -> entity candidate
            // Extract field-level #[cobol(...)] -> LDM attributes
        }
        Item::Fn(f) => {
            // Extract #[cobol(section, performs, reads, writes)]
            // -> process boundaries, dependency graph
        }
        Item::Mod(m) => {
            // Extract #[cobol(section = "...")] -> process grouping
        }
        _ => {}
    }
}
```

No custom Rust parser needed. No ANTLR grammar. `syn` already does this.

### The Single-Artifact Pipeline

```
COBOL source
    |
    v
Phase 1 codegen: parse COBOL -> generate Rust WITH #[cobol(...)] attributes
    |
    v
Phase 1 output: Rust source (executable + self-describing)
    |                                |
    v                                v
rustc compiles                  Phase 2 reads via syn
(ignores attributes)            (extracts all structural info)
    |                                |
    v                                v
production binary               nexflow DSL generation
```

One artifact. No sidecar files. No going back to COBOL. Phase 1 produces
everything Phase 2 needs, embedded in the Rust source itself.

---

## What Needs to Be Built

### In cobol2rust Phase 1 (attribute-enriched codegen)

| Component | Purpose | Complexity |
|---|---|---|
| Struct-level `#[cobol(...)]` | copy, record_len on generated structs | Low |
| Field-level `#[cobol(...)]` | level, pic, offset, comp3, redefines, level88, occurs | Medium |
| Function-level `#[cobol(...)]` | section, performs, reads, writes on paragraphs | Medium |
| Data dependency analysis | Static analysis: which fields each paragraph reads/writes | High |
| No-op proc macro crate | `cobol-attr` crate so rustc doesn't warn on attributes | Low |

### In cobol2rust Phase 2 (DSL emitter, reads Phase 1 Rust via `syn`)

| Component | Purpose | Complexity |
|---|---|---|
| Rust attribute parser | Read Phase 1 Rust via `syn`, extract all `#[cobol(...)]` | Low |
| LDM builder | Level hierarchy + co-access clustering -> entity boundaries | High |
| LDM -> .schema emitter | Entity definitions from attribute-derived LDM | Medium |
| Rule extractor -> .rules emitter | EVALUATE/IF patterns (from reads/writes graph) to decision tables | High |
| Arithmetic extractor -> .xform emitter | COMPUTE/MOVE patterns to transform mappings | Medium |
| Process detector -> .proc emitter | Section + performs graph -> paragraph clusters to process defs | High |
| Resource analyzer -> .infra emitter | File/DB/external refs to infra bindings | Low |

Dependency: `syn` crate (Rust source parser, standard ecosystem crate)

### In nexflow (grammar + compiler evolution)

| Component | Purpose | Complexity |
|---|---|---|
| .proc type extension | batch, service, scheduled process types | Medium |
| File source/sink support | File-based I/O for batch processes | Medium |
| Rust code generation backend | Generate Rust from validated DSL | High |
| .infra legacy resource types | DB2, flat files, VSAM | Low |
