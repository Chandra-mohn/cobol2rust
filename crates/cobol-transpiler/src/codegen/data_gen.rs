//! Data division code generator.
//!
//! Generates Rust struct definitions from COBOL DATA DIVISION entries.
//! Each 01-level record becomes a struct, with fields generated from
//! the child entries.

use crate::ast::{DataEntry, Literal};
use crate::codegen::rust_writer::RustWriter;
use crate::symbol_table::{resolve_type, RustType};

/// Generate the WorkingStorage struct and its `new()` constructor.
pub fn generate_working_storage(
    w: &mut RustWriter,
    records: &[DataEntry],
) {
    w.line("/// Working storage data fields.");
    w.line("#[allow(non_snake_case)]");
    w.open_block("pub struct WorkingStorage {");

    for record in records {
        if record.level == 77 {
            // Standalone field
            generate_field(w, record, "");
        } else if record.level == 1 {
            if record.children.is_empty() {
                // Elementary 01-level
                generate_field(w, record, "");
            } else {
                // Group: flatten children as fields
                generate_group_fields(w, record, "");
            }
        }
    }

    // Second pass: generate level-66 RENAMES fields
    for record in records {
        if record.level == 1 {
            generate_renames_fields(w, record, "");
        }
    }

    w.close_block("}");
    w.blank_line();

    // Generate new() constructor
    w.open_block("impl WorkingStorage {");
    w.line("#[allow(non_snake_case)]");
    w.open_block("pub fn new() -> Self {");
    w.open_block("Self {");

    for record in records {
        if record.level == 77 {
            generate_field_init(w, record, "");
        } else if record.level == 1 {
            if record.children.is_empty() {
                generate_field_init(w, record, "");
            } else {
                generate_group_field_inits(w, record, "");
            }
        }
    }

    // Second pass: initialize level-66 RENAMES fields
    for record in records {
        if record.level == 1 {
            generate_renames_field_inits(w, record, "");
        }
    }

    w.close_block("}");
    w.close_block("}");
    w.close_block("}");
}

/// Generate the LinkageSection struct and its `new()` constructor.
///
/// Mirrors `generate_working_storage` exactly but with struct name `LinkageSection`.
/// Linkage section items are used by called programs to receive parameters.
pub fn generate_linkage_section(
    w: &mut RustWriter,
    records: &[DataEntry],
) {
    if records.is_empty() {
        return;
    }

    w.line("/// Linkage section data fields (CALL parameters).");
    w.line("#[allow(non_snake_case)]");
    w.open_block("pub struct LinkageSection {");

    for record in records {
        if record.level == 77 {
            generate_field(w, record, "");
        } else if record.level == 1 {
            if record.children.is_empty() {
                generate_field(w, record, "");
            } else {
                generate_group_fields(w, record, "");
            }
        }
    }

    w.close_block("}");
    w.blank_line();

    w.open_block("impl LinkageSection {");
    w.line("#[allow(non_snake_case)]");
    w.open_block("pub fn new() -> Self {");
    w.open_block("Self {");

    for record in records {
        if record.level == 77 {
            generate_field_init(w, record, "");
        } else if record.level == 1 {
            if record.children.is_empty() {
                generate_field_init(w, record, "");
            } else {
                generate_group_field_inits(w, record, "");
            }
        }
    }

    w.close_block("}");
    w.close_block("}");
    w.close_block("}");
}

