//! Data division code generator.
//!
//! Generates Rust struct definitions from COBOL DATA DIVISION entries.
//! Each 01-level record becomes a struct, with fields generated from
//! the child entries.

use std::collections::HashSet;

use crate::ast::{AccessMode, DataEntry, FileDescription, FileOrganization, Literal};
use crate::codegen::rust_writer::RustWriter;
use crate::symbol_table::{resolve_type, RustType};

/// Generate the `WorkingStorage` struct and its `new()` constructor.
pub fn generate_working_storage(
    w: &mut RustWriter,
    records: &[DataEntry],
    file_section: &[FileDescription],
    has_sql: bool,
) {
    // Pre-pass: collect all leaf field names to detect duplicates
    let mut name_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for record in records {
        collect_leaf_names(record, &mut name_counts);
    }
    for fd in file_section {
        for record in &fd.records {
            collect_leaf_names(record, &mut name_counts);
        }
    }
    let duplicates: HashSet<String> = name_counts
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(name, _)| name.clone())
        .collect();

    let mut filler_counter = 0u32;

    w.line("/// Working storage data fields.");
    w.line("#[allow(non_snake_case)]");
    w.open_block("pub struct WorkingStorage {");

    // FILE SECTION: file handle fields
    for fd in file_section {
        let fname = cobol_to_rust_name(&fd.file_name, "");
        let file_type = file_handle_type(fd);
        w.line(&format!("pub {fname}: {file_type},"));
    }

    // FILE SECTION: record fields (same as WS fields)
    for fd in file_section {
        for record in &fd.records {
            generate_ws_record(w, record, &duplicates, &mut filler_counter);
        }
    }

    // WORKING-STORAGE SECTION fields
    for record in records {
        generate_ws_record(w, record, &duplicates, &mut filler_counter);
    }

    // Auto-inject SQLCA when EXEC SQL is present
    if has_sql {
        w.line("/// SQL Communication Area (auto-injected).");
        w.line("pub sqlca: Sqlca,");
    }

    // Second pass: generate level-66 RENAMES fields
    for fd in file_section {
        for record in &fd.records {
            if record.level == 1 {
                generate_renames_fields(w, record, "");
            }
        }
    }
    for record in records {
        if record.level == 1 {
            generate_renames_fields(w, record, "");
        }
    }

    w.close_block("}");
    w.blank_line();

    // Generate new() constructor
    filler_counter = 0;
    w.open_block("impl WorkingStorage {");
    w.line("#[allow(non_snake_case)]");
    w.open_block("pub fn new() -> Self {");
    w.open_block("Self {");

    // FILE SECTION: file handle inits
    for fd in file_section {
        let fname = cobol_to_rust_name(&fd.file_name, "");
        let init = file_handle_init(fd, records);
        w.line(&format!("{fname}: {init},"));
    }

    // FILE SECTION: record inits
    for fd in file_section {
        for record in &fd.records {
            generate_ws_record_init(w, record, &duplicates, &mut filler_counter);
        }
    }

    // WORKING-STORAGE SECTION inits
    for record in records {
        generate_ws_record_init(w, record, &duplicates, &mut filler_counter);
    }

    // Auto-inject SQLCA init
    if has_sql {
        w.line("sqlca: Sqlca::default(),");
    }

    // Second pass: initialize level-66 RENAMES fields
    for fd in file_section {
        for record in &fd.records {
            if record.level == 1 {
                generate_renames_field_inits(w, record, "");
            }
        }
    }
    for record in records {
        if record.level == 1 {
            generate_renames_field_inits(w, record, "");
        }
    }

    w.close_block("}");
    w.close_block("}");
    w.close_block("}");
}

/// Generate struct fields for a single WS or FILE SECTION record.
fn generate_ws_record(
    w: &mut RustWriter,
    record: &DataEntry,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
) {
    if record.level == 77 {
        generate_field(w, record, "", duplicates, filler_counter, &[]);
    } else if record.level == 1 {
        if has_data_children(record) {
            let group_size = compute_group_byte_length(record);
            if group_size > 0 {
                let group_name = cobol_to_rust_name(&record.name, "");
                w.line(&format!(
                    "pub {group_name}: PicX, /* GROUP {group_size} bytes */",
                ));
            }
            generate_group_fields(w, record, &record.name, duplicates, filler_counter, &[]);
        } else {
            generate_field(w, record, "", duplicates, filler_counter, &[]);
        }
    }
}

