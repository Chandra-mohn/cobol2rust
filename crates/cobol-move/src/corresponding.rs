use cobol_core::config::RuntimeConfig;
use cobol_core::traits::CobolGroup;

use crate::engine::cobol_move;

/// MOVE CORRESPONDING: move matching-named fields between two group items.
///
/// For each elementary field in `src_group`, if a field with the same name
/// exists in `dest_group`, perform `MOVE src_field TO dest_field`.
///
/// Name matching is case-insensitive (COBOL names are case-insensitive).
///
/// This uses the `field_names()` method on `CobolGroup` to discover field names
/// from the source, then looks up matching fields in the destination.
pub fn move_corresponding(
    src_group: &dyn CobolGroup,
    dest_group: &mut dyn CobolGroup,
    config: &RuntimeConfig,
) {
    let src_names = src_group.field_names();

    for name in &src_names {
        // Case-insensitive matching: COBOL names are case-insensitive
        let upper_name = name.to_uppercase();
        if let Some(src_field) = src_group.field_by_name(&upper_name) {
            if let Some(dest_field) = dest_group.field_by_name_mut(&upper_name) {
                cobol_move(src_field, dest_field, config);
            }
        }
    }
}

/// Move corresponding by name (explicit name list version).
///
/// Takes an explicit list of field names to match between source and destination.
/// This is the version the transpiler generates, since it knows the field names
/// at compile time.
pub fn move_corresponding_by_name(
    src_group: &dyn CobolGroup,
    dest_group: &mut dyn CobolGroup,
    field_names: &[&str],
    config: &RuntimeConfig,
) {
    for &name in field_names {
        if let Some(src_field) = src_group.field_by_name(name) {
            if let Some(dest_field) = dest_group.field_by_name_mut(name) {
                cobol_move(src_field, dest_field, config);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_core::category::DataCategory;
    use cobol_core::traits::CobolField;
    use cobol_types::PicX;

    /// A test group with named fields for MOVE CORRESPONDING tests.
    #[derive(Debug, Clone)]
    struct TestGroup {
        field_a: PicX,
        field_b: PicX,
        field_c: PicX,
    }

    impl TestGroup {
        fn new(a: &[u8], b: &[u8], c: &[u8]) -> Self {
            Self {
                field_a: PicX::new(10, a),
                field_b: PicX::new(10, b),
                field_c: PicX::new(10, c),
            }
        }
    }

    impl CobolField for TestGroup {
        fn category(&self) -> DataCategory {
            DataCategory::Group
        }
        fn byte_length(&self) -> usize {
            30
        }
        fn as_bytes(&self) -> &[u8] {
            &[]
        }
        fn as_bytes_mut(&mut self) -> &mut [u8] {
            &mut []
        }
        fn display_bytes(&self) -> Vec<u8> {
            let mut v = Vec::with_capacity(30);
            v.extend_from_slice(self.field_a.as_bytes());
            v.extend_from_slice(self.field_b.as_bytes());
            v.extend_from_slice(self.field_c.as_bytes());
            v
        }
        fn fill_bytes(&mut self, byte: u8) {
            self.field_a.fill_bytes(byte);
            self.field_b.fill_bytes(byte);
            self.field_c.fill_bytes(byte);
        }
        fn initialize_default(&mut self) {
            self.field_a.initialize_default();
            self.field_b.initialize_default();
            self.field_c.initialize_default();
        }
    }

    impl CobolGroup for TestGroup {
        fn elementary_fields(&self) -> Vec<&dyn CobolField> {
            vec![&self.field_a, &self.field_b, &self.field_c]
        }
        fn elementary_fields_mut(&mut self) -> Vec<&mut dyn CobolField> {
            vec![&mut self.field_a, &mut self.field_b, &mut self.field_c]
        }
        fn field_by_name(&self, name: &str) -> Option<&dyn CobolField> {
            match name.to_uppercase().as_str() {
                "FIELD-A" => Some(&self.field_a),
                "FIELD-B" => Some(&self.field_b),
                "FIELD-C" => Some(&self.field_c),
                _ => None,
            }
        }
        fn field_by_name_mut(&mut self, name: &str) -> Option<&mut dyn CobolField> {
            match name.to_uppercase().as_str() {
                "FIELD-A" => Some(&mut self.field_a),
                "FIELD-B" => Some(&mut self.field_b),
                "FIELD-C" => Some(&mut self.field_c),
                _ => None,
            }
        }
        fn field_names(&self) -> Vec<String> {
            vec![
                "FIELD-A".to_string(),
                "FIELD-B".to_string(),
                "FIELD-C".to_string(),
            ]
        }
    }

    /// A second group with partially overlapping field names.
    #[derive(Debug, Clone)]
    struct TestGroup2 {
        field_a: PicX,
        field_d: PicX,
    }

    impl TestGroup2 {
        fn new(a: &[u8], d: &[u8]) -> Self {
            Self {
                field_a: PicX::new(10, a),
                field_d: PicX::new(10, d),
            }
        }
    }

    impl CobolField for TestGroup2 {
        fn category(&self) -> DataCategory { DataCategory::Group }
        fn byte_length(&self) -> usize { 20 }
        fn as_bytes(&self) -> &[u8] { &[] }
        fn as_bytes_mut(&mut self) -> &mut [u8] { &mut [] }
        fn display_bytes(&self) -> Vec<u8> {
            let mut v = Vec::with_capacity(20);
            v.extend_from_slice(self.field_a.as_bytes());
            v.extend_from_slice(self.field_d.as_bytes());
            v
        }
        fn fill_bytes(&mut self, byte: u8) {
            self.field_a.fill_bytes(byte);
            self.field_d.fill_bytes(byte);
        }
        fn initialize_default(&mut self) {
            self.field_a.initialize_default();
            self.field_d.initialize_default();
        }
    }

    impl CobolGroup for TestGroup2 {
        fn elementary_fields(&self) -> Vec<&dyn CobolField> {
            vec![&self.field_a, &self.field_d]
        }
        fn elementary_fields_mut(&mut self) -> Vec<&mut dyn CobolField> {
            vec![&mut self.field_a, &mut self.field_d]
        }
        fn field_by_name(&self, name: &str) -> Option<&dyn CobolField> {
            match name.to_uppercase().as_str() {
                "FIELD-A" => Some(&self.field_a),
                "FIELD-D" => Some(&self.field_d),
                _ => None,
            }
        }
        fn field_by_name_mut(&mut self, name: &str) -> Option<&mut dyn CobolField> {
            match name.to_uppercase().as_str() {
                "FIELD-A" => Some(&mut self.field_a),
                "FIELD-D" => Some(&mut self.field_d),
                _ => None,
            }
        }
        fn field_names(&self) -> Vec<String> {
            vec!["FIELD-A".to_string(), "FIELD-D".to_string()]
        }
    }

    #[test]
    fn move_corr_all_matching() {
        let src = TestGroup::new(b"HELLO", b"WORLD", b"TEST");
        let mut dest = TestGroup::new(b"", b"", b"");
        let config = RuntimeConfig::default();

        move_corresponding(&src, &mut dest, &config);

        assert_eq!(dest.field_a.as_bytes(), b"HELLO     ");
        assert_eq!(dest.field_b.as_bytes(), b"WORLD     ");
        assert_eq!(dest.field_c.as_bytes(), b"TEST      ");
    }

    #[test]
    fn move_corr_partial_overlap() {
        // Source has FIELD-A, FIELD-B, FIELD-C
        // Dest has FIELD-A, FIELD-D
        // Only FIELD-A should be moved
        let src = TestGroup::new(b"HELLO", b"WORLD", b"TEST");
        let mut dest = TestGroup2::new(b"", b"ORIGINAL");
        let config = RuntimeConfig::default();

        move_corresponding(&src, &mut dest, &config);

        assert_eq!(dest.field_a.as_bytes(), b"HELLO     ");
        // FIELD-D should be untouched (no match in source)
        assert_eq!(dest.field_d.as_bytes(), b"ORIGINAL  ");
    }

    #[test]
    fn move_corr_by_name_explicit() {
        let src = TestGroup::new(b"HELLO", b"WORLD", b"TEST");
        let mut dest = TestGroup::new(b"", b"", b"");
        let config = RuntimeConfig::default();

        // Only move FIELD-A and FIELD-C, skip FIELD-B
        move_corresponding_by_name(&src, &mut dest, &["FIELD-A", "FIELD-C"], &config);

        assert_eq!(dest.field_a.as_bytes(), b"HELLO     ");
        assert_eq!(dest.field_b.as_bytes(), b"          "); // Unchanged (spaces from new)
        assert_eq!(dest.field_c.as_bytes(), b"TEST      ");
    }

    #[test]
    fn move_corr_no_matches() {
        let src = TestGroup2::new(b"HELLO", b"WORLD");
        let mut dest = TestGroup::new(b"ORIG-A", b"ORIG-B", b"ORIG-C");
        let config = RuntimeConfig::default();

        // Source has FIELD-A and FIELD-D; dest has FIELD-A, FIELD-B, FIELD-C
        // Only FIELD-A matches
        move_corresponding(&src, &mut dest, &config);

        assert_eq!(dest.field_a.as_bytes(), b"HELLO     ");
        assert_eq!(dest.field_b.as_bytes(), b"ORIG-B    "); // No match
        assert_eq!(dest.field_c.as_bytes(), b"ORIG-C    "); // No match
    }
}