/// Generate a single field declaration.
fn generate_field(w: &mut RustWriter, entry: &DataEntry, prefix: &str) {
    let field_name = cobol_to_rust_name(&entry.name, prefix);

    // REDEFINES: generate a RedefinesGroup for shared byte storage
    if entry.redefines.is_some() {
        let size = entry.byte_length.unwrap_or(0);
        w.line(&format!(
            "pub {field_name}: RedefinesGroup, /* REDEFINES, {size} bytes */"
        ));
        return;
    }

    let resolved = resolve_type(entry);
    let rust_type = rust_type_string(&resolved.rust_type);

    // OCCURS DEPENDING ON: variable-length array
    if entry.occurs.is_some() && entry.occurs_depending.is_some() {
        w.line(&format!(
            "pub {field_name}: CobolVarArray<{rust_type}>, /* OCCURS DEPENDING ON */"
        ));
        return;
    }

    // OCCURS: fixed-size array
    if let Some(count) = entry.occurs {
        w.line(&format!(
            "pub {field_name}: CobolArray<{rust_type}>, /* OCCURS {count} */"
        ));
        return;
    }

    w.line(&format!("pub {field_name}: {rust_type},"));
}

/// Generate flattened fields for a group record.
fn generate_group_fields(w: &mut RustWriter, entry: &DataEntry, prefix: &str) {
    let new_prefix = if prefix.is_empty() {
        entry.name.clone()
    } else {
        format!("{prefix}_{}", entry.name)
    };

    for child in &entry.children {
        if child.level == 88 || child.level == 66 {
            continue;
        }
        // REDEFINES group: emit a single RedefinesGroup field
        // (don't recurse into children -- they access via byte offsets)
        if child.redefines.is_some() {
            generate_field(w, child, &new_prefix);
            continue;
        }
        if child.children.is_empty() {
            generate_field(w, child, &new_prefix);
        } else {
            generate_group_fields(w, child, &new_prefix);
        }
    }
}

/// Generate a field initialization expression.
fn generate_field_init(w: &mut RustWriter, entry: &DataEntry, prefix: &str) {
    let field_name = cobol_to_rust_name(&entry.name, prefix);

    // REDEFINES: initialize RedefinesGroup with byte size
    if entry.redefines.is_some() {
        let size = entry.byte_length.unwrap_or(0);
        w.line(&format!("{field_name}: RedefinesGroup::new({size}),"));
        return;
    }

    let resolved = resolve_type(entry);
    let element_init = field_init_expr(entry, &resolved.rust_type);

    // OCCURS DEPENDING ON: variable-length array
    if let Some(count) = entry.occurs {
        if entry.occurs_depending.is_some() {
            w.line(&format!(
                "{field_name}: CobolVarArray::new(vec![{element_init}; {count}], {count}),"
            ));
            return;
        }

        // OCCURS: fixed-size array
        w.line(&format!(
            "{field_name}: CobolArray::new(vec![{element_init}; {count}]),"
        ));
        return;
    }

    w.line(&format!("{field_name}: {element_init},"));
}

/// Generate field initializations for a group.
fn generate_group_field_inits(w: &mut RustWriter, entry: &DataEntry, prefix: &str) {
    let new_prefix = if prefix.is_empty() {
        entry.name.clone()
    } else {
        format!("{prefix}_{}", entry.name)
    };

    for child in &entry.children {
        if child.level == 88 || child.level == 66 {
            continue;
        }
        // REDEFINES group: single RedefinesGroup init
        if child.redefines.is_some() {
            generate_field_init(w, child, &new_prefix);
            continue;
        }
        if child.children.is_empty() {
            generate_field_init(w, child, &new_prefix);
        } else {
            generate_group_field_inits(w, child, &new_prefix);
        }
    }
}

/// Generate struct fields for level-66 RENAMES entries within a record.
///
/// RENAMES creates an alias for another field (or a byte range of fields).
/// We emit a separate struct field with the resolved type from the symbol table.
fn generate_renames_fields(w: &mut RustWriter, record: &DataEntry, prefix: &str) {
    let record_prefix = if prefix.is_empty() {
        record.name.clone()
    } else {
        format!("{prefix}_{}", record.name)
    };

    for child in &record.children {
        if child.level == 66 {
            let field_name = cobol_to_rust_name(&child.name, &record_prefix);
            let resolved = resolve_renames_type_from_entry(child, record);
            let rust_type = rust_type_string(&resolved.rust_type);
            let comment = if child.renames_thru.is_some() {
                format!(
                    " /* RENAMES {} THRU {} */",
                    child.renames_target.as_deref().unwrap_or("?"),
                    child.renames_thru.as_deref().unwrap_or("?"),
                )
            } else {
                format!(
                    " /* RENAMES {} */",
                    child.renames_target.as_deref().unwrap_or("?"),
                )
            };
            w.line(&format!("pub {field_name}: {rust_type},{comment}"));
        }
    }
}

