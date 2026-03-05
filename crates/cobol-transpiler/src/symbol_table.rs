//! Symbol table for resolved data references.
//!
//! Built from the parsed DATA DIVISION hierarchy. Provides name resolution
//! with IN/OF qualification, type resolution (DataEntry -> Rust type info),
//! and field lookup by qualified name.

use std::collections::HashMap;

use crate::ast::{DataEntry, PicCategory, Usage};
use crate::error::{Result, TranspileError};

/// Resolved type information for a data item.
#[derive(Debug, Clone)]
pub struct ResolvedType {
    /// Rust type to generate (e.g., "PackedDecimal", "PicX", "CompBinary").
    pub rust_type: RustType,
    /// Storage size in bytes.
    pub byte_length: usize,
    /// Whether the field is a group (struct) vs elementary.
    pub is_group: bool,
}

/// Rust type mapping for COBOL data items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    /// PackedDecimal { precision, scale, signed }
    PackedDecimal {
        precision: u32,
        scale: u32,
        signed: bool,
    },
    /// PicX { length }
    PicX { length: u32 },
    /// PicA { length }
    PicA { length: u32 },
    /// CompBinary { precision, scale, signed, pic_limited }
    CompBinary {
        precision: u32,
        scale: u32,
        signed: bool,
        pic_limited: bool,
    },
    /// Display numeric (zoned decimal) -- no USAGE clause
    DisplayNumeric {
        precision: u32,
        scale: u32,
        signed: bool,
    },
    /// COMP-1 (f32)
    Float32,
    /// COMP-2 (f64)
    Float64,
    /// Index (usize)
    Index,
    /// Pointer
    Pointer,
    /// Group item (struct containing children)
    Group,
}

/// Symbol table entry for a data item.
#[derive(Debug, Clone)]
pub struct SymbolEntry {
    /// Fully qualified name path (e.g., ["WS-RECORD", "WS-GROUP", "WS-FIELD"]).
    pub name_path: Vec<String>,
    /// Level number.
    pub level: u8,
    /// Resolved type information.
    pub resolved_type: ResolvedType,
    /// Parent entry name (None for top-level).
    pub parent: Option<String>,
    /// OCCURS count (for array items).
    pub occurs: Option<u32>,
}

/// Symbol table built from DATA DIVISION entries.
///
/// Supports:
/// - Simple name lookup: `resolve("WS-FIELD")`
/// - Qualified name lookup: `resolve_qualified("WS-FIELD", &["WS-RECORD"])`
/// - Type resolution: `resolve_type(entry)` -> RustType
#[derive(Debug, Default)]
pub struct SymbolTable {
    /// Map from data name -> list of matching entries (multiple for ambiguous names).
    entries: HashMap<String, Vec<SymbolEntry>>,
    /// All entries in order (for iteration).
    all_entries: Vec<SymbolEntry>,
}

impl SymbolTable {
    /// Build a symbol table from parsed DATA DIVISION records.
    pub fn from_entries(records: &[DataEntry]) -> Self {
        let mut table = Self::default();
        for record in records {
            table.add_entry(record, &[]);
        }
        table
    }

    /// Add a data entry and its children recursively.
    fn add_entry(&mut self, entry: &DataEntry, parent_path: &[String]) {
        let mut name_path = parent_path.to_vec();
        name_path.push(entry.name.clone());

        let resolved_type = resolve_type(entry);
        let parent = if parent_path.is_empty() {
            None
        } else {
            parent_path.last().cloned()
        };

        let symbol = SymbolEntry {
            name_path: name_path.clone(),
            level: entry.level,
            resolved_type,
            parent,
            occurs: entry.occurs,
        };

        self.entries
            .entry(entry.name.clone())
            .or_default()
            .push(symbol.clone());
        self.all_entries.push(symbol);

        // Recurse into children (skip 88-level conditions)
        for child in &entry.children {
            if child.level != 88 {
                self.add_entry(child, &name_path);
            }
        }
    }

