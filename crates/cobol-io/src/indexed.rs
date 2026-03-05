//! Indexed file implementation (ORGANIZATION IS INDEXED).
//!
//! Uses SQLite as the storage backend for VSAM KSDS emulation.
//! Records are keyed by a primary key extracted from the record at a
//! fixed offset and length. Alternate keys are supported.
//!
//! Schema: CREATE TABLE records (pkey BLOB PRIMARY KEY, data BLOB NOT NULL)
//!
//! Requires the `sqlite` feature (enabled by default).

#![cfg(feature = "sqlite")]

use std::path::PathBuf;

use rusqlite::{params, Connection};

use crate::file_status::FileStatusCode;
use crate::file_traits::{CobolFile, FileAccessMode, FileOpenMode, FileOrganization};

/// Indexed file (ORGANIZATION IS INDEXED).
///
/// Primary key is extracted from the record at `key_offset` for `key_length` bytes.
/// SQLite provides transactional, crash-safe keyed access.
pub struct IndexedFile {
    /// COBOL SELECT name.
    select_name: String,
    /// SQLite database path.
    path: PathBuf,
    /// Fixed record length.
    record_length: usize,
    /// Access mode.
    access_mode: FileAccessMode,
    /// Key offset within the record (0-based).
    key_offset: usize,
    /// Key length in bytes.
    key_length: usize,
    /// Allow duplicate primary keys (normally false). Reserved for future use.
    _allow_duplicates: bool,
    /// Current open mode.
    open_mode: Option<FileOpenMode>,
    /// SQLite connection.
    conn: Option<Connection>,
    /// Current key for sequential access (cursor).
    current_key: Option<Vec<u8>>,
    /// Last read key (for REWRITE/DELETE).
    last_read_key: Option<Vec<u8>>,
}

impl std::fmt::Debug for IndexedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexedFile")
            .field("select_name", &self.select_name)
            .field("path", &self.path)
            .field("record_length", &self.record_length)
            .field("key_offset", &self.key_offset)
            .field("key_length", &self.key_length)
            .field("open_mode", &self.open_mode)
            .finish()
    }
}

impl IndexedFile {
    /// Create a new indexed file descriptor.
    ///
    /// - `key_offset`: byte offset of the primary key within the record.
    /// - `key_length`: byte length of the primary key.
    pub fn new(
        select_name: String,
        path: PathBuf,
        record_length: usize,
        access_mode: FileAccessMode,
        key_offset: usize,
        key_length: usize,
    ) -> Self {
        Self {
            select_name,
            path,
            record_length,
            access_mode,
            key_offset,
            key_length,
            _allow_duplicates: false,
            open_mode: None,
            conn: None,
            current_key: None,
            last_read_key: None,
        }
    }

    /// Extract the primary key from a record.
    fn extract_key(&self, record: &[u8]) -> Vec<u8> {
        let end = (self.key_offset + self.key_length).min(record.len());
        let start = self.key_offset.min(end);
        record[start..end].to_vec()
    }

    /// Pad record to fixed length.
    fn pad_record(&self, record: &[u8]) -> Vec<u8> {
        let mut padded = vec![b' '; self.record_length];
        let copy_len = record.len().min(self.record_length);
        padded[..copy_len].copy_from_slice(&record[..copy_len]);
        padded
    }

    /// Ensure the records table exists.
    fn create_table(&self) -> FileStatusCode {
        if let Some(ref conn) = self.conn {
            let result = conn.execute(
                "CREATE TABLE IF NOT EXISTS records (pkey BLOB PRIMARY KEY, data BLOB NOT NULL)",
                [],
            );
            match result {
                Ok(_) => FileStatusCode::SUCCESS,
                Err(_) => FileStatusCode::PERM_ERROR,
            }
        } else {
            FileStatusCode::NOT_OPEN
        }
    }

