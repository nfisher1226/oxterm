use {
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Scrollback {
    Finite(f64),
    Infinite,
}

impl Default for Scrollback {
    fn default() -> Self {
        Self::Finite(1500.0)
    }
}

impl fmt::Display for Scrollback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Finite(n) => write!(f, "{}", *n as i64),
            Self::Infinite => write!(f, "Infinite"),
        }
    }
}
