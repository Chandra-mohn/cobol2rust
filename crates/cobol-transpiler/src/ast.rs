//! Typed AST for COBOL programs.
//!
//! The AST is built from the ANTLR4 parse tree by listener walks.
//! It captures the semantically meaningful structure of a COBOL program
//! while discarding syntactic noise.

use serde::Serialize;
use std::fmt;

// ---------------------------------------------------------------------------
// Top-level program
// ---------------------------------------------------------------------------

/// A complete COBOL compilation unit.
#[derive(Debug, Clone, Serialize)]
pub struct CobolProgram {
    /// PROGRAM-ID from IDENTIFICATION DIVISION.
    pub program_id: String,
    /// DATA DIVISION (working storage, file section, etc.).
    pub data_division: Option<DataDivision>,
    /// PROCEDURE DIVISION.
    pub procedure_division: Option<ProcedureDivision>,
    /// Source file path (for diagnostics).
    pub source_path: Option<String>,
}

// ---------------------------------------------------------------------------
// DATA DIVISION
// ---------------------------------------------------------------------------

/// The DATA DIVISION, containing all data definitions.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DataDivision {
    /// WORKING-STORAGE SECTION items.
    pub working_storage: Vec<DataEntry>,
    /// LOCAL-STORAGE SECTION items.
    pub local_storage: Vec<DataEntry>,
    /// LINKAGE SECTION items (for CALL parameters).
    pub linkage: Vec<DataEntry>,
    /// FILE SECTION file descriptions.
    pub file_section: Vec<FileDescription>,
}

/// A single data entry (level number item).
#[derive(Debug, Clone, Serialize)]
pub struct DataEntry {
    /// Level number (01-49, 66, 77, 88).
    pub level: u8,
    /// Data name (or FILLER).
    pub name: String,
    /// PIC clause (None for group items).
    pub pic: Option<PicClause>,
    /// USAGE clause.
    pub usage: Usage,
    /// VALUE clause.
    pub value: Option<Literal>,
    /// REDEFINES target name.
    pub redefines: Option<String>,
    /// OCCURS count (fixed-size table).
    pub occurs: Option<u32>,
    /// OCCURS DEPENDING ON field name (variable-size table).
    pub occurs_depending: Option<String>,
    /// SIGN IS LEADING/TRAILING SEPARATE CHARACTER.
    pub sign: Option<SignClause>,
    /// JUSTIFIED RIGHT.
    pub justified_right: bool,
    /// BLANK WHEN ZERO.
    pub blank_when_zero: bool,
    /// Child entries (for group items).
    pub children: Vec<DataEntry>,
    /// Condition values (for 88-level items).
    pub condition_values: Vec<ConditionValue>,
    /// Byte offset within parent group (computed during layout).
    pub byte_offset: Option<usize>,
    /// Byte length (computed during layout or from PIC).
    pub byte_length: Option<usize>,
    /// For level 66 RENAMES: the target field name.
    pub renames_target: Option<String>,
    /// For level 66 RENAMES THRU: the range end field name.
    pub renames_thru: Option<String>,
}

/// Parsed PIC clause.
#[derive(Debug, Clone, Serialize)]
pub struct PicClause {
    /// Raw PIC string as written (e.g., "S9(5)V99").
    pub raw: String,
    /// Detected category.
    pub category: PicCategory,
    /// Total digits (integer + fractional).
    pub total_digits: u32,
    /// Fractional digits (after V).
    pub scale: u32,
    /// Whether signed (S prefix).
    pub signed: bool,
    /// Total display length in bytes.
    pub display_length: u32,
    /// Editing symbols (for edited categories).
    pub edit_symbols: Vec<EditSymbolEntry>,
}

/// Category derived from PIC clause analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PicCategory {
    /// PIC A -- alphabetic only.
    Alphabetic,
    /// PIC X -- alphanumeric.
    Alphanumeric,
    /// PIC 9 -- numeric (display, packed, binary depending on USAGE).
    Numeric,
    /// Numeric edited (contains Z, *, $, +, -, etc.).
    NumericEdited,
    /// Alphanumeric edited (A/X with B/0/slash insertions).
    AlphanumericEdited,
}