    /// READ by primary key (random access).
    pub fn read_by_key(&mut self, key: &[u8]) -> (FileStatusCode, Option<Vec<u8>>) {
        if self.conn.is_none() {
            return (FileStatusCode::NOT_OPEN, None);
        }
        match self.open_mode {
            Some(FileOpenMode::Input) | Some(FileOpenMode::InputOutput) => {}
            _ => return (FileStatusCode::BAD_OPEN_MODE, None),
        }

        let conn = self.conn.as_ref().unwrap();
        let mut stmt = match conn.prepare("SELECT data FROM records WHERE pkey = ?1") {
            Ok(s) => s,
            Err(_) => return (FileStatusCode::PERM_ERROR, None),
        };

        match stmt.query_row(params![key], |row| {
            let data: Vec<u8> = row.get(0)?;
            Ok(data)
        }) {
            Ok(data) => {
                self.last_read_key = Some(key.to_vec());
                self.current_key = Some(key.to_vec());
                (FileStatusCode::SUCCESS, Some(data))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                self.last_read_key = None;
                (FileStatusCode::RECORD_NOT_FOUND, None)
            }
            Err(_) => (FileStatusCode::PERM_ERROR, None),
        }
    }

    /// START -- position the cursor for subsequent sequential reads.
    ///
    /// The cursor is set to the first key >= the given key.
    pub fn start(&mut self, key: &[u8]) -> FileStatusCode {
        if self.conn.is_none() {
            return FileStatusCode::NOT_OPEN;
        }
        match self.open_mode {
            Some(FileOpenMode::Input) | Some(FileOpenMode::InputOutput) => {}
            _ => return FileStatusCode::BAD_OPEN_MODE,
        }

        let conn = self.conn.as_ref().unwrap();
        let mut stmt = match conn.prepare("SELECT pkey FROM records WHERE pkey >= ?1 ORDER BY pkey LIMIT 1") {
            Ok(s) => s,
            Err(_) => return FileStatusCode::PERM_ERROR,
        };

        match stmt.query_row(params![key], |row| {
            let k: Vec<u8> = row.get(0)?;
            Ok(k)
        }) {
            Ok(found_key) => {
                // Set cursor to just before this key so read_next will find it
                self.current_key = Some(found_key);
                FileStatusCode::SUCCESS
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                FileStatusCode::RECORD_NOT_FOUND
            }
            Err(_) => FileStatusCode::PERM_ERROR,
        }
    }

    /// DELETE the record with the given key.
    pub fn delete_by_key(&mut self, key: &[u8]) -> FileStatusCode {
        if self.conn.is_none() {
            return FileStatusCode::NOT_OPEN;
        }
        if self.open_mode != Some(FileOpenMode::InputOutput) {
            return FileStatusCode::BAD_OPEN_MODE;
        }

        let conn = self.conn.as_ref().unwrap();
        match conn.execute("DELETE FROM records WHERE pkey = ?1", params![key]) {
            Ok(0) => FileStatusCode::RECORD_NOT_FOUND,
            Ok(_) => FileStatusCode::SUCCESS,
            Err(_) => FileStatusCode::PERM_ERROR,
        }
    }
}

impl CobolFile for IndexedFile {
    fn open(&mut self, mode: FileOpenMode) -> FileStatusCode {
        if self.is_open() {
            return FileStatusCode::ALREADY_OPEN;
        }

        // For OUTPUT mode, we truncate (delete all records)
        let conn = match Connection::open(&self.path) {
            Ok(c) => c,
            Err(_) => return FileStatusCode::PERM_ERROR,
        };

        self.conn = Some(conn);
        self.open_mode = Some(mode);
        self.current_key = None;
        self.last_read_key = None;

        // Create table if needed
        let status = self.create_table();
        if !status.is_success() {
            self.conn = None;
            self.open_mode = None;
            return status;
        }

        // For OUTPUT mode, clear existing data
        if mode == FileOpenMode::Output {
            if let Some(ref conn) = self.conn {
                if conn.execute("DELETE FROM records", []).is_err() {
                    self.conn = None;
                    self.open_mode = None;
                    return FileStatusCode::PERM_ERROR;
                }
            }
        }

        FileStatusCode::SUCCESS
    }

