use std::io::{self, BufRead, Write};

use cobol_core::traits::CobolField;

/// DISPLAY ... UPON SYSOUT (or just DISPLAY).
///
/// Concatenates all items and writes to stdout with a trailing newline.
pub fn display_upon_sysout(items: &[&dyn CobolField]) {
    let mut output = Vec::new();
    for item in items {
        output.extend_from_slice(&item.display_bytes());
    }
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let _ = handle.write_all(&output);
    let _ = handle.write_all(b"\n");
    let _ = handle.flush();
}

/// DISPLAY ... UPON SYSOUT from raw string slices.
///
/// Convenience for displaying string literals (transpiler uses this for
/// DISPLAY "HELLO WORLD").
pub fn display_strings(items: &[&str]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for item in items {
        let _ = handle.write_all(item.as_bytes());
    }
    let _ = handle.write_all(b"\n");
    let _ = handle.flush();
}

/// DISPLAY ... UPON SYSERR.
///
/// Same as sysout but writes to stderr.
pub fn display_upon_syserr(items: &[&dyn CobolField]) {
    let mut output = Vec::new();
    for item in items {
        output.extend_from_slice(&item.display_bytes());
    }
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = handle.write_all(&output);
    let _ = handle.write_all(b"\n");
    let _ = handle.flush();
}

/// ACCEPT identifier FROM SYSIN.
///
/// Reads a line from stdin and stores it in the destination field.
pub fn accept_from_sysin(dest: &mut dyn CobolField) {
    let stdin = io::stdin();
    let mut line = String::new();
    let _ = stdin.lock().read_line(&mut line);

    // Remove trailing newline
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    // Store into destination (left-justified, space-padded, right-truncated)
    let dest_bytes = dest.as_bytes_mut();
    let src = line.as_bytes();
    let copy_len = src.len().min(dest_bytes.len());
    dest_bytes[..copy_len].copy_from_slice(&src[..copy_len]);
    dest_bytes[copy_len..].fill(b' ');
}