/// A single editing symbol in a PIC clause.
#[derive(Debug, Clone, Serialize)]
pub struct EditSymbolEntry {
    /// The symbol character (Z, *, $, +, -, B, 0, /).
    pub symbol: char,
    /// Number of repetitions.
    pub count: u32,
}

/// USAGE clause values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum Usage {
    /// DISPLAY (default) -- character representation.
    #[default]
    Display,
    /// COMP / COMP-4 / BINARY -- binary integer.
    Comp,
    /// COMP-3 / PACKED-DECIMAL -- BCD encoding.
    Comp3,
    /// COMP-5 -- native binary (full range, not PIC-limited).
    Comp5,
    /// COMP-1 -- single-precision float (4 bytes).
    Comp1,
    /// COMP-2 -- double-precision float (8 bytes).
    Comp2,
    /// INDEX -- address/offset for table indexing.
    Index,
    /// POINTER -- address pointer (IBM extension).
    Pointer,
}

/// SIGN clause specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SignClause {
    /// LEADING or TRAILING.
    pub position: SignPosition,
    /// SEPARATE CHARACTER (sign in its own byte).
    pub separate: bool,
}

/// Sign position within a numeric field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SignPosition {
    Leading,
    Trailing,
}

/// A literal value (for VALUE clauses, condition values, etc.).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Literal {
    /// Numeric literal (e.g., 42, -3.14, +100).
    Numeric(String),
    /// Alphanumeric literal (e.g., "HELLO").
    Alphanumeric(String),
    /// Figurative constant (SPACES, ZEROS, etc.).
    Figurative(FigurativeConstant),
}

/// Figurative constants in COBOL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum FigurativeConstant {
    Spaces,
    Zeros,
    HighValues,
    LowValues,
    Quotes,
    Nulls,
    /// ALL "x" -- repeats the given string to fill the field.
    All(String),
}

/// Condition value for 88-level items.
#[derive(Debug, Clone, Serialize)]
pub enum ConditionValue {
    /// Single value: VALUE IS literal.
    Single(Literal),
    /// Range: VALUE IS low THRU high.
    Range { low: Literal, high: Literal },
}

/// FILE SECTION file description (FD/SD).
#[derive(Debug, Clone, Serialize)]
pub struct FileDescription {
    /// FD or SD.
    pub descriptor_type: FileDescriptorType,
    /// Logical file name (from SELECT).
    pub file_name: String,
    /// Physical file assignment (from ASSIGN TO).
    pub assign_to: Option<String>,
    /// ORGANIZATION (SEQUENTIAL, INDEXED, RELATIVE).
    pub organization: FileOrganization,
    /// ACCESS MODE (SEQUENTIAL, RANDOM, DYNAMIC).
    pub access_mode: AccessMode,
    /// RECORD KEY field name (for INDEXED).
    pub record_key: Option<String>,
    /// ALTERNATE RECORD KEY field names (for INDEXED).
    pub alternate_keys: Vec<String>,
    /// RELATIVE KEY field name (for RELATIVE).
    pub relative_key: Option<String>,
    /// FILE STATUS field name.
    pub file_status: Option<String>,
    /// Record definitions (01-level items under FD).
    pub records: Vec<DataEntry>,
    /// Names of 01-level records under this FD (for building RecordFileMap).
    /// Populated by the file listener; `records` may be empty when records are
    /// captured separately by `DataDivisionListener`.
    pub record_names: Vec<String>,
    /// RECORDING MODE (F, V, U).
    pub recording_mode: Option<RecordingMode>,
    /// RECORD CONTAINS size specification.
    pub record_size: Option<RecordSize>,
}

/// FD vs SD.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum FileDescriptorType {
    /// File Description.
    Fd,
    /// Sort Description.
    Sd,
}

/// File organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum FileOrganization {
    #[default]
    Sequential,
    LineSequential,
    Indexed,
    Relative,
}

/// File access mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum AccessMode {
    #[default]
    Sequential,
    Random,
    Dynamic,
}

/// Recording mode (F, V, U).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecordingMode {
    Fixed,
    Variable,
    Undefined,
}

/// Record size specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecordSize {
    /// RECORD CONTAINS n CHARACTERS.
    Fixed(u32),
    /// RECORD CONTAINS min TO max CHARACTERS.
    Variable { min: u32, max: u32 },
}