/// Generate field initializations for level-66 RENAMES entries within a record.
fn generate_renames_field_inits(w: &mut RustWriter, record: &DataEntry, prefix: &str) {
    let record_prefix = if prefix.is_empty() {
        record.name.clone()
    } else {
        format!("{prefix}_{}", record.name)
    };

    for child in &record.children {
        if child.level == 66 {
            let field_name = cobol_to_rust_name(&child.name, &record_prefix);
            let resolved = resolve_renames_type_from_entry(child, record);
            let init = field_init_expr(child, &resolved.rust_type);
            w.line(&format!("{field_name}: {init},"));
        }
    }
}

/// Resolve the type for a level-66 RENAMES entry by looking at its target within the record.
///
/// - Single RENAMES: copies the target field's resolved type
/// - RENAMES THRU: creates a PicX spanning the combined byte lengths
fn resolve_renames_type_from_entry(
    renames_entry: &DataEntry,
    record: &DataEntry,
) -> crate::symbol_table::ResolvedType {
    use crate::symbol_table::{ResolvedType, RustType};

    let target_name = match &renames_entry.renames_target {
        Some(name) => name.to_uppercase(),
        None => {
            return ResolvedType {
                rust_type: RustType::PicX { length: 1 },
                byte_length: 1,
                is_group: false,
            };
        }
    };

    // Find the target entry within the record's children (recursively)
    let target = find_entry_by_name(record, &target_name);

    if let Some(ref thru_name) = renames_entry.renames_thru {
        // RENAMES X THRU Y: compute byte range -> PicX
        let thru_upper = thru_name.to_uppercase();
        let thru = find_entry_by_name(record, &thru_upper);

        if let (Some(t), Some(th)) = (target, thru) {
            let t_size = t.byte_length.unwrap_or(0);
            let th_size = th.byte_length.unwrap_or(0);
            let range_size = t_size + th_size;
            ResolvedType {
                rust_type: RustType::PicX {
                    length: range_size as u32,
                },
                byte_length: range_size,
                is_group: false,
            }
        } else {
            ResolvedType {
                rust_type: RustType::PicX { length: 1 },
                byte_length: 1,
                is_group: false,
            }
        }
    } else {
        // Single RENAMES: copy target's resolved type
        target
            .map(|t| resolve_type(t))
            .unwrap_or(ResolvedType {
                rust_type: RustType::PicX { length: 1 },
                byte_length: 1,
                is_group: false,
            })
    }
}

/// Recursively find a DataEntry by name within a record's children.
fn find_entry_by_name<'a>(record: &'a DataEntry, name: &str) -> Option<&'a DataEntry> {
    for child in &record.children {
        if child.name.to_uppercase() == name {
            return Some(child);
        }
        if let Some(found) = find_entry_by_name(child, name) {
            return Some(found);
        }
    }
    None
}

/// Convert a COBOL data name to a Rust field name.
///
/// COBOL names use hyphens; Rust uses snake_case.
pub fn cobol_to_rust_name(cobol_name: &str, prefix: &str) -> String {
    let base = cobol_name.to_lowercase().replace('-', "_");
    if prefix.is_empty() {
        base
    } else {
        let pfx = prefix.to_lowercase().replace('-', "_");
        // Avoid stuttering: if base already starts with prefix, don't double it
        if base.starts_with(&pfx) {
            base
        } else {
            format!("{pfx}_{base}")
        }
    }
}