    /// Resolve a simple (unqualified) data name reference.
    ///
    /// Returns an error if the name is ambiguous (multiple matches)
    /// or not found.
    pub fn resolve(&self, name: &str) -> Result<&SymbolEntry> {
        let upper = name.to_uppercase();
        match self.entries.get(&upper) {
            None => Err(TranspileError::UnresolvedReference {
                name: name.to_string(),
            }),
            Some(entries) if entries.len() == 1 => Ok(&entries[0]),
            Some(_) => Err(TranspileError::Semantic {
                message: format!(
                    "ambiguous reference '{name}' -- use IN/OF qualification to disambiguate"
                ),
            }),
        }
    }

    /// Resolve a qualified data name reference (e.g., FIELD IN GROUP).
    ///
    /// Qualifiers are checked from innermost to outermost.
    pub fn resolve_qualified(&self, name: &str, qualifiers: &[String]) -> Result<&SymbolEntry> {
        let upper = name.to_uppercase();
        let entries = self.entries.get(&upper).ok_or(TranspileError::UnresolvedReference {
            name: name.to_string(),
        })?;

        if entries.len() == 1 {
            return Ok(&entries[0]);
        }

        // Filter by qualifiers
        let upper_quals: Vec<String> = qualifiers.iter().map(|q| q.to_uppercase()).collect();
        let matches: Vec<&SymbolEntry> = entries
            .iter()
            .filter(|e| {
                // Check that all qualifiers appear in the name path (in order)
                let mut path_idx = 0;
                for qual in &upper_quals {
                    if let Some(pos) = e.name_path[path_idx..].iter().position(|p| p == qual) {
                        path_idx += pos + 1;
                    } else {
                        return false;
                    }
                }
                true
            })
            .collect();

        match matches.len() {
            0 => Err(TranspileError::UnresolvedReference {
                name: format!(
                    "{name} IN {}",
                    qualifiers.join(" IN ")
                ),
            }),
            1 => Ok(matches[0]),
            _ => Err(TranspileError::Semantic {
                message: format!(
                    "ambiguous qualified reference '{name} IN {}' -- {} matches found",
                    qualifiers.join(" IN "),
                    matches.len()
                ),
            }),
        }
    }

    /// Look up all entries with a given name (for MOVE CORRESPONDING matching).
    pub fn find_all(&self, name: &str) -> &[SymbolEntry] {
        let upper = name.to_uppercase();
        self.entries.get(&upper).map_or(&[], |v| v.as_slice())
    }

    /// Get all entries in the symbol table.
    pub fn all(&self) -> &[SymbolEntry] {
        &self.all_entries
    }
}