// ---------------------------------------------------------------------------
// PROCEDURE DIVISION
// ---------------------------------------------------------------------------

/// The PROCEDURE DIVISION, containing executable code.
#[derive(Debug, Clone, Serialize)]
pub struct ProcedureDivision {
    /// USING parameters (for called programs).
    pub using_params: Vec<UsingParam>,
    /// RETURNING field name.
    pub returning: Option<String>,
    /// Sections (each contains paragraphs).
    pub sections: Vec<Section>,
    /// Standalone paragraphs (not in any section).
    pub paragraphs: Vec<Paragraph>,
}

/// A USING parameter specification.
#[derive(Debug, Clone, Serialize)]
pub struct UsingParam {
    /// Parameter name (data item reference).
    pub name: String,
    /// BY REFERENCE (default) or BY VALUE.
    pub passing: PassingMode,
}

/// Parameter passing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum PassingMode {
    #[default]
    ByReference,
    ByValue,
    ByContent,
    Omitted,
}

/// A section in the PROCEDURE DIVISION.
#[derive(Debug, Clone, Serialize)]
pub struct Section {
    /// Section name.
    pub name: String,
    /// Paragraphs within this section.
    pub paragraphs: Vec<Paragraph>,
}

/// A paragraph (basic unit of executable code).
#[derive(Debug, Clone, Serialize)]
pub struct Paragraph {
    /// Paragraph name.
    pub name: String,
    /// Sentences (groups of statements ending with a period).
    pub sentences: Vec<Sentence>,
}

/// A sentence (one or more statements terminated by a period).
#[derive(Debug, Clone, Serialize)]
pub struct Sentence {
    pub statements: Vec<Statement>,
}

/// A COBOL statement (verb + operands).
#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    // -- Data manipulation --
    Move(MoveStatement),
    Initialize(InitializeStatement),

    // -- Arithmetic --
    Add(AddStatement),
    Subtract(SubtractStatement),
    Multiply(MultiplyStatement),
    Divide(DivideStatement),
    Compute(ComputeStatement),

    // -- Control flow --
    If(IfStatement),
    Evaluate(EvaluateStatement),
    Perform(PerformStatement),
    GoTo(GoToStatement),
    StopRun,
    GoBack,
    Continue,
    NextSentence,
    ExitProgram,
    ExitParagraph,
    ExitSection,

    // -- I/O --
    Display(DisplayStatement),
    Accept(AcceptStatement),
    Open(OpenStatement),
    Close(CloseStatement),
    Read(ReadStatement),
    Write(WriteStatement),
    Rewrite(RewriteStatement),
    Delete(DeleteStatement),
    Start(StartStatement),

    // -- String --
    String(StringStatement),
    Unstring(UnstringStatement),
    Inspect(InspectStatement),

    // -- Subprogram --
    Call(CallStatement),
    Cancel(CancelStatement),

    // -- Sort/Merge --
    Sort(SortStatement),
    Merge(MergeStatement),
    Release(ReleaseStatement),
    Return(ReturnStatement),

    // -- Misc --
    Set(SetStatement),
    Alter(AlterStatement),
}

// ---------------------------------------------------------------------------
// Statement details
// ---------------------------------------------------------------------------

/// MOVE src TO dest1 dest2 ...
#[derive(Debug, Clone, Serialize)]
pub struct MoveStatement {
    /// MOVE CORRESPONDING flag.
    pub corresponding: bool,
    /// Source operand.
    pub source: Operand,
    /// Destination operands.
    pub destinations: Vec<DataReference>,
}

/// INITIALIZE field REPLACING ...
#[derive(Debug, Clone, Serialize)]
pub struct InitializeStatement {
    /// Fields to initialize.
    pub targets: Vec<DataReference>,
    /// REPLACING clauses (category -> value).
    pub replacing: Vec<InitializeReplacing>,
}

/// INITIALIZE REPLACING clause.
#[derive(Debug, Clone, Serialize)]
pub struct InitializeReplacing {
    /// Category to replace.
    pub category: InitializeCategory,
    /// Replacement value.
    pub value: Operand,
}

/// Categories for INITIALIZE REPLACING.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum InitializeCategory {
    Alphabetic,
    Alphanumeric,
    Numeric,
    AlphanumericEdited,
    NumericEdited,
    National,
}

