use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};

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

// Helper: get current date/time components from system clock.
fn current_datetime() -> (u16, u8, u8, u16, u8, u8, u8, u32) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.subsec_millis());
    // Simple civil date calculation from Unix timestamp
    let days = (secs / 86400) as i64;
    let time_of_day = secs % 86400;
    let hours = (time_of_day / 3600) as u8;
    let minutes = ((time_of_day % 3600) / 60) as u8;
    let seconds = (time_of_day % 60) as u8;
    let _hundredths = (millis / 10) as u8;

    // Days since 1970-01-01 to Y/M/D
    let mut y = 1970i32;
    let mut remaining = days;
    loop {
        let year_days = if is_leap(y) { 366 } else { 365 };
        if remaining < year_days {
            break;
        }
        remaining -= year_days;
        y += 1;
    }
    let leap = is_leap(y);
    let month_days = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m = 0u8;
    for md in &month_days {
        if remaining < *md {
            break;
        }
        remaining -= md;
        m += 1;
    }
    let day_of_year = (days - {
        let mut d = 0i64;
        let mut yy = 1970;
        while yy < y {
            d += if is_leap(yy) { 366 } else { 365 };
            yy += 1;
        }
        d
    }) as u16 + 1;

    (y as u16, m + 1, remaining as u8 + 1, day_of_year, hours, minutes, seconds, millis)
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

/// Write formatted string into a CobolField, left-justified, space-padded.
fn write_str_to_field(dest: &mut dyn CobolField, s: &str) {
    let dest_bytes = dest.as_bytes_mut();
    let src = s.as_bytes();
    let copy_len = src.len().min(dest_bytes.len());
    dest_bytes[..copy_len].copy_from_slice(&src[..copy_len]);
    dest_bytes[copy_len..].fill(b' ');
}

/// ACCEPT identifier FROM DATE -- YYMMDD format (6 digits).
pub fn accept_date(dest: &mut dyn CobolField) {
    let (y, m, d, _, _, _, _, _) = current_datetime();
    let s = format!("{:02}{:02}{:02}", y % 100, m, d);
    write_str_to_field(dest, &s);
}

/// ACCEPT identifier FROM DATE YYYYMMDD -- YYYYMMDD format (8 digits).
pub fn accept_date_yyyymmdd(dest: &mut dyn CobolField) {
    let (y, m, d, _, _, _, _, _) = current_datetime();
    let s = format!("{:04}{:02}{:02}", y, m, d);
    write_str_to_field(dest, &s);
}

/// ACCEPT identifier FROM TIME -- HHMMSSCC format (8 digits, CC = hundredths).
pub fn accept_time(dest: &mut dyn CobolField) {
    let (_, _, _, _, h, min, sec, millis) = current_datetime();
    let cc = millis / 10;
    let s = format!("{:02}{:02}{:02}{:02}", h, min, sec, cc);
    write_str_to_field(dest, &s);
}

/// ACCEPT identifier FROM DAY -- YYDDD format (5 digits, DDD = day of year).
pub fn accept_day(dest: &mut dyn CobolField) {
    let (y, _, _, doy, _, _, _, _) = current_datetime();
    let s = format!("{:02}{:03}", y % 100, doy);
    write_str_to_field(dest, &s);
}

/// ACCEPT identifier FROM DAY YYYYDDD -- YYYYDDD format (7 digits).
pub fn accept_day_yyyyddd(dest: &mut dyn CobolField) {
    let (y, _, _, doy, _, _, _, _) = current_datetime();
    let s = format!("{:04}{:03}", y, doy);
    write_str_to_field(dest, &s);
}

/// ACCEPT identifier FROM DAY-OF-WEEK -- 1 digit (1=Monday ... 7=Sunday).
pub fn accept_day_of_week(dest: &mut dyn CobolField) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    // 1970-01-01 was a Thursday (ISO day 4)
    let days = secs / 86400;
    let dow = ((days + 3) % 7) + 1; // 1=Monday, 7=Sunday
    let s = format!("{}", dow);
    write_str_to_field(dest, &s);
}