/// Generate init expressions for a single WS or FILE SECTION record.
fn generate_ws_record_init(
    w: &mut RustWriter,
    record: &DataEntry,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
) {
    if record.level == 77 {
        generate_field_init(w, record, "", duplicates, filler_counter, &[]);
    } else if record.level == 1 {
        if has_data_children(record) {
            let group_size = compute_group_byte_length(record);
            if group_size > 0 {
                let group_name = cobol_to_rust_name(&record.name, "");
                w.line(&format!(
                    "{group_name}: PicX::spaces({group_size}),",
                ));
            }
            generate_group_field_inits(w, record, &record.name, duplicates, filler_counter, &[]);
        } else {
            generate_field_init(w, record, "", duplicates, filler_counter, &[]);
        }
    }
}

/// Determine the Rust file handle type for a `FileDescription`.
fn file_handle_type(fd: &FileDescription) -> &'static str {
    match fd.organization {
        FileOrganization::Indexed => "IndexedFile",
        FileOrganization::Relative => "RelativeFile",
        FileOrganization::Sequential | FileOrganization::LineSequential => "SequentialFile",
    }
}

/// Generate the initialization expression for a file handle field.
fn file_handle_init(fd: &FileDescription, ws_records: &[DataEntry]) -> String {
    let default_assign = fd.file_name.to_lowercase();
    let assign = fd
        .assign_to
        .as_deref()
        .unwrap_or(&default_assign);
    // Try fd.records first, then fall back to searching WS records
    let record_len = fd
        .records
        .first()
        .map(|r| compute_group_byte_length(r).max(1))
        .or_else(|| {
            // Find the first 01-level record in WS that isn't a simple field
            // (heuristic: records under FD are usually groups with children)
            ws_records.iter()
                .find(|r| r.level == 1 && !r.children.is_empty())
                .map(|r| compute_group_byte_length(r).max(1))
        })
        .unwrap_or(80);

    match fd.organization {
        FileOrganization::Sequential | FileOrganization::LineSequential => {
            let org = match fd.organization {
                FileOrganization::LineSequential => "FileOrganization::LineSequential",
                _ => "FileOrganization::Sequential",
            };
            format!(
                "SequentialFile::new(\"{}\".to_string(), std::path::PathBuf::from(\"{assign}\"), {org}, {record_len})",
                fd.file_name
            )
        }
        FileOrganization::Relative => {
            let access = match fd.access_mode {
                AccessMode::Random => "FileAccessMode::Random",
                AccessMode::Dynamic => "FileAccessMode::Dynamic",
                AccessMode::Sequential => "FileAccessMode::Sequential",
            };
            format!(
                "RelativeFile::new(\"{}\".to_string(), std::path::PathBuf::from(\"{assign}\"), {record_len}, {access})",
                fd.file_name
            )
        }
        FileOrganization::Indexed => {
            let access = match fd.access_mode {
                AccessMode::Random => "FileAccessMode::Random",
                AccessMode::Dynamic => "FileAccessMode::Dynamic",
                AccessMode::Sequential => "FileAccessMode::Sequential",
            };
            // Compute key offset and length from record layout
            let (key_offset, key_length) = compute_key_position(fd, ws_records);
            format!(
                "IndexedFile::new(\"{}\".to_string(), std::path::PathBuf::from(\"{assign}\"), {record_len}, {access}, {key_offset}, {key_length})",
                fd.file_name
            )
        }
    }
}

/// Compute the byte offset and length of the record key field within the record.
fn compute_key_position(fd: &FileDescription, ws_records: &[DataEntry]) -> (usize, usize) {
    let key_name = fd.record_key.as_deref().unwrap_or("");
    if key_name.is_empty() {
        return (0, 5); // default fallback
    }
    let key_upper = key_name.to_uppercase();
    // Walk the first record's children to find the key field
    // Try fd.records first, then WS records
    let search_records: Vec<&DataEntry> = if fd.records.is_empty() {
        ws_records.iter().filter(|r| r.level == 1).collect()
    } else {
        fd.records.iter().collect()
    };
    if let Some(record) = search_records.first() {
        if let Some((offset, length)) = find_field_position(record, &key_upper, 0) {
            return (offset, length);
        }
    }
    (0, 5) // fallback
}

/// Find a field's byte offset and length within a record hierarchy.
fn find_field_position(entry: &DataEntry, target: &str, base_offset: usize) -> Option<(usize, usize)> {
    if entry.name.to_uppercase() == target {
        let size = entry.byte_length.unwrap_or_else(|| compute_group_byte_length(entry));
        return Some((base_offset, size));
    }
    let mut offset = base_offset;
    for child in &entry.children {
        if child.level == 88 || child.level == 66 {
            continue;
        }
        if let Some(result) = find_field_position(child, target, offset) {
            return Some(result);
        }
        let child_size = child.byte_length.unwrap_or_else(|| compute_group_byte_length(child));
        offset += child_size;
    }
    None
}

