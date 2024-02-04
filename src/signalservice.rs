// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisioningUuid {
    #[prost(string, optional, tag="1")]
    pub uuid: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionEnvelope {
    #[prost(bytes="vec", optional, tag="1")]
    pub public_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Encrypted ProvisionMessage
    #[prost(bytes="vec", optional, tag="2")]
    pub body: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionMessage {
    #[prost(bytes="vec", optional, tag="1")]
    pub aci_identity_key_public: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub aci_identity_key_private: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub pni_identity_key_public: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="12")]
    pub pni_identity_key_private: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="8")]
    pub aci: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub pni: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub provisioning_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub user_agent: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="6")]
    pub profile_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag="7")]
    pub read_receipts: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="9")]
    pub provisioning_version: ::core::option::Option<u32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProvisioningVersion {
    Initial = 0,
    TabletSupport = 1,
}
impl ProvisioningVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProvisioningVersion::Initial => "INITIAL",
            ProvisioningVersion::TabletSupport => "TABLET_SUPPORT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INITIAL" => Some(Self::Initial),
            "TABLET_SUPPORT" => Some(Self::TabletSupport),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceName {
    #[prost(bytes="vec", optional, tag="1")]
    pub ephemeral_public: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub synthetic_iv: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub ciphertext: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCertificate {
    #[prost(bytes="vec", optional, tag="1")]
    pub certificate: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `ServerCertificate`.
pub mod server_certificate {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Certificate {
        #[prost(uint32, optional, tag="1")]
        pub id: ::core::option::Option<u32>,
        #[prost(bytes="vec", optional, tag="2")]
        pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SenderCertificate {
    #[prost(bytes="vec", optional, tag="1")]
    pub certificate: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `SenderCertificate`.
pub mod sender_certificate {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Certificate {
        #[prost(string, optional, tag="1")]
        pub sender_e164: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag="6")]
        pub sender_uuid: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag="2")]
        pub sender_device: ::core::option::Option<u32>,
        #[prost(fixed64, optional, tag="3")]
        pub expires: ::core::option::Option<u64>,
        #[prost(bytes="vec", optional, tag="4")]
        pub identity_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(message, optional, tag="5")]
        pub signer: ::core::option::Option<super::ServerCertificate>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnidentifiedSenderMessage {
    #[prost(bytes="vec", optional, tag="1")]
    pub ephemeral_public: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub encrypted_static: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub encrypted_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `UnidentifiedSenderMessage`.
pub mod unidentified_sender_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        #[prost(enumeration="message::Type", optional, tag="1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(message, optional, tag="2")]
        pub sender_certificate: ::core::option::Option<super::SenderCertificate>,
        #[prost(bytes="vec", optional, tag="3")]
        pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(enumeration="message::ContentHint", optional, tag="4")]
        pub content_hint: ::core::option::Option<i32>,
        #[prost(bytes="vec", optional, tag="5")]
        pub group_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    /// Nested message and enum types in `Message`.
    pub mod message {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            PrekeyMessage = 1,
            /// Further cases should line up with Envelope.Type, even though old cases don't.
            Message = 2,
            // Our parser does not handle reserved in enums: DESKTOP-1569
            // reserved 3 to 6;

            SenderkeyMessage = 7,
            PlaintextContent = 8,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::PrekeyMessage => "PREKEY_MESSAGE",
                    Type::Message => "MESSAGE",
                    Type::SenderkeyMessage => "SENDERKEY_MESSAGE",
                    Type::PlaintextContent => "PLAINTEXT_CONTENT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "PREKEY_MESSAGE" => Some(Self::PrekeyMessage),
                    "MESSAGE" => Some(Self::Message),
                    "SENDERKEY_MESSAGE" => Some(Self::SenderkeyMessage),
                    "PLAINTEXT_CONTENT" => Some(Self::PlaintextContent),
                    _ => None,
                }
            }
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ContentHint {
            /// Show an error immediately; it was important but we can't retry.
            Default = 0,
            /// Sender will try to resend; delay any error UI if possible
            Resendable = 1,
            /// Don't show any error UI at all; this is something sent implicitly like a typing message or a receipt
            Implicit = 2,
        }
        impl ContentHint {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ContentHint::Default => "DEFAULT",
                    ContentHint::Resendable => "RESENDABLE",
                    ContentHint::Implicit => "IMPLICIT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DEFAULT" => Some(Self::Default),
                    "RESENDABLE" => Some(Self::Resendable),
                    "IMPLICIT" => Some(Self::Implicit),
                    _ => None,
                }
            }
        }
    }
}
// @@protoc_insertion_point(module)
