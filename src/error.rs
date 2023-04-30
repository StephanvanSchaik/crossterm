//! This module implements the error type used throughout this crate.

/// The error type.
#[derive(Debug)]
pub enum Error {
    /// Represents [`std::io::Error`].
    Io(std::io::Error),
    CouldNotParseEvent,
    KeyboardEnhancementStatusTimeout,
    CursorPositionTimeout,
    InputReader,
    /// The requested terminal width is too small.
    TerminalWidthTooSmall,
    /// The requested terminal height is too small.
    TerminalHeightTooSmall,
    /// The requested terminal width is too large.
    TerminalWidthTooLarge,
    /// The requested terminal height is too large.
    TerminalHeightTooLarge,
    CursorXOutOfRange(usize),
    CursorYOutOfRange(usize),
    /// Setting an underline color is not supported.
    SetUnderlineColorUnsupported,
    /// Bracketed paste is not supported.
    BracketedPasteUnsupported,
    /// Keyboard progressive enhancement is not supported.
    KeyboardProgressiveEnhancementUnsupported,
    /// This error should only be used by unit tests.
    Test,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::CouldNotParseEvent => write!(f, "could not parse event"),
            Self::KeyboardEnhancementStatusTimeout => write!(
                f,
                "the keyboard enhancement status could not be read within a normal duration"
            ),
            Self::CursorPositionTimeout => write!(
                f,
                "the cursor position could not be read within a normal duration"
            ),
            Self::InputReader => write!(f, "failed to initialize input reader"),
            Self::TerminalWidthTooSmall => write!(f, "terminal width must be at least 1"),
            Self::TerminalHeightTooSmall => write!(f, "terminal height must be at least 1"),
            Self::TerminalWidthTooLarge => write!(f, "terminal width too large"),
            Self::TerminalHeightTooLarge => write!(f, "terminal height too large"),
            Self::CursorXOutOfRange(x) => write!(f, "cursor position X {x} is out of range"),
            Self::CursorYOutOfRange(y) => write!(f, "cursor position Y {y} is out of range"),
            Self::SetUnderlineColorUnsupported => {
                write!(f, "setting the underline color is not supported")
            }
            Self::BracketedPasteUnsupported => write!(f, "bracketed paste is not supported"),
            Self::KeyboardProgressiveEnhancementUnsupported => {
                write!(f, "keyboard progressive enhancement is not supported")
            }
            Self::Test => write!(f, "test"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
