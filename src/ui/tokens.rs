/// spacing const
pub const S0: u16 = 0;

pub const S1: u16 = 1;

pub const S2: u16 = 2;

pub const S3: u16 = 3;

pub const NARROW_WIDTH: u16 = 80;

pub const WIDE_WIDTH: u16 = 140;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_constants() {
        assert_eq!(S0, 0);
        assert_eq!(S1, 1);
        assert_eq!(S2, 2);
        assert_eq!(S3, 3);
    }

    #[test]
    fn test_responsive_thresholds() {
        assert_eq!(WIDE_WIDTH, 140);
    }
}
