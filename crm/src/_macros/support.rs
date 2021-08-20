use std::fmt::{self, Display};
use std::time::{Duration, Instant};

/// A helper struct that help formatting elapsed duration
pub struct DisplayWithAutoUnit(Duration);

impl Display for DisplayWithAutoUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = self.0;
        match d.as_secs() {
            0 => match d.as_millis() {
                0 => match d.as_micros() {
                    0 => write!(f, "{} ns", d.as_nanos()),
                    n => write!(f, "{} µs", n),
                },
                n => write!(f, "{} ms", n),
            },
            n => write!(f, "{} s", n),
        }
    }
}

impl From<Duration> for DisplayWithAutoUnit {
    fn from(duration: Duration) -> Self {
        DisplayWithAutoUnit(duration)
    }
}

impl From<Instant> for DisplayWithAutoUnit {
    fn from(start: Instant) -> Self {
        DisplayWithAutoUnit(start.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elapsed_duration() {
        let actual = DisplayWithAutoUnit(Duration::from_secs(7)).to_string();
        assert_eq!(actual, "7 s");
        let actual = DisplayWithAutoUnit(Duration::from_millis(15)).to_string();
        assert_eq!(actual, "15 ms");
        let actual = DisplayWithAutoUnit(Duration::from_micros(20)).to_string();
        assert_eq!(actual, "20 µs");
        let actual = DisplayWithAutoUnit(Duration::from_nanos(25)).to_string();
        assert_eq!(actual, "25 ns");
        // this is ok, it is zero anyway
        let actual = DisplayWithAutoUnit(Duration::from_secs(0)).to_string();
        assert_eq!(actual, "0 ns");
    }
}
