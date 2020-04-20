//! I2S - Inter-IC Sound Interface

use nb;

/// Full duplex
pub trait FullDuplex<Word> {
    /// Error type
    type Error;

    /// Reads the word from the slave.
    fn try_read(&mut self) -> nb::Result<Word, Self::Error>;

    /// Sends a word to the slave.
    fn try_send(&mut self, word: Word) -> nb::Result<(), Self::Error>;
}