/// ADD a TO b GIVING c ...
#[derive(Debug, Clone, Serialize)]
pub struct AddStatement {
    /// Addends.
    pub operands: Vec<Operand>,
    /// TO targets (mutated in place).
    pub to: Vec<ArithTarget>,
    /// GIVING targets (store result).
    pub giving: Vec<ArithTarget>,
    /// ON SIZE ERROR statements.
    pub on_size_error: Vec<Statement>,
    /// NOT ON SIZE ERROR statements.
    pub not_on_size_error: Vec<Statement>,
    /// CORRESPONDING flag.
    pub corresponding: bool,
}

/// SUBTRACT a FROM b GIVING c ...
#[derive(Debug, Clone, Serialize)]
pub struct SubtractStatement {
    /// Subtrahends.
    pub operands: Vec<Operand>,
    /// FROM targets (mutated in place).
    pub from: Vec<ArithTarget>,
    /// GIVING targets.
    pub giving: Vec<ArithTarget>,
    /// ON SIZE ERROR statements.
    pub on_size_error: Vec<Statement>,
    /// NOT ON SIZE ERROR statements.
    pub not_on_size_error: Vec<Statement>,
    /// CORRESPONDING flag.
    pub corresponding: bool,
}

/// MULTIPLY a BY b GIVING c ...
#[derive(Debug, Clone, Serialize)]
pub struct MultiplyStatement {
    /// Multiplicand.
    pub operand: Operand,
    /// BY targets (mutated in place).
    pub by: Vec<ArithTarget>,
    /// GIVING targets.
    pub giving: Vec<ArithTarget>,
    /// ON SIZE ERROR statements.
    pub on_size_error: Vec<Statement>,
    /// NOT ON SIZE ERROR statements.
    pub not_on_size_error: Vec<Statement>,
}

/// DIVIDE a INTO/BY b GIVING c REMAINDER d ...
#[derive(Debug, Clone, Serialize)]
pub struct DivideStatement {
    /// Dividend/divisor.
    pub operand: Operand,
    /// INTO or BY.
    pub direction: DivideDirection,
    /// INTO/BY targets (mutated in place).
    pub into: Vec<ArithTarget>,
    /// BY operand for DIVIDE x BY y GIVING z (can be a literal).
    pub by_operand: Option<Operand>,
    /// GIVING targets.
    pub giving: Vec<ArithTarget>,
    /// REMAINDER target.
    pub remainder: Option<ArithTarget>,
    /// ON SIZE ERROR statements.
    pub on_size_error: Vec<Statement>,
    /// NOT ON SIZE ERROR statements.
    pub not_on_size_error: Vec<Statement>,
}

/// DIVIDE direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DivideDirection {
    /// DIVIDE x INTO y (y = y / x).
    Into,
    /// DIVIDE x BY y GIVING z (z = x / y).
    By,
}

/// COMPUTE dest = expression.
#[derive(Debug, Clone, Serialize)]
pub struct ComputeStatement {
    /// Destination targets.
    pub targets: Vec<ArithTarget>,
    /// Arithmetic expression.
    pub expression: ArithExpr,
    /// ON SIZE ERROR statements.
    pub on_size_error: Vec<Statement>,
    /// NOT ON SIZE ERROR statements.
    pub not_on_size_error: Vec<Statement>,
}

/// Arithmetic target (field reference + optional ROUNDED).
#[derive(Debug, Clone, Serialize)]
pub struct ArithTarget {
    /// Target field.
    pub field: DataReference,
    /// ROUNDED flag.
    pub rounded: bool,
}

/// Arithmetic expression (for COMPUTE).
#[derive(Debug, Clone, Serialize)]
pub enum ArithExpr {
    /// Literal or field reference.
    Operand(Operand),
    /// Unary negation.
    Negate(Box<ArithExpr>),
    /// Binary operation.
    BinaryOp {
        left: Box<ArithExpr>,
        op: ArithOp,
        right: Box<ArithExpr>,
    },
    /// Parenthesized expression.
    Paren(Box<ArithExpr>),
}

/// Binary arithmetic operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ArithOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

