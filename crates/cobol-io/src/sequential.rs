//! Sequential and Line-Sequential file implementations.
//!
//! **SEQUENTIAL**: Fixed-length records, binary read/write.
//! **LINE SEQUENTIAL**: Variable-length text records, newline-delimited.
//!
//! Both support: OPEN INPUT/OUTPUT/EXTEND, sequential READ, WRITE.
//! REWRITE is supported only for OPEN I-O (fixed-length SEQUENTIAL only).

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use crate::file_status::FileStatusCode;
use crate::file_traits::{CobolFile, FileAccessMode, FileOpenMode, FileOrganization};

/// Internal state for an open file.
enum OpenState {
    /// File is not open.
    Closed,
    /// Open for reading.
    Reading(BufReader<File>),
    /// Open for writing.
    Writing(BufWriter<File>),
}

/// Sequential file (ORGANIZATION IS SEQUENTIAL or LINE SEQUENTIAL).
pub struct SequentialFile {
    /// COBOL SELECT name.
    select_name: String,
    /// Filesystem path (resolved by FileResolver).
    path: PathBuf,
    /// File organization (Sequential or LineSequential).
    organization: FileOrganization,
    /// Fixed record length (0 for LINE SEQUENTIAL = variable).
    record_length: usize,
    /// Current open mode.
    open_mode: Option<FileOpenMode>,
    /// Internal file state.
    state: OpenState,
    /// Whether the last READ was successful (for REWRITE).
    has_current_record: bool,
}

impl std::fmt::Debug for SequentialFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SequentialFile")
            .field("select_name", &self.select_name)
            .field("path", &self.path)
            .field("organization", &self.organization)
            .field("record_length", &self.record_length)
            .field("open_mode", &self.open_mode)
            .finish()
    }
}

impl SequentialFile {
    /// Create a new sequential file descriptor.
    ///
    /// - `select_name`: COBOL SELECT name.
    /// - `path`: Resolved filesystem path.
    /// - `organization`: Sequential or LineSequential.
    /// - `record_length`: Fixed record length (for SEQUENTIAL).
    ///   For LINE SEQUENTIAL, pass 0 (records are newline-delimited).
    pub fn new(
        select_name: String,
        path: PathBuf,
        organization: FileOrganization,
        record_length: usize,
    ) -> Self {
        assert!(
            organization == FileOrganization::Sequential
                || organization == FileOrganization::LineSequential,
            "SequentialFile only supports Sequential or LineSequential"
        );
        Self {
            select_name,
            path,
            organization,
            record_length,
            open_mode: None,
            state: OpenState::Closed,
            has_current_record: false,
        }
    }
}