/// Get the Rust type string for a resolved type.
fn rust_type_string(rt: &RustType) -> String {
    match rt {
        RustType::PackedDecimal {
            precision,
            scale,
            signed,
        } => format!(
            "PackedDecimal /* P{precision} S{scale} {} */",
            if *signed { "signed" } else { "unsigned" }
        ),
        RustType::PicX { length } => format!("PicX /* {length} */"),
        RustType::PicA { length } => format!("PicA /* {length} */"),
        RustType::AlphanumericEdited { length } => {
            format!("AlphanumericEdited /* {length} */")
        }
        RustType::CompBinary {
            precision,
            scale,
            signed,
            pic_limited,
        } => {
            let storage = if *precision <= 4 {
                if *signed { "i16" } else { "u16" }
            } else if *precision <= 9 {
                if *signed { "i32" } else { "u32" }
            } else {
                if *signed { "i64" } else { "u64" }
            };
            format!(
                "{storage} /* COMP P{precision} S{scale} {} */",
                if *pic_limited { "PIC-limited" } else { "full-range" }
            )
        }
        RustType::DisplayNumeric {
            precision,
            scale,
            signed,
        } => format!(
            "PackedDecimal /* Display P{precision} S{scale} {} */",
            if *signed { "signed" } else { "unsigned" }
        ),
        RustType::Float32 => "Comp1Float".to_string(),
        RustType::Float64 => "Comp2Float".to_string(),
        RustType::Index => "usize".to_string(),
        RustType::Pointer => "*const u8".to_string(),
        RustType::Group => "Vec<u8> /* GROUP */".to_string(),
    }
}

/// Generate an initialization expression for a field.
fn field_init_expr(entry: &DataEntry, rt: &RustType) -> String {
    // Check for VALUE clause
    if let Some(ref value) = entry.value {
        return value_to_init(value, rt);
    }

    // Default initialization
    match rt {
        RustType::PackedDecimal { precision, scale, signed } => {
            format!("PackedDecimal::new({precision}, {scale}, {signed})")
        }
        RustType::PicX { length } => {
            format!("PicX::spaces({length})")
        }
        RustType::PicA { length } => {
            format!("PicA::spaces({length})")
        }
        RustType::AlphanumericEdited { .. } => {
            alpha_edited_init_expr(entry)
        }
        RustType::CompBinary { signed, precision, .. } => {
            if *precision <= 4 {
                if *signed { "0i16" } else { "0u16" }.to_string()
            } else if *precision <= 9 {
                if *signed { "0i32" } else { "0u32" }.to_string()
            } else {
                if *signed { "0i64" } else { "0u64" }.to_string()
            }
        }
        RustType::DisplayNumeric { precision, scale, signed } => {
            format!("PackedDecimal::new({precision}, {scale}, {signed})")
        }
        RustType::Float32 => "Comp1Float::new()".to_string(),
        RustType::Float64 => "Comp2Float::new()".to_string(),
        RustType::Index => "0usize".to_string(),
        RustType::Pointer => "std::ptr::null()".to_string(),
        RustType::Group => "Vec::new()".to_string(),
    }
}