/// Check if a DataEntry has real (non-88, non-66) children making it a group.
fn has_data_children(entry: &DataEntry) -> bool {
    entry.children.iter().any(|c| c.level != 88 && c.level != 66)
}

/// Check if a DataEntry is a true group (has children but NO PIC clause).
/// A field with its own PIC clause is always a leaf, even if it has subordinate items
/// (which is technically invalid COBOL but can appear from the parser).
fn is_true_group(entry: &DataEntry) -> bool {
    entry.pic.is_none() && has_data_children(entry)
}

/// Collect all leaf field names to detect duplicates.
/// Public so transpile.rs/proc_gen can reuse the same logic.
pub fn collect_leaf_names(entry: &DataEntry, counts: &mut std::collections::HashMap<String, usize>) {
    if entry.level == 88 || entry.level == 66 {
        return;
    }
    let name_upper = entry.name.to_uppercase();
    if name_upper == "FILLER" || name_upper.is_empty() {
        return; // FILLER items are numbered separately
    }
    if entry.children.is_empty() {
        let rust_name = cobol_to_rust_name(&entry.name, "");
        *counts.entry(rust_name).or_insert(0) += 1;
    } else {
        for child in &entry.children {
            collect_leaf_names(child, counts);
        }
    }
}

/// Compute total byte length for a group record.
fn compute_group_byte_length(entry: &DataEntry) -> usize {
    if let Some(len) = entry.byte_length {
        return len;
    }
    // Fallback: sum children
    let mut total = 0usize;
    for child in &entry.children {
        if child.level == 88 || child.level == 66 {
            continue;
        }
        if child.children.is_empty() {
            total += child.byte_length.unwrap_or(0);
        } else {
            total += compute_group_byte_length(child);
        }
    }
    total
}

/// Resolve the Rust field name for a data entry, handling FILLER numbering
/// and duplicate-name disambiguation.
fn resolve_field_name(
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
) -> String {
    let name_upper = entry.name.to_uppercase();
    if name_upper == "FILLER" || name_upper.is_empty() {
        *filler_counter += 1;
        return format!("_filler_{}", *filler_counter);
    }
    let base_name = cobol_to_rust_name(&entry.name, "");
    if duplicates.contains(&base_name) && !parent_group.is_empty() {
        // Disambiguate with parent group prefix
        cobol_to_rust_name(&entry.name, parent_group)
    } else {
        base_name
    }
}

/// Generate the `LinkageSection` struct and its `new()` constructor.
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

    // Linkage sections are smaller; still apply duplicate/filler logic for consistency
    let mut name_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for record in records {
        collect_leaf_names(record, &mut name_counts);
    }
    let duplicates: HashSet<String> = name_counts
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(name, _)| name.clone())
        .collect();
    let mut filler_counter = 0u32;

    w.line("/// Linkage section data fields (CALL parameters).");
    w.line("#[allow(non_snake_case)]");
    w.open_block("pub struct LinkageSection {");

    for record in records {
        if record.level == 77 {
            generate_field(w, record, "", &duplicates, &mut filler_counter, &[]);
        } else if record.level == 1 {
            if has_data_children(record) {
                let group_size = compute_group_byte_length(record);
                if group_size > 0 {
                    let group_name = cobol_to_rust_name(&record.name, "");
                    w.line(&format!(
                        "pub {group_name}: PicX, /* GROUP {group_size} bytes */",
                    ));
                }
                generate_group_fields(w, record, &record.name, &duplicates, &mut filler_counter, &[]);
            } else {
                generate_field(w, record, "", &duplicates, &mut filler_counter, &[]);
            }
        }
    }

    w.close_block("}");
    w.blank_line();

    filler_counter = 0;
    w.open_block("impl LinkageSection {");
    w.line("#[allow(non_snake_case)]");
    w.open_block("pub fn new() -> Self {");
    w.open_block("Self {");

    for record in records {
        if record.level == 77 {
            generate_field_init(w, record, "", &duplicates, &mut filler_counter, &[]);
        } else if record.level == 1 {
            if has_data_children(record) {
                let group_size = compute_group_byte_length(record);
                if group_size > 0 {
                    let group_name = cobol_to_rust_name(&record.name, "");
                    w.line(&format!(
                        "{group_name}: PicX::spaces({group_size}),",
                    ));
                }
                generate_group_field_inits(w, record, &record.name, &duplicates, &mut filler_counter, &[]);
            } else {
                generate_field_init(w, record, "", &duplicates, &mut filler_counter, &[]);
            }
        }
    }

    w.close_block("}");
    w.close_block("}");
    w.close_block("}");
}