/// IF condition THEN ... ELSE ... END-IF.
#[derive(Debug, Clone, Serialize)]
pub struct IfStatement {
    /// Condition to test.
    pub condition: Condition,
    /// THEN statements.
    pub then_body: Vec<Statement>,
    /// ELSE statements.
    pub else_body: Vec<Statement>,
}

/// EVALUATE subject WHEN ... END-EVALUATE.
#[derive(Debug, Clone, Serialize)]
pub struct EvaluateStatement {
    /// EVALUATE subjects (ALSO).
    pub subjects: Vec<EvaluateSubject>,
    /// WHEN branches.
    pub when_branches: Vec<WhenBranch>,
    /// WHEN OTHER statements.
    pub when_other: Vec<Statement>,
}

/// EVALUATE subject.
#[derive(Debug, Clone, Serialize)]
pub enum EvaluateSubject {
    /// EVALUATE expression.
    Expr(Operand),
    /// EVALUATE TRUE / EVALUATE FALSE.
    Bool(bool),
}

/// A WHEN branch.
#[derive(Debug, Clone, Serialize)]
pub struct WhenBranch {
    /// Values to match (ALSO).
    pub values: Vec<WhenValue>,
    /// Statements to execute.
    pub body: Vec<Statement>,
}

/// A single WHEN value.
#[derive(Debug, Clone, Serialize)]
pub enum WhenValue {
    /// A single value.
    Value(Operand),
    /// A range (low THRU high).
    Range { low: Operand, high: Operand },
    /// Condition expression.
    Condition(Condition),
    /// ANY (matches anything).
    Any,
}

/// PERFORM [TIMES | UNTIL | VARYING] ...
#[derive(Debug, Clone, Serialize)]
pub struct PerformStatement {
    /// Target paragraph/section (None for inline PERFORM).
    pub target: Option<PerformTarget>,
    /// THRU paragraph/section.
    pub thru: Option<String>,
    /// Loop type.
    pub loop_type: PerformLoopType,
    /// Inline body (for PERFORM ... END-PERFORM).
    pub body: Vec<Statement>,
}

/// PERFORM target.
#[derive(Debug, Clone, Serialize)]
pub struct PerformTarget {
    pub name: String,
}

/// PERFORM loop variants.
#[derive(Debug, Clone, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum PerformLoopType {
    /// Simple PERFORM (execute once).
    Once,
    /// PERFORM n TIMES.
    Times(Operand),
    /// PERFORM UNTIL condition.
    Until {
        /// WITH TEST BEFORE (default) or AFTER.
        test_before: bool,
        condition: Condition,
    },
    /// PERFORM VARYING counter FROM start BY increment UNTIL condition.
    Varying {
        /// WITH TEST BEFORE (default) or AFTER.
        test_before: bool,
        /// Primary counter.
        counter: DataReference,
        from: Operand,
        by: Operand,
        until: Condition,
        /// AFTER clauses (nested varying).
        after: Vec<VaryingAfter>,
    },
}

/// AFTER clause in PERFORM VARYING.
#[derive(Debug, Clone, Serialize)]
pub struct VaryingAfter {
    pub counter: DataReference,
    pub from: Operand,
    pub by: Operand,
    pub until: Condition,
}

/// GO TO paragraph.
#[derive(Debug, Clone, Serialize)]
pub struct GoToStatement {
    /// Target paragraph(s).
    pub targets: Vec<String>,
    /// DEPENDING ON field (for GO TO ... DEPENDING ON).
    pub depending: Option<DataReference>,
}

/// DISPLAY items UPON target.
#[derive(Debug, Clone, Serialize)]
pub struct DisplayStatement {
    /// Items to display.
    pub items: Vec<Operand>,
    /// UPON target (SYSOUT, SYSERR, mnemonic).
    pub upon: DisplayTarget,
    /// WITH NO ADVANCING.
    pub no_advancing: bool,
}

/// DISPLAY target device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum DisplayTarget {
    #[default]
    Sysout,
    Syserr,
    Console,
}

/// ACCEPT field FROM source.
#[derive(Debug, Clone, Serialize)]
pub struct AcceptStatement {
    /// Destination field.
    pub target: DataReference,
    /// Source (SYSIN, DATE, TIME, etc.).
    pub from: AcceptSource,
}