/// Convert a VALUE literal to a Rust initialization expression.
fn value_to_init(lit: &Literal, rt: &RustType) -> String {
    match (lit, rt) {
        (Literal::Numeric(n), RustType::PackedDecimal { precision, scale, signed }) => {
            format!(
                "{{ let mut _p = PackedDecimal::new({precision}, {scale}, {signed}); _p.pack(dec!({n})); _p }}"
            )
        }
        (Literal::Numeric(n), RustType::CompBinary { signed, precision, .. }) => {
            if *precision <= 4 {
                if *signed {
                    format!("{n}i16")
                } else {
                    format!("{n}u16")
                }
            } else if *precision <= 9 {
                if *signed {
                    format!("{n}i32")
                } else {
                    format!("{n}u32")
                }
            } else {
                if *signed {
                    format!("{n}i64")
                } else {
                    format!("{n}u64")
                }
            }
        }
        (Literal::Numeric(n), RustType::DisplayNumeric { precision, scale, signed }) => {
            format!(
                "{{ let mut _p = PackedDecimal::new({precision}, {scale}, {signed}); _p.pack(dec!({n})); _p }}"
            )
        }
        (Literal::Numeric(n), RustType::Float32) => format!("Comp1Float::from_f32({n}f32)"),
        (Literal::Numeric(n), RustType::Float64) => format!("Comp2Float::from_f64({n}f64)"),
        (Literal::Numeric(n), _) => format!("{n}"),
        (Literal::Alphanumeric(s), RustType::PicX { length }) => {
            format!("PicX::new({length}, b\"{s}\")")
        }
        (Literal::Alphanumeric(s), RustType::PicA { length }) => {
            format!("PicA::new({length}, b\"{s}\")")
        }
        (Literal::Alphanumeric(s), _) => format!("\"{s}\".to_string()"),
        (Literal::Figurative(fig), _) => {
            use crate::ast::FigurativeConstant;
            match fig {
                FigurativeConstant::Spaces => match rt {
                    RustType::PicX { length } => format!("PicX::spaces({length})"),
                    RustType::PicA { length } => format!("PicA::spaces({length})"),
                    _ => "Default::default()".to_string(),
                },
                FigurativeConstant::Zeros => match rt {
                    RustType::PackedDecimal { precision, scale, signed } => {
                        format!("PackedDecimal::new({precision}, {scale}, {signed})")
                    }
                    RustType::DisplayNumeric { precision, scale, signed } => {
                        format!("PackedDecimal::new({precision}, {scale}, {signed})")
                    }
                    RustType::CompBinary { .. } => "0".to_string(),
                    _ => "Default::default()".to_string(),
                },
                _ => "Default::default()".to_string(),
            }
        }
    }
}

