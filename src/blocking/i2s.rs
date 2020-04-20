//! Blocking I2S API

/// Blocking read
pub trait Read<W> {
    /// Error type
    type Error;

    /// Reads enough bytes from the slave to fill `data`.
    ///
    /// The data is filled up with interleaved data as appropriate.
    fn try_read<'w>(&mut self, data: &'w mut [W]) -> Result<(), Self::Error>;
}

/// Blocking write
pub trait Write<W> {
    /// Error type
    type Error;

    /// Sends `data` to the slave.
    ///
    /// The data should be filled with interleaved data as appropriate.
    fn try_write<'w>(&mut self, data: &'w [W]) -> Result<(), Self::Error>;
}

/// Blocking write (iterator version)
pub trait WriteIter<W> {
    /// Error type
    type Error;

    /// Sends `data` to the slave.
    ///
    /// The data should be filled with interleaved data as appropriate.
    fn try_write<I>(&mut self, data: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = W>;
}
