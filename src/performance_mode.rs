//! Rider ride mode (`VAL_ 192 PerformanceMode`).

/// Rider performance / ride mode (`VAL_ 192 PerformanceMode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PerformanceMode {
    Rain,
    #[default]
    Std,
    Sport,
    Track,
}

impl PerformanceMode {
    /// Map a raw DBC value onto a mode (unknown values fall back to `Sport`).
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Rain,
            1 => Self::Std,
            3 => Self::Track,
            _ => Self::Sport,
        }
    }

    /// The raw DBC value for this mode.
    pub fn to_raw(self) -> u8 {
        match self {
            Self::Rain => 0,
            Self::Std => 1,
            Self::Sport => 2,
            Self::Track => 3,
        }
    }

    /// The human-readable label (matches the DBC `VAL_` table).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rain => "RAIN",
            Self::Std => "STD",
            Self::Sport => "SPORT",
            Self::Track => "TRACK",
        }
    }

    /// Parse a label back into a mode.
    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "RAIN" => Some(Self::Rain),
            "STD" => Some(Self::Std),
            "SPORT" => Some(Self::Sport),
            "TRACK" => Some(Self::Track),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PerformanceMode;

    #[test]
    fn performance_mode_round_trips() {
        for mode in [
            PerformanceMode::Rain,
            PerformanceMode::Std,
            PerformanceMode::Sport,
            PerformanceMode::Track,
        ] {
            assert_eq!(PerformanceMode::from_raw(mode.to_raw()), mode);
            assert_eq!(PerformanceMode::from_label(mode.as_str()), Some(mode));
        }
    }
}
