//! Relative file implementation (ORGANIZATION IS RELATIVE).
//!
//! Records are stored at fixed-size slots addressed by an integer key (1-based).
//! Each slot has a 1-byte control byte: 0x00 = empty, 0x01 = occupied.
//! Slot size = 1 (control) + `record_length` bytes.
//!
//! Supports: OPEN INPUT/OUTPUT/I-O, READ by key, READ NEXT, WRITE, REWRITE, DELETE.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use crate::file_status::FileStatusCode;
use crate::file_traits::{CobolFile, FileAccessMode, FileOpenMode, FileOrganization};

/// Control byte values for slot state.
const SLOT_EMPTY: u8 = 0x00;
const SLOT_OCCUPIED: u8 = 0x01;

/// Relative file (ORGANIZATION IS RELATIVE).
///
/// Records are stored in fixed-size slots addressed by integer key.
/// Access can be sequential, random, or dynamic.
pub struct RelativeFile {
    /// COBOL SELECT name.
    select_name: String,
    /// Filesystem path.
    path: PathBuf,
    /// Fixed record length.
    record_length: usize,
    /// Access mode.
    access_mode: FileAccessMode,
    /// Current open mode.
    open_mode: Option<FileOpenMode>,
    /// File handle.
    file: Option<File>,
    /// Current sequential position (1-based key for READ NEXT).
    current_key: usize,
}

impl std::fmt::Debug for RelativeFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RelativeFile")
            .field("select_name", &self.select_name)
            .field("path", &self.path)
            .field("record_length", &self.record_length)
            .field("access_mode", &self.access_mode)
            .field("open_mode", &self.open_mode)
            .finish()
    }
}

impl RelativeFile {
    /// Create a new relative file descriptor.
    pub fn new(
        select_name: String,
        path: PathBuf,
        record_length: usize,
        access_mode: FileAccessMode,
    ) -> Self {
        Self {
            select_name,
            path,
            record_length,
            access_mode,
            open_mode: None,
            file: None,
            current_key: 0,
        }
    }

    /// Slot size = control byte + record bytes.
    fn slot_size(&self) -> u64 {
        1 + self.record_length as u64
    }

