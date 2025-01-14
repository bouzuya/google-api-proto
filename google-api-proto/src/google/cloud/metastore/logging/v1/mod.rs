/// Cloud Logging log schema for scheduled backup events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduledBackupLogEntry {
    /// The ID of the backup.
    #[prost(string, tag = "1")]
    pub backup_id: ::prost::alloc::string::String,
    /// The relative resource name of a Metastore service in the form of
    /// `projects/{project_id}/locations/{location_id}/services/{service_id}`
    #[prost(string, tag = "2")]
    pub service: ::prost::alloc::string::String,
    /// Timestamp when the backup was started.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Timestamp when the backup was completed.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the backup.
    #[prost(enumeration = "scheduled_backup_log_entry::State", tag = "5")]
    pub state: i32,
    /// Size of the backup data in bytes.
    #[prost(int64, tag = "6")]
    pub backup_size_bytes: i64,
    /// A Cloud Storage URI of a folder, in the format
    /// `gs://<bucket_name>/<path_inside_bucket>`.
    #[prost(string, tag = "7")]
    pub backup_location: ::prost::alloc::string::String,
    /// Message that provides (optional) details about the backup.
    #[prost(string, tag = "8")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ScheduledBackupLogEntry`.
pub mod scheduled_backup_log_entry {
    /// The current state of the backup.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The state of the backup is unknown.
        Unspecified = 0,
        /// The backup is in progress.
        InProgress = 1,
        /// The backup completed.
        Succeeded = 2,
        /// The backup failed.
        Failed = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::InProgress => "IN_PROGRESS",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Stackdriver structured-payload for events generated from Hive Metastore
/// API requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestsLogEntry {
    /// A free-text string describing the request.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Stackdriver structured-payload for events generated from Hive Metastore
/// system activity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemActivityLogEntry {
    /// A free-text string describing the system activity.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
