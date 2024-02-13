use derivative::Derivative;
use libsignal_protocol::SignalProtocolError;
use prost::Message as ProtobufMessage;
use serde::{Deserialize, Serialize};

use crate::libsignal_service::service_address::ParseServiceAddressError;

/// This type is used in registration lock handling.
/// It's identical with HttpAuth, but used to avoid type confusion.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct AuthCredentials {
    pub username: String,
    #[derivative(Debug = "ignore")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationLockFailure {
    pub length: u32,
    pub time_remaining: u64,
    #[serde(rename = "backup_credentials")]
    pub svr1_credentials: Option<AuthCredentials>,
    pub svr2_credentials: Option<AuthCredentials>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofRequired {
    pub token: String,
    pub options: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MismatchedDevices {
    pub missing_devices: Vec<u32>,
    pub extra_devices: Vec<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaleDevices {
    pub stale_devices: Vec<u32>,
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Service request timed out: {reason}")]
    Timeout { reason: String },

    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Error sending request: {reason}")]
    SendError { reason: String },

    #[error("Error decoding response: {reason}")]
    ResponseError { reason: String },

    #[error("Error decoding JSON response: {reason}")]
    JsonDecodeError { reason: String },
    #[error("Error decoding protobuf frame: {0}")]
    ProtobufDecodeError(#[from] prost::DecodeError),
    #[error("error encoding or decoding bincode: {0}")]
    BincodeError(#[from] bincode::Error),
    #[error("error decoding base64 string: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Authorization failed")]
    Unauthorized,
    #[error("Registration lock is set: {0:?}")]
    Locked(RegistrationLockFailure),
    #[error("Unexpected response: HTTP {http_code}")]
    UnhandledResponseCode { http_code: u16 },

    #[error("Websocket error: {reason}")]
    WsError { reason: String },
    #[error("Websocket closing: {reason}")]
    WsClosing { reason: String },

    #[error("Invalid frame: {reason}")]
    InvalidFrameError { reason: String },

    #[error("MAC error")]
    MacError,

    #[error("Protocol error: {0}")]
    SignalProtocolError(#[from] SignalProtocolError),

    #[error("Proof required: {0:?}")]
    ProofRequiredError(ProofRequired),

    #[error("{0:?}")]
    MismatchedDevicesException(MismatchedDevices),

    #[error("{0:?}")]
    StaleDevices(StaleDevices),

    // TODO: group support
    //#[error(transparent)]
    //CredentialsCacheError(#[from] crate::groups_v2::CredentialsCacheError),
    #[error("groups v2 (zero-knowledge) error")]
    GroupsV2Error,

    //#[error(transparent)]
    //GroupsV2DecryptionError(#[from] GroupDecodingError),
    #[error("unsupported content")]
    UnsupportedContent,

    #[error(transparent)]
    ParseServiceAddress(#[from] ParseServiceAddressError),

    #[error("Not found.")]
    NotFoundError,

    #[error("invalid device name")]
    InvalidDeviceName,
}
