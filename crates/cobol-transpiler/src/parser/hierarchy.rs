//! Hierarchy builder for COBOL data items.
//!
//! Converts the flat list of data entries (as collected by the listener)
//! into a nested tree based on COBOL level numbering rules.
//!
//! ## Level numbering rules
//!
//! - Level 01 or 77: top-level record or standalone item
//! - Levels 02-49: nested within the nearest ancestor with a lower level number
//! - Level 66: RENAMES (handled separately)
//! - Level 88: condition name, always child of the immediately preceding item

use crate::ast::DataEntry;

/// Build a hierarchy tree from flat data entries.
///
/// The input should be a flat list of `DataEntry` items in the order they
/// appeared in the DATA DIVISION. Each entry has a `level` number but
/// empty `children` (the listener collects them flat).
///
/// Returns a list of top-level records (level 01 and 77 items), each with
/// their children nested recursively.
pub fn build_hierarchy(flat_items: Vec<DataEntry>) -> Vec<DataEntry> {
    if flat_items.is_empty() {
        return Vec::new();
    }

    let mut records: Vec<DataEntry> = Vec::new();
    let mut stack: Vec<DataEntry> = Vec::new();

    for item in flat_items {
        match item.level {
            // Level 01 or 77: start a new record
            1 | 77 => {
                // Flush current stack into a record
                flush_stack(&mut stack, &mut records);
                stack.push(item);
            }
            // Level 88: condition name -- attach as child of stack top
            88 => {
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(item);
                } else {
                    // Orphan 88 level -- just push it
                    stack.push(item);
                }
            }
            // Level 66: RENAMES -- attach as child of current record
            66 => {
                if stack.len() > 1 {
                    // Pop back to level 01 and add as child
                    while stack.len() > 1 {
                        let child = stack.pop().unwrap();
                        stack.last_mut().unwrap().children.push(child);
                    }
                }
                if let Some(record) = stack.last_mut() {
                    record.children.push(item);
                }
            }
            // Levels 02-49: nested items
            level @ 2..=49 => {
                // Pop stack until we find an ancestor with a lower level number
                while stack.len() > 1 {
                    let top_level = stack.last().unwrap().level;
                    if top_level >= level {
                        let child = stack.pop().unwrap();
                        stack.last_mut().unwrap().children.push(child);
                    } else {
                        break;
                    }
                }
                stack.push(item);
            }
            // Unknown levels: treat as level 01
            _ => {
                flush_stack(&mut stack, &mut records);
                stack.push(item);
            }
        }
    }

    // Flush remaining stack
    flush_stack(&mut stack, &mut records);

    records
}

/// Flush the stack into a single record and push to records list.
fn flush_stack(stack: &mut Vec<DataEntry>, records: &mut Vec<DataEntry>) {
    if stack.is_empty() {
        return;
    }

    // Pop everything back to the root (level 01/77)
    while stack.len() > 1 {
        let child = stack.pop().unwrap();
        stack.last_mut().unwrap().children.push(child);
    }

    if let Some(record) = stack.pop() {
        records.push(record);
    }
}

/// Compute byte offsets and lengths for all items in a record tree.
///
/// Group items get their length from the sum of their non-REDEFINES children.
/// Elementary items get their length from PIC + USAGE.
pub fn compute_layout(record: &mut DataEntry) {
    compute_layout_recursive(record, 0);
}