/// Generate a single field declaration.
///
/// `ancestor_occurs` tracks OCCURS counts from parent groups (outermost first),
/// enabling nested `CobolArray` types for multi-dimensional COBOL tables.
fn generate_field(
    w: &mut RustWriter,
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
    ancestor_occurs: &[usize],
) {
    let field_name = resolve_field_name(entry, parent_group, duplicates, filler_counter);

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
        let inner = format!("CobolVarArray<{rust_type}>");
        let wrapped = wrap_ancestor_occurs(&inner, ancestor_occurs);
        w.line(&format!(
            "pub {field_name}: {wrapped}, /* OCCURS DEPENDING ON */"
        ));
        return;
    }

    // OCCURS: fixed-size array
    if let Some(count) = entry.occurs {
        let inner = format!("CobolArray<{rust_type}>");
        let wrapped = wrap_ancestor_occurs(&inner, ancestor_occurs);
        w.line(&format!(
            "pub {field_name}: {wrapped}, /* OCCURS {count} */"
        ));
        // Generate implicit index fields from INDEXED BY
        for idx_name in &entry.index_names {
            let idx_field = cobol_to_rust_name(idx_name, "");
            w.line(&format!(
                "pub {idx_field}: PackedDecimal, /* INDEXED BY {idx_name} */"
            ));
        }
        return;
    }

    // Plain field -- wrap in ancestor CobolArrays if any
    let wrapped = wrap_ancestor_occurs(&rust_type, ancestor_occurs);
    w.line(&format!("pub {field_name}: {wrapped},"));
}

