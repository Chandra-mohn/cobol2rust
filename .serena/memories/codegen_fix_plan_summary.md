# Codegen Fix Plan Summary

## Goal: 7/35 -> 30+/35 compiling stress tests

## Phase 1: Data Generation (data_gen.rs) -- S42
- **1A**: Group fields as struct members (10 programs) - add PicX overlay for group
- **1B**: COMP fields -> CompBinary not i16/i32/i64 (6 programs)
- **1C**: Duplicate field names -> prefix with parent group (2 programs)
- **1D**: PicX Default impl (1 program)
- **1E**: Keyword escaping r#true r#false (1 program)

## Phase 2: Condition Codegen (proc_gen.rs) -- S43
- **2A**: Figurative constants in IF conditions (2+ programs)
- **2B**: Alphanumeric comparisons use .as_bytes() (2 programs)
- **2C**: Sign conditions -> .to_decimal() comparison (3 programs)
- **2D**: CLASS conditions -> add is_numeric/is_alphabetic to PicX (3 programs)
- **2E**: Comparison chaining -> flatten to && (2 programs)

## Phase 3: Statement Codegen (proc_gen.rs) -- S44-S45
- **3A**: Array indexing -> Index trait on CobolArray (7 programs) 
- **3B**: EVALUATE WHEN -> proper THRU/ALSO/TRUE (2 programs)
- **3C**: Intrinsic functions -> standalone function calls (2 programs)
- **3D**: Section PERFORM -> wrapper function (1 program)
- **3E**: MOVE CORRESPONDING -> check prelude export (1 program)
- **3F**: INSPECT/STRING arg patterns (2 programs)
- **3G**: Reference modification Decimal->usize casts (2 programs)

## Phase 4: File I/O and SORT/MERGE -- S46
- **4A**: File descriptor fields in WorkingStorage (5 programs)
- **4B**: SORT/MERGE RELEASE/RETURN codegen (2 programs)

## Key Dependencies
Phase 1 -> Phase 2 -> Phase 3
1B before 2C (sign conditions need CobolNumeric on COMP fields)
1A before 4A (file records are group fields)

## Full plan doc: codegen_fix_plan.md in project root
