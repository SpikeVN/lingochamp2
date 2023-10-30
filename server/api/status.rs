pub enum Status {
    /// Running as usual.
    RUNNING,
    /// Skipping the current part and jump on the next part.
    SKIP,
    /// Stopping the current part and returning to the previous one.
    REWIND,
    /// Pause the current part.
    PAUSED,
    /// The show is stopping.
    STOP,
}