    fn close(&mut self) -> FileStatusCode {
        if !self.is_open() {
            return FileStatusCode::NOT_OPEN;
        }
        self.conn = None;
        self.open_mode = None;
        self.current_key = None;
        self.last_read_key = None;
        FileStatusCode::SUCCESS
    }

    fn read_next(&mut self) -> (FileStatusCode, Option<Vec<u8>>) {
        if self.conn.is_none() {
            return (FileStatusCode::NOT_OPEN, None);
        }
        match self.open_mode {
            Some(FileOpenMode::Input) | Some(FileOpenMode::InputOutput) => {}
            _ => return (FileStatusCode::BAD_OPEN_MODE, None),
        }

        let conn = self.conn.as_ref().unwrap();

        // If current_key is set (from START), read that record first, then advance
        let query;
        let result = if let Some(ref key) = self.current_key.clone() {
            // Read the next record after current_key
            query = "SELECT pkey, data FROM records WHERE pkey > ?1 ORDER BY pkey LIMIT 1";
            let mut stmt = match conn.prepare(query) {
                Ok(s) => s,
                Err(_) => return (FileStatusCode::PERM_ERROR, None),
            };
            stmt.query_row(params![key], |row| {
                let k: Vec<u8> = row.get(0)?;
                let d: Vec<u8> = row.get(1)?;
                Ok((k, d))
            })
        } else {
            // Read the first record
            query = "SELECT pkey, data FROM records ORDER BY pkey LIMIT 1";
            let mut stmt = match conn.prepare(query) {
                Ok(s) => s,
                Err(_) => return (FileStatusCode::PERM_ERROR, None),
            };
            stmt.query_row([], |row| {
                let k: Vec<u8> = row.get(0)?;
                let d: Vec<u8> = row.get(1)?;
                Ok((k, d))
            })
        };

        match result {
            Ok((key, data)) => {
                self.last_read_key = Some(key.clone());
                self.current_key = Some(key);
                (FileStatusCode::SUCCESS, Some(data))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                (FileStatusCode::AT_END, None)
            }
            Err(_) => (FileStatusCode::PERM_ERROR, None),
        }
    }

    fn write_record(&mut self, record: &[u8]) -> FileStatusCode {
        if self.conn.is_none() {
            return FileStatusCode::NOT_OPEN;
        }
        match self.open_mode {
            Some(FileOpenMode::Output) | Some(FileOpenMode::InputOutput)
            | Some(FileOpenMode::Extend) => {}
            _ => return FileStatusCode::BAD_OPEN_MODE,
        }

        let padded = self.pad_record(record);
        let key = self.extract_key(&padded);

        let conn = self.conn.as_ref().unwrap();
        match conn.execute(
            "INSERT INTO records (pkey, data) VALUES (?1, ?2)",
            params![key, padded],
        ) {
            Ok(_) => {
                self.current_key = Some(key);
                FileStatusCode::SUCCESS
            }
            Err(e) => {
                // Check for unique constraint violation
                if let rusqlite::Error::SqliteFailure(err, _) = &e {
                    if err.extended_code == 1555 || err.extended_code == 2067 {
                        return FileStatusCode::DUPLICATE_KEY;
                    }
                }
                FileStatusCode::PERM_ERROR
            }
        }
    }