    /// Seek to the start of a slot (1-based key).
    fn seek_to_slot(&mut self, key: usize) -> std::io::Result<()> {
        if key == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Relative key must be >= 1",
            ));
        }
        let offset = (key as u64 - 1) * self.slot_size();
        if let Some(ref mut f) = self.file {
            f.seek(SeekFrom::Start(offset))?;
        }
        Ok(())
    }

    /// Read the control byte at the current position.
    fn read_control(&mut self) -> std::io::Result<u8> {
        let mut buf = [0u8; 1];
        if let Some(ref mut f) = self.file {
            f.read_exact(&mut buf)?;
        }
        Ok(buf[0])
    }

    /// Read a record at the current file position (after control byte).
    fn read_record_data(&mut self) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; self.record_length];
        if let Some(ref mut f) = self.file {
            f.read_exact(&mut buf)?;
        }
        Ok(buf)
    }

    /// READ by relative key (random access).
    pub fn read_by_key(&mut self, key: usize) -> (FileStatusCode, Option<Vec<u8>>) {
        if self.file.is_none() {
            return (FileStatusCode::NOT_OPEN, None);
        }
        match self.open_mode {
            Some(FileOpenMode::Input | FileOpenMode::InputOutput) => {}
            _ => return (FileStatusCode::BAD_OPEN_MODE, None),
        }

        if self.seek_to_slot(key).is_err() {
            return (FileStatusCode::RECORD_NOT_FOUND, None);
        }

        match self.read_control() {
            Ok(SLOT_OCCUPIED) => {
                match self.read_record_data() {
                    Ok(data) => {
                        self.current_key = key;
                        (FileStatusCode::SUCCESS, Some(data))
                    }
                    Err(_) => (FileStatusCode::PERM_ERROR, None),
                }
            }
            Ok(SLOT_EMPTY) => (FileStatusCode::RECORD_NOT_FOUND, None),
            Ok(_) => (FileStatusCode::PERM_ERROR, None),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                (FileStatusCode::RECORD_NOT_FOUND, None)
            }
            Err(_) => (FileStatusCode::PERM_ERROR, None),
        }
    }

    /// WRITE a record at the given key (random access).
    pub fn write_at_key(&mut self, key: usize, record: &[u8]) -> FileStatusCode {
        if self.file.is_none() {
            return FileStatusCode::NOT_OPEN;
        }
        match self.open_mode {
            Some(FileOpenMode::Output | FileOpenMode::InputOutput) => {}
            _ => return FileStatusCode::BAD_OPEN_MODE,
        }

        if self.seek_to_slot(key).is_err() {
            return FileStatusCode::PERM_ERROR;
        }

        // Check if slot already occupied (for I-O mode, this is a duplicate key error)
        if self.open_mode == Some(FileOpenMode::InputOutput) {
            if let Ok(SLOT_OCCUPIED) = self.read_control() {
                return FileStatusCode::DUPLICATE_KEY;
            }
            // Seek back to slot start
            if self.seek_to_slot(key).is_err() {
                return FileStatusCode::PERM_ERROR;
            }
        }

        // Write control byte + padded record
        let mut slot = vec![0u8; self.slot_size() as usize];
        slot[0] = SLOT_OCCUPIED;
        let copy_len = record.len().min(self.record_length);
        slot[1..=copy_len].copy_from_slice(&record[..copy_len]);
        // Pad remainder with spaces
        for b in &mut slot[1 + copy_len..] {
            *b = b' ';
        }

        if let Some(ref mut f) = self.file {
            if f.write_all(&slot).is_err() {
                return FileStatusCode::PERM_ERROR;
            }
        }
        FileStatusCode::SUCCESS
    }

    /// REWRITE the record at the given key.
    pub fn rewrite_at_key(&mut self, key: usize, record: &[u8]) -> FileStatusCode {
        if self.file.is_none() {
            return FileStatusCode::NOT_OPEN;
        }
        if self.open_mode != Some(FileOpenMode::InputOutput) {
            return FileStatusCode::BAD_OPEN_MODE;
        }

        if self.seek_to_slot(key).is_err() {
            return FileStatusCode::RECORD_NOT_FOUND;
        }

        // Verify slot is occupied
        match self.read_control() {
            Ok(SLOT_OCCUPIED) => {}
            Ok(_) => return FileStatusCode::RECORD_NOT_FOUND,
            Err(_) => return FileStatusCode::RECORD_NOT_FOUND,
        }

        // Write record data (control byte already read, file is positioned at record)
        let mut padded = vec![b' '; self.record_length];
        let copy_len = record.len().min(self.record_length);
        padded[..copy_len].copy_from_slice(&record[..copy_len]);

        if let Some(ref mut f) = self.file {
            if f.write_all(&padded).is_err() {
                return FileStatusCode::PERM_ERROR;
            }
        }
        FileStatusCode::SUCCESS
    }

    /// DELETE the record at the given key.
    pub fn delete_at_key(&mut self, key: usize) -> FileStatusCode {
        if self.file.is_none() {
            return FileStatusCode::NOT_OPEN;
        }
        if self.open_mode != Some(FileOpenMode::InputOutput) {
            return FileStatusCode::BAD_OPEN_MODE;
        }

        if self.seek_to_slot(key).is_err() {
            return FileStatusCode::RECORD_NOT_FOUND;
        }

        match self.read_control() {
            Ok(SLOT_OCCUPIED) => {}
            Ok(_) => return FileStatusCode::RECORD_NOT_FOUND,
            Err(_) => return FileStatusCode::RECORD_NOT_FOUND,
        }

        // Seek back to control byte and write EMPTY
        if self.seek_to_slot(key).is_err() {
            return FileStatusCode::PERM_ERROR;
        }
        if let Some(ref mut f) = self.file {
            if f.write_all(&[SLOT_EMPTY]).is_err() {
                return FileStatusCode::PERM_ERROR;
            }
        }
        FileStatusCode::SUCCESS
    }
}