impl CobolFile for SequentialFile {
    fn open(&mut self, mode: FileOpenMode) -> FileStatusCode {
        if self.is_open() {
            return FileStatusCode::ALREADY_OPEN;
        }

        match mode {
            FileOpenMode::Input => {
                match File::open(&self.path) {
                    Ok(f) => {
                        self.state = OpenState::Reading(BufReader::new(f));
                        self.open_mode = Some(mode);
                        FileStatusCode::SUCCESS
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        FileStatusCode::PERM_FILENAME
                    }
                    Err(_) => FileStatusCode::PERM_ERROR,
                }
            }
            FileOpenMode::Output => {
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&self.path)
                {
                    Ok(f) => {
                        self.state = OpenState::Writing(BufWriter::new(f));
                        self.open_mode = Some(mode);
                        FileStatusCode::SUCCESS
                    }
                    Err(_) => FileStatusCode::PERM_ERROR,
                }
            }
            FileOpenMode::Extend => {
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&self.path)
                {
                    Ok(f) => {
                        self.state = OpenState::Writing(BufWriter::new(f));
                        self.open_mode = Some(mode);
                        FileStatusCode::SUCCESS
                    }
                    Err(_) => FileStatusCode::PERM_ERROR,
                }
            }
            FileOpenMode::InputOutput => {
                // I-O mode: for sequential, we open for reading first.
                // REWRITE will need the file handle -- not fully supported for sequential.
                // In COBOL, I-O on sequential means READ then REWRITE in place.
                // For simplicity, we open for reading. REWRITE is a future enhancement.
                match File::open(&self.path) {
                    Ok(f) => {
                        self.state = OpenState::Reading(BufReader::new(f));
                        self.open_mode = Some(mode);
                        FileStatusCode::SUCCESS
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        FileStatusCode::PERM_FILENAME
                    }
                    Err(_) => FileStatusCode::PERM_ERROR,
                }
            }
        }
    }

    fn close(&mut self) -> FileStatusCode {
        if !self.is_open() {
            return FileStatusCode::NOT_OPEN;
        }

        // Flush if writing
        if let OpenState::Writing(ref mut writer) = self.state {
            if writer.flush().is_err() {
                self.state = OpenState::Closed;
                self.open_mode = None;
                return FileStatusCode::PERM_ERROR;
            }
        }

        self.state = OpenState::Closed;
        self.open_mode = None;
        self.has_current_record = false;
        FileStatusCode::SUCCESS
    }

    fn read_next(&mut self) -> (FileStatusCode, Option<Vec<u8>>) {
        match self.open_mode {
            Some(FileOpenMode::Input) | Some(FileOpenMode::InputOutput) => {}
            Some(_) => return (FileStatusCode::BAD_OPEN_MODE, None),
            None => return (FileStatusCode::NOT_OPEN, None),
        }

        match self.state {
            OpenState::Reading(ref mut reader) => {
                match self.organization {
                    FileOrganization::LineSequential => {
                        let mut line = String::new();
                        match reader.read_line(&mut line) {
                            Ok(0) => {
                                self.has_current_record = false;
                                (FileStatusCode::AT_END, None)
                            }
                            Ok(_) => {
                                // Strip trailing newline
                                if line.ends_with('\n') {
                                    line.pop();
                                    if line.ends_with('\r') {
                                        line.pop();
                                    }
                                }
                                self.has_current_record = true;
                                (FileStatusCode::SUCCESS, Some(line.into_bytes()))
                            }
                            Err(_) => {
                                self.has_current_record = false;
                                (FileStatusCode::PERM_ERROR, None)
                            }
                        }
                    }
                    FileOrganization::Sequential => {
                        let mut buf = vec![0u8; self.record_length];
                        match reader.read_exact(&mut buf) {
                            Ok(()) => {
                                self.has_current_record = true;
                                (FileStatusCode::SUCCESS, Some(buf))
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                                self.has_current_record = false;
                                (FileStatusCode::AT_END, None)
                            }
                            Err(_) => {
                                self.has_current_record = false;
                                (FileStatusCode::PERM_ERROR, None)
                            }
                        }
                    }
                    _ => (FileStatusCode::PERM_ERROR, None),
                }
            }
            _ => (FileStatusCode::BAD_OPEN_MODE, None),
        }
    }

    fn write_record(&mut self, record: &[u8]) -> FileStatusCode {
        match self.open_mode {
            Some(FileOpenMode::Output) | Some(FileOpenMode::Extend) => {}
            Some(_) => return FileStatusCode::BAD_OPEN_MODE,
            None => return FileStatusCode::NOT_OPEN,
        }

        match self.state {
            OpenState::Writing(ref mut writer) => {
                match self.organization {
                    FileOrganization::LineSequential => {
                        if writer.write_all(record).is_err() {
                            return FileStatusCode::PERM_ERROR;
                        }
                        if writer.write_all(b"\n").is_err() {
                            return FileStatusCode::PERM_ERROR;
                        }
                        FileStatusCode::SUCCESS
                    }
                    FileOrganization::Sequential => {
                        // For fixed-length sequential: pad or truncate to record_length
                        let mut padded = vec![b' '; self.record_length];
                        let copy_len = record.len().min(self.record_length);
                        padded[..copy_len].copy_from_slice(&record[..copy_len]);
                        if writer.write_all(&padded).is_err() {
                            return FileStatusCode::PERM_ERROR;
                        }
                        FileStatusCode::SUCCESS
                    }
                    _ => FileStatusCode::PERM_ERROR,
                }
            }
            _ => FileStatusCode::BAD_OPEN_MODE,
        }
    }

    fn rewrite_record(&mut self, _record: &[u8]) -> FileStatusCode {
        // REWRITE on sequential files requires I-O mode and a prior READ.
        // Full implementation would require seeking back. For now, return error.
        match self.open_mode {
            Some(FileOpenMode::InputOutput) => {
                if !self.has_current_record {
                    FileStatusCode::NO_PRIOR_READ
                } else {
                    // Sequential REWRITE is complex (seek back by record_length).
                    // Not yet implemented -- return permanent error.
                    FileStatusCode::PERM_ERROR
                }
            }
            Some(_) => FileStatusCode::BAD_OPEN_MODE,
            None => FileStatusCode::NOT_OPEN,
        }
    }

    fn organization(&self) -> FileOrganization {
        self.organization
    }

    fn access_mode(&self) -> FileAccessMode {
        FileAccessMode::Sequential
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
    use std::io::Write as IoWrite;

    fn temp_path(name: &str) -> PathBuf {
        let dir = tempfile::tempdir().unwrap();
        // Leak the TempDir so it persists for the test
        let path = dir.path().join(name);
        std::mem::forget(dir);
        path
    }

    // --- LINE SEQUENTIAL ---

    #[test]
    fn line_seq_write_and_read() {
        let path = temp_path("test_line_seq.txt");

        // Write records
        {
            let mut f = SequentialFile::new(
                "TEST-FILE".to_string(),
                path.clone(),
                FileOrganization::LineSequential,
                0,
            );
            assert_eq!(f.open(FileOpenMode::Output), FileStatusCode::SUCCESS);
            assert!(f.is_open());
            assert_eq!(f.write_record(b"HELLO WORLD"), FileStatusCode::SUCCESS);
            assert_eq!(f.write_record(b"SECOND LINE"), FileStatusCode::SUCCESS);
            assert_eq!(f.close(), FileStatusCode::SUCCESS);
        }

        // Read records back
        {
            let mut f = SequentialFile::new(
                "TEST-FILE".to_string(),
                path.clone(),
                FileOrganization::LineSequential,
                0,
            );
            assert_eq!(f.open(FileOpenMode::Input), FileStatusCode::SUCCESS);

            let (status, data) = f.read_next();
            assert_eq!(status, FileStatusCode::SUCCESS);
            assert_eq!(data.unwrap(), b"HELLO WORLD");

            let (status, data) = f.read_next();
            assert_eq!(status, FileStatusCode::SUCCESS);
            assert_eq!(data.unwrap(), b"SECOND LINE");

            let (status, data) = f.read_next();
            assert_eq!(status, FileStatusCode::AT_END);
            assert!(data.is_none());

            assert_eq!(f.close(), FileStatusCode::SUCCESS);
        }

        // Cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn line_seq_extend() {
        let path = temp_path("test_extend.txt");

        // Write initial records
        {
            let mut f = SequentialFile::new(
                "F".to_string(),
                path.clone(),
                FileOrganization::LineSequential,
                0,
            );
            f.open(FileOpenMode::Output);
            f.write_record(b"LINE1");
            f.close();
        }

        // Extend
        {
            let mut f = SequentialFile::new(
                "F".to_string(),
                path.clone(),
                FileOrganization::LineSequential,
                0,
            );
            f.open(FileOpenMode::Extend);
            f.write_record(b"LINE2");
            f.close();
        }

        // Read all
        {
            let mut f = SequentialFile::new(
                "F".to_string(),
                path.clone(),
                FileOrganization::LineSequential,
                0,
            );
            f.open(FileOpenMode::Input);
            let (_, d1) = f.read_next();
            let (_, d2) = f.read_next();
            let (s3, _) = f.read_next();
            assert_eq!(d1.unwrap(), b"LINE1");
            assert_eq!(d2.unwrap(), b"LINE2");
            assert_eq!(s3, FileStatusCode::AT_END);
            f.close();
        }

        let _ = std::fs::remove_file(&path);
    }

    // --- FIXED-LENGTH SEQUENTIAL ---

    #[test]
    fn fixed_seq_write_and_read() {
        let path = temp_path("test_fixed_seq.dat");

        // Write 80-byte records
        {
            let mut f = SequentialFile::new(
                "FIXED-FILE".to_string(),
                path.clone(),
                FileOrganization::Sequential,
                80,
            );
            assert_eq!(f.open(FileOpenMode::Output), FileStatusCode::SUCCESS);
            assert_eq!(f.write_record(b"RECORD ONE"), FileStatusCode::SUCCESS);
            assert_eq!(f.write_record(b"RECORD TWO"), FileStatusCode::SUCCESS);
            assert_eq!(f.close(), FileStatusCode::SUCCESS);
        }

        // Read back
        {
            let mut f = SequentialFile::new(
                "FIXED-FILE".to_string(),
                path.clone(),
                FileOrganization::Sequential,
                80,
            );
            assert_eq!(f.open(FileOpenMode::Input), FileStatusCode::SUCCESS);

            let (status, data) = f.read_next();
            assert_eq!(status, FileStatusCode::SUCCESS);
            let rec = data.unwrap();
            assert_eq!(rec.len(), 80);
            assert!(rec.starts_with(b"RECORD ONE"));
            // Remainder should be spaces
            assert!(rec[10..].iter().all(|&b| b == b' '));

            let (status, data) = f.read_next();
            assert_eq!(status, FileStatusCode::SUCCESS);
            assert!(data.unwrap().starts_with(b"RECORD TWO"));

            let (status, _) = f.read_next();
            assert_eq!(status, FileStatusCode::AT_END);

            assert_eq!(f.close(), FileStatusCode::SUCCESS);
        }

        let _ = std::fs::remove_file(&path);
    }

    // --- Error Cases ---

    #[test]
    fn open_nonexistent_input() {
        let mut f = SequentialFile::new(
            "MISSING".to_string(),
            PathBuf::from("/nonexistent/path/file.dat"),
            FileOrganization::LineSequential,
            0,
        );
        let status = f.open(FileOpenMode::Input);
        assert_eq!(status, FileStatusCode::PERM_FILENAME);
    }

    #[test]
    fn double_open() {
        let path = temp_path("test_double.txt");
        let mut f = SequentialFile::new(
            "F".to_string(),
            path.clone(),
            FileOrganization::LineSequential,
            0,
        );
        f.open(FileOpenMode::Output);
        assert_eq!(f.open(FileOpenMode::Output), FileStatusCode::ALREADY_OPEN);
        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn close_when_not_open() {
        let mut f = SequentialFile::new(
            "F".to_string(),
            PathBuf::from("unused"),
            FileOrganization::LineSequential,
            0,
        );
        assert_eq!(f.close(), FileStatusCode::NOT_OPEN);
    }

    #[test]
    fn read_when_not_open() {
        let mut f = SequentialFile::new(
            "F".to_string(),
            PathBuf::from("unused"),
            FileOrganization::LineSequential,
            0,
        );
        let (status, _) = f.read_next();
        assert_eq!(status, FileStatusCode::NOT_OPEN);
    }

    #[test]
    fn write_on_input_mode() {
        let path = temp_path("test_write_input.txt");
        // Create the file first
        { let _ = File::create(&path).unwrap(); }

        let mut f = SequentialFile::new(
            "F".to_string(),
            path.clone(),
            FileOrganization::LineSequential,
            0,
        );
        f.open(FileOpenMode::Input);
        assert_eq!(
            f.write_record(b"SHOULD FAIL"),
            FileStatusCode::BAD_OPEN_MODE
        );
        f.close();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn select_name_and_org() {
        let f = SequentialFile::new(
            "MY-FILE".to_string(),
            PathBuf::from("test"),
            FileOrganization::LineSequential,
            0,
        );
        assert_eq!(f.select_name(), "MY-FILE");
        assert_eq!(f.organization(), FileOrganization::LineSequential);
        assert_eq!(f.access_mode(), FileAccessMode::Sequential);
        assert_eq!(f.record_length(), 0);
    }

    #[test]
    fn rewrite_without_prior_read() {
        let path = temp_path("test_rewrite.dat");
        // Create the file with some data
        {
            let mut file = File::create(&path).unwrap();
            file.write_all(&[b' '; 80]).unwrap();
        }

        let mut f = SequentialFile::new(
            "F".to_string(),
            path.clone(),
            FileOrganization::Sequential,
            80,
        );
        f.open(FileOpenMode::InputOutput);
        // No READ before REWRITE
        assert_eq!(
            f.rewrite_record(&[0u8; 80]),
            FileStatusCode::NO_PRIOR_READ
        );
        f.close();
        let _ = std::fs::remove_file(&path);
    }
}
