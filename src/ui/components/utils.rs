/// Formats byte count into human-readable size with appropriate units.
///
/// Uses binary units (KiB, MiB, GiB) following IEC 80000-13 standard.
///
/// ```
/// use ferric::ui::components::human_size;
/// assert_eq!(human_size(1024), "1 KiB");
/// assert_eq!(human_size(1536), "2 KiB");
/// assert_eq!(human_size(1048576), "1.0 MiB");
/// ```
pub fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} GiB", b / GB)
    } else if b >= MB {
        format!("{:.1} MiB", b / MB)
    } else if b >= KB {
        format!("{:.0} KiB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

/// Formats seconds into human-readable time duration.
///
/// Returns format "Xs", "XmYs", or "XhYm" depending on magnitude.
///
/// ```
/// use ferric::ui::components::format_seconds;
/// assert_eq!(format_seconds(45), "45s");
/// assert_eq!(format_seconds(90), "1m 30s");
/// assert_eq!(format_seconds(3661), "1h 1m");
/// ```
pub fn format_seconds(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_size_bytes() {
        assert_eq!(human_size(0), "0 B");
        assert_eq!(human_size(512), "512 B");
        assert_eq!(human_size(1023), "1023 B");
    }

    #[test]
    fn test_human_size_kilobytes() {
        assert_eq!(human_size(1024), "1 KiB");
        assert_eq!(human_size(1536), "2 KiB");
        assert_eq!(human_size(10240), "10 KiB");
    }

    #[test]
    fn test_human_size_megabytes() {
        assert_eq!(human_size(1048576), "1.0 MiB");
        assert_eq!(human_size(10485760), "10.0 MiB");
        assert_eq!(human_size(1572864), "1.5 MiB");
    }

    #[test]
    fn test_human_size_gigabytes() {
        assert_eq!(human_size(1073741824), "1.0 GiB");
        assert_eq!(human_size(2147483648), "2.0 GiB");
    }

    #[test]
    fn test_format_seconds_short() {
        assert_eq!(format_seconds(0), "0s");
        assert_eq!(format_seconds(30), "30s");
        assert_eq!(format_seconds(59), "59s");
    }

    #[test]
    fn test_format_seconds_minutes() {
        assert_eq!(format_seconds(60), "1m 0s");
        assert_eq!(format_seconds(90), "1m 30s");
        assert_eq!(format_seconds(3599), "59m 59s");
    }

    #[test]
    fn test_format_seconds_hours() {
        assert_eq!(format_seconds(3600), "1h 0m");
        assert_eq!(format_seconds(3661), "1h 1m");
        assert_eq!(format_seconds(7200), "2h 0m");
    }
}
