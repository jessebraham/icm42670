#[derive(Debug)]
pub enum Error<E> {
    /// Pass on error caused when using the bus itself
    Raw(E),
    /// The chip at the specified address is not reporting the corrcet self
    /// identification code.
    ///
    /// For I²C this is most likely if the ID change jumper is in the wrong
    /// state or there is anther chip on the bus with this address.
    BadChip,
    /// Returned if the register bank is set to a invalid value
    ///
    /// There are 4 banks, 0-3
    BankOutOfRange,
    /// Attempted to write to a read-only register
    WriteToReadOnly,
}

impl<E> From<E> for Error<E> {
    fn from(err: E) -> Self {
        Error::Raw(err)
    }
}