impl CobolFile for RelativeFile {
    fn open(&mut self, mode: FileOpenMode) -> FileStatusCode {
        if self.is_open() {
            return FileStatusCode::ALREADY_OPEN;
        }

        let file = match mode {
            FileOpenMode::Input => {
                match File::open(&self.path) {
                    Ok(f) => f,
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        return FileStatusCode::PERM_FILENAME;
                    }
                    Err(_) => return FileStatusCode::PERM_ERROR,
                }
            }
            FileOpenMode::Output => {
                match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&self.path)
                {
                    Ok(f) => f,
                    Err(_) => return FileStatusCode::PERM_ERROR,
                }
            }
            FileOpenMode::InputOutput => {
                match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&self.path)
                {
                    Ok(f) => f,
                    Err(_) => return FileStatusCode::PERM_ERROR,
                }
            }
            FileOpenMode::Extend => {
                // EXTEND not meaningful for relative -- treat as error
                return FileStatusCode::BAD_OPEN_MODE;
            }
        };

        self.file = Some(file);
        self.open_mode = Some(mode);
        self.current_key = 0;
        FileStatusCode::SUCCESS
    }

    fn close(&mut self) -> FileStatusCode {
        if !self.is_open() {
            return FileStatusCode::NOT_OPEN;
        }
        self.file = None;
        self.open_mode = None;
        self.current_key = 0;
        FileStatusCode::SUCCESS
    }

    fn read_next(&mut self) -> (FileStatusCode, Option<Vec<u8>>) {
        if self.file.is_none() {
            return (FileStatusCode::NOT_OPEN, None);
        }
        match self.open_mode {
            Some(FileOpenMode::Input | FileOpenMode::InputOutput) => {}
            _ => return (FileStatusCode::BAD_OPEN_MODE, None),
        }

        // Scan forward from current_key + 1 looking for an occupied slot
        loop {
            self.current_key += 1;
            if self.seek_to_slot(self.current_key).is_err() {
                return (FileStatusCode::AT_END, None);
            }
            match self.read_control() {
                Ok(SLOT_OCCUPIED) => {
                    match self.read_record_data() {
                        Ok(data) => return (FileStatusCode::SUCCESS, Some(data)),
                        Err(_) => return (FileStatusCode::PERM_ERROR, None),
                    }
                }
                Ok(SLOT_EMPTY) => continue, // Skip empty slots
                Ok(_) => return (FileStatusCode::PERM_ERROR, None),
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return (FileStatusCode::AT_END, None);
                }
                Err(_) => return (FileStatusCode::PERM_ERROR, None),
            }
        }
    }

    fn write_record(&mut self, record: &[u8]) -> FileStatusCode {
        // For sequential WRITE in OUTPUT mode, append at next position
        match self.open_mode {
            Some(FileOpenMode::Output) => {
                self.current_key += 1;
                self.write_at_key(self.current_key, record)
            }
            Some(_) => FileStatusCode::BAD_OPEN_MODE,
            None => FileStatusCode::NOT_OPEN,
        }
    }

    fn rewrite_record(&mut self, record: &[u8]) -> FileStatusCode {
        if self.current_key == 0 {
            return FileStatusCode::NO_PRIOR_READ;
        }
        self.rewrite_at_key(self.current_key, record)
    }

    fn organization(&self) -> FileOrganization {
        FileOrganization::Relative
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

    #[test]
    fn write_and_read_by_key() {
        let path = temp_path("rel_rw.dat");
        let mut f = RelativeFile::new(
            "REL-FILE".to_string(),
            path.clone(),
            20,
            FileAccessMode::Random,
        );

        f.open(FileOpenMode::Output);
        assert_eq!(f.write_at_key(1, b"FIRST"), FileStatusCode::SUCCESS);
        assert_eq!(f.write_at_key(3, b"THIRD"), FileStatusCode::SUCCESS);
        f.close();

        // Read back
        f.open(FileOpenMode::Input);
        let (s, d) = f.read_by_key(1);
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"FIRST"));

        let (s, d) = f.read_by_key(2);
        assert_eq!(s, FileStatusCode::RECORD_NOT_FOUND);
        assert!(d.is_none());

        let (s, d) = f.read_by_key(3);
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"THIRD"));

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn sequential_read_skips_empty() {
        let path = temp_path("rel_seq.dat");
        let mut f = RelativeFile::new(
            "F".to_string(),
            path.clone(),
            10,
            FileAccessMode::Dynamic,
        );

        // Write at keys 1, 3, 5 (skip 2, 4)
        f.open(FileOpenMode::Output);
        f.write_at_key(1, b"REC1");
        f.write_at_key(3, b"REC3");
        f.write_at_key(5, b"REC5");
        f.close();

        // Sequential read should skip empty slots
        f.open(FileOpenMode::Input);
        let (s, d) = f.read_next();
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"REC1"));

        let (s, d) = f.read_next();
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"REC3"));

        let (s, d) = f.read_next();
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"REC5"));

        let (s, _) = f.read_next();
        assert_eq!(s, FileStatusCode::AT_END);

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn rewrite_record() {
        let path = temp_path("rel_rewrite.dat");
        let mut f = RelativeFile::new(
            "F".to_string(),
            path.clone(),
            10,
            FileAccessMode::Random,
        );

        f.open(FileOpenMode::InputOutput);
        f.write_at_key(1, b"OLD VALUE");
        assert_eq!(f.rewrite_at_key(1, b"NEW VALUE"), FileStatusCode::SUCCESS);

        let (s, d) = f.read_by_key(1);
        assert_eq!(s, FileStatusCode::SUCCESS);
        assert!(d.unwrap().starts_with(b"NEW VALUE"));

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn delete_record() {
        let path = temp_path("rel_delete.dat");
        let mut f = RelativeFile::new(
            "F".to_string(),
            path.clone(),
            10,
            FileAccessMode::Random,
        );

        f.open(FileOpenMode::InputOutput);
        f.write_at_key(1, b"DATA");
        assert_eq!(f.delete_at_key(1), FileStatusCode::SUCCESS);

        // Now reading should return not found
        let (s, _) = f.read_by_key(1);
        assert_eq!(s, FileStatusCode::RECORD_NOT_FOUND);

        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn delete_nonexistent() {
        let path = temp_path("rel_del_ne.dat");
        let mut f = RelativeFile::new(
            "F".to_string(),
            path.clone(),
            10,
            FileAccessMode::Random,
        );

        f.open(FileOpenMode::InputOutput);
        assert_eq!(f.delete_at_key(99), FileStatusCode::RECORD_NOT_FOUND);
        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn sequential_write_appends() {
        let path = temp_path("rel_seqw.dat");
        let mut f = RelativeFile::new(
            "F".to_string(),
            path.clone(),
            10,
            FileAccessMode::Sequential,
        );

        f.open(FileOpenMode::Output);
        f.write_record(b"REC1");
        f.write_record(b"REC2");
        f.write_record(b"REC3");
        f.close();

        f.open(FileOpenMode::Input);
        let (_, d) = f.read_next();
        assert!(d.unwrap().starts_with(b"REC1"));
        let (_, d) = f.read_next();
        assert!(d.unwrap().starts_with(b"REC2"));
        let (_, d) = f.read_next();
        assert!(d.unwrap().starts_with(b"REC3"));
        let (s, _) = f.read_next();
        assert_eq!(s, FileStatusCode::AT_END);
        f.close();

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn organization_and_metadata() {
        let f = RelativeFile::new(
            "MY-REL".to_string(),
            PathBuf::from("test"),
            80,
            FileAccessMode::Dynamic,
        );
        assert_eq!(f.organization(), FileOrganization::Relative);
        assert_eq!(f.access_mode(), FileAccessMode::Dynamic);
        assert_eq!(f.select_name(), "MY-REL");
        assert_eq!(f.record_length(), 80);
        assert!(!f.is_open());
    }
}