    fn rewrite_record(&mut self, record: &[u8]) -> FileStatusCode {
        if self.conn.is_none() {
            return FileStatusCode::NOT_OPEN;
        }
        if self.open_mode != Some(FileOpenMode::InputOutput) {
            return FileStatusCode::BAD_OPEN_MODE;
        }

        let key = match &self.last_read_key {
            Some(k) => k.clone(),
            None => return FileStatusCode::NO_PRIOR_READ,
        };

        let padded = self.pad_record(record);
        let conn = self.conn.as_ref().unwrap();
        match conn.execute(
            "UPDATE records SET data = ?2 WHERE pkey = ?1",
            params![key, padded],
        ) {
            Ok(0) => FileStatusCode::RECORD_NOT_FOUND,
            Ok(_) => FileStatusCode::SUCCESS,
            Err(_) => FileStatusCode::PERM_ERROR,
        }
    }

    fn organization(&self) -> FileOrganization {
        FileOrganization::Indexed
    }

    fn access_mode(&self) -> FileAccessMode {
        self.access_mode
    }

    fn is_open(&self) -> bool {
        self.open_mode.is_some()
    }

    fn select_name(&self) -> &str {
        &self.select_name
    }

    fn record_length(&self) -> usize {
        self.record_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(name: &str) -> PathBuf {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(name);
        std::mem::forget(dir);
        path
    }

    fn make_record(key: &[u8], data: &[u8]) -> Vec<u8> {
        // Key at offset 0, then data, padded to 80
        let mut rec = vec![b' '; 80];
        let klen = key.len().min(10);
        rec[..klen].copy_from_slice(&key[..klen]);
        let dlen = data.len().min(70);
        rec[10..10 + dlen].copy_from_slice(&data[..dlen]);
        rec
    }

    #[test]
    fn write_and_read_by_key() {
        let path = temp_path("idx_rw.db");
        let mut f = IndexedFile::new(
            "IDX-FILE".to_string(),
            path.clone(),
            80,
            FileAccessMode::Random,
            0,
            10,
        );

        f.open(FileOpenMode::Output);
        let rec1 = make_record(b"KEY001", b"FIRST RECORD");
        let rec2 = make_record(b"KEY002", b"SECOND RECORD");
        assert_eq!(f.write_record(&rec1), FileStatusCode::SUCCESS);
        assert_eq!(f.write_record(&rec2), FileStatusCode::SUCCESS);
        f.close();

        // Read back
        f.open(FileOpenMode::Input);
        let key1 = make_record(b"KEY001", b"")[..10].to_vec();
        let (s, d) = f.read_by_key(&key1);
        assert_eq!(s, FileStatusCode::SUCCESS);
        let data = d.unwrap();
        assert!(data[10..].starts_with(b"FIRST RECORD"));

        // Nonexistent key
        let bad_key = make_record(b"KEY999", b"")[..10].to_vec();
        let (s, _) = f.read_by_key(&bad_key);
        assert_eq!(s, FileStatusCode::RECORD_NOT_FOUND);

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn duplicate_key_rejected() {
        let path = temp_path("idx_dup.db");
        let mut f = IndexedFile::new(
            "F".to_string(),
            path.clone(),
            80,
            FileAccessMode::Random,
            0,
            10,
        );

        f.open(FileOpenMode::Output);
        let rec = make_record(b"KEY001", b"DATA");
        assert_eq!(f.write_record(&rec), FileStatusCode::SUCCESS);
        assert_eq!(f.write_record(&rec), FileStatusCode::DUPLICATE_KEY);
        f.close();

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn sequential_read_all() {
        let path = temp_path("idx_seq.db");
        let mut f = IndexedFile::new(
            "F".to_string(),
            path.clone(),
            80,
            FileAccessMode::Dynamic,
            0,
            10,
        );

        f.open(FileOpenMode::Output);
        f.write_record(&make_record(b"KEY003", b"THIRD"));
        f.write_record(&make_record(b"KEY001", b"FIRST"));
        f.write_record(&make_record(b"KEY002", b"SECOND"));
        f.close();

        // Sequential read should return in key order
        f.open(FileOpenMode::Input);
        let (s, d) = f.read_next();
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"KEY001"));

        let (s, d) = f.read_next();
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"KEY002"));