/// Generate the init expression for an AlphanumericEdited field.
///
/// Emits `AlphanumericEdited::new(vec![AlphaEditSymbol::Data, ...])`.
fn alpha_edited_init_expr(entry: &DataEntry) -> String {
    use crate::parser::pic_parser::build_alpha_edit_pattern;

    if let Some(ref pic) = entry.pic {
        if let Some(pattern) = build_alpha_edit_pattern(pic) {
            let symbols: Vec<&str> = pattern
                .iter()
                .map(|ch| match ch {
                    'X' => "AlphaEditSymbol::Data",
                    'B' => "AlphaEditSymbol::Space",
                    '0' => "AlphaEditSymbol::Zero",
                    '/' => "AlphaEditSymbol::Slash",
                    _ => "AlphaEditSymbol::Data",
                })
                .collect();
            return format!(
                "AlphanumericEdited::new(vec![{}])",
                symbols.join(", ")
            );
        }
    }

    // Fallback: create with all Data positions
    let length = entry.byte_length.unwrap_or(1);
    let syms = vec!["AlphaEditSymbol::Data"; length];
    format!(
        "AlphanumericEdited::new(vec![{}])",
        syms.join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{PicCategory, PicClause, Usage};

    fn make_entry(name: &str, level: u8) -> DataEntry {
        DataEntry {
            level,
            name: name.to_string(),
            pic: None,
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: Vec::new(),
            condition_values: Vec::new(),
            byte_offset: None,
            byte_length: Some(5),
            renames_target: None,
            renames_thru: None,
        }
    }

    fn make_picx_entry(name: &str, level: u8, length: u32) -> DataEntry {
        DataEntry {
            pic: Some(PicClause {
                category: PicCategory::Alphanumeric,
                total_digits: length,
                scale: 0,
                raw: format!("X({})", length),
                signed: false,
                display_length: length,
                edit_symbols: Vec::new(),
            }),
            byte_length: Some(length as usize),
            ..make_entry(name, level)
        }
    }

    fn make_comp1_entry(name: &str, level: u8) -> DataEntry {
        DataEntry {
            pic: None,
            usage: Usage::Comp1,
            byte_length: Some(4),
            ..make_entry(name, level)
        }
    }

    fn make_comp2_entry(name: &str, level: u8) -> DataEntry {
        DataEntry {
            pic: None,
            usage: Usage::Comp2,
            byte_length: Some(8),
            ..make_entry(name, level)
        }
    }

    fn make_numeric_entry(name: &str, level: u8, prec: u32, scale: u32) -> DataEntry {
        DataEntry {
            pic: Some(PicClause {
                category: PicCategory::Numeric,
                total_digits: prec,
                scale,
                raw: format!("9({})", prec),
                signed: false,
                display_length: prec,
                edit_symbols: Vec::new(),
            }),
            byte_length: Some(prec as usize),
            ..make_entry(name, level)
        }
    }

    #[test]
    fn cobol_name_to_rust() {
        assert_eq!(cobol_to_rust_name("WS-COUNTER", ""), "ws_counter");
        assert_eq!(cobol_to_rust_name("FIELD-A", "WS-RECORD"), "ws_record_field_a");
    }

    #[test]
    fn generate_simple_struct() {
        let records = vec![make_entry("WS-COUNT", 77)];
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();
        assert!(output.contains("pub struct WorkingStorage"));
        assert!(output.contains("ws_count"));
        assert!(output.contains("impl WorkingStorage"));
        assert!(output.contains("fn new()"));
    }

    #[test]
    fn generate_occurs_array() {
        let mut entry = make_picx_entry("WS-TABLE-ITEM", 77, 10);
        entry.occurs = Some(5);

        let records = vec![entry];
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();

        assert!(output.contains("CobolArray<PicX"), "should wrap in CobolArray: {output}");
        assert!(output.contains("OCCURS 5"), "should note OCCURS count: {output}");
        assert!(
            output.contains("CobolArray::new(vec![PicX::spaces(10); 5])"),
            "should init with vec!: {output}"
        );
    }

    #[test]
    fn generate_occurs_depending_var_array() {
        let mut entry = make_picx_entry("WS-VAR-ITEM", 77, 8);
        entry.occurs = Some(100);
        entry.occurs_depending = Some("WS-COUNT".to_string());

        let records = vec![entry];
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();

        assert!(
            output.contains("CobolVarArray<PicX"),
            "should wrap in CobolVarArray: {output}"
        );
        assert!(
            output.contains("OCCURS DEPENDING ON"),
            "should note DEPENDING ON: {output}"
        );
        assert!(
            output.contains("CobolVarArray::new(vec![PicX::spaces(8); 100], 100)"),
            "should init with max count: {output}"
        );
    }

    #[test]
    fn generate_redefines_field() {
        let mut entry = make_entry("WS-DATE-PARTS", 5);
        entry.redefines = Some("WS-DATE".to_string());
        entry.byte_length = Some(8);

        let records = vec![
            // 01-level group with two children
            DataEntry {
                children: vec![
                    make_picx_entry("WS-DATE", 5, 8),
                    entry,
                ],
                ..make_entry("WS-RECORD", 1)
            },
        ];

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();

        assert!(
            output.contains("RedefinesGroup"),
            "should generate RedefinesGroup type: {output}"
        );
        assert!(
            output.contains("REDEFINES"),
            "should note REDEFINES: {output}"
        );
        assert!(
            output.contains("RedefinesGroup::new(8)"),
            "should init with byte size: {output}"
        );
    }

    #[test]
    fn generate_redefines_group_not_flattened() {
        // REDEFINES group with children should NOT flatten its children
        let mut redef_entry = DataEntry {
            children: vec![
                make_numeric_entry("WS-YEAR", 10, 4, 0),
                make_numeric_entry("WS-MONTH", 10, 2, 0),
                make_numeric_entry("WS-DAY", 10, 2, 0),
            ],
            ..make_entry("WS-DATE-PARTS", 5)
        };
        redef_entry.redefines = Some("WS-DATE".to_string());
        redef_entry.byte_length = Some(8);

        let records = vec![
            DataEntry {
                children: vec![
                    make_picx_entry("WS-DATE", 5, 8),
                    redef_entry,
                ],
                ..make_entry("WS-RECORD", 1)
            },
        ];

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();

        // Should have WS-DATE as PicX and WS-DATE-PARTS as RedefinesGroup
        assert!(output.contains("ws_record_ws_date: PicX"), "original field: {output}");
        assert!(
            output.contains("ws_record_ws_date_parts: RedefinesGroup"),
            "redefines field: {output}"
        );

        // Should NOT have the children flattened (WS-YEAR, WS-MONTH, WS-DAY)
        assert!(
            !output.contains("ws_year"),
            "should not flatten redefines children: {output}"
        );
        assert!(
            !output.contains("ws_month"),
            "should not flatten redefines children: {output}"
        );
    }


    #[test]
    fn generate_renames_single_field() {
        // 66 ALIAS RENAMES WS-NAME
        let mut renames = make_entry("WS-ALIAS", 66);
        renames.renames_target = Some("WS-NAME".to_string());

        let record = DataEntry {
            children: vec![
                make_picx_entry("WS-NAME", 5, 20),
                make_numeric_entry("WS-AGE", 5, 3, 0),
                renames,
            ],
            ..make_entry("WS-RECORD", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record]);
        let output = w.finish();

        // RENAMES field should appear in the struct
        assert!(
            output.contains("ws_record_ws_alias: PicX"),
            "RENAMES single should copy target type (PicX): {output}"
        );
        assert!(
            output.contains("RENAMES WS-NAME"),
            "should have RENAMES comment: {output}"
        );
    }

    #[test]
    fn generate_renames_thru_field() {
        // 66 ALIAS RENAMES WS-FIELD-A THRU WS-FIELD-B
        let mut renames = make_entry("WS-RANGE", 66);
        renames.renames_target = Some("WS-FIELD-A".to_string());
        renames.renames_thru = Some("WS-FIELD-B".to_string());

        let record = DataEntry {
            children: vec![
                make_picx_entry("WS-FIELD-A", 5, 10),
                make_picx_entry("WS-FIELD-B", 5, 15),
                renames,
            ],
            ..make_entry("WS-RECORD", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record]);
        let output = w.finish();

        // RENAMES THRU should create a PicX spanning both fields
        assert!(
            output.contains("ws_record_ws_range: PicX"),
            "RENAMES THRU should create PicX: {output}"
        );
        assert!(
            output.contains("RENAMES WS-FIELD-A THRU WS-FIELD-B"),
            "should have THRU comment: {output}"
        );
    }

    #[test]
    fn generate_renames_numeric_target() {
        // 66 ALIAS RENAMES WS-AMOUNT (numeric -> PackedDecimal)
        let mut renames = make_entry("WS-AMT-ALIAS", 66);
        renames.renames_target = Some("WS-AMOUNT".to_string());

        let record = DataEntry {
            children: vec![
                make_numeric_entry("WS-AMOUNT", 5, 7, 2),
                renames,
            ],
            ..make_entry("WS-RECORD", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record]);
        let output = w.finish();

        // Single RENAMES of numeric should copy the numeric type
        assert!(
            output.contains("ws_record_ws_amt_alias: PackedDecimal"),
            "RENAMES numeric target should produce PackedDecimal: {output}"
        );
        assert!(
            output.contains("PackedDecimal::new(7, 2, false)"),
            "init should match target's precision/scale: {output}"
        );
    }

    #[test]
    fn generate_renames_no_level66_noop() {
        // Record with no level-66 children: RENAMES pass should be a no-op
        let record = DataEntry {
            children: vec![
                make_picx_entry("WS-NAME", 5, 20),
                make_numeric_entry("WS-AGE", 5, 3, 0),
            ],
            ..make_entry("WS-RECORD", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record]);
        let output = w.finish();

        // Should still compile and produce valid output without level-66
        assert!(output.contains("pub struct WorkingStorage"));
        assert!(output.contains("ws_record_ws_name: PicX"));
        assert!(!output.contains("RENAMES"), "no RENAMES comment expected: {output}");
    }

    #[test]
    fn generate_numeric_occurs_array() {
        let mut entry = make_numeric_entry("WS-AMOUNTS", 77, 9, 2);
        entry.occurs = Some(10);

        let records = vec![entry];
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();

        assert!(
            output.contains("CobolArray<PackedDecimal"),
            "numeric array wraps PackedDecimal: {output}"
        );
        assert!(
            output.contains("CobolArray::new(vec![PackedDecimal::new(9, 2, false); 10])"),
            "init with vec!: {output}"
        );
    }

    fn make_alpha_edited_entry(name: &str, level: u8, pic_raw: &str, display_length: u32) -> DataEntry {
        DataEntry {
            pic: Some(PicClause {
                category: PicCategory::AlphanumericEdited,
                total_digits: 0,
                scale: 0,
                raw: pic_raw.to_string(),
                signed: false,
                display_length,
                edit_symbols: Vec::new(),
            }),
            byte_length: Some(display_length as usize),
            ..make_entry(name, level)
        }
    }

    #[test]
    fn generate_alpha_edited_field() {
        // PIC X(3)BX(3) -- alphanumeric edited with space insertion
        let entry = make_alpha_edited_entry("WS-FORMATTED", 77, "X(3)BX(3)", 7);
        let records = vec![entry];
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();

        assert!(
            output.contains("AlphanumericEdited"),
            "should generate AlphanumericEdited type: {output}"
        );
        assert!(
            output.contains("ws_formatted"),
            "should have field name: {output}"
        );
    }

    #[test]
    fn generate_alpha_edited_init() {
        // PIC X(2)/X(2) -- slash insertion
        let entry = make_alpha_edited_entry("WS-DATE-FMT", 77, "X(2)/X(2)", 5);
        let records = vec![entry];
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();

        assert!(
            output.contains("AlphanumericEdited::new(vec!["),
            "should generate new() with pattern: {output}"
        );
        assert!(
            output.contains("AlphaEditSymbol::Data"),
            "should have Data symbols: {output}"
        );
        assert!(
            output.contains("AlphaEditSymbol::Slash"),
            "should have Slash symbol: {output}"
        );
    }

    #[test]
    fn generate_comp1_field_type() {
        let entry = make_comp1_entry("WS-FLOAT", 5);
        let record = DataEntry {
            children: vec![entry],
            ..make_entry("WS-RECORD", 1)
        };
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record]);
        let output = w.finish();
        assert!(
            output.contains("Comp1Float"),
            "should generate Comp1Float type: {output}"
        );
    }

    #[test]
    fn generate_comp2_field_type() {
        let entry = make_comp2_entry("WS-DOUBLE", 5);
        let record = DataEntry {
            children: vec![entry],
            ..make_entry("WS-RECORD", 1)
        };
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record]);
        let output = w.finish();
        assert!(
            output.contains("Comp2Float"),
            "should generate Comp2Float type: {output}"
        );
    }

    #[test]
    fn generate_comp1_value_literal() {
        let mut entry = make_comp1_entry("WS-RATE", 5);
        entry.value = Some(crate::ast::Literal::Numeric("3.14".to_string()));
        let record = DataEntry {
            children: vec![entry],
            ..make_entry("WS-RECORD", 1)
        };
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record]);
        let output = w.finish();
        assert!(
            output.contains("Comp1Float::from_f32(3.14f32)"),
            "should generate Comp1Float::from_f32 init: {output}"
        );
    }
}
