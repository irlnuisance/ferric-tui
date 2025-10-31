use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteSize(u64);

impl ByteSize {
    pub fn new(bytes: u64) -> Self {
        Self(bytes)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn zero() -> Self {
        Self(0)
    }
}

impl From<u64> for ByteSize {
    fn from(bytes: u64) -> Self {
        Self(bytes)
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;
        let b = self.0 as f64;
        if b >= GB {
            write!(f, "{:.1} GiB", b / GB)
        } else if b >= MB {
            write!(f, "{:.1} MiB", b / MB)
        } else if b >= KB {
            write!(f, "{:.0} KiB", b / KB)
        } else {
            write!(f, "{} B", self.0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(f64);

impl Percent {
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 100.0))
    }

    pub fn from_ratio(current: u64, total: u64) -> Self {
        Self::new(if total > 0 {
            (current as f64 / total as f64) * 100.0
        } else {
            0.0
        })
    }

    pub fn as_f64(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Throughput(f64);

impl Throughput {
    pub fn new(bps: f64) -> Self {
        Self(bps.max(0.0))
    }

    pub fn as_f64(self) -> f64 {
        self.0
    }

    pub fn zero() -> Self {
        Self(0.0)
    }
}

impl fmt::Display for Throughput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/s", ByteSize::new(self.0 as u64))
    }
}