/// ACCEPT source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum AcceptSource {
    #[default]
    Sysin,
    Date,
    DayOfWeek,
    Time,
    Day,
    DateYyyyMmDd,
    DayYyyyDdd,
}

/// OPEN mode file-name.
#[derive(Debug, Clone, Serialize)]
pub struct OpenStatement {
    pub files: Vec<OpenFile>,
}

/// A single file in an OPEN statement.
#[derive(Debug, Clone, Serialize)]
pub struct OpenFile {
    pub mode: OpenMode,
    pub file_name: String,
}

/// OPEN modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OpenMode {
    Input,
    Output,
    IoMode,
    Extend,
}

/// CLOSE file-name.
#[derive(Debug, Clone, Serialize)]
pub struct CloseStatement {
    pub files: Vec<String>,
}

/// READ file INTO dest.
#[derive(Debug, Clone, Serialize)]
pub struct ReadStatement {
    pub file_name: String,
    pub into: Option<DataReference>,
    pub key: Option<DataReference>,
    pub at_end: Vec<Statement>,
    pub not_at_end: Vec<Statement>,
    pub invalid_key: Vec<Statement>,
    pub not_invalid_key: Vec<Statement>,
}

/// WRITE record FROM src.
#[derive(Debug, Clone, Serialize)]
pub struct WriteStatement {
    pub record_name: String,
    pub from: Option<DataReference>,
    pub advancing: Option<Advancing>,
    pub invalid_key: Vec<Statement>,
    pub not_invalid_key: Vec<Statement>,
    pub at_eop: Vec<Statement>,
    pub not_at_eop: Vec<Statement>,
}

/// WRITE ADVANCING clause.
#[derive(Debug, Clone, Serialize)]
pub enum Advancing {
    Lines(Operand),
    Page,
}

/// REWRITE record FROM src.
#[derive(Debug, Clone, Serialize)]
pub struct RewriteStatement {
    pub record_name: String,
    pub from: Option<DataReference>,
    pub invalid_key: Vec<Statement>,
    pub not_invalid_key: Vec<Statement>,
}

/// DELETE file.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteStatement {
    pub file_name: String,
    pub invalid_key: Vec<Statement>,
    pub not_invalid_key: Vec<Statement>,
}

/// START file KEY condition.
#[derive(Debug, Clone, Serialize)]
pub struct StartStatement {
    pub file_name: String,
    pub key_condition: Option<StartKeyCondition>,
    pub invalid_key: Vec<Statement>,
    pub not_invalid_key: Vec<Statement>,
}

/// START key comparison.
#[derive(Debug, Clone, Serialize)]
pub struct StartKeyCondition {
    pub key: DataReference,
    pub op: ComparisonOp,
}

/// STRING ... DELIMITED BY ... INTO dest.
#[derive(Debug, Clone, Serialize)]
pub struct StringStatement {
    pub sources: Vec<StringSource>,
    pub into: DataReference,
    pub pointer: Option<DataReference>,
    pub on_overflow: Vec<Statement>,
    pub not_on_overflow: Vec<Statement>,
}

/// STRING source item.
#[derive(Debug, Clone, Serialize)]
pub struct StringSource {
    pub operand: Operand,
    pub delimited_by: StringDelimiter,
}

/// STRING delimiter.
#[derive(Debug, Clone, Serialize)]
pub enum StringDelimiter {
    Size,
    Literal(Operand),
}

/// UNSTRING source DELIMITED BY ... INTO dest1 dest2 ...
#[derive(Debug, Clone, Serialize)]
pub struct UnstringStatement {
    pub source: DataReference,
    pub delimiters: Vec<UnstringDelimiter>,
    pub into: Vec<UnstringInto>,
    pub pointer: Option<DataReference>,
    pub tallying: Option<DataReference>,
    pub on_overflow: Vec<Statement>,
    pub not_on_overflow: Vec<Statement>,
}

/// UNSTRING delimiter.
#[derive(Debug, Clone, Serialize)]
pub struct UnstringDelimiter {
    pub value: Operand,
    pub all: bool,
}

/// UNSTRING INTO target.
#[derive(Debug, Clone, Serialize)]
pub struct UnstringInto {
    pub target: DataReference,
    pub delimiter_in: Option<DataReference>,
    pub count_in: Option<DataReference>,
}