/// Resolve the Rust type for a data entry.
pub fn resolve_type(entry: &DataEntry) -> ResolvedType {
    let byte_length = entry.byte_length.unwrap_or(0);

    // Group item (has children, no PIC)
    if !entry.children.is_empty() && entry.pic.is_none() {
        return ResolvedType {
            rust_type: RustType::Group,
            byte_length,
            is_group: true,
        };
    }

    // Elementary item with PIC
    if let Some(pic) = &entry.pic {
        let rust_type = match entry.usage {
            Usage::Comp3 => RustType::PackedDecimal {
                precision: pic.total_digits,
                scale: pic.scale,
                signed: pic.signed,
            },
            Usage::Comp | Usage::Comp5 => RustType::CompBinary {
                precision: pic.total_digits,
                scale: pic.scale,
                signed: pic.signed,
                pic_limited: entry.usage == Usage::Comp,
            },
            Usage::Comp1 => RustType::Float32,
            Usage::Comp2 => RustType::Float64,
            Usage::Index => RustType::Index,
            Usage::Pointer => RustType::Pointer,
            Usage::Display => match pic.category {
                PicCategory::Alphabetic => RustType::PicA {
                    length: pic.display_length,
                },
                PicCategory::Alphanumeric | PicCategory::AlphanumericEdited => RustType::PicX {
                    length: pic.display_length,
                },
                PicCategory::Numeric | PicCategory::NumericEdited => RustType::DisplayNumeric {
                    precision: pic.total_digits,
                    scale: pic.scale,
                    signed: pic.signed,
                },
            },
        };

        return ResolvedType {
            rust_type,
            byte_length,
            is_group: false,
        };
    }

    // Elementary item without PIC (e.g., USAGE INDEX, USAGE POINTER)
    let rust_type = match entry.usage {
        Usage::Comp1 => RustType::Float32,
        Usage::Comp2 => RustType::Float64,
        Usage::Index => RustType::Index,
        Usage::Pointer => RustType::Pointer,
        _ => RustType::Group, // Fallback for items without PIC
    };

    ResolvedType {
        rust_type,
        byte_length,
        is_group: entry.pic.is_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::pic_parser::parse_pic;

    fn make_entry(level: u8, name: &str, pic_str: Option<&str>, usage: Usage) -> DataEntry {
        let pic = pic_str.and_then(|s| parse_pic(s).ok());
        let byte_length = pic
            .as_ref()
            .map(|p| crate::parser::pic_parser::compute_storage_size(p, usage) as usize);
        DataEntry {
            level,
            name: name.to_string(),
            pic,
            usage,
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
            byte_length,
        }
    }

    #[test]
    fn resolve_simple_name() {
        let mut record = make_entry(1, "WS-RECORD", None, Usage::Display);
        record.children.push(make_entry(5, "WS-NAME", Some("X(20)"), Usage::Display));
        record.children.push(make_entry(5, "WS-AGE", Some("9(3)"), Usage::Display));

        let table = SymbolTable::from_entries(&[record]);

        let name_entry = table.resolve("WS-NAME").unwrap();
        assert_eq!(name_entry.name_path, vec!["WS-RECORD", "WS-NAME"]);

        let age_entry = table.resolve("WS-AGE").unwrap();
        assert_eq!(age_entry.level, 5);
    }

    #[test]
    fn resolve_unqualified_ambiguous() {
        let mut record_a = make_entry(1, "RECORD-A", None, Usage::Display);
        record_a.children.push(make_entry(5, "FIELD-X", Some("X(10)"), Usage::Display));

        let mut record_b = make_entry(1, "RECORD-B", None, Usage::Display);
        record_b.children.push(make_entry(5, "FIELD-X", Some("X(20)"), Usage::Display));

        let table = SymbolTable::from_entries(&[record_a, record_b]);

        // Unqualified should fail (ambiguous)
        assert!(table.resolve("FIELD-X").is_err());
    }

    #[test]
    fn resolve_qualified_disambiguation() {
        let mut record_a = make_entry(1, "RECORD-A", None, Usage::Display);
        record_a.children.push(make_entry(5, "FIELD-X", Some("X(10)"), Usage::Display));

        let mut record_b = make_entry(1, "RECORD-B", None, Usage::Display);
        record_b.children.push(make_entry(5, "FIELD-X", Some("X(20)"), Usage::Display));

        let table = SymbolTable::from_entries(&[record_a, record_b]);

        let entry = table
            .resolve_qualified("FIELD-X", &["RECORD-A".to_string()])
            .unwrap();
        assert_eq!(entry.name_path, vec!["RECORD-A", "FIELD-X"]);
    }

    #[test]
    fn resolve_type_packed_decimal() {
        let entry = make_entry(5, "WS-AMOUNT", Some("S9(7)V99"), Usage::Comp3);
        let resolved = resolve_type(&entry);
        assert_eq!(
            resolved.rust_type,
            RustType::PackedDecimal {
                precision: 9,
                scale: 2,
                signed: true
            }
        );
    }

    #[test]
    fn resolve_type_pic_x() {
        let entry = make_entry(5, "WS-NAME", Some("X(20)"), Usage::Display);
        let resolved = resolve_type(&entry);
        assert_eq!(resolved.rust_type, RustType::PicX { length: 20 });
    }

    #[test]
    fn resolve_type_comp_binary() {
        let entry = make_entry(5, "WS-COUNT", Some("S9(9)"), Usage::Comp);
        let resolved = resolve_type(&entry);
        assert_eq!(
            resolved.rust_type,
            RustType::CompBinary {
                precision: 9,
                scale: 0,
                signed: true,
                pic_limited: true
            }
        );
    }

    #[test]
    fn resolve_type_group() {
        let mut group = make_entry(1, "WS-RECORD", None, Usage::Display);
        group.children.push(make_entry(5, "FIELD", Some("X(5)"), Usage::Display));
        let resolved = resolve_type(&group);
        assert_eq!(resolved.rust_type, RustType::Group);
        assert!(resolved.is_group);
    }

    #[test]
    fn not_found_error() {
        let table = SymbolTable::default();
        assert!(table.resolve("NONEXISTENT").is_err());
    }
}