/// Generate flattened fields for a group record.
///
/// Children use their own COBOL name without parent group prefix,
/// UNLESS the name is duplicated across groups (then prefixed with parent).
/// FILLER items get unique numbered names.
///
/// `ancestor_occurs` tracks OCCURS counts from parent groups for
/// multi-dimensional array support.
fn generate_group_fields(
    w: &mut RustWriter,
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
    ancestor_occurs: &[usize],
) {
    for child in &entry.children {
        if child.level == 88 || child.level == 66 {
            continue;
        }
        // REDEFINES group: emit a single RedefinesGroup field
        // (don't recurse into children -- they access via byte offsets)
        if child.redefines.is_some() {
            generate_field(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
            continue;
        }
        if is_true_group(child) {
            // Sub-group (no PIC): emit a PicX overlay for the sub-group itself
            let sub_size = compute_group_byte_length(child);
            if sub_size > 0 {
                let sub_name = resolve_field_name(child, parent_group, duplicates, filler_counter);
                let pic_type = wrap_ancestor_occurs("PicX", ancestor_occurs);
                w.line(&format!(
                    "pub {sub_name}: {pic_type}, /* sub-GROUP {sub_size} bytes */",
                ));
            }
            // If this group has OCCURS, push it onto the ancestor chain
            let mut child_ancestors = ancestor_occurs.to_vec();
            if let Some(count) = child.occurs {
                child_ancestors.push(count as usize);
            }
            generate_group_fields(w, child, &child.name, duplicates, filler_counter, &child_ancestors);
        } else if has_data_children(child) && child.pic.is_some() {
            // Field with PIC AND children (invalid COBOL but tolerated):
            // emit as leaf, then also recurse into children
            generate_field(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
            generate_group_fields(w, child, &child.name, duplicates, filler_counter, ancestor_occurs);
        } else {
            generate_field(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
        }
    }
}

/// Generate a field initialization expression.
///
/// `ancestor_occurs` tracks OCCURS counts from parent groups (outermost first),
/// enabling nested `CobolArray` inits for multi-dimensional COBOL tables.
fn generate_field_init(
    w: &mut RustWriter,
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
    ancestor_occurs: &[usize],
) {
    let field_name = resolve_field_name(entry, parent_group, duplicates, filler_counter);

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
            let inner = format!("CobolVarArray::new(vec![{element_init}; {count}], {count})");
            let wrapped = wrap_ancestor_occurs_init(&inner, ancestor_occurs);
            w.line(&format!("{field_name}: {wrapped},"));
            // Initialize implicit index fields from INDEXED BY
            for idx_name in &entry.index_names {
                let idx_field = cobol_to_rust_name(idx_name, "");
                w.line(&format!("{idx_field}: PackedDecimal::new(4, 0, false),"));
            }
            return;
        }

        // OCCURS: fixed-size array
        let inner = format!("CobolArray::new(vec![{element_init}; {count}])");
        let wrapped = wrap_ancestor_occurs_init(&inner, ancestor_occurs);
        w.line(&format!("{field_name}: {wrapped},"));
        // Initialize implicit index fields from INDEXED BY
        for idx_name in &entry.index_names {
            let idx_field = cobol_to_rust_name(idx_name, "");
            w.line(&format!("{idx_field}: PackedDecimal::new(4, 0, false),"));
        }
        return;
    }

    // Plain field -- wrap in ancestor CobolArrays if any
    let wrapped = wrap_ancestor_occurs_init(&element_init, ancestor_occurs);
    w.line(&format!("{field_name}: {wrapped},"));
}

/// Generate field initializations for a group.
///
/// Matches `generate_group_fields`: children use their own name, with
/// duplicate disambiguation and FILLER numbering.
///
/// `ancestor_occurs` tracks OCCURS counts from parent groups for
/// multi-dimensional array support.
fn generate_group_field_inits(
    w: &mut RustWriter,
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
    ancestor_occurs: &[usize],
) {
    for child in &entry.children {
        if child.level == 88 || child.level == 66 {
            continue;
        }
        // REDEFINES group: single RedefinesGroup init
        if child.redefines.is_some() {
            generate_field_init(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
            continue;
        }
        if is_true_group(child) {
            // Sub-group (no PIC): init the PicX overlay
            let sub_size = compute_group_byte_length(child);
            if sub_size > 0 {
                let sub_name = resolve_field_name(child, parent_group, duplicates, filler_counter);
                let init = wrap_ancestor_occurs_init(&format!("PicX::spaces({sub_size})"), ancestor_occurs);
                w.line(&format!(
                    "{sub_name}: {init},",
                ));
            }
            // If this group has OCCURS, push it onto the ancestor chain
            let mut child_ancestors = ancestor_occurs.to_vec();
            if let Some(count) = child.occurs {
                child_ancestors.push(count as usize);
            }
            generate_group_field_inits(w, child, &child.name, duplicates, filler_counter, &child_ancestors);
        } else if has_data_children(child) && child.pic.is_some() {
            // Field with PIC AND children (invalid COBOL but tolerated):
            // emit as leaf, then also recurse into children
            generate_field_init(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
            generate_group_field_inits(w, child, &child.name, duplicates, filler_counter, ancestor_occurs);
        } else {
            generate_field_init(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
        }
    }
}

/// Generate struct fields for level-66 RENAMES entries within a record.
///
/// RENAMES creates an alias for another field (or a byte range of fields).
/// We emit a separate struct field with the resolved type from the symbol table.
fn generate_renames_fields(w: &mut RustWriter, record: &DataEntry, _prefix: &str) {
    for child in &record.children {
        if child.level == 66 {
            let field_name = cobol_to_rust_name(&child.name, "");
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
fn generate_renames_field_inits(w: &mut RustWriter, record: &DataEntry, _prefix: &str) {
    for child in &record.children {
        if child.level == 66 {
            let field_name = cobol_to_rust_name(&child.name, "");
            let resolved = resolve_renames_type_from_entry(child, record);
            let init = field_init_expr(child, &resolved.rust_type);
            w.line(&format!("{field_name}: {init},"));
        }
    }
}

/// Resolve the type for a level-66 RENAMES entry by looking at its target within the record.
///
/// - Single RENAMES: copies the target field's resolved type
/// - RENAMES THRU: creates a `PicX` spanning the combined byte lengths
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
            .map_or(ResolvedType {
                rust_type: RustType::PicX { length: 1 },
                byte_length: 1,
                is_group: false,
            }, resolve_type)
    }
}

/// Recursively find a `DataEntry` by name within a record's children.
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
/// COBOL names use hyphens; Rust uses `snake_case`.
/// Rust keywords are escaped with `r#` prefix.
/// Names starting with a digit get a `n` prefix (Rust identifiers can't start with digits).
pub fn cobol_to_rust_name(cobol_name: &str, prefix: &str) -> String {
    let base = cobol_name.to_lowercase().replace('-', "_");
    let name = if prefix.is_empty() {
        base
    } else {
        let pfx = prefix.to_lowercase().replace('-', "_");
        // Avoid stuttering: if base already starts with prefix, don't double it
        if base.starts_with(&pfx) {
            base
        } else {
            format!("{pfx}_{base}")
        }
    };
    // Rust identifiers cannot start with a digit
    let name = if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("n{name}")
    } else {
        name
    };
    escape_rust_keyword(&name)
}

/// Escape Rust reserved keywords by prefixing with `r#`.
fn escape_rust_keyword(name: &str) -> String {
    match name {
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum"
        | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let"
        | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return"
        | "self" | "static" | "struct" | "super" | "trait" | "true" | "type"
        | "unsafe" | "use" | "where" | "while" | "async" | "await" | "dyn"
        | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override"
        | "priv" | "typeof" | "unsized" | "virtual" | "yield" | "try" => {
            format!("r#{name}")
        }
        _ => name.to_string(),
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
            format!(
                "CompBinary /* COMP P{precision} S{scale} {} {} */",
                if *signed { "signed" } else { "unsigned" },
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
        RustType::PackedDecimal { precision, scale, signed }
        | RustType::DisplayNumeric { precision, scale, signed } => {
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
        RustType::CompBinary { signed, precision, scale, pic_limited } => {
            format!("CompBinary::new({precision}, {scale}, {signed}, {pic_limited})")
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
        (Literal::Numeric(n), RustType::PackedDecimal { precision, scale, signed }
        | RustType::DisplayNumeric { precision, scale, signed }) => {
            format!(
                "{{ let mut _p = PackedDecimal::new({precision}, {scale}, {signed}); _p.pack(\"{n}\".parse::<Decimal>().unwrap()); _p }}"
            )
        }
        (Literal::Numeric(n), RustType::CompBinary { signed, precision, scale, pic_limited }) => {
            format!(
                "{{ let mut _c = CompBinary::new({precision}, {scale}, {signed}, {pic_limited}); _c.set_decimal(\"{n}\".parse::<Decimal>().unwrap()); _c }}"
            )
        }
        (Literal::Numeric(n), RustType::Float32) => format!("Comp1Float::from_f32({n}f32)"),
        (Literal::Numeric(n), RustType::Float64) => format!("Comp2Float::from_f64({n}f64)"),
        // Numeric value on alphanumeric field (parser may mis-classify quoted strings)
        (Literal::Numeric(n), RustType::PicX { length }) => {
            format!("PicX::new({length}, b\"{n}\")")
        }
        (Literal::Numeric(n), RustType::PicA { length }) => {
            format!("PicA::new({length}, b\"{n}\")")
        }
        (Literal::Numeric(n), _) => n.clone(),
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
                    RustType::PackedDecimal { precision, scale, signed }
                    | RustType::DisplayNumeric { precision, scale, signed } => {
                        format!("PackedDecimal::new({precision}, {scale}, {signed})")
                    }
                    RustType::CompBinary { precision, scale, signed, pic_limited } => {
                        format!("CompBinary::new({precision}, {scale}, {signed}, {pic_limited})")
                    }
                    _ => "Default::default()".to_string(),
                },
                FigurativeConstant::All(pattern) => match rt {
                    RustType::PicX { length } => {
                        let len = *length as usize;
                        let repeated = pattern.repeat(len.div_ceil(pattern.len()));
                        let truncated = &repeated[..len];
                        format!("PicX::new({length}, b\"{truncated}\")")
                    }
                    RustType::PicA { length } => {
                        let len = *length as usize;
                        let repeated = pattern.repeat(len.div_ceil(pattern.len()));
                        let truncated = &repeated[..len];
                        format!("PicA::new({length}, b\"{truncated}\")")
                    }
                    _ => "Default::default()".to_string(),
                },
                _ => "Default::default()".to_string(),
            }
        }
    }
}

/// Generate the init expression for an `AlphanumericEdited` field.
///
/// Emits `AlphanumericEdited::new(vec![AlphaEditSymbol::Data, ...])`.
fn alpha_edited_init_expr(entry: &DataEntry) -> String {
    use crate::parser::pic_parser::build_alpha_edit_pattern;

    if let Some(ref pic) = entry.pic {
        if let Some(pattern) = build_alpha_edit_pattern(pic) {
            let symbols: Vec<&str> = pattern
                .iter()
                .map(|ch| match ch {
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

/// Wrap a Rust type string in nested `CobolArray<...>` for each ancestor OCCURS.
///
/// For example, if `ancestor_occurs` is `[3]` and `inner` is `CobolArray<PackedDecimal>`,
/// the result is `CobolArray<CobolArray<PackedDecimal>>`.
/// The outermost ancestor (first in the slice) becomes the outermost wrapper.
fn wrap_ancestor_occurs(inner: &str, ancestor_occurs: &[usize]) -> String {
    if ancestor_occurs.is_empty() {
        return inner.to_string();
    }
    let mut result = inner.to_string();
    // Wrap from innermost ancestor to outermost so outer is the outermost CobolArray
    for _count in ancestor_occurs.iter().rev() {
        result = format!("CobolArray<{result}>");
    }
    result
}

/// Wrap an init expression in nested `CobolArray::new(vec![...])` for each ancestor OCCURS.
///
/// For example, if `ancestor_occurs` is `[3]` and `inner` is
/// `CobolArray::new(vec![PackedDecimal::new(3, 0, false); 3])`,
/// the result wraps it in `CobolArray::new(vec![...; 3])`.
fn wrap_ancestor_occurs_init(inner: &str, ancestor_occurs: &[usize]) -> String {
    if ancestor_occurs.is_empty() {
        return inner.to_string();
    }
    let mut result = inner.to_string();
    // Wrap from innermost ancestor to outermost
    for count in ancestor_occurs.iter().rev() {
        result = format!("CobolArray::new(vec![{result}; {count}])");
    }
    result
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
            index_names: Vec::new(),
        }
    }

    fn make_picx_entry(name: &str, level: u8, length: u32) -> DataEntry {
        DataEntry {
            pic: Some(PicClause {
                category: PicCategory::Alphanumeric,
                total_digits: length,
                scale: 0,
                raw: format!("X({length})"),
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
                raw: format!("9({prec})"),
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
        generate_working_storage(&mut w, &records, &[], false);
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
        generate_working_storage(&mut w, &records, &[], false);
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
        generate_working_storage(&mut w, &records, &[], false);
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
        generate_working_storage(&mut w, &records, &[], false);
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
        generate_working_storage(&mut w, &records, &[], false);
        let output = w.finish();

        // Should have WS-DATE as PicX and WS-DATE-PARTS as RedefinesGroup
        assert!(output.contains("ws_date: PicX"), "original field: {output}");
        assert!(
            output.contains("ws_date_parts: RedefinesGroup"),
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
        generate_working_storage(&mut w, &[record], &[], false);
        let output = w.finish();

        // RENAMES field should appear in the struct
        assert!(
            output.contains("ws_alias: PicX"),
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
        generate_working_storage(&mut w, &[record], &[], false);
        let output = w.finish();

        // RENAMES THRU should create a PicX spanning both fields
        assert!(
            output.contains("ws_range: PicX"),
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
        generate_working_storage(&mut w, &[record], &[], false);
        let output = w.finish();

        // Single RENAMES of numeric should copy the numeric type
        assert!(
            output.contains("ws_amt_alias: PackedDecimal"),
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
        generate_working_storage(&mut w, &[record], &[], false);
        let output = w.finish();

        // Should still compile and produce valid output without level-66
        assert!(output.contains("pub struct WorkingStorage"));
        assert!(output.contains("ws_name: PicX"));
        assert!(!output.contains("RENAMES"), "no RENAMES comment expected: {output}");
    }

    #[test]
    fn generate_numeric_occurs_array() {
        let mut entry = make_numeric_entry("WS-AMOUNTS", 77, 9, 2);
        entry.occurs = Some(10);

        let records = vec![entry];
        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &records, &[], false);
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
        generate_working_storage(&mut w, &records, &[], false);
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
        generate_working_storage(&mut w, &records, &[], false);
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
        generate_working_storage(&mut w, &[record], &[], false);
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
        generate_working_storage(&mut w, &[record], &[], false);
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
        generate_working_storage(&mut w, &[record], &[], false);
        let output = w.finish();
        assert!(
            output.contains("Comp1Float::from_f32(3.14f32)"),
            "should generate Comp1Float::from_f32 init: {output}"
        );
    }

    // -----------------------------------------------------------------------
    // PIC + children: field with PIC should be emitted as leaf
    // even if it has subordinate items (invalid COBOL, but tolerated)
    // -----------------------------------------------------------------------

    #[test]
    fn pic_with_children_emits_leaf_and_children() {
        // Level 11 WS-FLAG PIC X with child level 12 WS-FLAG-V PIC X
        let flag_v = make_picx_entry("WS-FLAG-V", 12, 1);
        let mut flag = make_picx_entry("WS-FLAG", 11, 1);
        flag.children = vec![flag_v];

        let parent = DataEntry {
            children: vec![
                make_picx_entry("WS-ALPHA", 5, 10),
                flag,
            ],
            ..make_entry("WS-RECORD", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[parent], &[], false);
        let output = w.finish();

        // Both WS-FLAG and WS-FLAG-V should be in the struct
        assert!(output.contains("ws_flag:"), "PIC field with children should still appear: {output}");
        assert!(output.contains("ws_flag_v:"), "child of PIC field should also appear: {output}");
    }

    #[test]
    fn pic_with_children_both_initialized() {
        let flag_v = make_picx_entry("WS-FLAG-V", 12, 1);
        let mut flag = make_picx_entry("WS-FLAG", 11, 1);
        flag.children = vec![flag_v];

        let parent = DataEntry {
            children: vec![flag],
            ..make_entry("WS-RECORD", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[parent], &[], false);
        let output = w.finish();

        // Both should be initialized in new()
        let new_section = output.split("fn new()").nth(1).unwrap_or("");
        assert!(new_section.contains("ws_flag:"), "PIC field should be initialized: {output}");
        assert!(new_section.contains("ws_flag_v:"), "child should be initialized: {output}");
    }

    #[test]
    fn is_true_group_with_pic() {
        let mut entry = make_picx_entry("WS-FIELD", 5, 1);
        entry.children = vec![make_picx_entry("WS-CHILD", 10, 1)];
        // Has PIC + children -> NOT a true group
        assert!(!is_true_group(&entry), "PIC field with children is not a true group");
    }

    #[test]
    fn is_true_group_without_pic() {
        let mut entry = make_entry("WS-GROUP", 5);
        entry.children = vec![make_picx_entry("WS-CHILD", 10, 5)];
        // No PIC + has children -> IS a true group
        assert!(is_true_group(&entry), "no-PIC field with children is a true group");
    }

    // -----------------------------------------------------------------------
    // INDEXED BY: generates PackedDecimal fields for implicit index names
    // -----------------------------------------------------------------------

    #[test]
    fn indexed_by_generates_field() {
        let mut entry = make_picx_entry("WS-ITEM", 5, 10);
        entry.occurs = Some(10);
        entry.index_names = vec!["WS-IX".to_string()];

        let parent = DataEntry {
            children: vec![entry],
            ..make_entry("WS-TABLE", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[parent], &[], false);
        let output = w.finish();

        assert!(output.contains("pub ws_ix: PackedDecimal"), "should generate INDEXED BY field: {output}");
        assert!(output.contains("INDEXED BY WS-IX"), "should have INDEXED BY comment: {output}");
    }

    #[test]
    fn indexed_by_initializes_field() {
        let mut entry = make_picx_entry("WS-ITEM", 5, 10);
        entry.occurs = Some(10);
        entry.index_names = vec!["WS-IX".to_string()];

        let parent = DataEntry {
            children: vec![entry],
            ..make_entry("WS-TABLE", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[parent], &[], false);
        let output = w.finish();

        let new_section = output.split("fn new()").nth(1).unwrap_or("");
        assert!(new_section.contains("ws_ix: PackedDecimal::new(4, 0, false)"), "should init INDEXED BY as PackedDecimal: {output}");
    }

    #[test]
    fn indexed_by_multiple_indices() {
        let mut entry = make_picx_entry("WS-ITEM", 5, 10);
        entry.occurs = Some(10);
        entry.index_names = vec!["WS-IX1".to_string(), "WS-IX2".to_string()];

        let parent = DataEntry {
            children: vec![entry],
            ..make_entry("WS-TABLE", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[parent], &[], false);
        let output = w.finish();

        assert!(output.contains("pub ws_ix1: PackedDecimal"), "should generate first index: {output}");
        assert!(output.contains("pub ws_ix2: PackedDecimal"), "should generate second index: {output}");
    }

    // -----------------------------------------------------------------------
    // Deep nesting: multi-level group hierarchy
    // -----------------------------------------------------------------------

    #[test]
    fn deep_nested_groups_flatten_correctly() {
        // 01 WS-REC
        //   05 WS-OUTER (group)
        //     10 WS-INNER (group)
        //       15 WS-LEAF PIC X(5)
        let leaf = make_picx_entry("WS-LEAF", 15, 5);
        let inner = DataEntry {
            children: vec![leaf],
            ..make_entry("WS-INNER", 10)
        };
        let outer = DataEntry {
            children: vec![inner],
            ..make_entry("WS-OUTER", 5)
        };
        let record = DataEntry {
            children: vec![outer],
            ..make_entry("WS-REC", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record], &[], false);
        let output = w.finish();

        assert!(output.contains("ws_rec:"), "should have group overlay: {output}");
        assert!(output.contains("ws_outer:"), "should have sub-group overlay: {output}");
        assert!(output.contains("ws_inner:"), "should have sub-sub-group overlay: {output}");
        assert!(output.contains("ws_leaf:"), "should have leaf field: {output}");
    }

    // -----------------------------------------------------------------------
    // Duplicate field names: disambiguation with parent prefix
    // -----------------------------------------------------------------------

    #[test]
    fn duplicate_names_disambiguated() {
        // Two groups with same child name "WS-VALUE"
        let group_a = DataEntry {
            children: vec![make_picx_entry("WS-VALUE", 10, 5)],
            ..make_entry("WS-GROUP-A", 5)
        };
        let group_b = DataEntry {
            children: vec![make_numeric_entry("WS-VALUE", 10, 5, 0)],
            ..make_entry("WS-GROUP-B", 5)
        };
        let record = DataEntry {
            children: vec![group_a, group_b],
            ..make_entry("WS-REC", 1)
        };

        let mut w = RustWriter::new();
        generate_working_storage(&mut w, &[record], &[], false);
        let output = w.finish();

        // Both should be in the struct with parent-prefixed names
        assert!(output.contains("ws_group_a_ws_value"), "should disambiguate with parent A: {output}");
        assert!(output.contains("ws_group_b_ws_value"), "should disambiguate with parent B: {output}");
    }
}