/// INSPECT field TALLYING/REPLACING ...
#[derive(Debug, Clone, Serialize)]
pub struct InspectStatement {
    pub target: DataReference,
    pub tallying: Vec<InspectTallying>,
    pub replacing: Vec<InspectReplacing>,
    pub converting: Option<InspectConverting>,
}

/// INSPECT TALLYING clause.
#[derive(Debug, Clone, Serialize)]
pub struct InspectTallying {
    pub counter: DataReference,
    pub what: InspectWhat,
}

/// INSPECT REPLACING clause.
#[derive(Debug, Clone, Serialize)]
pub struct InspectReplacing {
    pub what: InspectWhat,
    pub by: Operand,
}

/// What to tally/replace in INSPECT.
#[derive(Debug, Clone, Serialize)]
pub enum InspectWhat {
    Characters,
    All(Operand),
    Leading(Operand),
    First(Operand),
}

/// INSPECT CONVERTING clause.
#[derive(Debug, Clone, Serialize)]
pub struct InspectConverting {
    pub from: Operand,
    pub to: Operand,
}

/// CALL program-name USING params.
#[derive(Debug, Clone, Serialize)]
pub struct CallStatement {
    /// Program name (literal or identifier).
    pub program: Operand,
    /// USING parameters.
    pub using: Vec<CallParam>,
    /// RETURNING field.
    pub returning: Option<DataReference>,
    /// ON OVERFLOW / ON EXCEPTION statements.
    pub on_exception: Vec<Statement>,
    /// NOT ON OVERFLOW / NOT ON EXCEPTION statements.
    pub not_on_exception: Vec<Statement>,
}

/// CALL USING parameter.
#[derive(Debug, Clone, Serialize)]
pub struct CallParam {
    pub mode: PassingMode,
    /// The operand. `None` when OMITTED.
    pub operand: Option<Operand>,
}

/// CANCEL program-name ...
#[derive(Debug, Clone, Serialize)]
pub struct CancelStatement {
    pub programs: Vec<Operand>,
}

/// SORT file ON KEY ... USING/INPUT PROCEDURE ... GIVING/OUTPUT PROCEDURE ...
#[derive(Debug, Clone, Serialize)]
pub struct SortStatement {
    pub file_name: String,
    pub keys: Vec<SortKey>,
    pub duplicates: bool,
    pub collating: Option<String>,
    pub input: SortInput,
    pub output: SortOutput,
}

/// SORT key specification.
#[derive(Debug, Clone, Serialize)]
pub struct SortKey {
    pub ascending: bool,
    pub field: DataReference,
}

/// SORT input source.
#[derive(Debug, Clone, Serialize)]
pub enum SortInput {
    /// USING file-name.
    Using(Vec<String>),
    /// INPUT PROCEDURE IS section.
    InputProcedure { name: String, thru: Option<String> },
}

/// SORT output destination.
#[derive(Debug, Clone, Serialize)]
pub enum SortOutput {
    /// GIVING file-name.
    Giving(Vec<String>),
    /// OUTPUT PROCEDURE IS section.
    OutputProcedure { name: String, thru: Option<String> },
}

/// MERGE file ON KEY ...
#[derive(Debug, Clone, Serialize)]
pub struct MergeStatement {
    pub file_name: String,
    pub keys: Vec<SortKey>,
    pub collating: Option<String>,
    pub using: Vec<String>,
    pub output: SortOutput,
}

/// RELEASE record FROM src.
#[derive(Debug, Clone, Serialize)]
pub struct ReleaseStatement {
    pub record_name: String,
    pub from: Option<DataReference>,
}

/// RETURN file INTO dest AT END ...
#[derive(Debug, Clone, Serialize)]
pub struct ReturnStatement {
    pub file_name: String,
    pub into: Option<DataReference>,
    pub at_end: Vec<Statement>,
    pub not_at_end: Vec<Statement>,
}

/// SET field TO/UP/DOWN value.
#[derive(Debug, Clone, Serialize)]
pub struct SetStatement {
    pub targets: Vec<DataReference>,
    pub action: SetAction,
}

