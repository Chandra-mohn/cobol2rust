//! ANTLR4 listener for parsing ENVIRONMENT DIVISION FILE-CONTROL SELECT
//! entries and DATA DIVISION FILE SECTION FD/SD entries.
//!
//! Produces `FileDescription` AST nodes by combining SELECT clause metadata
//! with FD entries. Record definitions under FD are already captured by
//! `DataDivisionListener` as working-storage items, so this listener only
//! needs to capture file-level metadata (organization, access mode, keys,
//! assign-to, etc.) plus the FD-to-file-name associations.

#![allow(clippy::wildcard_imports)] // Generated ANTLR4 code has enormous trait lists

use std::collections::HashMap;

use crate::ast::{
    AccessMode, FileDescription, FileDescriptorType, FileOrganization,
};
use crate::generated::cobol85listener::Cobol85Listener;
#[allow(clippy::wildcard_imports)]
use crate::generated::cobol85parser::*;

use antlr_rust::tree::{ParseTree, ParseTreeListener};

/// Metadata collected from a SELECT clause in FILE-CONTROL.
#[derive(Debug, Default, Clone)]
struct SelectInfo {
    file_name: String,
    assign_to: Option<String>,
    organization: FileOrganization,
    access_mode: AccessMode,
    record_key: Option<String>,
    relative_key: Option<String>,
    file_status: Option<String>,
}

/// FD entry metadata (file name, descriptor type, and record names).
#[derive(Debug, Clone)]
struct FdEntry {
    descriptor_type: FileDescriptorType,
    file_name: String,
    /// 01-level record names under this FD (for building RecordFileMap).
    record_names: Vec<String>,
}

/// Listener that walks the ANTLR4 parse tree to collect FILE-CONTROL
/// SELECT entries and FILE SECTION FD/SD entry names.
#[derive(Debug)]
pub(crate) struct FileListener {
    /// SELECT clause info, keyed by uppercase file name.
    selects: HashMap<String, SelectInfo>,
    /// Current SELECT being built.
    current_select: Option<SelectInfo>,
    /// FD entries from FILE SECTION.
    fds: Vec<FdEntry>,
    /// Whether we're inside a fileDescriptionEntry.
    in_fd: bool,
    /// Index of current FD being built.
    current_fd_idx: Option<usize>,
}

impl FileListener {
    pub fn new() -> Self {
        Self {
            selects: HashMap::new(),
            current_select: None,
            fds: Vec::new(),
            in_fd: false,
            current_fd_idx: None,
        }
    }

    /// Consume the listener and produce `FileDescription` structs.
    ///
    /// Records are empty here because `DataDivisionListener` already collects
    /// them as part of working-storage items.
    pub fn into_file_descriptions(self) -> Vec<FileDescription> {
        let mut result = Vec::new();
        for fd in self.fds {
            let key = fd.file_name.to_uppercase();
            let select = self.selects.get(&key);

            result.push(FileDescription {
                descriptor_type: fd.descriptor_type,
                file_name: fd.file_name,
                assign_to: select.and_then(|s| s.assign_to.clone()),
                organization: select.map_or(FileOrganization::Sequential, |s| s.organization),
                access_mode: select.map_or(AccessMode::Sequential, |s| s.access_mode),
                record_key: select.and_then(|s| s.record_key.clone()),
                alternate_keys: Vec::new(),
                relative_key: select.and_then(|s| s.relative_key.clone()),
                file_status: select.and_then(|s| s.file_status.clone()),
                records: Vec::new(), // Records already in WS via DataDivisionListener
                record_names: fd.record_names,
                recording_mode: None,
                record_size: None,
            });
        }
        result
    }
}

