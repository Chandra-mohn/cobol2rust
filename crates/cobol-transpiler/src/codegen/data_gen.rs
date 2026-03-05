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

    w.close_block("}");
    w.close_block("}");
    w.close_block("}");
}

/// Generate a single field declaration.
fn generate_field(w: &mut RustWriter, entry: &DataEntry, prefix: &str) {
    let field_name = cobol_to_rust_name(&entry.name, prefix);
    let resolved = resolve_type(entry);
    let rust_type = rust_type_string(&resolved.rust_type);
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
    let resolved = resolve_type(entry);
    let init_expr = field_init_expr(entry, &resolved.rust_type);
    w.line(&format!("{field_name}: {init_expr},"));
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
        if child.children.is_empty() {
            generate_field_init(w, child, &new_prefix);
        } else {
            generate_group_field_inits(w, child, &new_prefix);
        }
    }
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
        RustType::Float32 => "f32".to_string(),
        RustType::Float64 => "f64".to_string(),
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
        RustType::Float32 => "0.0f32".to_string(),
        RustType::Float64 => "0.0f64".to_string(),
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
        (Literal::Numeric(n), RustType::Float32) => format!("{n}f32"),
        (Literal::Numeric(n), RustType::Float64) => format!("{n}f64"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Usage;

    #[test]
    fn cobol_name_to_rust() {
        assert_eq!(cobol_to_rust_name("WS-COUNTER", ""), "ws_counter");
        assert_eq!(cobol_to_rust_name("FIELD-A", "WS-RECORD"), "ws_record_field_a");
    }

    #[test]
    fn generate_simple_struct() {
        let records = vec![
            DataEntry {
                level: 77,
                name: "WS-COUNT".to_string(),
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
            },
        ];

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records);
        let output = w.finish();
        assert!(output.contains("pub struct WorkingStorage"));
        assert!(output.contains("ws_count"));
        assert!(output.contains("impl WorkingStorage"));
        assert!(output.contains("fn new()"));
    }
}