/// Recursive layout computation. Returns the byte length of this item.
fn compute_layout_recursive(entry: &mut DataEntry, offset: usize) -> usize {
    entry.byte_offset = Some(offset);

    if entry.children.is_empty() {
        // Elementary item -- byte_length should already be set from PIC
        let len = entry.byte_length.unwrap_or(0);
        return len;
    }

    // Group item -- compute children
    let mut current_offset = offset;
    let mut max_redefines_len: usize = 0;
    let mut group_len: usize = 0;

    for child in &mut entry.children {
        // 88-level conditions don't occupy storage
        if child.level == 88 || child.level == 66 {
            continue;
        }

        if child.redefines.is_some() {
            // REDEFINES: overlays the same storage as the base field
            // Use the base field's offset
            let redef_len = compute_layout_recursive(child, current_offset - group_len);
            if redef_len > max_redefines_len {
                max_redefines_len = redef_len;
            }
        } else {
            let child_len = compute_layout_recursive(child, current_offset);
            current_offset += child_len;
            group_len += child_len;
        }
    }

    // Group length is the max of accumulated children or the largest REDEFINES
    let total = group_len.max(max_redefines_len);
    entry.byte_length = Some(total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Usage;

    fn make_entry(level: u8, name: &str, byte_length: Option<usize>) -> DataEntry {
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
            byte_length,
        }
    }

    #[test]
    fn empty_input() {
        assert!(build_hierarchy(Vec::new()).is_empty());
    }

    #[test]
    fn single_record_flat() {
        let items = vec![
            make_entry(1, "WS-RECORD", None),
            make_entry(5, "WS-FIELD-A", Some(10)),
            make_entry(5, "WS-FIELD-B", Some(5)),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "WS-RECORD");
        assert_eq!(records[0].children.len(), 2);
        assert_eq!(records[0].children[0].name, "WS-FIELD-A");
        assert_eq!(records[0].children[1].name, "WS-FIELD-B");
    }

    #[test]
    fn nested_groups() {
        let items = vec![
            make_entry(1, "WS-RECORD", None),
            make_entry(5, "WS-GROUP", None),
            make_entry(10, "WS-CHILD-A", Some(5)),
            make_entry(10, "WS-CHILD-B", Some(3)),
            make_entry(5, "WS-FIELD-C", Some(10)),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].children.len(), 2); // WS-GROUP and WS-FIELD-C
        assert_eq!(records[0].children[0].name, "WS-GROUP");
        assert_eq!(records[0].children[0].children.len(), 2);
    }

    #[test]
    fn level_88_conditions() {
        let items = vec![
            make_entry(1, "WS-RECORD", None),
            make_entry(5, "WS-STATUS", Some(1)),
            make_entry(88, "STATUS-ACTIVE", None),
            make_entry(88, "STATUS-INACTIVE", None),
            make_entry(5, "WS-NAME", Some(20)),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records[0].children.len(), 2); // WS-STATUS and WS-NAME
        assert_eq!(records[0].children[0].children.len(), 2); // two 88-levels
    }

    #[test]
    fn multiple_records() {
        let items = vec![
            make_entry(1, "RECORD-A", None),
            make_entry(5, "FIELD-A", Some(10)),
            make_entry(1, "RECORD-B", None),
            make_entry(5, "FIELD-B", Some(20)),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].name, "RECORD-A");
        assert_eq!(records[1].name, "RECORD-B");
    }

    #[test]
    fn level_77_standalone() {
        let items = vec![
            make_entry(77, "WS-COUNTER", Some(4)),
            make_entry(1, "WS-RECORD", None),
            make_entry(5, "WS-FIELD", Some(10)),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].name, "WS-COUNTER");
        assert_eq!(records[1].name, "WS-RECORD");
    }

    #[test]
    fn compute_layout_simple() {
        let mut record = make_entry(1, "WS-RECORD", None);
        record.children.push(make_entry(5, "FIELD-A", Some(10)));
        record.children.push(make_entry(5, "FIELD-B", Some(5)));

        compute_layout(&mut record);
        assert_eq!(record.byte_offset, Some(0));
        assert_eq!(record.byte_length, Some(15));
        assert_eq!(record.children[0].byte_offset, Some(0));
        assert_eq!(record.children[1].byte_offset, Some(10));
    }

    #[test]
    fn compute_layout_nested() {
        let mut record = make_entry(1, "WS-RECORD", None);
        let mut group = make_entry(5, "WS-GROUP", None);
        group.children.push(make_entry(10, "CHILD-A", Some(5)));
        group.children.push(make_entry(10, "CHILD-B", Some(3)));
        record.children.push(group);
        record.children.push(make_entry(5, "FIELD-C", Some(10)));

        compute_layout(&mut record);
        assert_eq!(record.byte_length, Some(18)); // 8 (group) + 10
        assert_eq!(record.children[0].byte_length, Some(8));
        assert_eq!(record.children[0].children[0].byte_offset, Some(0));
        assert_eq!(record.children[0].children[1].byte_offset, Some(5));
    }

    // --- Edge case tests ---

    #[test]
    fn orphan_88_level_without_parent() {
        // 88-level with no parent on stack -- should not crash
        let items = vec![
            make_entry(88, "STATUS-ACTIVE", None),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "STATUS-ACTIVE");
    }

    #[test]
    fn level_66_renames_attaches_to_record() {
        let items = vec![
            make_entry(1, "WS-RECORD", None),
            make_entry(5, "WS-FIELD-A", Some(10)),
            make_entry(5, "WS-FIELD-B", Some(5)),
            make_entry(66, "WS-RENAME", None),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records.len(), 1);
        // Level 66 should be a child of the 01-level record
        let record = &records[0];
        let has_rename = record.children.iter().any(|c| c.name == "WS-RENAME" && c.level == 66);
        assert!(has_rename, "level 66 should be attached to the record");
    }

    #[test]
    fn deeply_nested_5_levels() {
        let items = vec![
            make_entry(1, "RECORD", None),
            make_entry(5, "GROUP-A", None),
            make_entry(10, "GROUP-B", None),
            make_entry(15, "GROUP-C", None),
            make_entry(20, "LEAF", Some(4)),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].children.len(), 1); // GROUP-A
        assert_eq!(records[0].children[0].children.len(), 1); // GROUP-B
        assert_eq!(records[0].children[0].children[0].children.len(), 1); // GROUP-C
        assert_eq!(records[0].children[0].children[0].children[0].children.len(), 1); // LEAF
        assert_eq!(records[0].children[0].children[0].children[0].children[0].name, "LEAF");
    }

    #[test]
    fn sibling_then_deeper_child() {
        // Test that after a sibling at level 5, a new child at level 10 attaches correctly
        let items = vec![
            make_entry(1, "RECORD", None),
            make_entry(5, "GROUP-A", None),
            make_entry(10, "CHILD-1", Some(5)),
            make_entry(5, "GROUP-B", None),
            make_entry(10, "CHILD-2", Some(3)),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records[0].children.len(), 2);
        assert_eq!(records[0].children[0].name, "GROUP-A");
        assert_eq!(records[0].children[0].children.len(), 1);
        assert_eq!(records[0].children[1].name, "GROUP-B");
        assert_eq!(records[0].children[1].children.len(), 1);
    }

    #[test]
    fn unknown_level_treated_as_01() {
        let items = vec![
            make_entry(1, "RECORD-A", None),
            make_entry(5, "FIELD-A", Some(10)),
            make_entry(99, "WEIRD-RECORD", None),
            make_entry(5, "FIELD-B", Some(5)),
        ];
        let records = build_hierarchy(items);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].name, "RECORD-A");
        assert_eq!(records[1].name, "WEIRD-RECORD");
    }

    #[test]
    fn multiple_88_levels_on_different_parents() {
        let items = vec![
            make_entry(1, "RECORD", None),
            make_entry(5, "STATUS", Some(1)),
            make_entry(88, "ACTIVE", None),
            make_entry(88, "INACTIVE", None),
            make_entry(5, "TYPE", Some(1)),
            make_entry(88, "TYPE-A", None),
            make_entry(88, "TYPE-B", None),
        ];
        let records = build_hierarchy(items);
        let record = &records[0];
        assert_eq!(record.children.len(), 2); // STATUS and TYPE
        assert_eq!(record.children[0].children.len(), 2); // ACTIVE, INACTIVE
        assert_eq!(record.children[1].children.len(), 2); // TYPE-A, TYPE-B
    }

    #[test]
    fn compute_layout_with_redefines() {
        let mut record = make_entry(1, "WS-RECORD", None);
        record.children.push(make_entry(5, "FIELD-A", Some(10)));

        let mut redef = make_entry(5, "FIELD-A-NUM", Some(10));
        redef.redefines = Some("FIELD-A".to_string());
        record.children.push(redef);

        record.children.push(make_entry(5, "FIELD-B", Some(5)));

        compute_layout(&mut record);
        // Total: max(10 + 5, 10 redefines) = 15
        assert_eq!(record.byte_length, Some(15));
    }
}