/// Strip surrounding quotes from a string literal.
fn strip_quotes(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

#[allow(clippy::elidable_lifetime_names)] // Required by ANTLR4 trait signatures
impl<'input> ParseTreeListener<'input, Cobol85ParserContextType> for FileListener {}

#[allow(clippy::elidable_lifetime_names)]
impl<'input> Cobol85Listener<'input> for FileListener {
    // -----------------------------------------------------------------------
    // FILE-CONTROL: SELECT clause entries
    // -----------------------------------------------------------------------

    fn enter_fileControlEntry(&mut self, _ctx: &FileControlEntryContext<'input>) {
        self.current_select = Some(SelectInfo::default());
    }

    fn enter_selectClause(&mut self, ctx: &SelectClauseContext<'input>) {
        if let Some(ref mut sel) = self.current_select {
            if let Some(fname) = ctx.fileName() {
                sel.file_name = fname.get_text().trim().to_uppercase();
            }
        }
    }

    fn enter_assignClause(&mut self, ctx: &AssignClauseContext<'input>) {
        if let Some(ref mut sel) = self.current_select {
            if let Some(aname) = ctx.assignmentName() {
                sel.assign_to = Some(aname.get_text().trim().to_string());
            } else if let Some(lit) = ctx.literal() {
                sel.assign_to = Some(strip_quotes(&lit.get_text()));
            }
        }
    }

    fn enter_organizationClause(&mut self, ctx: &OrganizationClauseContext<'input>) {
        if let Some(ref mut sel) = self.current_select {
            let text = ctx.get_text().to_uppercase();
            if text.contains("INDEXED") {
                sel.organization = FileOrganization::Indexed;
            } else if text.contains("RELATIVE") {
                sel.organization = FileOrganization::Relative;
            } else if text.contains("LINE") {
                sel.organization = FileOrganization::LineSequential;
            } else {
                sel.organization = FileOrganization::Sequential;
            }
        }
    }

    fn enter_accessModeClause(&mut self, ctx: &AccessModeClauseContext<'input>) {
        if let Some(ref mut sel) = self.current_select {
            let text = ctx.get_text().to_uppercase();
            if text.contains("DYNAMIC") {
                sel.access_mode = AccessMode::Dynamic;
            } else if text.contains("RANDOM") {
                sel.access_mode = AccessMode::Random;
            } else {
                sel.access_mode = AccessMode::Sequential;
            }
        }
    }

    fn enter_recordKeyClause(&mut self, ctx: &RecordKeyClauseContext<'input>) {
        if let Some(ref mut sel) = self.current_select {
            if let Some(qdn) = ctx.qualifiedDataName() {
                sel.record_key = Some(qdn.get_text().trim().to_uppercase());
            }
        }
    }

    fn enter_relativeKeyClause(&mut self, ctx: &RelativeKeyClauseContext<'input>) {
        if let Some(ref mut sel) = self.current_select {
            if let Some(qdn) = ctx.qualifiedDataName() {
                sel.relative_key = Some(qdn.get_text().trim().to_uppercase());
            }
        }
    }

    fn enter_fileStatusClause(&mut self, ctx: &FileStatusClauseContext<'input>) {
        if let Some(ref mut sel) = self.current_select {
            let qdns = ctx.qualifiedDataName_all();
            if let Some(first) = qdns.first() {
                sel.file_status = Some(first.get_text().trim().to_uppercase());
            }
        }
    }

    fn exit_fileControlEntry(&mut self, _ctx: &FileControlEntryContext<'input>) {
        if let Some(sel) = self.current_select.take() {
            if !sel.file_name.is_empty() {
                let key = sel.file_name.clone();
                self.selects.insert(key, sel);
            }
        }
    }

    // -----------------------------------------------------------------------
    // FILE SECTION: FD/SD entry names and 01-level record names
    // -----------------------------------------------------------------------

    fn enter_fileDescriptionEntry(&mut self, ctx: &FileDescriptionEntryContext<'input>) {
        let desc_type = if ctx.SD().is_some() {
            FileDescriptorType::Sd
        } else {
            FileDescriptorType::Fd
        };

        let file_name = ctx
            .fileName()
            .map_or_else(String::new, |f| f.get_text().trim().to_uppercase());

        if !file_name.is_empty() {
            let idx = self.fds.len();
            self.fds.push(FdEntry {
                descriptor_type: desc_type,
                file_name,
                record_names: Vec::new(),
            });
            self.in_fd = true;
            self.current_fd_idx = Some(idx);
        }
    }

    fn exit_fileDescriptionEntry(&mut self, _ctx: &FileDescriptionEntryContext<'input>) {
        self.in_fd = false;
        self.current_fd_idx = None;
    }

    fn enter_dataDescriptionEntryFormat1(
        &mut self,
        ctx: &DataDescriptionEntryFormat1Context<'input>,
    ) {
        if !self.in_fd {
            return;
        }
        // Only capture 01-level record names
        let level = if let Some(lit) = ctx.INTEGERLITERAL() {
            lit.get_text().trim().parse::<u8>().unwrap_or(0)
        } else {
            return;
        };
        if level != 1 {
            return;
        }
        let name = if let Some(dn) = ctx.dataName() {
            dn.get_text().trim().to_uppercase()
        } else {
            return; // Skip FILLER records
        };
        if let Some(idx) = self.current_fd_idx {
            if let Some(fd) = self.fds.get_mut(idx) {
                fd.record_names.push(name);
            }
        }
    }
}