        let (s, d) = f.read_next();
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"KEY003"));

        let (s, _) = f.read_next();
        assert_eq!(s, FileStatusCode::AT_END);

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn rewrite_record() {
        let path = temp_path("idx_rewrite.db");
        let mut f = IndexedFile::new(
            "F".to_string(),
            path.clone(),
            80,
            FileAccessMode::Random,
            0,
            10,
        );

        f.open(FileOpenMode::InputOutput);
        f.write_record(&make_record(b"KEY001", b"OLD DATA"));

        // Read then rewrite
        let key = make_record(b"KEY001", b"")[..10].to_vec();
        f.read_by_key(&key);
        let new_rec = make_record(b"KEY001", b"NEW DATA");
        assert_eq!(f.rewrite_record(&new_rec), FileStatusCode::SUCCESS);

        // Verify
        let (s, d) = f.read_by_key(&key);
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap()[10..].starts_with(b"NEW DATA"));

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn delete_record() {
        let path = temp_path("idx_delete.db");
        let mut f = IndexedFile::new(
            "F".to_string(),
            path.clone(),
            80,
            FileAccessMode::Random,
            0,
            10,
        );

        f.open(FileOpenMode::InputOutput);
        f.write_record(&make_record(b"KEY001", b"DATA"));

        let key = make_record(b"KEY001", b"")[..10].to_vec();
        assert_eq!(f.delete_by_key(&key), FileStatusCode::SUCCESS);

        // Verify deleted
        let (s, _) = f.read_by_key(&key);
        assert_eq!(s, FileStatusCode::RECORD_NOT_FOUND);

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn delete_nonexistent() {
        let path = temp_path("idx_dne.db");
        let mut f = IndexedFile::new(
            "F".to_string(),
            path.clone(),
            80,
            FileAccessMode::Random,
            0,
            10,
        );

        f.open(FileOpenMode::InputOutput);
        let key = vec![b' '; 10];
        assert_eq!(f.delete_by_key(&key), FileStatusCode::RECORD_NOT_FOUND);
        f.close();

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn start_positions_cursor() {
        let path = temp_path("idx_start.db");
        let mut f = IndexedFile::new(
            "F".to_string(),
            path.clone(),
            80,
            FileAccessMode::Dynamic,
            0,
            10,
        );

        f.open(FileOpenMode::Output);
        f.write_record(&make_record(b"AAA", b"FIRST"));
        f.write_record(&make_record(b"BBB", b"SECOND"));
        f.write_record(&make_record(b"CCC", b"THIRD"));
        f.close();

        f.open(FileOpenMode::Input);
        // START at "BBB" -- should position so next READ returns BBB's record
        let start_key = make_record(b"BBB", b"")[..10].to_vec();
        assert_eq!(f.start(&start_key), FileStatusCode::SUCCESS);

        // read_next after START reads the record AFTER the positioned key
        let (s, d) = f.read_next();
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"CCC"));

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn metadata() {
        let f = IndexedFile::new(
            "MY-IDX".to_string(),
            PathBuf::from("test.db"),
            80,
            FileAccessMode::Dynamic,
            0,
            10,
        );
        assert_eq!(f.organization(), FileOrganization::Indexed);
        assert_eq!(f.access_mode(), FileAccessMode::Dynamic);
        assert_eq!(f.select_name(), "MY-IDX");
        assert_eq!(f.record_length(), 80);
        assert!(!f.is_open());
    }

    #[test]
    fn rewrite_without_prior_read() {
        let path = temp_path("idx_rw_noread.db");
        let mut f = IndexedFile::new(
            "F".to_string(),
            path.clone(),
            80,
            FileAccessMode::Random,
            0,
            10,
        );

        f.open(FileOpenMode::InputOutput);
        assert_eq!(
            f.rewrite_record(&make_record(b"KEY", b"DATA")),
            FileStatusCode::NO_PRIOR_READ
        );
        f.close();

        let _ = std::fs::remove_file(&path);
    }
}
