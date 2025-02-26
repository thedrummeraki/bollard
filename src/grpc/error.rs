#![cfg(feature = "buildkit")]

/// Errors related to the Grpc functionality
#[derive(Debug, thiserror::Error)]
pub enum GrpcError {
    /// Generic error emitted by the bollard codebase.
    #[error(transparent)]
    DockerError {
        /// The original error of the bollard codebase.
        #[from]
        err: crate::errors::Error,
    },
    /// Error emitted when log output generates an I/O error.
    #[error("Invalid UTF-8 string: {}", err)]
    StrParseError {
        /// The original error emitted.
        #[from]
        err: std::str::Utf8Error,
    },
    /// Error emitted when a GRPC network request or response fails with docker's buildkit client
    #[error("Grpc network failure: description = {}", err)]
    TonicError {
        /// The tonic error emitted.
        #[from]
        err: tonic::transport::Error,
    },
    /// Error emitted when a GRPC network request emits a non-OK status code
    #[error("Grpc response failure: status = {}, message = {}", err.code(), err.message())]
    TonicStatus {
        /// The tonic status emitted.
        #[from]
        err: tonic::Status,
    },
    /// Error emitted when a GRPC metadata value does not parse correctly
    #[error("Invalid grpc metadata value: {}", err)]
    MetadataValue {
        /// The tonic metadata value.
        #[from]
        err: tonic::metadata::errors::InvalidMetadataValue,
    },
}

/// Errors related to the Grpc functionality
#[derive(Debug, thiserror::Error)]
pub enum GrpcAuthError {
    /// Error triggered when the registry responds with a status code less than 200 or more than or
    /// equal to 400
    #[error("Registry responded with a non-OK status code: {status_code}")]
    BadRegistryResponse {
        /// The status code responded by the registry
        status_code: u16,
    },
    /// I/O error during request/response with the registry
    #[error("I/O failure during GRPC authentication with registry")]
    IOError {
        /// The i/o error
        #[from]
        err: std::io::Error,
    },
    /// Error emitted from the rustls library during communication with the registry
    #[error("TLS error during GRPC authentication with registry")]
    RustTlsError {
        /// The source error from Rustls
        #[from]
        err: rustls::Error,
    },
    /// Failure to encode query parameters when creating the URL to authenticate with the registry
    #[error("Failure encoding query parameters for GRPC authentication with registry")]
    SerdeUrlEncodedError {
        /// The serde urlencoded error
        #[from]
        err: serde_urlencoded::ser::Error,
    },
    /// Failure to parse the URL when building the URL to authenticate with the registry
    #[error("Failure parsing the registry url for GRPC authentication")]
    UrlParseError {
        /// The original parse error
        #[from]
        err: url::ParseError,
    },
    /// Error while building http headers when calling the registry
    #[error("Invalid HTTP request failure during GRPC authentication with registry")]
    HTTPError {
        /// The original http error
        #[from]
        err: http::Error,
    },
    /// Error emitted by the hyper library during authentication with the registry
    #[error("Hyper error during GRPC authentication with registry")]
    HyperError {
        /// The source hyper error
        #[from]
        err: hyper::Error,
    },
    /// Error while deserializing the payload emitted by the registry
    #[error("Serde payload deserializing error during GRPC authentication")]
    SerdeJsonError {
        /// The source serde json error
        #[from]
        err: serde_json::Error,
    },
    /// Error emitted when the URI is deemed invalid by the http protocol
    #[error("Invalid URI during GRPC authentication with registry")]
    InvalidUriError {
        /// The invalid uri error
        #[from]
        err: http::uri::InvalidUri,
    },
}
