use thiserror::Error;

#[derive(Error, Debug)]
pub enum WledJsonApiError {
    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Error adding port to url, honestly no idea how this happens, should not be possible")]
    UnableToAddPortToURL,
    #[error("Attempted to flush with nothing in internal buffer. either set it through the public data member, or get from the server and (presumably) change something")]
    FlushNone,
    #[error(
        "Attempted to read a key that doesn't exist \
            (either you need to read it from the server, or the server didn't send one)"
    )]
    MissingKey,
}