/// SET action.
#[derive(Debug, Clone, Serialize)]
pub enum SetAction {
    /// SET field TO value.
    To(Operand),
    /// SET index UP BY value.
    UpBy(Operand),
    /// SET index DOWN BY value.
    DownBy(Operand),
    /// SET condition TO TRUE/FALSE.
    ToBool(bool),
}

/// ALTER paragraph TO PROCEED TO target.
#[derive(Debug, Clone, Serialize)]
pub struct AlterStatement {
    pub source: String,
    pub target: String,
}

// ---------------------------------------------------------------------------
// Conditions
// ---------------------------------------------------------------------------

/// A COBOL condition expression.
#[derive(Debug, Clone, Serialize)]
pub enum Condition {
    /// Simple comparison: field op value.
    Comparison {
        left: Operand,
        op: ComparisonOp,
        right: Operand,
    },
    /// Class test: field IS NUMERIC / ALPHABETIC / etc.
    ClassTest {
        field: DataReference,
        class: ClassCondition,
    },
    /// Sign test: field IS POSITIVE / NEGATIVE / ZERO.
    SignTest {
        field: DataReference,
        sign: SignCondition,
    },
    /// 88-level condition name test.
    ConditionName(DataReference),
    /// NOT condition.
    Not(Box<Condition>),
    /// AND combination.
    And(Box<Condition>, Box<Condition>),
    /// OR combination.
    Or(Box<Condition>, Box<Condition>),
}

/// Comparison operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
}

/// Class conditions for IS NUMERIC, IS ALPHABETIC, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ClassCondition {
    Numeric,
    Alphabetic,
    AlphabeticLower,
    AlphabeticUpper,
}

/// Sign conditions for IS POSITIVE, IS NEGATIVE, IS ZERO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SignCondition {
    Positive,
    Negative,
    Zero,
}

// ---------------------------------------------------------------------------
// Operands and references
// ---------------------------------------------------------------------------

/// An operand (value source in a statement).
#[derive(Debug, Clone, Serialize)]
pub enum Operand {
    /// A literal value.
    Literal(Literal),
    /// A data item reference.
    DataRef(DataReference),
    /// An intrinsic function call.
    Function(FunctionCall),
}

/// Reference to a data item.
#[derive(Debug, Clone, Serialize)]
pub struct DataReference {
    /// Data item name.
    pub name: String,
    /// IN/OF qualification chain (innermost first).
    pub qualifiers: Vec<String>,
    /// Subscripts (for table items).
    pub subscripts: Vec<Subscript>,
    /// Reference modification (offset:length).
    pub ref_mod: Option<RefMod>,
}

/// A subscript expression (for OCCURS items).
#[derive(Debug, Clone, Serialize)]
pub enum Subscript {
    /// Integer literal subscript.
    IntLiteral(i32),
    /// Field reference subscript.
    DataRef(DataReference),
    /// Arithmetic expression subscript.
    Expr(ArithExpr),
}

/// Reference modification (field(offset:length)).
#[derive(Debug, Clone, Serialize)]
pub struct RefMod {
    /// Starting position (1-based).
    pub offset: Box<ArithExpr>,
    /// Length (optional -- if omitted, rest of field).
    pub length: Option<Box<ArithExpr>>,
}

/// Intrinsic function call.
#[derive(Debug, Clone, Serialize)]
pub struct FunctionCall {
    /// Function name (e.g., LENGTH, CURRENT-DATE, TRIM).
    pub name: String,
    /// Arguments.
    pub arguments: Vec<Operand>,
}

// ---------------------------------------------------------------------------
// Display impls for diagnostics
// ---------------------------------------------------------------------------

impl fmt::Display for DataReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for q in &self.qualifiers {
            write!(f, " IN {q}")?;
        }
        if !self.subscripts.is_empty() {
            write!(f, "(")?;
            for (i, s) in self.subscripts.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                match s {
                    Subscript::IntLiteral(n) => write!(f, "{n}")?,
                    Subscript::DataRef(dr) => write!(f, "{dr}")?,
                    Subscript::Expr(_) => write!(f, "<expr>")?,
                }
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "="),
            Self::NotEqual => write!(f, "!="),
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
            Self::LessOrEqual => write!(f, "<="),
            Self::GreaterOrEqual => write!(f, ">="),
        }
    }
}

impl fmt::Display for ArithOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Power => write!(f, "**"),
        }
    }
}
