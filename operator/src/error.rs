#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Io Error creating bundle: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Object [{obj}] needs to be namespaced!")]
    NamespaceError { obj: String },
}
