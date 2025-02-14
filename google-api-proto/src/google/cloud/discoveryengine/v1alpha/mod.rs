/// A floating point interval.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interval {
    /// The lower bound of the interval. If neither of the min fields are
    /// set, then the lower bound is negative infinity.
    ///
    /// This field must be not larger than max.
    /// Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(oneof = "interval::Min", tags = "1, 2")]
    pub min: ::core::option::Option<interval::Min>,
    /// The upper bound of the interval. If neither of the max fields are
    /// set, then the upper bound is positive infinity.
    ///
    /// This field must be not smaller than min.
    /// Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(oneof = "interval::Max", tags = "3, 4")]
    pub max: ::core::option::Option<interval::Max>,
}
/// Nested message and enum types in `Interval`.
pub mod interval {
    /// The lower bound of the interval. If neither of the min fields are
    /// set, then the lower bound is negative infinity.
    ///
    /// This field must be not larger than max.
    /// Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Min {
        /// Inclusive lower bound.
        #[prost(double, tag = "1")]
        Minimum(f64),
        /// Exclusive lower bound.
        #[prost(double, tag = "2")]
        ExclusiveMinimum(f64),
    }
    /// The upper bound of the interval. If neither of the max fields are
    /// set, then the upper bound is positive infinity.
    ///
    /// This field must be not smaller than min.
    /// Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Max {
        /// Inclusive upper bound.
        #[prost(double, tag = "3")]
        Maximum(f64),
        /// Exclusive upper bound.
        #[prost(double, tag = "4")]
        ExclusiveMaximum(f64),
    }
}
/// A custom attribute that is not explicitly modeled in a resource, e.g.
/// [UserEvent][google.cloud.discoveryengine.v1alpha.UserEvent].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The textual values of this custom attribute. For example, `["yellow",
    /// "green"]` when the key is "color".
    ///
    /// Empty string is not allowed. Otherwise, an `INVALID_ARGUMENT` error is
    /// returned.
    ///
    /// Exactly one of
    /// [CustomAttribute.text][google.cloud.discoveryengine.v1alpha.CustomAttribute.text]
    /// or
    /// [CustomAttribute.numbers][google.cloud.discoveryengine.v1alpha.CustomAttribute.numbers]
    /// should be set. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, repeated, tag = "1")]
    pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The numerical values of this custom attribute. For example, `\[2.3, 15.4\]`
    /// when the key is "lengths_cm".
    ///
    /// Exactly one of
    /// [CustomAttribute.text][google.cloud.discoveryengine.v1alpha.CustomAttribute.text]
    /// or
    /// [CustomAttribute.numbers][google.cloud.discoveryengine.v1alpha.CustomAttribute.numbers]
    /// should be set. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(double, repeated, tag = "2")]
    pub numbers: ::prost::alloc::vec::Vec<f64>,
}
/// Information of an end user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// Highly recommended for logged-in users. Unique identifier for logged-in
    /// user, such as a user name. Don't set for anonymous users.
    ///
    /// Always use a hashed value for this ID.
    ///
    /// Don't set the field to the same fixed ID for different users. This mixes
    /// the event history of those users together, which results in degraded
    /// model quality.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// User agent as included in the HTTP header.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    ///
    /// This should not be set when using the client side event reporting with
    /// GTM or JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.discoveryengine.v1alpha.UserEventService.CollectUserEvent]
    /// or if
    /// [UserEvent.direct_user_request][google.cloud.discoveryengine.v1alpha.UserEvent.direct_user_request]
    /// is set.
    #[prost(string, tag = "2")]
    pub user_agent: ::prost::alloc::string::String,
}
/// Double list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleList {
    /// Double values.
    #[prost(double, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f64>,
}
/// The industry vertical associated with the
/// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndustryVertical {
    /// Value used when unset.
    Unspecified = 0,
    /// The generic vertical for documents that are not specific to any industry
    /// vertical.
    Generic = 1,
    /// The media industry vertical.
    Media = 2,
}
impl IndustryVertical {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IndustryVertical::Unspecified => "INDUSTRY_VERTICAL_UNSPECIFIED",
            IndustryVertical::Generic => "GENERIC",
            IndustryVertical::Media => "MEDIA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INDUSTRY_VERTICAL_UNSPECIFIED" => Some(Self::Unspecified),
            "GENERIC" => Some(Self::Generic),
            "MEDIA" => Some(Self::Media),
            _ => None,
        }
    }
}
/// The type of solution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SolutionType {
    /// Default value.
    Unspecified = 0,
    /// Used for Recommendations AI.
    Recommendation = 1,
    /// Used for Discovery Search.
    Search = 2,
    /// Used for use cases related to the Generative AI agent.
    Chat = 3,
}
impl SolutionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SolutionType::Unspecified => "SOLUTION_TYPE_UNSPECIFIED",
            SolutionType::Recommendation => "SOLUTION_TYPE_RECOMMENDATION",
            SolutionType::Search => "SOLUTION_TYPE_SEARCH",
            SolutionType::Chat => "SOLUTION_TYPE_CHAT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SOLUTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SOLUTION_TYPE_RECOMMENDATION" => Some(Self::Recommendation),
            "SOLUTION_TYPE_SEARCH" => Some(Self::Search),
            "SOLUTION_TYPE_CHAT" => Some(Self::Chat),
            _ => None,
        }
    }
}
/// Tiers of search features. Different tiers might have different
/// pricing. To learn more, please check the pricing documentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchTier {
    /// Default value when the enum is unspecified. This is invalid to use.
    Unspecified = 0,
    /// Standard tier.
    Standard = 1,
    /// Enterprise tier.
    Enterprise = 2,
}
impl SearchTier {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SearchTier::Unspecified => "SEARCH_TIER_UNSPECIFIED",
            SearchTier::Standard => "SEARCH_TIER_STANDARD",
            SearchTier::Enterprise => "SEARCH_TIER_ENTERPRISE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEARCH_TIER_UNSPECIFIED" => Some(Self::Unspecified),
            "SEARCH_TIER_STANDARD" => Some(Self::Standard),
            "SEARCH_TIER_ENTERPRISE" => Some(Self::Enterprise),
            _ => None,
        }
    }
}
/// Add-on that provides additional functionality for search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchAddOn {
    /// Default value when the enum is unspecified. This is invalid to use.
    Unspecified = 0,
    /// Large language model add-on.
    Llm = 1,
}
impl SearchAddOn {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SearchAddOn::Unspecified => "SEARCH_ADD_ON_UNSPECIFIED",
            SearchAddOn::Llm => "SEARCH_ADD_ON_LLM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEARCH_ADD_ON_UNSPECIFIED" => Some(Self::Unspecified),
            "SEARCH_ADD_ON_LLM" => Some(Self::Llm),
            _ => None,
        }
    }
}
/// Metadata that describes the training and serving parameters of an
/// [Engine][google.cloud.discoveryengine.v1alpha.Engine].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Engine {
    /// Immutable. The fully qualified resource name of the engine.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1024
    /// characters.
    ///
    /// Format:
    /// `projects/{project_number}/locations/{location}/collections/{collection}/engines/{engine}`
    /// engine should be 1-63 characters, and valid characters are
    /// /[a-z0-9][a-z0-9-_]*/. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the engine. Should be human readable. UTF-8
    /// encoded string with limit of 1024 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Timestamp the Recommendation Engine was created at.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp the Recommendation Engine was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The data stores associated with this engine.
    ///
    /// For
    /// [SOLUTION_TYPE_SEARCH][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_SEARCH]
    /// and
    /// [SOLUTION_TYPE_RECOMMENDATION][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_RECOMMENDATION]
    /// type of engines, they can only associate with at most one data store.
    ///
    /// If
    /// [solution_type][google.cloud.discoveryengine.v1alpha.Engine.solution_type]
    /// is
    /// [SOLUTION_TYPE_CHAT][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_CHAT],
    /// multiple [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]s in
    /// the same [Collection][google.cloud.discoveryengine.v1alpha.Collection] can
    /// be associated here.
    ///
    /// Note that when used in
    /// [CreateEngineRequest][google.cloud.discoveryengine.v1alpha.CreateEngineRequest],
    /// one DataStore id must be provided as the system will use it for necessary
    /// intializations.
    #[prost(string, repeated, tag = "5")]
    pub data_store_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The solutions of the engine.
    #[prost(enumeration = "SolutionType", tag = "6")]
    pub solution_type: i32,
    /// The industry vertical that the engine registers.
    /// The restriction of the Engine industry vertical is based on
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]: If
    /// unspecified, default to `GENERIC`. Vertical on Engine has to match vertical
    /// of the DataStore liniked to the engine.
    #[prost(enumeration = "IndustryVertical", tag = "16")]
    pub industry_vertical: i32,
    /// Common config spec that specifies the metadata of the engine.
    #[prost(message, optional, tag = "15")]
    pub common_config: ::core::option::Option<engine::CommonConfig>,
    /// Additional config specs that defines the behavior of the engine.
    #[prost(oneof = "engine::EngineConfig", tags = "9, 11, 13, 14")]
    pub engine_config: ::core::option::Option<engine::EngineConfig>,
    /// Engine metadata to monitor the status of the engine.
    #[prost(oneof = "engine::EngineMetadata", tags = "10, 12")]
    pub engine_metadata: ::core::option::Option<engine::EngineMetadata>,
}
/// Nested message and enum types in `Engine`.
pub mod engine {
    /// Configurations for a Search Engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SearchEngineConfig {
        /// The search feature tier of this engine.
        ///
        /// Different tiers might have different
        /// pricing. To learn more, please check the pricing documentation.
        ///
        /// Defaults to
        /// [SearchTier.SEARCH_TIER_STANDARD][google.cloud.discoveryengine.v1alpha.SearchTier.SEARCH_TIER_STANDARD]
        /// if not specified.
        #[prost(enumeration = "super::SearchTier", tag = "1")]
        pub search_tier: i32,
        /// The add-on that this search engine enables.
        #[prost(enumeration = "super::SearchAddOn", repeated, tag = "2")]
        pub search_add_ons: ::prost::alloc::vec::Vec<i32>,
    }
    /// Additional config specs for a `similar-items` engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SimilarDocumentsEngineConfig {}
    /// Additional config specs for a Media Recommendation engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MediaRecommendationEngineConfig {
        /// Required. The type of engine e.g. `recommended-for-you`.
        ///
        /// This field together with
        /// [optimization_objective][Engine.optimization_objective] describe engine
        /// metadata to use to control engine training and serving.
        ///
        /// Currently supported values: `recommended-for-you`, `others-you-may-like`,
        /// `more-like-this`, `most-popular-items`.
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        /// The optimization objective e.g. `cvr`.
        ///
        /// This field together with
        /// [optimization_objective][google.cloud.discoveryengine.v1alpha.Engine.MediaRecommendationEngineConfig.type]
        /// describe engine metadata to use to control engine training and serving.
        ///
        /// Currently supported
        /// values: `ctr`, `cvr`.
        ///
        ///   If not specified, we choose default based on engine type.
        /// Default depends on type of recommendation:
        ///
        /// `recommended-for-you` => `ctr`
        ///
        /// `others-you-may-like` => `ctr`
        #[prost(string, tag = "2")]
        pub optimization_objective: ::prost::alloc::string::String,
        /// Name and value of the custom threshold for cvr optimization_objective.
        /// For target_field `watch-time`, target_field_value must be an integer
        /// value indicating the media progress time in seconds between (0, 86400]
        /// (excludes 0, includes 86400) (e.g., 90).
        /// For target_field `watch-percentage`, the target_field_value must be a
        /// valid float value between (0, 1.0] (excludes 0, includes 1.0) (e.g.,
        /// 0.5).
        #[prost(message, optional, tag = "3")]
        pub optimization_objective_config: ::core::option::Option<
            media_recommendation_engine_config::OptimizationObjectiveConfig,
        >,
        /// The training state that the engine is in (e.g.
        /// `TRAINING` or `PAUSED`).
        ///
        /// Since part of the cost of running the service
        /// is frequency of training - this can be used to determine when to train
        /// engine in order to control cost. If not specified: the default value for
        /// `CreateEngine` method is `TRAINING`. The default value for
        /// `UpdateEngine` method is to keep the state the same as before.
        #[prost(
            enumeration = "media_recommendation_engine_config::TrainingState",
            tag = "4"
        )]
        pub training_state: i32,
    }
    /// Nested message and enum types in `MediaRecommendationEngineConfig`.
    pub mod media_recommendation_engine_config {
        /// Custom threshold for `cvr` optimization_objective.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct OptimizationObjectiveConfig {
            /// Required. The name of the field to target. Currently supported
            /// values: `watch-percentage`, `watch-time`.
            #[prost(string, tag = "1")]
            pub target_field: ::prost::alloc::string::String,
            /// Required. The threshold to be applied to the target (e.g., 0.5).
            #[prost(float, tag = "2")]
            pub target_field_value_float: f32,
        }
        /// The training state of the engine.
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
        pub enum TrainingState {
            /// Unspecified training state.
            Unspecified = 0,
            /// The engine training is paused.
            Paused = 1,
            /// The engine is training.
            Training = 2,
        }
        impl TrainingState {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    TrainingState::Unspecified => "TRAINING_STATE_UNSPECIFIED",
                    TrainingState::Paused => "PAUSED",
                    TrainingState::Training => "TRAINING",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TRAINING_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "PAUSED" => Some(Self::Paused),
                    "TRAINING" => Some(Self::Training),
                    _ => None,
                }
            }
        }
    }
    /// Configurations for a Chat Engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChatEngineConfig {
        /// The configurationt generate the Dialogflow agent that is associated to
        /// this Engine.
        ///
        /// Note that these configurations are one-time consumed by
        /// and passed to Dialogflow service. It means they cannot be retrieved using
        /// [EngineService.GetEngine][google.cloud.discoveryengine.v1alpha.EngineService.GetEngine]
        /// or
        /// [EngineService.ListEngines][google.cloud.discoveryengine.v1alpha.EngineService.ListEngines]
        /// API after engine creation.
        #[prost(message, optional, tag = "1")]
        pub agent_creation_config: ::core::option::Option<
            chat_engine_config::AgentCreationConfig,
        >,
        /// The resource name of an exist Dialogflow agent to link to this Chat
        /// Engine. Customers can either provide `agent_creation_config` to create
        /// agent or provide an agent name that links the agent with the Chat engine.
        ///
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>`.
        ///
        /// Note that the `dialogflow_agent_to_link` are one-time consumed by and
        /// passed to Dialogflow service. It means they cannot be retrieved using
        /// [EngineService.GetEngine][google.cloud.discoveryengine.v1alpha.EngineService.GetEngine]
        /// or
        /// [EngineService.ListEngines][google.cloud.discoveryengine.v1alpha.EngineService.ListEngines]
        /// API after engine creation. Please use
        /// [chat_engine_metadata.dialogflow_agent][] for actual agent
        /// association after Engine is created.
        #[prost(string, tag = "2")]
        pub dialogflow_agent_to_link: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `ChatEngineConfig`.
    pub mod chat_engine_config {
        /// Configurations for generating a Dialogflow agent.
        ///
        /// Note that these configurations are one-time consumed by
        /// and passed to Dialogflow service. It means they cannot be retrieved using
        /// [EngineService.GetEngine][google.cloud.discoveryengine.v1alpha.EngineService.GetEngine]
        /// or
        /// [EngineService.ListEngines][google.cloud.discoveryengine.v1alpha.EngineService.ListEngines]
        /// API after engine creation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AgentCreationConfig {
            /// Name of the company, organization or other entity that the agent
            /// represents. Used for knowledge connector LLM prompt and for knowledge
            /// search.
            #[prost(string, tag = "1")]
            pub business: ::prost::alloc::string::String,
            /// Required. The default language of the agent as a language tag.
            /// See [Language
            /// Support](<https://cloud.google.com/dialogflow/docs/reference/language>)
            /// for a list of the currently supported language codes.
            #[prost(string, tag = "2")]
            pub default_language_code: ::prost::alloc::string::String,
            /// Required. The time zone of the agent from the [time zone
            /// database](<https://www.iana.org/time-zones>), e.g., America/New_York,
            /// Europe/Paris.
            #[prost(string, tag = "3")]
            pub time_zone: ::prost::alloc::string::String,
        }
    }
    /// Common configurations for an Engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommonConfig {
        /// The name of the company, business or entity that is associated with the
        /// engine. Setting this may help improve LLM related features.
        #[prost(string, tag = "1")]
        pub company_name: ::prost::alloc::string::String,
    }
    /// Additional information of a recommendation engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecommendationMetadata {
        /// Output only. The serving state of the engine: `ACTIVE`, `NOT_ACTIVE`.
        #[prost(enumeration = "recommendation_metadata::ServingState", tag = "1")]
        pub serving_state: i32,
        /// Output only. The state of data requirements for this engine: `DATA_OK`
        /// and `DATA_ERROR`.
        ///
        /// Engine cannot be trained if the data is in
        /// `DATA_ERROR` state. Engine can have `DATA_ERROR` state even
        /// if serving state is `ACTIVE`: engines were trained successfully before,
        /// but cannot be refreshed because the underlying engine no longer has
        /// sufficient data for training.
        #[prost(enumeration = "recommendation_metadata::DataState", tag = "2")]
        pub data_state: i32,
        /// Output only. The timestamp when the latest successful tune finished. Only
        /// applicable on Media Recommendation engines.
        #[prost(message, optional, tag = "3")]
        pub last_tune_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. The latest tune operation id associated with the engine.
        /// Only applicable on Media Recommendation engines.
        ///
        /// If present, this operation id can be used to determine if there is an
        /// ongoing tune for this engine. To check the operation status, send the
        /// GetOperation request with this operation id in the engine resource
        /// format. If no tuning has happened for this engine, the string is empty.
        #[prost(string, tag = "4")]
        pub tuning_operation: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `RecommendationMetadata`.
    pub mod recommendation_metadata {
        /// The serving state of the recommendation engine.
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
        pub enum ServingState {
            /// Unspecified serving state.
            Unspecified = 0,
            /// The engine is not serving.
            Inactive = 1,
            /// The engine is serving and can be queried.
            Active = 2,
            /// The engine is trained on tuned hyperparameters and can be
            /// queried.
            Tuned = 3,
        }
        impl ServingState {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ServingState::Unspecified => "SERVING_STATE_UNSPECIFIED",
                    ServingState::Inactive => "INACTIVE",
                    ServingState::Active => "ACTIVE",
                    ServingState::Tuned => "TUNED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SERVING_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "INACTIVE" => Some(Self::Inactive),
                    "ACTIVE" => Some(Self::Active),
                    "TUNED" => Some(Self::Tuned),
                    _ => None,
                }
            }
        }
        /// Describes whether this engine have sufficient training data
        /// to be continuously trained.
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
        pub enum DataState {
            /// Unspecified default value, should never be explicitly set.
            Unspecified = 0,
            /// The engine has sufficient training data.
            DataOk = 1,
            /// The engine does not have sufficient training data. Error
            /// messages can be queried via Stackdriver.
            DataError = 2,
        }
        impl DataState {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DataState::Unspecified => "DATA_STATE_UNSPECIFIED",
                    DataState::DataOk => "DATA_OK",
                    DataState::DataError => "DATA_ERROR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DATA_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                    "DATA_OK" => Some(Self::DataOk),
                    "DATA_ERROR" => Some(Self::DataError),
                    _ => None,
                }
            }
        }
    }
    /// Additional information of a Chat Engine.
    /// Fields in this message are output only.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChatEngineMetadata {
        /// The resource name of a Dialogflow agent, that this Chat Engine refers
        /// to.
        ///
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>`.
        #[prost(string, tag = "1")]
        pub dialogflow_agent: ::prost::alloc::string::String,
    }
    /// Additional config specs that defines the behavior of the engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EngineConfig {
        /// Additional config specs for a `similar-items` engine.
        #[prost(message, tag = "9")]
        SimilarDocumentsConfig(SimilarDocumentsEngineConfig),
        /// Configurations for the Chat Engine. Only applicable if
        /// [solution_type][google.cloud.discoveryengine.v1alpha.Engine.solution_type]
        /// is
        /// [SOLUTION_TYPE_CHAT][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_CHAT].
        #[prost(message, tag = "11")]
        ChatEngineConfig(ChatEngineConfig),
        /// Configurations for the Search Engine. Only applicable if
        /// [solution_type][google.cloud.discoveryengine.v1alpha.Engine.solution_type]
        /// is
        /// [SOLUTION_TYPE_SEARCH][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_SEARCH].
        #[prost(message, tag = "13")]
        SearchEngineConfig(SearchEngineConfig),
        /// Configurations for the Media Engine. Only applicable on the data
        /// stores with
        /// [solution_type][google.cloud.discoveryengine.v1alpha.Engine.solution_type]
        /// [SOLUTION_TYPE_RECOMMENDATION][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_RECOMMENDATION]
        /// and
        /// [IndustryVertical.MEDIA][google.cloud.discoveryengine.v1alpha.IndustryVertical.MEDIA]
        /// vertical.
        #[prost(message, tag = "14")]
        MediaRecommendationEngineConfig(MediaRecommendationEngineConfig),
    }
    /// Engine metadata to monitor the status of the engine.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EngineMetadata {
        /// Output only. Additional information of a recommendation engine. Only
        /// applicable if
        /// [solution_type][google.cloud.discoveryengine.v1alpha.Engine.solution_type]
        /// is
        /// [SOLUTION_TYPE_RECOMMENDATION][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_RECOMMENDATION].
        #[prost(message, tag = "10")]
        RecommendationMetadata(RecommendationMetadata),
        /// Output only. Additional information of the Chat Engine. Only applicable
        /// if
        /// [solution_type][google.cloud.discoveryengine.v1alpha.Engine.solution_type]
        /// is
        /// [SOLUTION_TYPE_CHAT][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_CHAT].
        #[prost(message, tag = "12")]
        ChatEngineMetadata(ChatEngineMetadata),
    }
}
/// Request for
/// [EngineService.CreateEngine][google.cloud.discoveryengine.v1alpha.EngineService.CreateEngine]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEngineRequest {
    /// Required. The parent resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Engine][google.cloud.discoveryengine.v1alpha.Engine] to
    /// create.
    #[prost(message, optional, tag = "2")]
    pub engine: ::core::option::Option<Engine>,
    /// Required. The ID to use for the
    /// [Engine][google.cloud.discoveryengine.v1alpha.Engine], which will become
    /// the final component of the
    /// [Engine][google.cloud.discoveryengine.v1alpha.Engine]'s resource name.
    ///
    /// This field must conform to [RFC-1034](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub engine_id: ::prost::alloc::string::String,
}
/// Metadata related to the progress of the
/// [EngineService.CreateEngine][google.cloud.discoveryengine.v1alpha.EngineService.CreateEngine]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEngineMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// [EngineService.DeleteEngine][google.cloud.discoveryengine.v1alpha.EngineService.DeleteEngine]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEngineRequest {
    /// Required. Full resource name of
    /// [Engine][google.cloud.discoveryengine.v1alpha.Engine], such as
    /// `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`.
    ///
    /// If the caller does not have permission to delete the
    /// [Engine][google.cloud.discoveryengine.v1alpha.Engine], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Engine][google.cloud.discoveryengine.v1alpha.Engine] to delete does
    /// not exist, a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata related to the progress of the
/// [EngineService.DeleteEngine][google.cloud.discoveryengine.v1alpha.EngineService.DeleteEngine]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEngineMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// [EngineService.GetEngine][google.cloud.discoveryengine.v1alpha.EngineService.GetEngine]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEngineRequest {
    /// Required. Full resource name of
    /// [Engine][google.cloud.discoveryengine.v1alpha.Engine], such as
    /// `projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [EngineService.ListEngines][google.cloud.discoveryengine.v1alpha.EngineService.ListEngines]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnginesRequest {
    /// Required. The parent resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Not supported.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Not supported.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter by solution type. For example:
    /// solution_type=SOLUTION_TYPE_SEARCH
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for
/// [EngineService.ListEngines][google.cloud.discoveryengine.v1alpha.EngineService.ListEngines]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnginesResponse {
    /// All the customer's [Engine][google.cloud.discoveryengine.v1alpha.Engine]s.
    #[prost(message, repeated, tag = "1")]
    pub engines: ::prost::alloc::vec::Vec<Engine>,
    /// Not supported.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [EngineService.UpdateEngine][google.cloud.discoveryengine.v1alpha.EngineService.UpdateEngine]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEngineRequest {
    /// Required. The [Engine][google.cloud.discoveryengine.v1alpha.Engine] to
    /// update.
    ///
    /// If the caller does not have permission to update the
    /// [Engine][google.cloud.discoveryengine.v1alpha.Engine], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Engine][google.cloud.discoveryengine.v1alpha.Engine] to update does
    /// not exist, a NOT_FOUND error is returned.
    #[prost(message, optional, tag = "1")]
    pub engine: ::core::option::Option<Engine>,
    /// Indicates which fields in the provided
    /// [Engine][google.cloud.discoveryengine.v1alpha.Engine] to update.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for pausing training of an engine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseEngineRequest {
    /// Required. The name of the engine to pause.
    /// Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection_id}/engines/{engine_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for resuming training of an engine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeEngineRequest {
    /// Required. The name of the engine to resume.
    /// Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection_id}/engines/{engine_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to manually start a tuning process now (instead of waiting for
/// the periodically scheduled tuning to happen).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TuneEngineRequest {
    /// Required. The resource name of the engine to tune.
    /// Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection_id}/engines/{engine_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata associated with a tune operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TuneEngineMetadata {
    /// Required. The resource name of the engine that this tune applies to.
    /// Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection_id}/engines/{engine_id}`
    #[prost(string, tag = "1")]
    pub engine: ::prost::alloc::string::String,
}
/// Response associated with a tune operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TuneEngineResponse {}
/// Generated client implementations.
pub mod engine_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Engine][google.cloud.discoveryengine.v1alpha.Engine]
    /// configuration.
    #[derive(Debug, Clone)]
    pub struct EngineServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EngineServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> EngineServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            EngineServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Creates a [Engine][google.cloud.discoveryengine.v1alpha.Engine].
        pub async fn create_engine(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEngineRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.EngineService/CreateEngine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.EngineService",
                        "CreateEngine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a [Engine][google.cloud.discoveryengine.v1alpha.Engine].
        pub async fn delete_engine(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEngineRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.EngineService/DeleteEngine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.EngineService",
                        "DeleteEngine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an [Engine][google.cloud.discoveryengine.v1alpha.Engine]
        pub async fn update_engine(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEngineRequest>,
        ) -> std::result::Result<tonic::Response<super::Engine>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.EngineService/UpdateEngine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.EngineService",
                        "UpdateEngine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a [Engine][google.cloud.discoveryengine.v1alpha.Engine].
        pub async fn get_engine(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEngineRequest>,
        ) -> std::result::Result<tonic::Response<super::Engine>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.EngineService/GetEngine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.EngineService",
                        "GetEngine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all the [Engine][google.cloud.discoveryengine.v1alpha.Engine]s
        /// associated with the project.
        pub async fn list_engines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEnginesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEnginesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.EngineService/ListEngines",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.EngineService",
                        "ListEngines",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Pauses the training of an existing engine. Only applicable if
        /// [solution_type][] is
        /// [SOLUTION_TYPE_RECOMMENDATION][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_RECOMMENDATION].
        pub async fn pause_engine(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseEngineRequest>,
        ) -> std::result::Result<tonic::Response<super::Engine>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.EngineService/PauseEngine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.EngineService",
                        "PauseEngine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resumes the training of an existing engine. Only applicable if
        /// [SolutionType][google.cloud.discoveryengine.v1alpha.SolutionType] is
        /// [SOLUTION_TYPE_RECOMMENDATION][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_RECOMMENDATION].
        pub async fn resume_engine(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeEngineRequest>,
        ) -> std::result::Result<tonic::Response<super::Engine>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.EngineService/ResumeEngine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.EngineService",
                        "ResumeEngine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Tunes an existing engine. Only applicable if [solution_type][] is
        /// [SOLUTION_TYPE_RECOMMENDATION][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_RECOMMENDATION].
        pub async fn tune_engine(
            &mut self,
            request: impl tonic::IntoRequest<super::TuneEngineRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.EngineService/TuneEngine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.EngineService",
                        "TuneEngine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Defines the structure and layout of a type of document data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// Immutable. The full resource name of the schema, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1024
    /// characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Configurations for fields of the schema.
    #[prost(message, repeated, tag = "4")]
    pub field_configs: ::prost::alloc::vec::Vec<FieldConfig>,
    /// Schema representation. One of
    /// [struct_schema][google.cloud.discoveryengine.v1alpha.Schema.struct_schema]
    /// or [json_schema][google.cloud.discoveryengine.v1alpha.Schema.json_schema]
    /// should be provided otherwise an `INVALID_ARGUMENT` error is thrown.
    #[prost(oneof = "schema::Schema", tags = "2, 3")]
    pub schema: ::core::option::Option<schema::Schema>,
}
/// Nested message and enum types in `Schema`.
pub mod schema {
    /// Schema representation. One of
    /// [struct_schema][google.cloud.discoveryengine.v1alpha.Schema.struct_schema]
    /// or [json_schema][google.cloud.discoveryengine.v1alpha.Schema.json_schema]
    /// should be provided otherwise an `INVALID_ARGUMENT` error is thrown.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// The structured representation of the schema.
        #[prost(message, tag = "2")]
        StructSchema(::prost_types::Struct),
        /// The JSON representation of the schema.
        #[prost(string, tag = "3")]
        JsonSchema(::prost::alloc::string::String),
    }
}
/// Configurations for fields of a schema. For example, configuring a field is
/// indexable, or searchable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldConfig {
    /// Required. Field path of the schema field.
    /// For example: `title`, `description`, `release_info.release_year`.
    #[prost(string, tag = "1")]
    pub field_path: ::prost::alloc::string::String,
    /// Output only. Raw type of the field.
    #[prost(enumeration = "field_config::FieldType", tag = "2")]
    pub field_type: i32,
    /// If
    /// [indexable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.indexable_option]
    /// is
    /// [INDEXABLE_ENABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.IndexableOption.INDEXABLE_ENABLED],
    /// field values are indexed so that it can be filtered or faceted in
    /// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search].
    ///
    /// If
    /// [indexable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.indexable_option]
    /// is unset, the server behavior defaults to
    /// [INDEXABLE_DISABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.IndexableOption.INDEXABLE_DISABLED]
    /// for fields that support setting indexable options. For those fields that do
    /// not support setting indexable options, such as `object` and `boolean` and
    /// key properties, the server will skip
    /// [indexable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.indexable_option]
    /// setting, and setting
    /// [indexable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.indexable_option]
    /// for those fields will throw `INVALID_ARGUMENT` error.
    #[prost(enumeration = "field_config::IndexableOption", tag = "3")]
    pub indexable_option: i32,
    /// If
    /// [dynamic_facetable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.dynamic_facetable_option]
    /// is
    /// [DYNAMIC_FACETABLE_ENABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.DynamicFacetableOption.DYNAMIC_FACETABLE_ENABLED],
    /// field values are available for dynamic facet. Could only be
    /// [DYNAMIC_FACETABLE_DISABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.DynamicFacetableOption.DYNAMIC_FACETABLE_DISABLED]
    /// if
    /// [FieldConfig.indexable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.indexable_option]
    /// is
    /// [INDEXABLE_DISABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.IndexableOption.INDEXABLE_DISABLED].
    /// Otherwise, an `INVALID_ARGUMENT` error will be returned.
    ///
    /// If
    /// [dynamic_facetable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.dynamic_facetable_option]
    /// is unset, the server behavior defaults to
    /// [DYNAMIC_FACETABLE_DISABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.DynamicFacetableOption.DYNAMIC_FACETABLE_DISABLED]
    /// for fields that support setting dynamic facetable options. For those fields
    /// that do not support setting dynamic facetable options, such as `object` and
    /// `boolean`, the server will skip dynamic facetable option setting, and
    /// setting
    /// [dynamic_facetable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.dynamic_facetable_option]
    /// for those fields will throw `INVALID_ARGUMENT` error.
    #[prost(enumeration = "field_config::DynamicFacetableOption", tag = "4")]
    pub dynamic_facetable_option: i32,
    /// If
    /// [searchable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.searchable_option]
    /// is
    /// [SEARCHABLE_ENABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.SearchableOption.SEARCHABLE_ENABLED],
    /// field values are searchable by text queries in
    /// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search].
    ///
    /// If
    /// [SEARCHABLE_ENABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.SearchableOption.SEARCHABLE_ENABLED]
    /// but field type is numerical, field values will not be searchable by text
    /// queries in
    /// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search],
    /// as there are no text values associated to numerical fields.
    ///
    /// If
    /// [searchable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.searchable_option]
    /// is unset, the server behavior defaults to
    /// [SEARCHABLE_DISABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.SearchableOption.SEARCHABLE_DISABLED]
    /// for fields that support setting searchable options. Only `string` fields
    /// that have no key property mapping support setting
    /// [searchable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.searchable_option].
    ///
    /// For those fields that do not support setting searchable options, the server
    /// will skip searchable option setting, and setting
    /// [searchable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.searchable_option]
    /// for those fields will throw `INVALID_ARGUMENT` error.
    #[prost(enumeration = "field_config::SearchableOption", tag = "5")]
    pub searchable_option: i32,
    /// If
    /// [retrievable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.retrievable_option]
    /// is
    /// [RETRIEVABLE_ENABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.RetrievableOption.RETRIEVABLE_ENABLED],
    /// field values are included in the search results.
    ///
    /// If
    /// [retrievable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.retrievable_option]
    /// is unset, the server behavior defaults to
    /// [RETRIEVABLE_DISABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.RetrievableOption.RETRIEVABLE_DISABLED]
    /// for fields that support setting retrievable options. For those fields
    /// that do not support setting retrievable options, such as `object` and
    /// `boolean`, the server will skip retrievable option setting, and setting
    /// [retrievable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.retrievable_option]
    /// for those fields will throw `INVALID_ARGUMENT` error.
    #[prost(enumeration = "field_config::RetrievableOption", tag = "6")]
    pub retrievable_option: i32,
    /// If
    /// [completable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.completable_option]
    /// is
    /// [COMPLETABLE_ENABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.CompletableOption.COMPLETABLE_ENABLED],
    /// field values are directly used and returned as suggestions for Autocomplete
    /// in
    /// [CompletionService.CompleteQuery][google.cloud.discoveryengine.v1alpha.CompletionService.CompleteQuery].
    ///
    /// If
    /// [completable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.completable_option]
    /// is unset, the server behavior defaults to
    /// [COMPLETABLE_DISABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.CompletableOption.COMPLETABLE_DISABLED]
    /// for fields that support setting completable options, which are just
    /// `string` fields. For those fields that do not support setting completable
    /// options, the server will skip completable option setting, and setting
    /// [completable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.completable_option]
    /// for those fields will throw `INVALID_ARGUMENT` error.
    #[prost(enumeration = "field_config::CompletableOption", tag = "8")]
    pub completable_option: i32,
    /// If
    /// [recs_filterable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.recs_filterable_option]
    /// is
    /// [FILTERABLE_ENABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.FilterableOption.FILTERABLE_ENABLED],
    /// field values are filterable by filter expression in
    /// [RecommendationService.Recommend][google.cloud.discoveryengine.v1alpha.RecommendationService.Recommend].
    ///
    /// If
    /// [FILTERABLE_ENABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.FilterableOption.FILTERABLE_ENABLED]
    /// but the field type is numerical, field values are not filterable by text
    /// queries in
    /// [RecommendationService.Recommend][google.cloud.discoveryengine.v1alpha.RecommendationService.Recommend].
    /// Only textual fields are supported.
    ///
    /// If
    /// [recs_filterable_option][google.cloud.discoveryengine.v1alpha.FieldConfig.recs_filterable_option]
    /// is unset, the default setting is
    /// [FILTERABLE_DISABLED][google.cloud.discoveryengine.v1alpha.FieldConfig.FilterableOption.FILTERABLE_DISABLED]
    /// for fields that support setting filterable options.
    ///
    /// When a field set to \[FILTERABLE_DISABLED\] is filtered, a warning is
    /// generated and an empty result is returned.
    #[prost(enumeration = "field_config::FilterableOption", tag = "9")]
    pub recs_filterable_option: i32,
    /// Output only. Type of the key property that this field is mapped to. Empty
    /// string if this is not annotated as mapped to a key property.
    ///
    /// Example types are `title`, `description`. Full list is defined
    /// by `keyPropertyMapping` in the schema field annotation.
    ///
    /// If the schema field has a `KeyPropertyMapping` annotation,
    /// `indexable_option` and `searchable_option` of this field cannot be
    /// modified.
    #[prost(string, tag = "7")]
    pub key_property_type: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FieldConfig`.
pub mod field_config {
    /// Field value type in the Schema.
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
    pub enum FieldType {
        /// Field type is unspecified.
        Unspecified = 0,
        /// Field value type is Object.
        Object = 1,
        /// Field value type is String.
        String = 2,
        /// Field value type is Number.
        Number = 3,
        /// Field value type is Integer.
        Integer = 4,
        /// Field value type is Boolean.
        Boolean = 5,
        /// Field value type is Geolocation.
        Geolocation = 6,
    }
    impl FieldType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FieldType::Unspecified => "FIELD_TYPE_UNSPECIFIED",
                FieldType::Object => "OBJECT",
                FieldType::String => "STRING",
                FieldType::Number => "NUMBER",
                FieldType::Integer => "INTEGER",
                FieldType::Boolean => "BOOLEAN",
                FieldType::Geolocation => "GEOLOCATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIELD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "OBJECT" => Some(Self::Object),
                "STRING" => Some(Self::String),
                "NUMBER" => Some(Self::Number),
                "INTEGER" => Some(Self::Integer),
                "BOOLEAN" => Some(Self::Boolean),
                "GEOLOCATION" => Some(Self::Geolocation),
                _ => None,
            }
        }
    }
    /// The setting of Indexable options in schema.
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
    pub enum IndexableOption {
        /// Value used when unset.
        Unspecified = 0,
        /// Indexable option enabled for a schema field.
        IndexableEnabled = 1,
        /// Indexable option disabled for a schema field.
        IndexableDisabled = 2,
    }
    impl IndexableOption {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IndexableOption::Unspecified => "INDEXABLE_OPTION_UNSPECIFIED",
                IndexableOption::IndexableEnabled => "INDEXABLE_ENABLED",
                IndexableOption::IndexableDisabled => "INDEXABLE_DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INDEXABLE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                "INDEXABLE_ENABLED" => Some(Self::IndexableEnabled),
                "INDEXABLE_DISABLED" => Some(Self::IndexableDisabled),
                _ => None,
            }
        }
    }
    /// The status of the dynamic facetable option of a schema field.
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
    pub enum DynamicFacetableOption {
        /// Value used when unset.
        Unspecified = 0,
        /// Dynamic facetable option enabled for a schema field.
        DynamicFacetableEnabled = 1,
        /// Dynamic facetable option disabled for a schema field.
        DynamicFacetableDisabled = 2,
    }
    impl DynamicFacetableOption {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DynamicFacetableOption::Unspecified => {
                    "DYNAMIC_FACETABLE_OPTION_UNSPECIFIED"
                }
                DynamicFacetableOption::DynamicFacetableEnabled => {
                    "DYNAMIC_FACETABLE_ENABLED"
                }
                DynamicFacetableOption::DynamicFacetableDisabled => {
                    "DYNAMIC_FACETABLE_DISABLED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DYNAMIC_FACETABLE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                "DYNAMIC_FACETABLE_ENABLED" => Some(Self::DynamicFacetableEnabled),
                "DYNAMIC_FACETABLE_DISABLED" => Some(Self::DynamicFacetableDisabled),
                _ => None,
            }
        }
    }
    /// The setting of Searchable options in schema.
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
    pub enum SearchableOption {
        /// Value used when unset.
        Unspecified = 0,
        /// Searchable option enabled for a schema field.
        SearchableEnabled = 1,
        /// Searchable option disabled for a schema field.
        SearchableDisabled = 2,
    }
    impl SearchableOption {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SearchableOption::Unspecified => "SEARCHABLE_OPTION_UNSPECIFIED",
                SearchableOption::SearchableEnabled => "SEARCHABLE_ENABLED",
                SearchableOption::SearchableDisabled => "SEARCHABLE_DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEARCHABLE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                "SEARCHABLE_ENABLED" => Some(Self::SearchableEnabled),
                "SEARCHABLE_DISABLED" => Some(Self::SearchableDisabled),
                _ => None,
            }
        }
    }
    /// The setting of Retrievable options in schema.
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
    pub enum RetrievableOption {
        /// Value used when unset.
        Unspecified = 0,
        /// Retrievable option enabled for a schema field.
        RetrievableEnabled = 1,
        /// Retrievable option disabled for a schema field.
        RetrievableDisabled = 2,
    }
    impl RetrievableOption {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RetrievableOption::Unspecified => "RETRIEVABLE_OPTION_UNSPECIFIED",
                RetrievableOption::RetrievableEnabled => "RETRIEVABLE_ENABLED",
                RetrievableOption::RetrievableDisabled => "RETRIEVABLE_DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RETRIEVABLE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                "RETRIEVABLE_ENABLED" => Some(Self::RetrievableEnabled),
                "RETRIEVABLE_DISABLED" => Some(Self::RetrievableDisabled),
                _ => None,
            }
        }
    }
    /// The setting of Completable options in schema.
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
    pub enum CompletableOption {
        /// Value used when unset.
        Unspecified = 0,
        /// Completable option enabled for a schema field.
        CompletableEnabled = 1,
        /// Completable option disabled for a schema field.
        CompletableDisabled = 2,
    }
    impl CompletableOption {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompletableOption::Unspecified => "COMPLETABLE_OPTION_UNSPECIFIED",
                CompletableOption::CompletableEnabled => "COMPLETABLE_ENABLED",
                CompletableOption::CompletableDisabled => "COMPLETABLE_DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPLETABLE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                "COMPLETABLE_ENABLED" => Some(Self::CompletableEnabled),
                "COMPLETABLE_DISABLED" => Some(Self::CompletableDisabled),
                _ => None,
            }
        }
    }
    /// Sets the filterable option for schema fields.
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
    pub enum FilterableOption {
        /// Value used when unset.
        Unspecified = 0,
        /// Filterable option enabled for a schema field.
        FilterableEnabled = 1,
        /// Filterable option disabled for a schema field.
        FilterableDisabled = 2,
    }
    impl FilterableOption {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FilterableOption::Unspecified => "FILTERABLE_OPTION_UNSPECIFIED",
                FilterableOption::FilterableEnabled => "FILTERABLE_ENABLED",
                FilterableOption::FilterableDisabled => "FILTERABLE_DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILTERABLE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                "FILTERABLE_ENABLED" => Some(Self::FilterableEnabled),
                "FILTERABLE_DISABLED" => Some(Self::FilterableDisabled),
                _ => None,
            }
        }
    }
}
/// Request message for
/// [SchemaService.GetSchema][google.cloud.discoveryengine.v1alpha.SchemaService.GetSchema]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    /// Required. The full resource name of the schema, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [SchemaService.ListSchemas][google.cloud.discoveryengine.v1alpha.SchemaService.ListSchemas]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasRequest {
    /// Required. The parent data store resource name, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of
    /// [Schema][google.cloud.discoveryengine.v1alpha.Schema]s to return. The
    /// service may return fewer than this value.
    ///
    /// If unspecified, at most 100
    /// [Schema][google.cloud.discoveryengine.v1alpha.Schema]s will be returned.
    ///
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// [SchemaService.ListSchemas][google.cloud.discoveryengine.v1alpha.SchemaService.ListSchemas]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [SchemaService.ListSchemas][google.cloud.discoveryengine.v1alpha.SchemaService.ListSchemas]
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [SchemaService.ListSchemas][google.cloud.discoveryengine.v1alpha.SchemaService.ListSchemas]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasResponse {
    /// The [Schema][google.cloud.discoveryengine.v1alpha.Schema]s.
    #[prost(message, repeated, tag = "1")]
    pub schemas: ::prost::alloc::vec::Vec<Schema>,
    /// A token that can be sent as
    /// [ListSchemasRequest.page_token][google.cloud.discoveryengine.v1alpha.ListSchemasRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [SchemaService.CreateSchema][google.cloud.discoveryengine.v1alpha.SchemaService.CreateSchema]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSchemaRequest {
    /// Required. The parent data store resource name, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Schema][google.cloud.discoveryengine.v1alpha.Schema] to
    /// create.
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<Schema>,
    /// Required. The ID to use for the
    /// [Schema][google.cloud.discoveryengine.v1alpha.Schema], which will become
    /// the final component of the
    /// [Schema.name][google.cloud.discoveryengine.v1alpha.Schema.name].
    ///
    /// This field should conform to
    /// [RFC-1034](<https://tools.ietf.org/html/rfc1034>) standard with a length
    /// limit of 63 characters.
    #[prost(string, tag = "3")]
    pub schema_id: ::prost::alloc::string::String,
}
/// Request message for
/// [SchemaService.UpdateSchema][google.cloud.discoveryengine.v1alpha.SchemaService.UpdateSchema]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSchemaRequest {
    /// Required. The [Schema][google.cloud.discoveryengine.v1alpha.Schema] to
    /// update.
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<Schema>,
    /// If set to true, and the
    /// [Schema][google.cloud.discoveryengine.v1alpha.Schema] is not found, a new
    /// [Schema][google.cloud.discoveryengine.v1alpha.Schema] will be created. In
    /// this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// Request message for
/// [SchemaService.DeleteSchema][google.cloud.discoveryengine.v1alpha.SchemaService.DeleteSchema]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSchemaRequest {
    /// Required. The full resource name of the schema, in the format of
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/schemas/{schema}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata for Create Schema LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSchemaMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for UpdateSchema LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSchemaMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for DeleteSchema LRO.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSchemaMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod schema_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing [Schema][google.cloud.discoveryengine.v1alpha.Schema]s.
    #[derive(Debug, Clone)]
    pub struct SchemaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SchemaServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SchemaServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SchemaServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Gets a [Schema][google.cloud.discoveryengine.v1alpha.Schema].
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::Schema>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SchemaService/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SchemaService",
                        "GetSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of [Schema][google.cloud.discoveryengine.v1alpha.Schema]s.
        pub async fn list_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSchemasResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SchemaService/ListSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SchemaService",
                        "ListSchemas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a [Schema][google.cloud.discoveryengine.v1alpha.Schema].
        pub async fn create_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SchemaService/CreateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SchemaService",
                        "CreateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a [Schema][google.cloud.discoveryengine.v1alpha.Schema].
        pub async fn update_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SchemaService/UpdateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SchemaService",
                        "UpdateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a [Schema][google.cloud.discoveryengine.v1alpha.Schema].
        pub async fn delete_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SchemaService/DeleteSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SchemaService",
                        "DeleteSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// DataStore captures global settings and configs at the DataStore level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataStore {
    /// Immutable. The full resource name of the data store.
    /// Format:
    /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1024
    /// characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The data store display name.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Immutable. The industry vertical that the data store registers.
    #[prost(enumeration = "IndustryVertical", tag = "3")]
    pub industry_vertical: i32,
    /// The solutions that the data store enrolls. Available solutions for each
    /// [industry_vertical][google.cloud.discoveryengine.v1alpha.DataStore.industry_vertical]:
    ///
    /// * `MEDIA`: `SOLUTION_TYPE_RECOMMENDATION` and `SOLUTION_TYPE_SEARCH`.
    /// * `SITE_SEARCH`: `SOLUTION_TYPE_SEARCH` is automatically enrolled. Other
    ///    solutions cannot be enrolled.
    #[prost(enumeration = "SolutionType", repeated, tag = "5")]
    pub solution_types: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The id of the default
    /// [Schema][google.cloud.discoveryengine.v1alpha.Schema] asscociated to this
    /// data store.
    #[prost(string, tag = "7")]
    pub default_schema_id: ::prost::alloc::string::String,
    /// Immutable. The content config of the data store. If this field is unset,
    /// the server behavior defaults to
    /// [ContentConfig.NO_CONTENT][google.cloud.discoveryengine.v1alpha.DataStore.ContentConfig.NO_CONTENT].
    #[prost(enumeration = "data_store::ContentConfig", tag = "6")]
    pub content_config: i32,
    /// Output only. Timestamp the
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore] was created at.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `DataStore`.
pub mod data_store {
    /// Content config of the data store.
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
    pub enum ContentConfig {
        /// Default value.
        Unspecified = 0,
        /// Only contains documents without any
        /// [Document.content][google.cloud.discoveryengine.v1alpha.Document.content].
        NoContent = 1,
        /// Only contains documents with
        /// [Document.content][google.cloud.discoveryengine.v1alpha.Document.content].
        ContentRequired = 2,
        /// The data store is used for public website search.
        PublicWebsite = 3,
    }
    impl ContentConfig {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ContentConfig::Unspecified => "CONTENT_CONFIG_UNSPECIFIED",
                ContentConfig::NoContent => "NO_CONTENT",
                ContentConfig::ContentRequired => "CONTENT_REQUIRED",
                ContentConfig::PublicWebsite => "PUBLIC_WEBSITE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTENT_CONFIG_UNSPECIFIED" => Some(Self::Unspecified),
                "NO_CONTENT" => Some(Self::NoContent),
                "CONTENT_REQUIRED" => Some(Self::ContentRequired),
                "PUBLIC_WEBSITE" => Some(Self::PublicWebsite),
                _ => None,
            }
        }
    }
}
/// Request for
/// [DataStoreService.CreateDataStore][google.cloud.discoveryengine.v1alpha.DataStoreService.CreateDataStore]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataStoreRequest {
    /// Required. The parent resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]
    /// to create.
    #[prost(message, optional, tag = "2")]
    pub data_store: ::core::option::Option<DataStore>,
    /// Required. The ID to use for the
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore], which will
    /// become the final component of the
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]'s resource
    /// name.
    ///
    /// This field must conform to [RFC-1034](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub data_store_id: ::prost::alloc::string::String,
    /// A boolean flag indicating whether user want to directly create an advanced
    /// data store for site search.
    /// If the data store is not configured as site
    /// search (GENERIC vertical and PUBLIC_WEBSITE content_config), this flag will
    /// be ignored.
    #[prost(bool, tag = "4")]
    pub create_advanced_site_search: bool,
}
/// Request message for
/// [DataStoreService.GetDataStore][google.cloud.discoveryengine.v1alpha.DataStoreService.GetDataStore]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataStoreRequest {
    /// Required. Full resource name of
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore], such as
    /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`.
    ///
    /// If the caller does not have permission to access the
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the requested
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore] does not exist,
    /// a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata related to the progress of the
/// [DataStoreService.CreateDataStore][google.cloud.discoveryengine.v1alpha.DataStoreService.CreateDataStore]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataStoreMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// [DataStoreService.ListDataStores][google.cloud.discoveryengine.v1alpha.DataStoreService.ListDataStores]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStoresRequest {
    /// Required. The parent branch resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection_id}`.
    ///
    /// If the caller does not have permission to list [DataStores][]s under this
    /// location, regardless of whether or not this data store exists, a
    /// PERMISSION_DENIED error is returned.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]s to return. If
    /// unspecified, defaults to 10. The maximum allowed value is 50. Values above
    /// 50 will be coerced to 50.
    ///
    /// If this field is negative, an INVALID_ARGUMENT is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token
    /// [ListDataStoresResponse.next_page_token][google.cloud.discoveryengine.v1alpha.ListDataStoresResponse.next_page_token],
    /// received from a previous
    /// [DataStoreService.ListDataStores][google.cloud.discoveryengine.v1alpha.DataStoreService.ListDataStores]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [DataStoreService.ListDataStores][google.cloud.discoveryengine.v1alpha.DataStoreService.ListDataStores]
    /// must match the call that provided the page token. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter by solution type. For example: filter =
    /// 'solution_type:SOLUTION_TYPE_SEARCH'
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for
/// [DataStoreService.ListDataStores][google.cloud.discoveryengine.v1alpha.DataStoreService.ListDataStores]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStoresResponse {
    /// All the customer's
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]s.
    #[prost(message, repeated, tag = "1")]
    pub data_stores: ::prost::alloc::vec::Vec<DataStore>,
    /// A token that can be sent as
    /// [ListDataStoresRequest.page_token][google.cloud.discoveryengine.v1alpha.ListDataStoresRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [DataStoreService.DeleteDataStore][google.cloud.discoveryengine.v1alpha.DataStoreService.DeleteDataStore]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataStoreRequest {
    /// Required. Full resource name of
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore], such as
    /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`.
    ///
    /// If the caller does not have permission to delete the
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [DataStore][google.cloud.discoveryengine.v1alpha.DataStore] to
    /// delete does not exist, a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [DataStoreService.UpdateDataStore][google.cloud.discoveryengine.v1alpha.DataStoreService.UpdateDataStore]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataStoreRequest {
    /// Required. The [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]
    /// to update.
    ///
    /// If the caller does not have permission to update the
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [DataStore][google.cloud.discoveryengine.v1alpha.DataStore] to
    /// update does not exist, a NOT_FOUND error is returned.
    #[prost(message, optional, tag = "1")]
    pub data_store: ::core::option::Option<DataStore>,
    /// Indicates which fields in the provided
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore] to update.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Metadata related to the progress of the
/// [DataStoreService.DeleteDataStore][google.cloud.discoveryengine.v1alpha.DataStoreService.DeleteDataStore]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataStoreMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod data_store_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing
    /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore] configuration.
    #[derive(Debug, Clone)]
    pub struct DataStoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataStoreServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DataStoreServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DataStoreServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Creates a [DataStore][google.cloud.discoveryengine.v1alpha.DataStore].
        ///
        /// DataStore is for storing
        /// [Documents][google.cloud.discoveryengine.v1alpha.Document]. To serve these
        /// documents for Search, or Recommendation use case, an
        /// [Engine][google.cloud.discoveryengine.v1alpha.Engine] needs to be created
        /// separately.
        pub async fn create_data_store(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DataStoreService/CreateDataStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DataStoreService",
                        "CreateDataStore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a [DataStore][google.cloud.discoveryengine.v1alpha.DataStore].
        pub async fn get_data_store(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataStoreRequest>,
        ) -> std::result::Result<tonic::Response<super::DataStore>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DataStoreService/GetDataStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DataStoreService",
                        "GetDataStore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all the [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]s
        /// associated with the project.
        pub async fn list_data_stores(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataStoresRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDataStoresResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DataStoreService/ListDataStores",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DataStoreService",
                        "ListDataStores",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a [DataStore][google.cloud.discoveryengine.v1alpha.DataStore].
        pub async fn delete_data_store(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DataStoreService/DeleteDataStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DataStoreService",
                        "DeleteDataStore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a [DataStore][google.cloud.discoveryengine.v1alpha.DataStore]
        pub async fn update_data_store(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataStoreRequest>,
        ) -> std::result::Result<tonic::Response<super::DataStore>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DataStoreService/UpdateDataStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DataStoreService",
                        "UpdateDataStore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Document captures all raw metadata information of items to be recommended or
/// searched.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// Immutable. The full resource name of the document.
    /// Format:
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document_id}`.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1024
    /// characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The identifier of the document.
    ///
    /// Id should conform to [RFC-1034](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// The identifier of the schema located in the same data store.
    #[prost(string, tag = "3")]
    pub schema_id: ::prost::alloc::string::String,
    /// The unstructured data linked to this document. Content must be set if this
    /// document is under a
    /// `CONTENT_REQUIRED` data store.
    #[prost(message, optional, tag = "10")]
    pub content: ::core::option::Option<document::Content>,
    /// The identifier of the parent document. Currently supports at most two level
    /// document hierarchy.
    ///
    /// Id should conform to [RFC-1034](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters.
    #[prost(string, tag = "7")]
    pub parent_document_id: ::prost::alloc::string::String,
    /// Output only. This field is OUTPUT_ONLY.
    /// It contains derived data that are not in the original input document.
    #[prost(message, optional, tag = "6")]
    pub derived_struct_data: ::core::option::Option<::prost_types::Struct>,
    /// Data representation. One of
    /// [struct_data][google.cloud.discoveryengine.v1alpha.Document.struct_data] or
    /// [json_data][google.cloud.discoveryengine.v1alpha.Document.json_data] should
    /// be provided otherwise an `INVALID_ARGUMENT` error is thrown.
    #[prost(oneof = "document::Data", tags = "4, 5")]
    pub data: ::core::option::Option<document::Data>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// Unstructured data linked to this document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Content {
        /// The MIME type of the content. Supported types:
        ///
        /// * `application/pdf` (PDF, only native PDFs are supported for now)
        /// * `text/html` (HTML)
        /// * `application/vnd.openxmlformats-officedocument.wordprocessingml.document` (DOCX)
        /// * `application/vnd.openxmlformats-officedocument.presentationml.presentation` (PPTX)
        /// * `text/plain` (TXT)
        ///
        /// See <https://www.iana.org/assignments/media-types/media-types.xhtml.>
        #[prost(string, tag = "1")]
        pub mime_type: ::prost::alloc::string::String,
        #[prost(oneof = "content::Content", tags = "2, 3")]
        pub content: ::core::option::Option<content::Content>,
    }
    /// Nested message and enum types in `Content`.
    pub mod content {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Content {
            /// The content represented as a stream of bytes. The maximum length is
            /// 1,000,000 bytes (1 MB / ~0.95 MiB).
            ///
            /// Note: As with all `bytes` fields, this field is represented as pure
            /// binary in Protocol Buffers and base64-encoded string in JSON. For
            /// example, `abc123!?$*&()'-=@~` should be represented as
            /// `YWJjMTIzIT8kKiYoKSctPUB+` in JSON. See
            /// <https://developers.google.com/protocol-buffers/docs/proto3#json.>
            #[prost(bytes, tag = "2")]
            RawBytes(::prost::bytes::Bytes),
            /// The URI of the content. Only Cloud Storage URIs (e.g.
            /// `gs://bucket-name/path/to/file`) are supported. The maximum file size
            /// is 2.5 MB for text-based formats, 100 MB for other formats.
            #[prost(string, tag = "3")]
            Uri(::prost::alloc::string::String),
        }
    }
    /// Data representation. One of
    /// [struct_data][google.cloud.discoveryengine.v1alpha.Document.struct_data] or
    /// [json_data][google.cloud.discoveryengine.v1alpha.Document.json_data] should
    /// be provided otherwise an `INVALID_ARGUMENT` error is thrown.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// The structured JSON data for the document. It should conform to the
        /// registered [Schema][google.cloud.discoveryengine.v1alpha.Schema] or an
        /// `INVALID_ARGUMENT` error is thrown.
        #[prost(message, tag = "4")]
        StructData(::prost_types::Struct),
        /// The JSON string representation of the document. It should conform to the
        /// registered [Schema][google.cloud.discoveryengine.v1alpha.Schema] or an
        /// `INVALID_ARGUMENT` error is thrown.
        #[prost(string, tag = "5")]
        JsonData(::prost::alloc::string::String),
    }
}
/// UserEvent captures all metadata information Discovery Engine API needs to
/// know about how end users interact with customers' website.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEvent {
    /// Required. User event type. Allowed values are:
    ///
    /// Generic values:
    ///
    /// * `search`: Search for Documents.
    /// * `view-item`: Detailed page view of a Document.
    /// * `view-item-list`: View of a panel or ordered list of Documents.
    /// * `view-home-page`: View of the home page.
    /// * `view-category-page`: View of a category page, e.g. Home > Men > Jeans
    ///
    /// Retail-related values:
    ///
    /// * `add-to-cart`: Add an item(s) to cart, e.g. in Retail online shopping
    /// * `purchase`: Purchase an item(s)
    ///
    /// Media-related values:
    ///
    /// * `media-play`: Start/resume watching a video, playing a song, etc.
    /// * `media-complete`: Finished or stopped midway through a video, song, etc.
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    /// Required. A unique identifier for tracking visitors.
    ///
    /// For example, this could be implemented with an HTTP cookie, which should be
    /// able to uniquely identify a visitor on a single device. This unique
    /// identifier should not change if the visitor log in/out of the website.
    ///
    /// Do not set the field to the same fixed ID for different users. This mixes
    /// the event history of those users together, which results in degraded model
    /// quality.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    ///
    /// The field should not contain PII or user-data. We recommend to use Google
    /// Analytics [Client
    /// ID](<https://developers.google.com/analytics/devguides/collection/analyticsjs/field-reference#clientId>)
    /// for this field.
    #[prost(string, tag = "2")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// Only required for
    /// [UserEventService.ImportUserEvents][google.cloud.discoveryengine.v1alpha.UserEventService.ImportUserEvents]
    /// method. Timestamp of when the user event happened.
    #[prost(message, optional, tag = "3")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Information about the end user.
    #[prost(message, optional, tag = "4")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// Should set to true if the request is made directly from the end user, in
    /// which case the
    /// [UserEvent.user_info.user_agent][google.cloud.discoveryengine.v1alpha.UserInfo.user_agent]
    /// can be populated from the HTTP request.
    ///
    /// This flag should be set only if the API request is made directly from the
    /// end user such as a mobile app (and not if a gateway or a server is
    /// processing and pushing the user events).
    ///
    /// This should not be set when using the JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.discoveryengine.v1alpha.UserEventService.CollectUserEvent].
    #[prost(bool, tag = "5")]
    pub direct_user_request: bool,
    /// A unique identifier for tracking a visitor session with a length limit of
    /// 128 bytes. A session is an aggregation of an end user behavior in a time
    /// span.
    ///
    /// A general guideline to populate the session_id:
    ///
    /// 1. If user has no activity for 30 min, a new session_id should be assigned.
    /// 2. The session_id should be unique across users, suggest use uuid or add
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1alpha.UserEvent.user_pseudo_id]
    /// as prefix.
    #[prost(string, tag = "6")]
    pub session_id: ::prost::alloc::string::String,
    /// Page metadata such as categories and other critical information for certain
    /// event types such as `view-category-page`.
    #[prost(message, optional, tag = "7")]
    pub page_info: ::core::option::Option<PageInfo>,
    /// Token to attribute an API response to user action(s) to trigger the event.
    ///
    /// Highly recommended for user events that are the result of
    /// [RecommendationService.Recommend][google.cloud.discoveryengine.v1alpha.RecommendationService.Recommend].
    /// This field enables accurate attribution of recommendation model
    /// performance.
    ///
    /// The value must be one of:
    ///
    /// * [RecommendResponse.attribution_token][google.cloud.discoveryengine.v1alpha.RecommendResponse.attribution_token] for events that are the result of
    /// [RecommendationService.Recommend][google.cloud.discoveryengine.v1alpha.RecommendationService.Recommend].
    /// * [SearchResponse.attribution_token][google.cloud.discoveryengine.v1alpha.SearchResponse.attribution_token] for events that are the result of
    /// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search].
    ///
    /// This token enables us to accurately attribute page view or conversion
    /// completion back to the event and the particular predict response containing
    /// this clicked/purchased product. If user clicks on product K in the
    /// recommendation results, pass
    /// [RecommendResponse.attribution_token][google.cloud.discoveryengine.v1alpha.RecommendResponse.attribution_token]
    /// as a URL parameter to product K's page. When recording events on product
    /// K's page, log the
    /// [RecommendResponse.attribution_token][google.cloud.discoveryengine.v1alpha.RecommendResponse.attribution_token]
    /// to this field.
    #[prost(string, tag = "8")]
    pub attribution_token: ::prost::alloc::string::String,
    /// The filter syntax consists of an expression language for constructing a
    /// predicate from one or more fields of the documents being filtered.
    ///
    /// One example is for `search` events, the associated
    /// [SearchRequest][google.cloud.discoveryengine.v1alpha.SearchRequest] may
    /// contain a filter expression in
    /// [SearchRequest.filter][google.cloud.discoveryengine.v1alpha.SearchRequest.filter]
    /// conforming to <https://google.aip.dev/160#filtering.>
    ///
    /// Similarly, for `view-item-list` events that are generated from a
    /// [RecommendationService.RecommendRequest][], this field may be populated
    /// directly from [RecommendationService.RecommendRequest.filter][] conforming
    /// to <https://google.aip.dev/160#filtering.>
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "9")]
    pub filter: ::prost::alloc::string::String,
    /// List of [Document][google.cloud.discoveryengine.v1alpha.Document]s
    /// associated with this user event.
    ///
    /// This field is optional except for the following event types:
    ///
    /// * `view-item`
    /// * `add-to-cart`
    /// * `purchase`
    /// * `media-play`
    /// * `media-complete`
    ///
    /// In a `search` event, this field represents the documents returned to the
    /// end user on the current page (the end user may have not finished browsing
    /// the whole page yet). When a new page is returned to the end user, after
    /// pagination/filtering/ordering even for the same query, a new `search` event
    /// with different
    /// [UserEvent.documents][google.cloud.discoveryengine.v1alpha.UserEvent.documents]
    /// is desired.
    #[prost(message, repeated, tag = "10")]
    pub documents: ::prost::alloc::vec::Vec<DocumentInfo>,
    /// Panel metadata associated with this user event.
    #[prost(message, optional, tag = "11")]
    pub panel: ::core::option::Option<PanelInfo>,
    /// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search]
    /// details related to the event.
    ///
    /// This field should be set for `search` event.
    #[prost(message, optional, tag = "12")]
    pub search_info: ::core::option::Option<SearchInfo>,
    /// [CompletionService.CompleteQuery][google.cloud.discoveryengine.v1alpha.CompletionService.CompleteQuery]
    /// details related to the event.
    ///
    /// This field should be set for `search` event when autocomplete function is
    /// enabled and the user clicks a suggestion for search.
    #[prost(message, optional, tag = "13")]
    pub completion_info: ::core::option::Option<CompletionInfo>,
    /// The transaction metadata (if any) associated with this user event.
    #[prost(message, optional, tag = "14")]
    pub transaction_info: ::core::option::Option<TransactionInfo>,
    /// A list of identifiers for the independent experiment groups this user event
    /// belongs to. This is used to distinguish between user events associated with
    /// different experiment setups on the customer end.
    #[prost(string, repeated, tag = "15")]
    pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The promotion IDs if this is an event associated with promotions.
    /// Currently, this field is restricted to at most one ID.
    #[prost(string, repeated, tag = "16")]
    pub promotion_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Extra user event features to include in the recommendation model.
    /// These attributes must NOT contain data that needs to be parsed or processed
    /// further, e.g. JSON or other encodings.
    ///
    /// If you provide custom attributes for ingested user events, also include
    /// them in the user events that you associate with prediction requests. Custom
    /// attribute formatting must be consistent between imported events and events
    /// provided with prediction requests. This lets the Discovery Engine API use
    /// those custom attributes when training models and serving predictions, which
    /// helps improve recommendation quality.
    ///
    /// This field needs to pass all below criteria, otherwise an
    /// `INVALID_ARGUMENT` error is returned:
    ///
    /// * The key must be a UTF-8 encoded string with a length limit of 5,000
    ///    characters.
    /// * For text attributes, at most 400 values are allowed. Empty values are not
    ///    allowed. Each value must be a UTF-8 encoded string with a length limit of
    ///    256 characters.
    /// * For number attributes, at most 400 values are allowed.
    ///
    /// For product recommendations, an example of extra user information is
    /// `traffic_channel`, which is how a user arrives at the site. Users can
    /// arrive
    /// at the site by coming to the site directly, coming through Google
    /// search, or in other ways.
    #[prost(btree_map = "string, message", tag = "17")]
    pub attributes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        CustomAttribute,
    >,
    /// Media-specific info.
    #[prost(message, optional, tag = "18")]
    pub media_info: ::core::option::Option<MediaInfo>,
}
/// Detailed page information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageInfo {
    /// A unique ID of a web page view.
    ///
    /// This should be kept the same for all user events triggered from the same
    /// pageview. For example, an item detail page view could trigger multiple
    /// events as the user is browsing the page. The `pageview_id` property should
    /// be kept the same for all these events so that they can be grouped together
    /// properly.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically.
    #[prost(string, tag = "1")]
    pub pageview_id: ::prost::alloc::string::String,
    /// The most specific category associated with a category page.
    ///
    /// To represent full path of category, use '>' sign to separate different
    /// hierarchies. If '>' is part of the category name, please replace it with
    /// other character(s).
    ///
    /// Category pages include special pages such as sales or promotions. For
    /// instance, a special sale page may have the category hierarchy:
    /// `"pageCategory" : "Sales > 2017 Black Friday Deals"`.
    ///
    /// Required for `view-category-page` events. Other event types should not set
    /// this field. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "2")]
    pub page_category: ::prost::alloc::string::String,
    /// Complete URL (window.location.href) of the user's current page.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically. Maximum length 5,000
    /// characters.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// The referrer URL of the current page.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically. However, some browser
    /// privacy restrictions may cause this field to be empty.
    #[prost(string, tag = "4")]
    pub referrer_uri: ::prost::alloc::string::String,
}
/// Detailed search information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchInfo {
    /// The user's search query.
    ///
    /// See
    /// [SearchRequest.query][google.cloud.discoveryengine.v1alpha.SearchRequest.query]
    /// for definition.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    ///
    /// At least one of
    /// [search_query][google.cloud.discoveryengine.v1alpha.SearchInfo.search_query]
    /// or
    /// [PageInfo.page_category][google.cloud.discoveryengine.v1alpha.PageInfo.page_category]
    /// is required for `search` events. Other event types should not set this
    /// field. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "1")]
    pub search_query: ::prost::alloc::string::String,
    /// The order in which products are returned, if applicable.
    ///
    /// See
    /// [SearchRequest.order_by][google.cloud.discoveryengine.v1alpha.SearchRequest.order_by]
    /// for definition and syntax.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    ///
    /// This can only be set for `search` events. Other event types should not set
    /// this field. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "2")]
    pub order_by: ::prost::alloc::string::String,
    /// An integer that specifies the current offset for pagination (the 0-indexed
    /// starting location, amongst the products deemed by the API as relevant).
    ///
    /// See
    /// [SearchRequest.offset][google.cloud.discoveryengine.v1alpha.SearchRequest.offset]
    /// for definition.
    ///
    /// If this field is negative, an `INVALID_ARGUMENT` is returned.
    ///
    /// This can only be set for `search` events. Other event types should not set
    /// this field. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(int32, optional, tag = "3")]
    pub offset: ::core::option::Option<i32>,
}
/// Detailed completion information including completion attribution token and
/// clicked completion info.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionInfo {
    /// End user selected
    /// [CompleteQueryResponse.QuerySuggestion.suggestion][google.cloud.discoveryengine.v1alpha.CompleteQueryResponse.QuerySuggestion.suggestion].
    #[prost(string, tag = "1")]
    pub selected_suggestion: ::prost::alloc::string::String,
    /// End user selected
    /// [CompleteQueryResponse.QuerySuggestion.suggestion][google.cloud.discoveryengine.v1alpha.CompleteQueryResponse.QuerySuggestion.suggestion]
    /// position, starting from 0.
    #[prost(int32, tag = "2")]
    pub selected_position: i32,
}
/// A transaction represents the entire purchase transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// Required. Total non-zero value associated with the transaction. This value
    /// may include shipping, tax, or other adjustments to the total value that you
    /// want to include.
    #[prost(float, optional, tag = "1")]
    pub value: ::core::option::Option<f32>,
    /// Required. Currency code. Use three-character ISO-4217 code.
    #[prost(string, tag = "2")]
    pub currency: ::prost::alloc::string::String,
    /// The transaction ID with a length limit of 128 characters.
    #[prost(string, tag = "3")]
    pub transaction_id: ::prost::alloc::string::String,
    /// All the taxes associated with the transaction.
    #[prost(float, optional, tag = "4")]
    pub tax: ::core::option::Option<f32>,
    /// All the costs associated with the products. These can be manufacturing
    /// costs, shipping expenses not borne by the end user, or any other costs,
    /// such that:
    ///
    /// * Profit =
    /// [value][google.cloud.discoveryengine.v1alpha.TransactionInfo.value] -
    /// [tax][google.cloud.discoveryengine.v1alpha.TransactionInfo.tax] -
    /// [cost][google.cloud.discoveryengine.v1alpha.TransactionInfo.cost]
    #[prost(float, optional, tag = "5")]
    pub cost: ::core::option::Option<f32>,
    /// The total discount(s) value applied to this transaction.
    /// This figure should be excluded from
    /// [TransactionInfo.value][google.cloud.discoveryengine.v1alpha.TransactionInfo.value]
    ///
    /// For example, if a user paid
    /// [TransactionInfo.value][google.cloud.discoveryengine.v1alpha.TransactionInfo.value]
    /// amount, then nominal (pre-discount) value of the transaction is the sum of
    /// [TransactionInfo.value][google.cloud.discoveryengine.v1alpha.TransactionInfo.value]
    /// and
    /// [TransactionInfo.discount_value][google.cloud.discoveryengine.v1alpha.TransactionInfo.discount_value]
    ///
    /// This means that profit is calculated the same way, regardless of the
    /// discount value, and that
    /// [TransactionInfo.discount_value][google.cloud.discoveryengine.v1alpha.TransactionInfo.discount_value]
    /// can be larger than
    /// [TransactionInfo.value][google.cloud.discoveryengine.v1alpha.TransactionInfo.value]:
    ///
    /// * Profit =
    /// [value][google.cloud.discoveryengine.v1alpha.TransactionInfo.value] -
    /// [tax][google.cloud.discoveryengine.v1alpha.TransactionInfo.tax] -
    /// [cost][google.cloud.discoveryengine.v1alpha.TransactionInfo.cost]
    #[prost(float, optional, tag = "6")]
    pub discount_value: ::core::option::Option<f32>,
}
/// Detailed document information associated with a user event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentInfo {
    /// Quantity of the Document associated with the user event. Defaults to 1.
    ///
    /// For example, this field will be 2 if two quantities of the same Document
    /// are involved in a `add-to-cart` event.
    ///
    /// Required for events of the following event types:
    ///
    /// * `add-to-cart`
    /// * `purchase`
    #[prost(int32, optional, tag = "3")]
    pub quantity: ::core::option::Option<i32>,
    /// The promotion IDs associated with this Document.
    /// Currently, this field is restricted to at most one ID.
    #[prost(string, repeated, tag = "4")]
    pub promotion_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A required descriptor of the associated
    /// [Document][google.cloud.discoveryengine.v1alpha.Document].
    ///
    /// * If [id][google.cloud.discoveryengine.v1alpha.DocumentInfo.id] is
    /// specified, then the default values for
    /// `{location}`, `{collection_id}`, `{data_store_id}`, and `{branch_id}` are
    /// used when annotating with the stored Document.
    ///
    /// * If [name][google.cloud.discoveryengine.v1alpha.DocumentInfo.name] is
    /// specified, then the provided values (default values allowed) for
    /// `{location}`, `{collection_id}`, `{data_store_id}`, and
    /// `{branch_id}` are used when annotating with the stored Document.
    #[prost(oneof = "document_info::DocumentDescriptor", tags = "1, 2, 6")]
    pub document_descriptor: ::core::option::Option<document_info::DocumentDescriptor>,
}
/// Nested message and enum types in `DocumentInfo`.
pub mod document_info {
    /// A required descriptor of the associated
    /// [Document][google.cloud.discoveryengine.v1alpha.Document].
    ///
    /// * If [id][google.cloud.discoveryengine.v1alpha.DocumentInfo.id] is
    /// specified, then the default values for
    /// `{location}`, `{collection_id}`, `{data_store_id}`, and `{branch_id}` are
    /// used when annotating with the stored Document.
    ///
    /// * If [name][google.cloud.discoveryengine.v1alpha.DocumentInfo.name] is
    /// specified, then the provided values (default values allowed) for
    /// `{location}`, `{collection_id}`, `{data_store_id}`, and
    /// `{branch_id}` are used when annotating with the stored Document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DocumentDescriptor {
        /// The [Document][google.cloud.discoveryengine.v1alpha.Document] resource
        /// ID.
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        /// The [Document][google.cloud.discoveryengine.v1alpha.Document] resource
        /// full name, of the form:
        /// `projects/{project_id}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/branches/{branch_id}/documents/{document_id}`
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
        /// The [Document][google.cloud.discoveryengine.v1alpha.Document] URI - only
        /// allowed for website data stores.
        #[prost(string, tag = "6")]
        Uri(::prost::alloc::string::String),
    }
}
/// Detailed panel information associated with a user event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PanelInfo {
    /// Required. The panel ID.
    #[prost(string, tag = "2")]
    pub panel_id: ::prost::alloc::string::String,
    /// The display name of the panel.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// The ordered position of the panel, if shown to the user with other panels.
    /// If set, then
    /// [total_panels][google.cloud.discoveryengine.v1alpha.PanelInfo.total_panels]
    /// must also be set.
    #[prost(int32, optional, tag = "4")]
    pub panel_position: ::core::option::Option<i32>,
    /// The total number of panels, including this one, shown to the user.
    /// Must be set if
    /// [panel_position][google.cloud.discoveryengine.v1alpha.PanelInfo.panel_position]
    /// is set.
    #[prost(int32, optional, tag = "5")]
    pub total_panels: ::core::option::Option<i32>,
}
/// Media-specific user event information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaInfo {
    /// The media progress time in seconds, if applicable.
    /// For example, if the end user has finished 90 seconds of a playback video,
    /// then
    /// [MediaInfo.media_progress_duration.seconds][google.protobuf.Duration.seconds]
    /// should be set to 90.
    #[prost(message, optional, tag = "1")]
    pub media_progress_duration: ::core::option::Option<::prost_types::Duration>,
    /// Media progress should be computed using only the
    /// [media_progress_duration][google.cloud.discoveryengine.v1alpha.MediaInfo.media_progress_duration]
    /// relative to the media total length.
    ///
    /// This value must be between `\[0, 1.0\]` inclusive.
    ///
    /// If this is not a playback or the progress cannot be computed (e.g. ongoing
    /// livestream), this field should be unset.
    #[prost(float, optional, tag = "2")]
    pub media_progress_percentage: ::core::option::Option<f32>,
}
/// Cloud Storage location for input content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Cloud Storage URIs to input files. URI can be up to
    /// 2000 characters long. URIs can match the full object path (for example,
    /// `gs://bucket/directory/object.json`) or a pattern matching one or more
    /// files, such as `gs://bucket/directory/*.json`.
    ///
    /// A request can contain at most 100 files (or 100,000 files if `data_schema`
    /// is `content`). Each file can be up to 2 GB (or 100 MB if `data_schema` is
    /// `content`).
    #[prost(string, repeated, tag = "1")]
    pub input_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for document imports:
    ///
    /// * `document` (default): One JSON
    /// [Document][google.cloud.discoveryengine.v1alpha.Document] per line. Each
    /// document must
    ///    have a valid
    ///    [Document.id][google.cloud.discoveryengine.v1alpha.Document.id].
    /// * `content`: Unstructured data (e.g. PDF, HTML). Each file matched by
    ///    `input_uris` becomes a document, with the ID set to the first 128
    ///    bits of SHA256(URI) encoded as a hex string.
    /// * `custom`: One custom data JSON per row in arbitrary format that conforms
    ///    to the defined [Schema][google.cloud.discoveryengine.v1alpha.Schema] of
    ///    the data store. This can only be used by Gen App Builder.
    /// * `csv`: A CSV file with header conforming to the defined
    /// [Schema][google.cloud.discoveryengine.v1alpha.Schema] of the
    ///    data store. Each entry after the header is imported as a Document.
    ///    This can only be used by Gen App Builder.
    ///
    /// Supported values for user even imports:
    ///
    /// * `user_event` (default): One JSON
    /// [UserEvent][google.cloud.discoveryengine.v1alpha.UserEvent] per line.
    #[prost(string, tag = "2")]
    pub data_schema: ::prost::alloc::string::String,
}
/// BigQuery source import data from.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQuerySource {
    /// The project ID (can be project # or ID) that the BigQuery source is in with
    /// a length limit of 128 characters. If not specified, inherits the project
    /// ID from the parent request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The BigQuery data set to copy the data from with a length limit
    /// of 1,024 characters.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. The BigQuery table to copy the data from with a length limit of
    /// 1,024 characters.
    #[prost(string, tag = "3")]
    pub table_id: ::prost::alloc::string::String,
    /// Intermediate Cloud Storage directory used for the import with a length
    /// limit of 2,000 characters. Can be specified if one wants to have the
    /// BigQuery export to a specific Cloud Storage directory.
    #[prost(string, tag = "4")]
    pub gcs_staging_dir: ::prost::alloc::string::String,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for user event imports:
    ///
    /// * `user_event` (default): One
    /// [UserEvent][google.cloud.discoveryengine.v1alpha.UserEvent] per row.
    ///
    /// Supported values for document imports:
    ///
    /// * `document` (default): One
    /// [Document][google.cloud.discoveryengine.v1alpha.Document] format per
    ///    row. Each document must have a valid
    ///    [Document.id][google.cloud.discoveryengine.v1alpha.Document.id] and one
    ///    of
    ///    [Document.json_data][google.cloud.discoveryengine.v1alpha.Document.json_data]
    ///    or
    ///    [Document.struct_data][google.cloud.discoveryengine.v1alpha.Document.struct_data].
    /// * `custom`: One custom data per row in arbitrary format that conforms to
    ///    the defined [Schema][google.cloud.discoveryengine.v1alpha.Schema] of the
    ///    data store. This can only be used by Gen App Builder.
    #[prost(string, tag = "6")]
    pub data_schema: ::prost::alloc::string::String,
    /// BigQuery table partition info. Leave this empty if the BigQuery table
    /// is not partitioned.
    #[prost(oneof = "big_query_source::Partition", tags = "5")]
    pub partition: ::core::option::Option<big_query_source::Partition>,
}
/// Nested message and enum types in `BigQuerySource`.
pub mod big_query_source {
    /// BigQuery table partition info. Leave this empty if the BigQuery table
    /// is not partitioned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Partition {
        /// BigQuery time partitioned table's _PARTITIONDATE in YYYY-MM-DD format.
        #[prost(message, tag = "5")]
        PartitionDate(super::super::super::super::r#type::Date),
    }
}
/// Configuration of destination for Import related errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportErrorConfig {
    /// Required. Errors destination.
    #[prost(oneof = "import_error_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<import_error_config::Destination>,
}
/// Nested message and enum types in `ImportErrorConfig`.
pub mod import_error_config {
    /// Required. Errors destination.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Cloud Storage prefix for import errors. This must be an empty,
        /// existing Cloud Storage directory. Import errors are written to
        /// sharded files in this directory, one per line, as a JSON-encoded
        /// `google.rpc.Status` message.
        #[prost(string, tag = "1")]
        GcsPrefix(::prost::alloc::string::String),
    }
}
/// Request message for the ImportUserEvents request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsRequest {
    /// Required. Parent DataStore resource name, of the form
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The desired location of errors incurred during the Import. Cannot be set
    /// for inline user event imports.
    #[prost(message, optional, tag = "5")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
    /// Required - The desired input source of the user event data.
    #[prost(oneof = "import_user_events_request::Source", tags = "2, 3, 4")]
    pub source: ::core::option::Option<import_user_events_request::Source>,
}
/// Nested message and enum types in `ImportUserEventsRequest`.
pub mod import_user_events_request {
    /// The inline source for the input config for ImportUserEvents method.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineSource {
        /// Required. A list of user events to import. Recommended max of 10k items.
        #[prost(message, repeated, tag = "1")]
        pub user_events: ::prost::alloc::vec::Vec<super::UserEvent>,
    }
    /// Required - The desired input source of the user event data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Inline source for the input content for UserEvents.
        #[prost(message, tag = "2")]
        InlineSource(InlineSource),
        /// Cloud Storage location for the input content.
        #[prost(message, tag = "3")]
        GcsSource(super::GcsSource),
        /// BigQuery input source.
        #[prost(message, tag = "4")]
        BigquerySource(super::BigQuerySource),
    }
}
/// Response of the ImportUserEventsRequest. If the long running
/// operation was successful, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors if this field was set in
    /// the request.
    #[prost(message, optional, tag = "2")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
    /// Count of user events imported with complete existing Documents.
    #[prost(int64, tag = "3")]
    pub joined_events_count: i64,
    /// Count of user events imported, but with Document information not found
    /// in the existing Branch.
    #[prost(int64, tag = "4")]
    pub unjoined_events_count: i64,
}
/// Metadata related to the progress of the Import operation. This is
/// returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag = "3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "4")]
    pub failure_count: i64,
}
/// Metadata related to the progress of the ImportDocuments operation. This is
/// returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag = "3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "4")]
    pub failure_count: i64,
}
/// Request message for Import methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsRequest {
    /// Required. The parent branch resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
    /// Requires create/update permission.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "5")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
    /// The mode of reconciliation between existing documents and the documents to
    /// be imported. Defaults to
    /// [ReconciliationMode.INCREMENTAL][google.cloud.discoveryengine.v1alpha.ImportDocumentsRequest.ReconciliationMode.INCREMENTAL].
    #[prost(enumeration = "import_documents_request::ReconciliationMode", tag = "6")]
    pub reconciliation_mode: i32,
    /// Whether to automatically generate IDs for the documents if absent.
    ///
    /// If set to `true`,
    /// [Document.id][google.cloud.discoveryengine.v1alpha.Document.id]s are
    /// automatically generated based on the hash of the payload, where IDs may not
    /// be consistent during multiple imports. In which case
    /// [ReconciliationMode.FULL][google.cloud.discoveryengine.v1alpha.ImportDocumentsRequest.ReconciliationMode.FULL]
    /// is highly recommended to avoid duplicate contents. If unset or set to
    /// `false`, [Document.id][google.cloud.discoveryengine.v1alpha.Document.id]s
    /// have to be specified using
    /// [id_field][google.cloud.discoveryengine.v1alpha.ImportDocumentsRequest.id_field],
    /// otherwise, documents without IDs fail to be imported.
    ///
    /// Only set this field when using
    /// [GcsSource][google.cloud.discoveryengine.v1alpha.GcsSource] or
    /// [BigQuerySource][google.cloud.discoveryengine.v1alpha.BigQuerySource], and
    /// when
    /// [GcsSource.data_schema][google.cloud.discoveryengine.v1alpha.GcsSource.data_schema]
    /// or
    /// [BigQuerySource.data_schema][google.cloud.discoveryengine.v1alpha.BigQuerySource.data_schema]
    /// is `custom` or `csv`. Otherwise, an INVALID_ARGUMENT error is thrown.
    #[prost(bool, tag = "8")]
    pub auto_generate_ids: bool,
    /// The field in the Cloud Storage and BigQuery sources that indicates the
    /// unique IDs of the documents.
    ///
    /// For [GcsSource][google.cloud.discoveryengine.v1alpha.GcsSource] it is the
    /// key of the JSON field. For instance, `my_id` for JSON `{"my_id":
    /// "some_uuid"}`. For
    /// [BigQuerySource][google.cloud.discoveryengine.v1alpha.BigQuerySource] it is
    /// the column name of the BigQuery table where the unique ids are stored.
    ///
    /// The values of the JSON field or the BigQuery column are used as the
    /// [Document.id][google.cloud.discoveryengine.v1alpha.Document.id]s. The JSON
    /// field or the BigQuery column must be of string type, and the values must be
    /// set as valid strings conform to
    /// [RFC-1034](<https://tools.ietf.org/html/rfc1034>) with 1-63 characters.
    /// Otherwise, documents without valid IDs fail to be imported.
    ///
    /// Only set this field when using
    /// [GcsSource][google.cloud.discoveryengine.v1alpha.GcsSource] or
    /// [BigQuerySource][google.cloud.discoveryengine.v1alpha.BigQuerySource], and
    /// when
    /// [GcsSource.data_schema][google.cloud.discoveryengine.v1alpha.GcsSource.data_schema]
    /// or
    /// [BigQuerySource.data_schema][google.cloud.discoveryengine.v1alpha.BigQuerySource.data_schema]
    /// is `custom`. And only set this field when
    /// [auto_generate_ids][google.cloud.discoveryengine.v1alpha.ImportDocumentsRequest.auto_generate_ids]
    /// is unset or set as `false`. Otherwise, an INVALID_ARGUMENT error is thrown.
    ///
    /// If it is unset, a default value `_id` is used when importing from the
    /// allowed data sources.
    #[prost(string, tag = "9")]
    pub id_field: ::prost::alloc::string::String,
    /// Required. The source of the input.
    #[prost(oneof = "import_documents_request::Source", tags = "2, 3, 4")]
    pub source: ::core::option::Option<import_documents_request::Source>,
}
/// Nested message and enum types in `ImportDocumentsRequest`.
pub mod import_documents_request {
    /// The inline source for the input config for ImportDocuments method.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineSource {
        /// Required. A list of documents to update/create. Each document must have a
        /// valid [Document.id][google.cloud.discoveryengine.v1alpha.Document.id].
        /// Recommended max of 100 items.
        #[prost(message, repeated, tag = "1")]
        pub documents: ::prost::alloc::vec::Vec<super::Document>,
    }
    /// Indicates how imported documents are reconciled with the existing documents
    /// created or imported before.
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
    pub enum ReconciliationMode {
        /// Defaults to `INCREMENTAL`.
        Unspecified = 0,
        /// Inserts new documents or updates existing documents.
        Incremental = 1,
        /// Calculates diff and replaces the entire document dataset. Existing
        /// documents may be deleted if they are not present in the source location.
        Full = 2,
    }
    impl ReconciliationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReconciliationMode::Unspecified => "RECONCILIATION_MODE_UNSPECIFIED",
                ReconciliationMode::Incremental => "INCREMENTAL",
                ReconciliationMode::Full => "FULL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RECONCILIATION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "INCREMENTAL" => Some(Self::Incremental),
                "FULL" => Some(Self::Full),
                _ => None,
            }
        }
    }
    /// Required. The source of the input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Inline source for the input content for documents.
        #[prost(message, tag = "2")]
        InlineSource(InlineSource),
        /// Cloud Storage location for the input content.
        #[prost(message, tag = "3")]
        GcsSource(super::GcsSource),
        /// BigQuery input source.
        #[prost(message, tag = "4")]
        BigquerySource(super::BigQuerySource),
    }
}
/// Response of the
/// [ImportDocumentsRequest][google.cloud.discoveryengine.v1alpha.ImportDocumentsRequest].
/// If the long running operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag = "2")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
}
/// Request message for PurgeUserEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsRequest {
    /// Required. The resource name of the catalog under which the events are
    /// created. The format is
    /// `projects/${projectId}/locations/global/collections/{$collectionId}/dataStores/${dataStoreId}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The filter string to specify the events to be deleted with a
    /// length limit of 5,000 characters. The eligible fields for filtering are:
    ///
    /// * `eventType`: Double quoted
    /// [UserEvent.event_type][google.cloud.discoveryengine.v1alpha.UserEvent.event_type]
    /// string.
    /// * `eventTime`: in ISO 8601 "zulu" format.
    /// * `userPseudoId`: Double quoted string. Specifying this will delete all
    ///    events associated with a visitor.
    /// * `userId`: Double quoted string. Specifying this will delete all events
    ///    associated with a user.
    ///
    /// Examples:
    ///
    /// * Deleting all events in a time range:
    ///    `eventTime > "2012-04-23T18:25:43.511Z"
    ///    eventTime < "2012-04-23T18:30:43.511Z"`
    /// * Deleting specific eventType:
    ///    `eventType = "search"`
    /// * Deleting all events for a specific visitor:
    ///    `userPseudoId = "visitor1024"`
    /// * Deleting all events inside a DataStore:
    ///    `*`
    ///
    /// The filtering fields are assumed to have an implicit AND.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The `force` field is currently not supported. Purge user event requests
    /// will permanently delete all purgeable events. Once the development is
    /// complete:
    /// If `force` is set to false, the method will return the expected
    /// purge count without deleting any user events. This field will default to
    /// false if not included in the request.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Response of the PurgeUserEventsRequest. If the long running operation is
/// successfully done, then this message is returned by the
/// google.longrunning.Operations.response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsResponse {
    /// The total count of events purged as a result of the operation.
    #[prost(int64, tag = "1")]
    pub purge_count: i64,
}
/// Metadata related to the progress of the PurgeUserEvents operation.
/// This will be returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were deleted successfully.
    #[prost(int64, tag = "3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "4")]
    pub failure_count: i64,
}
/// Request message for
/// [DocumentService.PurgeDocuments][google.cloud.discoveryengine.v1alpha.DocumentService.PurgeDocuments]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeDocumentsRequest {
    /// Required. The parent resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Filter matching documents to purge. Only currently supported
    /// value is
    /// `*` (all items).
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Actually performs the purge. If `force` is set to false, return the
    /// expected purge count without deleting any documents.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Response message for
/// [DocumentService.PurgeDocuments][google.cloud.discoveryengine.v1alpha.DocumentService.PurgeDocuments]
/// method. If the long running operation is successfully done, then this message
/// is returned by the google.longrunning.Operations.response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeDocumentsResponse {
    /// The total count of documents purged as a result of the operation.
    #[prost(int64, tag = "1")]
    pub purge_count: i64,
    /// A sample of document names that will be deleted. Only populated if `force`
    /// is set to false. A max of 100 names will be returned and the names are
    /// chosen at random.
    #[prost(string, repeated, tag = "2")]
    pub purge_sample: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata related to the progress of the PurgeDocuments operation.
/// This will be returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeDocumentsMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were deleted successfully.
    #[prost(int64, tag = "3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "4")]
    pub failure_count: i64,
}
/// Request message for
/// [DocumentService.GetDocument][google.cloud.discoveryengine.v1alpha.DocumentService.GetDocument]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentRequest {
    /// Required. Full resource name of
    /// [Document][google.cloud.discoveryengine.v1alpha.Document], such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document}`.
    ///
    /// If the caller does not have permission to access the
    /// [Document][google.cloud.discoveryengine.v1alpha.Document], regardless of
    /// whether or not it exists, a `PERMISSION_DENIED` error is returned.
    ///
    /// If the requested [Document][google.cloud.discoveryengine.v1alpha.Document]
    /// does not exist, a `NOT_FOUND` error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1alpha.DocumentService.ListDocuments]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsRequest {
    /// Required. The parent branch resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
    /// Use `default_branch` as the branch ID, to list documents under the default
    /// branch.
    ///
    /// If the caller does not have permission to list
    /// [Document][google.cloud.discoveryengine.v1alpha.Document]s under this
    /// branch, regardless of whether or not this branch exists, a
    /// `PERMISSION_DENIED` error is returned.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of
    /// [Document][google.cloud.discoveryengine.v1alpha.Document]s to return. If
    /// unspecified, defaults to 100. The maximum allowed value is 1000. Values
    /// above 1000 will be coerced to 1000.
    ///
    /// If this field is negative, an `INVALID_ARGUMENT` error is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token
    /// [ListDocumentsResponse.next_page_token][google.cloud.discoveryengine.v1alpha.ListDocumentsResponse.next_page_token],
    /// received from a previous
    /// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1alpha.DocumentService.ListDocuments]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1alpha.DocumentService.ListDocuments]
    /// must match the call that provided the page token. Otherwise, an
    /// `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1alpha.DocumentService.ListDocuments]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentsResponse {
    /// The [Document][google.cloud.discoveryengine.v1alpha.Document]s.
    #[prost(message, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<Document>,
    /// A token that can be sent as
    /// [ListDocumentsRequest.page_token][google.cloud.discoveryengine.v1alpha.ListDocumentsRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// [DocumentService.CreateDocument][google.cloud.discoveryengine.v1alpha.DocumentService.CreateDocument]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentRequest {
    /// Required. The parent resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Document][google.cloud.discoveryengine.v1alpha.Document] to
    /// create.
    #[prost(message, optional, tag = "2")]
    pub document: ::core::option::Option<Document>,
    /// Required. The ID to use for the
    /// [Document][google.cloud.discoveryengine.v1alpha.Document], which will
    /// become the final component of the
    /// [Document.name][google.cloud.discoveryengine.v1alpha.Document.name].
    ///
    /// If the caller does not have permission to create the
    /// [Document][google.cloud.discoveryengine.v1alpha.Document], regardless of
    /// whether or not it exists, a `PERMISSION_DENIED` error is returned.
    ///
    /// This field must be unique among all
    /// [Document][google.cloud.discoveryengine.v1alpha.Document]s with the same
    /// [parent][google.cloud.discoveryengine.v1alpha.CreateDocumentRequest.parent].
    /// Otherwise, an `ALREADY_EXISTS` error is returned.
    ///
    /// This field must conform to [RFC-1034](<https://tools.ietf.org/html/rfc1034>)
    /// standard with a length limit of 63 characters. Otherwise, an
    /// `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "3")]
    pub document_id: ::prost::alloc::string::String,
}
/// Request message for
/// [DocumentService.UpdateDocument][google.cloud.discoveryengine.v1alpha.DocumentService.UpdateDocument]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentRequest {
    /// Required. The document to update/create.
    ///
    /// If the caller does not have permission to update the
    /// [Document][google.cloud.discoveryengine.v1alpha.Document], regardless of
    /// whether or not it exists, a `PERMISSION_DENIED` error is returned.
    ///
    /// If the [Document][google.cloud.discoveryengine.v1alpha.Document] to update
    /// does not exist and
    /// [allow_missing][google.cloud.discoveryengine.v1alpha.UpdateDocumentRequest.allow_missing]
    /// is not set, a `NOT_FOUND` error is returned.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// If set to true, and the
    /// [Document][google.cloud.discoveryengine.v1alpha.Document] is not found, a
    /// new [Document][google.cloud.discoveryengine.v1alpha.Document] will be
    /// created.
    #[prost(bool, tag = "2")]
    pub allow_missing: bool,
}
/// Request message for
/// [DocumentService.DeleteDocument][google.cloud.discoveryengine.v1alpha.DocumentService.DeleteDocument]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentRequest {
    /// Required. Full resource name of
    /// [Document][google.cloud.discoveryengine.v1alpha.Document], such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/branches/{branch}/documents/{document}`.
    ///
    /// If the caller does not have permission to delete the
    /// [Document][google.cloud.discoveryengine.v1alpha.Document], regardless of
    /// whether or not it exists, a `PERMISSION_DENIED` error is returned.
    ///
    /// If the [Document][google.cloud.discoveryengine.v1alpha.Document] to delete
    /// does not exist, a `NOT_FOUND` error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod document_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for ingesting
    /// [Document][google.cloud.discoveryengine.v1alpha.Document] information of the
    /// customer's website.
    #[derive(Debug, Clone)]
    pub struct DocumentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DocumentServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DocumentServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DocumentServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Gets a [Document][google.cloud.discoveryengine.v1alpha.Document].
        pub async fn get_document(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DocumentService/GetDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DocumentService",
                        "GetDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of [Document][google.cloud.discoveryengine.v1alpha.Document]s.
        pub async fn list_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDocumentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DocumentService/ListDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DocumentService",
                        "ListDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a [Document][google.cloud.discoveryengine.v1alpha.Document].
        pub async fn create_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DocumentService/CreateDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DocumentService",
                        "CreateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a [Document][google.cloud.discoveryengine.v1alpha.Document].
        pub async fn update_document(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentRequest>,
        ) -> std::result::Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DocumentService/UpdateDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DocumentService",
                        "UpdateDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a [Document][google.cloud.discoveryengine.v1alpha.Document].
        pub async fn delete_document(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DocumentService/DeleteDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DocumentService",
                        "DeleteDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Bulk import of multiple
        /// [Document][google.cloud.discoveryengine.v1alpha.Document]s. Request
        /// processing may be synchronous. Non-existing items will be created.
        ///
        /// Note: It is possible for a subset of the
        /// [Document][google.cloud.discoveryengine.v1alpha.Document]s to be
        /// successfully updated.
        pub async fn import_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DocumentService/ImportDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DocumentService",
                        "ImportDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Permanently deletes all selected
        /// [Document][google.cloud.discoveryengine.v1alpha.Document]s in a branch.
        ///
        /// This process is asynchronous. Depending on the number of
        /// [Document][google.cloud.discoveryengine.v1alpha.Document]s to be deleted,
        /// this operation can take hours to complete. Before the delete operation
        /// completes, some [Document][google.cloud.discoveryengine.v1alpha.Document]s
        /// might still be returned by
        /// [DocumentService.GetDocument][google.cloud.discoveryengine.v1alpha.DocumentService.GetDocument]
        /// or
        /// [DocumentService.ListDocuments][google.cloud.discoveryengine.v1alpha.DocumentService.ListDocuments].
        ///
        /// To get a list of the
        /// [Document][google.cloud.discoveryengine.v1alpha.Document]s to be deleted,
        /// set
        /// [PurgeDocumentsRequest.force][google.cloud.discoveryengine.v1alpha.PurgeDocumentsRequest.force]
        /// to false.
        pub async fn purge_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.DocumentService/PurgeDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.DocumentService",
                        "PurgeDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for
/// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    /// Required. The resource name of the Search serving config, such as
    /// `projects/*/locations/global/collections/default_collection/engines/*/servingConfigs/default_serving_config`,
    /// or
    /// `projects/*/locations/global/collections/default_collection/dataStores/default_data_store/servingConfigs/default_serving_config`.
    /// This field is used to identify the serving configuration name, set
    /// of models used to make the search.
    #[prost(string, tag = "1")]
    pub serving_config: ::prost::alloc::string::String,
    /// The branch resource name, such as
    /// `projects/*/locations/global/collections/default_collection/dataStores/default_data_store/branches/0`.
    ///
    /// Use `default_branch` as the branch ID or leave this field empty, to search
    /// documents under the default branch.
    #[prost(string, tag = "2")]
    pub branch: ::prost::alloc::string::String,
    /// Raw search query.
    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,
    /// Raw image query.
    #[prost(message, optional, tag = "19")]
    pub image_query: ::core::option::Option<search_request::ImageQuery>,
    /// Maximum number of
    /// [Document][google.cloud.discoveryengine.v1alpha.Document]s to return. If
    /// unspecified, defaults to a reasonable value. The maximum allowed value is
    /// 100. Values above 100 are coerced to 100.
    ///
    /// If this field is negative, an  `INVALID_ARGUMENT`  is returned.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// A page token received from a previous
    /// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search]
    /// must match the call that provided the page token. Otherwise, an
    ///   `INVALID_ARGUMENT`  error is returned.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// A 0-indexed integer that specifies the current offset (that is, starting
    /// result location, amongst the
    /// [Document][google.cloud.discoveryengine.v1alpha.Document]s deemed by the
    /// API as relevant) in search results. This field is only considered if
    /// [page_token][google.cloud.discoveryengine.v1alpha.SearchRequest.page_token]
    /// is unset.
    ///
    /// If this field is negative, an  `INVALID_ARGUMENT`  is returned.
    #[prost(int32, tag = "6")]
    pub offset: i32,
    /// The filter syntax consists of an expression language for constructing a
    /// predicate from one or more fields of the documents being filtered. Filter
    /// expression is case-sensitive.
    ///
    /// If this field is unrecognizable, an  `INVALID_ARGUMENT`  is returned.
    ///
    /// Filtering in Vertex AI Search is done by mapping the LHS filter key to a
    /// key property defined in the Vertex AI Search backend -- this mapping is
    /// defined by the customer in their schema. For example a media customer might
    /// have a field 'name' in their schema. In this case the filter would look
    /// like this: filter --> name:'ANY("king kong")'
    ///
    /// For more information about filtering including syntax and filter
    /// operators, see
    /// [Filter](<https://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata>)
    #[prost(string, tag = "7")]
    pub filter: ::prost::alloc::string::String,
    /// The default filter that is applied when a user performs a search without
    /// checking any filters on the search page.
    ///
    /// The filter applied to every search request when quality improvement such as
    /// query expansion is needed. In the case a query does not have a sufficient
    /// amount of results this filter will be used to determine whether or not to
    /// enable the query expansion flow. The original filter will still be used for
    /// the query expanded search.
    /// This field is strongly recommended to achieve high search quality.
    ///
    /// For more information about filter syntax, see
    /// [SearchRequest.filter][google.cloud.discoveryengine.v1alpha.SearchRequest.filter].
    #[prost(string, tag = "29")]
    pub canonical_filter: ::prost::alloc::string::String,
    /// The order in which documents are returned. Documents can be ordered by
    /// a field in an [Document][google.cloud.discoveryengine.v1alpha.Document]
    /// object. Leave it unset if ordered by relevance. `order_by` expression is
    /// case-sensitive. For more information on ordering, see
    /// [Ordering](<https://cloud.google.com/retail/docs/filter-and-order#order>)
    ///
    /// If this field is unrecognizable, an `INVALID_ARGUMENT` is returned.
    #[prost(string, tag = "8")]
    pub order_by: ::prost::alloc::string::String,
    /// Information about the end user.
    /// Highly recommended for analytics.
    /// [UserInfo.user_agent][google.cloud.discoveryengine.v1alpha.UserInfo.user_agent]
    /// is used to deduce `device_type` for analytics.
    #[prost(message, optional, tag = "21")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// Facet specifications for faceted search. If empty, no facets are returned.
    ///
    /// A maximum of 100 values are allowed. Otherwise, an  `INVALID_ARGUMENT`
    /// error is returned.
    #[prost(message, repeated, tag = "9")]
    pub facet_specs: ::prost::alloc::vec::Vec<search_request::FacetSpec>,
    /// Boost specification to boost certain documents.
    /// For more information on boosting, see
    /// [Boosting](<https://cloud.google.com/retail/docs/boosting#boost>)
    #[prost(message, optional, tag = "10")]
    pub boost_spec: ::core::option::Option<search_request::BoostSpec>,
    /// Additional search parameters.
    ///
    /// For public website search only, supported values are:
    ///
    /// * `user_country_code`: string. Default empty. If set to non-empty, results
    ///     are restricted or boosted based on the location provided.
    ///     Example:
    ///     user_country_code: "au"
    ///
    ///     For available codes see [Country
    ///     Codes](<https://developers.google.com/custom-search/docs/json_api_reference#countryCodes>)
    ///
    /// * `search_type`: double. Default empty. Enables non-webpage searching
    ///     depending on the value. The only valid non-default value is 1,
    ///     which enables image searching.
    ///     Example:
    ///     search_type: 1
    #[prost(btree_map = "string, message", tag = "11")]
    pub params: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// The query expansion specification that specifies the conditions under which
    /// query expansion occurs.
    #[prost(message, optional, tag = "13")]
    pub query_expansion_spec: ::core::option::Option<search_request::QueryExpansionSpec>,
    /// The spell correction specification that specifies the mode under
    /// which spell correction takes effect.
    #[prost(message, optional, tag = "14")]
    pub spell_correction_spec: ::core::option::Option<
        search_request::SpellCorrectionSpec,
    >,
    /// A unique identifier for tracking visitors. For example, this could be
    /// implemented with an HTTP cookie, which should be able to uniquely identify
    /// a visitor on a single device. This unique identifier should not change if
    /// the visitor logs in or out of the website.
    ///
    /// This field should NOT have a fixed value such as `unknown_visitor`.
    ///
    /// This should be the same identifier as
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1alpha.UserEvent.user_pseudo_id]
    /// and
    /// [CompleteQueryRequest.user_pseudo_id][google.cloud.discoveryengine.v1alpha.CompleteQueryRequest.user_pseudo_id]
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an  `INVALID_ARGUMENT`  error is returned.
    #[prost(string, tag = "15")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// A specification for configuring the behavior of content search.
    #[prost(message, optional, tag = "24")]
    pub content_search_spec: ::core::option::Option<search_request::ContentSearchSpec>,
    /// Uses the provided embedding to do additional semantic document retrieval.
    /// The retrieval is based on the dot product of
    /// [SearchRequest.embedding_spec.embedding_vectors.vector][] and the document
    /// embedding that is provided in
    /// [SearchRequest.embedding_spec.embedding_vectors.field_path][].
    ///
    /// If [SearchRequest.embedding_spec.embedding_vectors.field_path][] is not
    /// provided, it will use [ServingConfig.embedding_config.field_paths][].
    #[prost(message, optional, tag = "23")]
    pub embedding_spec: ::core::option::Option<search_request::EmbeddingSpec>,
    /// The ranking expression controls the customized ranking on retrieval
    /// documents. This overrides
    /// [ServingConfig.ranking_expression][google.cloud.discoveryengine.v1alpha.ServingConfig.ranking_expression].
    /// The ranking expression is a single function or multiple functions that are
    /// joint by "+".
    ///    * ranking_expression = function, { " + ", function };
    /// Supported functions:
    ///    * double * relevance_score
    ///    * double * dotProduct(embedding_field_path)
    /// Function variables:
    ///    `relevance_score`: pre-defined keywords, used for measure relevance
    ///    between query and document.
    ///    `embedding_field_path`: the document embedding field
    ///    used with query embedding vector.
    ///    `dotProduct`: embedding function between embedding_field_path and query
    ///    embedding vector.
    ///
    ///   Example ranking expression:
    ///     If document has an embedding field doc_embedding, the ranking expression
    ///     could be `0.5 * relevance_score + 0.3 * dotProduct(doc_embedding)`.
    #[prost(string, tag = "26")]
    pub ranking_expression: ::prost::alloc::string::String,
    /// Whether to turn on safe search. This is only supported for
    /// website search.
    #[prost(bool, tag = "20")]
    pub safe_search: bool,
    /// The user labels applied to a resource must meet the following requirements:
    ///
    /// * Each resource can have multiple labels, up to a maximum of 64.
    /// * Each label must be a key-value pair.
    /// * Keys have a minimum length of 1 character and a maximum length of 63
    ///    characters and cannot be empty. Values can be empty and have a maximum
    ///    length of 63 characters.
    /// * Keys and values can contain only lowercase letters, numeric characters,
    ///    underscores, and dashes. All characters must use UTF-8 encoding, and
    ///    international characters are allowed.
    /// * The key portion of a label must be unique. However, you can use the same
    ///    key with multiple resources.
    /// * Keys must start with a lowercase letter or international character.
    ///
    /// See [Google Cloud
    /// Document](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>)
    /// for more details.
    #[prost(btree_map = "string, string", tag = "22")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `SearchRequest`.
pub mod search_request {
    /// Specifies the image query input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImageQuery {
        #[prost(oneof = "image_query::Image", tags = "1")]
        pub image: ::core::option::Option<image_query::Image>,
    }
    /// Nested message and enum types in `ImageQuery`.
    pub mod image_query {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Image {
            /// Base64 encoded image bytes. Supported image formats: JPEG, PNG, and
            /// BMP.
            #[prost(string, tag = "1")]
            ImageBytes(::prost::alloc::string::String),
        }
    }
    /// A facet specification to perform faceted search.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FacetSpec {
        /// Required. The facet key specification.
        #[prost(message, optional, tag = "1")]
        pub facet_key: ::core::option::Option<facet_spec::FacetKey>,
        /// Maximum of facet values that should be returned for this facet. If
        /// unspecified, defaults to 20. The maximum allowed value is 300. Values
        /// above 300 are coerced to 300.
        ///
        /// If this field is negative, an  `INVALID_ARGUMENT`  is returned.
        #[prost(int32, tag = "2")]
        pub limit: i32,
        /// List of keys to exclude when faceting.
        ///
        ///
        /// By default,
        /// [FacetKey.key][google.cloud.discoveryengine.v1alpha.SearchRequest.FacetSpec.FacetKey.key]
        /// is not excluded from the filter unless it is listed in this field.
        ///
        /// Listing a facet key in this field allows its values to appear as facet
        /// results, even when they are filtered out of search results. Using this
        /// field does not affect what search results are returned.
        ///
        /// For example, suppose there are 100 documents with the color facet "Red"
        /// and 200 documents with the color facet "Blue". A query containing the
        /// filter "color:ANY("Red")" and having "color" as
        /// [FacetKey.key][google.cloud.discoveryengine.v1alpha.SearchRequest.FacetSpec.FacetKey.key]
        /// would by default return only "Red" documents in the search results, and
        /// also return "Red" with count 100 as the only color facet. Although there
        /// are also blue documents available, "Blue" would not be shown as an
        /// available facet value.
        ///
        /// If "color" is listed in "excludedFilterKeys", then the query returns the
        /// facet values "Red" with count 100 and "Blue" with count 200, because the
        /// "color" key is now excluded from the filter. Because this field doesn't
        /// affect search results, the search results are still correctly filtered to
        /// return only "Red" documents.
        ///
        /// A maximum of 100 values are allowed. Otherwise, an  `INVALID_ARGUMENT`
        /// error is returned.
        #[prost(string, repeated, tag = "3")]
        pub excluded_filter_keys: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Enables dynamic position for this facet. If set to true, the position of
        /// this facet among all facets in the response is determined automatically.
        /// If dynamic facets are enabled, it is ordered together.
        /// If set to false, the position of this facet in the
        /// response is the same as in the request, and it is ranked before
        /// the facets with dynamic position enable and all dynamic facets.
        ///
        /// For example, you may always want to have rating facet returned in
        /// the response, but it's not necessarily to always display the rating facet
        /// at the top. In that case, you can set enable_dynamic_position to true so
        /// that the position of rating facet in response is determined
        /// automatically.
        ///
        /// Another example, assuming you have the following facets in the request:
        ///
        /// * "rating", enable_dynamic_position = true
        ///
        /// * "price", enable_dynamic_position = false
        ///
        /// * "brands", enable_dynamic_position = false
        ///
        /// And also you have a dynamic facets enabled, which generates a facet
        /// `gender`. Then the final order of the facets in the response can be
        /// ("price", "brands", "rating", "gender") or ("price", "brands", "gender",
        /// "rating") depends on how API orders "gender" and "rating" facets.
        /// However, notice that "price" and "brands" are always
        /// ranked at first and second position because their enable_dynamic_position
        /// is false.
        #[prost(bool, tag = "4")]
        pub enable_dynamic_position: bool,
    }
    /// Nested message and enum types in `FacetSpec`.
    pub mod facet_spec {
        /// Specifies how a facet is computed.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FacetKey {
            /// Required. Supported textual and numerical facet keys in
            /// [Document][google.cloud.discoveryengine.v1alpha.Document] object, over
            /// which the facet values are computed. Facet key is case-sensitive.
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
            /// Set only if values should be bucketed into intervals. Must be set
            /// for facets with numerical values. Must not be set for facet with text
            /// values. Maximum number of intervals is 30.
            #[prost(message, repeated, tag = "2")]
            pub intervals: ::prost::alloc::vec::Vec<super::super::Interval>,
            /// Only get facet for the given restricted values. Only supported on
            /// textual fields. For example, suppose "category" has three values
            /// "Action > 2022", "Action > 2021" and "Sci-Fi > 2022". If set
            /// "restricted_values" to "Action > 2022", the "category" facet only
            /// contains "Action > 2022". Only supported on textual fields. Maximum
            /// is 10.
            #[prost(string, repeated, tag = "3")]
            pub restricted_values: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            /// Only get facet values that start with the given string prefix. For
            /// example, suppose "category" has three values "Action > 2022",
            /// "Action > 2021" and "Sci-Fi > 2022". If set "prefixes" to "Action", the
            /// "category" facet only contains "Action > 2022" and "Action > 2021".
            /// Only supported on textual fields. Maximum is 10.
            #[prost(string, repeated, tag = "4")]
            pub prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Only get facet values that contains the given strings. For example,
            /// suppose "category" has three values "Action > 2022",
            /// "Action > 2021" and "Sci-Fi > 2022". If set "contains" to "2022", the
            /// "category" facet only contains "Action > 2022" and "Sci-Fi > 2022".
            /// Only supported on textual fields. Maximum is 10.
            #[prost(string, repeated, tag = "5")]
            pub contains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// True to make facet keys case insensitive when getting faceting
            /// values with prefixes or contains; false otherwise.
            #[prost(bool, tag = "6")]
            pub case_insensitive: bool,
            /// The order in which documents are returned.
            ///
            /// Allowed values are:
            ///
            /// * "count desc", which means order by
            /// [SearchResponse.Facet.values.count][google.cloud.discoveryengine.v1alpha.SearchResponse.Facet.FacetValue.count]
            /// descending.
            ///
            /// * "value desc", which means order by
            /// [SearchResponse.Facet.values.value][google.cloud.discoveryengine.v1alpha.SearchResponse.Facet.FacetValue.value]
            /// descending.
            ///    Only applies to textual facets.
            ///
            /// If not set, textual values are sorted in [natural
            /// order](<https://en.wikipedia.org/wiki/Natural_sort_order>); numerical
            /// intervals are sorted in the order given by
            /// [FacetSpec.FacetKey.intervals][google.cloud.discoveryengine.v1alpha.SearchRequest.FacetSpec.FacetKey.intervals].
            #[prost(string, tag = "7")]
            pub order_by: ::prost::alloc::string::String,
        }
    }
    /// Boost specification to boost certain documents.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BoostSpec {
        /// Condition boost specifications. If a document matches multiple conditions
        /// in the specifictions, boost scores from these specifications are all
        /// applied and combined in a non-linear way. Maximum number of
        /// specifications is 20.
        #[prost(message, repeated, tag = "1")]
        pub condition_boost_specs: ::prost::alloc::vec::Vec<
            boost_spec::ConditionBoostSpec,
        >,
    }
    /// Nested message and enum types in `BoostSpec`.
    pub mod boost_spec {
        /// Boost applies to documents which match a condition.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConditionBoostSpec {
            /// An expression which specifies a boost condition. The syntax and
            /// supported fields are the same as a filter expression. See
            /// [SearchRequest.filter][google.cloud.discoveryengine.v1alpha.SearchRequest.filter]
            /// for detail syntax and limitations.
            ///
            /// Examples:
            ///
            /// * To boost documents with document ID "doc_1" or "doc_2", and
            /// color
            ///    "Red" or "Blue":
            ///      * (id: ANY("doc_1", "doc_2")) AND (color: ANY("Red","Blue"))
            #[prost(string, tag = "1")]
            pub condition: ::prost::alloc::string::String,
            /// Strength of the condition boost, which should be in \[-1, 1\]. Negative
            /// boost means demotion. Default is 0.0.
            ///
            /// Setting to 1.0 gives the document a big promotion. However, it does not
            /// necessarily mean that the boosted document will be the top result at
            /// all times, nor that other documents will be excluded. Results could
            /// still be shown even when none of them matches the condition. And
            /// results that are significantly more relevant to the search query can
            /// still trump your heavily favored but irrelevant documents.
            ///
            /// Setting to -1.0 gives the document a big demotion. However, results
            /// that are deeply relevant might still be shown. The document will have
            /// an upstream battle to get a fairly high ranking, but it is not blocked
            /// out completely.
            ///
            /// Setting to 0.0 means no boost applied. The boosting condition is
            /// ignored.
            #[prost(float, tag = "2")]
            pub boost: f32,
        }
    }
    /// Specification to determine under which conditions query expansion should
    /// occur.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryExpansionSpec {
        /// The condition under which query expansion should occur. Default to
        /// [Condition.DISABLED][google.cloud.discoveryengine.v1alpha.SearchRequest.QueryExpansionSpec.Condition.DISABLED].
        #[prost(enumeration = "query_expansion_spec::Condition", tag = "1")]
        pub condition: i32,
        /// Whether to pin unexpanded results. If this field is set to true,
        /// unexpanded products are always at the top of the search results, followed
        /// by the expanded results.
        #[prost(bool, tag = "2")]
        pub pin_unexpanded_results: bool,
    }
    /// Nested message and enum types in `QueryExpansionSpec`.
    pub mod query_expansion_spec {
        /// Enum describing under which condition query expansion should occur.
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
        pub enum Condition {
            /// Unspecified query expansion condition. In this case, server behavior
            /// defaults to
            /// [Condition.DISABLED][google.cloud.discoveryengine.v1alpha.SearchRequest.QueryExpansionSpec.Condition.DISABLED].
            Unspecified = 0,
            /// Disabled query expansion. Only the exact search query is used, even if
            /// [SearchResponse.total_size][google.cloud.discoveryengine.v1alpha.SearchResponse.total_size]
            /// is zero.
            Disabled = 1,
            /// Automatic query expansion built by the Search API.
            Auto = 2,
        }
        impl Condition {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Condition::Unspecified => "CONDITION_UNSPECIFIED",
                    Condition::Disabled => "DISABLED",
                    Condition::Auto => "AUTO",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CONDITION_UNSPECIFIED" => Some(Self::Unspecified),
                    "DISABLED" => Some(Self::Disabled),
                    "AUTO" => Some(Self::Auto),
                    _ => None,
                }
            }
        }
    }
    /// The specification for query spell correction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpellCorrectionSpec {
        /// The mode under which spell correction should take effect to
        /// replace the original search query. Default to
        /// [Mode.AUTO][google.cloud.discoveryengine.v1alpha.SearchRequest.SpellCorrectionSpec.Mode.AUTO].
        #[prost(enumeration = "spell_correction_spec::Mode", tag = "1")]
        pub mode: i32,
    }
    /// Nested message and enum types in `SpellCorrectionSpec`.
    pub mod spell_correction_spec {
        /// Enum describing under which mode spell correction should occur.
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
        pub enum Mode {
            /// Unspecified spell correction mode. In this case, server behavior
            /// defaults to
            /// [Mode.AUTO][google.cloud.discoveryengine.v1alpha.SearchRequest.SpellCorrectionSpec.Mode.AUTO].
            Unspecified = 0,
            /// Search API will try to find a spell suggestion if there
            /// is any and put in the
            /// [SearchResponse.corrected_query][google.cloud.discoveryengine.v1alpha.SearchResponse.corrected_query].
            /// The spell suggestion will not be used as the search query.
            SuggestionOnly = 1,
            /// Automatic spell correction built by the Search API. Search will
            /// be based on the corrected query if found.
            Auto = 2,
        }
        impl Mode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Mode::Unspecified => "MODE_UNSPECIFIED",
                    Mode::SuggestionOnly => "SUGGESTION_ONLY",
                    Mode::Auto => "AUTO",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                    "SUGGESTION_ONLY" => Some(Self::SuggestionOnly),
                    "AUTO" => Some(Self::Auto),
                    _ => None,
                }
            }
        }
    }
    /// A specification for configuring the behavior of content search.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContentSearchSpec {
        /// If `snippetSpec` is not specified, snippets are not included in the
        /// search response.
        #[prost(message, optional, tag = "1")]
        pub snippet_spec: ::core::option::Option<content_search_spec::SnippetSpec>,
        /// If `summarySpec` is not specified, summaries are not included in the
        /// search response.
        #[prost(message, optional, tag = "2")]
        pub summary_spec: ::core::option::Option<content_search_spec::SummarySpec>,
        /// If there is no extractive_content_spec provided, there will be no
        /// extractive answer in the search response.
        #[prost(message, optional, tag = "3")]
        pub extractive_content_spec: ::core::option::Option<
            content_search_spec::ExtractiveContentSpec,
        >,
    }
    /// Nested message and enum types in `ContentSearchSpec`.
    pub mod content_search_spec {
        /// A specification for configuring snippets in a search response.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SnippetSpec {
            /// \[DEPRECATED\] This field is deprecated. To control snippet return, use
            /// `return_snippet` field. For backwards compatibility, we will return
            /// snippet if max_snippet_count > 0.
            #[deprecated]
            #[prost(int32, tag = "1")]
            pub max_snippet_count: i32,
            /// \[DEPRECATED\] This field is deprecated and will have no affect on the
            /// snippet.
            #[deprecated]
            #[prost(bool, tag = "2")]
            pub reference_only: bool,
            /// If `true`, then return snippet. If no snippet can be generated, we
            /// return "No snippet is available for this page." A `snippet_status` with
            /// `SUCCESS` or `NO_SNIPPET_AVAILABLE` will also be returned.
            #[prost(bool, tag = "3")]
            pub return_snippet: bool,
        }
        /// A specification for configuring a summary returned in a search
        /// response.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SummarySpec {
            /// The number of top results to generate the summary from. If the number
            /// of results returned is less than `summaryResultCount`, the summary is
            /// generated from all of the results.
            ///
            /// At most five results can be used to generate a summary.
            #[prost(int32, tag = "1")]
            pub summary_result_count: i32,
            /// Specifies whether to include citations in the summary. The default
            /// value is `false`.
            ///
            /// When this field is set to `true`, summaries include in-line citation
            /// numbers.
            ///
            /// Example summary including citations:
            ///
            /// BigQuery is Google Cloud's fully managed and completely serverless
            /// enterprise data warehouse \[1\]. BigQuery supports all data types, works
            /// across clouds, and has built-in machine learning and business
            /// intelligence, all within a unified platform \[2, 3\].
            ///
            /// The citation numbers refer to the returned search results and are
            /// 1-indexed. For example, \[1\] means that the sentence is attributed to
            /// the first search result. \[2, 3\] means that the sentence is attributed
            /// to both the second and third search results.
            #[prost(bool, tag = "2")]
            pub include_citations: bool,
            /// Specifies whether to filter out adversarial queries. The default value
            /// is `false`.
            ///
            /// Google employs search-query classification to detect adversarial
            /// queries. No summary is returned if the search query is classified as an
            /// adversarial query. For example, a user might ask a question regarding
            /// negative comments about the company or submit a query designed to
            /// generate unsafe, policy-violating output. If this field is set to
            /// `true`, we skip generating summaries for adversarial queries and return
            /// fallback messages instead.
            #[prost(bool, tag = "3")]
            pub ignore_adversarial_query: bool,
            /// Specifies whether to filter out queries that are not summary-seeking.
            /// The default value is `false`.
            ///
            /// Google employs search-query classification to detect summary-seeking
            /// queries. No summary is returned if the search query is classified as a
            /// non-summary seeking query. For example, `why is the sky blue` and `Who
            /// is the best soccer player in the world?` are summary-seeking queries,
            /// but `SFO airport` and `world cup 2026` are not. They are most likely
            /// navigational queries. If this field is set to `true`, we skip
            /// generating summaries for non-summary seeking queries and return
            /// fallback messages instead.
            #[prost(bool, tag = "4")]
            pub ignore_non_summary_seeking_query: bool,
            /// If specified, the spec will be used to modify the prompt provided to
            /// the LLM.
            #[prost(message, optional, tag = "5")]
            pub model_prompt_spec: ::core::option::Option<summary_spec::ModelPromptSpec>,
            /// Language code for Summary. Use language tags defined by
            /// [BCP47](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>).
            /// Note: This is an experimental feature.
            #[prost(string, tag = "6")]
            pub language_code: ::prost::alloc::string::String,
            /// If specified, the spec will be used to modify the model specification
            /// provided to the LLM.
            #[prost(message, optional, tag = "7")]
            pub model_spec: ::core::option::Option<summary_spec::ModelSpec>,
        }
        /// Nested message and enum types in `SummarySpec`.
        pub mod summary_spec {
            /// Specification of the prompt to use with the model.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ModelPromptSpec {
                /// Text at the beginning of the prompt that instructs the assistant.
                /// Examples are available in the user guide.
                #[prost(string, tag = "1")]
                pub preamble: ::prost::alloc::string::String,
            }
            /// Specification of the model.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ModelSpec {
                /// The string format of the model version.
                /// e.g. stable, preview, etc.
                #[prost(string, tag = "1")]
                pub version: ::prost::alloc::string::String,
            }
        }
        /// A specification for configuring the extractive content in a search
        /// response.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExtractiveContentSpec {
            /// The maximum number of extractive answers returned in each search
            /// result.
            ///
            /// An extractive answer is a verbatim answer extracted from the original
            /// document, which provides a precise and contextually relevant answer to
            /// the search query.
            ///
            /// If the number of matching answers is less than the
            /// `max_extractive_answer_count`, return all of the answers. Otherwise,
            /// return the `max_extractive_answer_count`.
            ///
            /// At most one answer is returned for each
            /// [SearchResult][google.cloud.discoveryengine.v1alpha.SearchResponse.SearchResult].
            #[prost(int32, tag = "1")]
            pub max_extractive_answer_count: i32,
            /// The max number of extractive segments returned in each search result.
            /// Only applied if the
            /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore] is set to
            /// [DataStore.ContentConfig.CONTENT_REQUIRED][google.cloud.discoveryengine.v1alpha.DataStore.ContentConfig.CONTENT_REQUIRED]
            /// or
            /// [DataStore.solution_types][google.cloud.discoveryengine.v1alpha.DataStore.solution_types]
            /// is
            /// [SOLUTION_TYPE_CHAT][google.cloud.discoveryengine.v1alpha.SolutionType.SOLUTION_TYPE_CHAT].
            ///
            /// An extractive segment is a text segment extracted from the original
            /// document that is relevant to the search query, and, in general, more
            /// verbose than an extractive answer. The segment could then be used as
            /// input for LLMs to generate summaries and answers.
            ///
            /// If the number of matching segments is less than
            /// `max_extractive_segment_count`, return all of the segments. Otherwise,
            /// return the `max_extractive_segment_count`.
            #[prost(int32, tag = "2")]
            pub max_extractive_segment_count: i32,
            /// Specifies whether to return the confidence score from the extractive
            /// segments in each search result. The default value is `false`.
            ///
            /// Note: this is a priavte preview feature and only works for allowlisted
            /// users, please reach out to Cloud Support team if you want to use it.
            #[prost(bool, tag = "3")]
            pub return_extractive_segment_score: bool,
            /// Specifies whether to also include the adjacent from each selected
            /// segments.
            /// Return at most `num_previous_segments` segments before each selected
            /// segments.
            #[prost(int32, tag = "4")]
            pub num_previous_segments: i32,
            /// Return at most `num_next_segments` segments after each selected
            /// segments.
            #[prost(int32, tag = "5")]
            pub num_next_segments: i32,
        }
    }
    /// The specification that uses customized query embedding vector to do
    /// semantic document retrieval.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EmbeddingSpec {
        /// The embedding vector used for retrieval. Limit to 1.
        #[prost(message, repeated, tag = "1")]
        pub embedding_vectors: ::prost::alloc::vec::Vec<embedding_spec::EmbeddingVector>,
    }
    /// Nested message and enum types in `EmbeddingSpec`.
    pub mod embedding_spec {
        /// Embedding vector.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EmbeddingVector {
            /// Embedding field path in schema.
            #[prost(string, tag = "1")]
            pub field_path: ::prost::alloc::string::String,
            /// Query embedding vector.
            #[prost(float, repeated, tag = "2")]
            pub vector: ::prost::alloc::vec::Vec<f32>,
        }
    }
}
/// Response message for
/// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// A list of matched documents. The order represents the ranking.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<search_response::SearchResult>,
    /// Results of facets requested by user.
    #[prost(message, repeated, tag = "2")]
    pub facets: ::prost::alloc::vec::Vec<search_response::Facet>,
    /// Guided search result.
    #[prost(message, optional, tag = "8")]
    pub guided_search_result: ::core::option::Option<
        search_response::GuidedSearchResult,
    >,
    /// The estimated total count of matched items irrespective of pagination. The
    /// count of
    /// [results][google.cloud.discoveryengine.v1alpha.SearchResponse.results]
    /// returned by pagination may be less than the
    /// [total_size][google.cloud.discoveryengine.v1alpha.SearchResponse.total_size]
    /// that matches.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
    /// A unique search token. This should be included in the
    /// [UserEvent][google.cloud.discoveryengine.v1alpha.UserEvent] logs resulting
    /// from this search, which enables accurate attribution of search model
    /// performance.
    #[prost(string, tag = "4")]
    pub attribution_token: ::prost::alloc::string::String,
    /// The URI of a customer-defined redirect page. If redirect action is
    /// triggered, no search is performed, and only
    /// [redirect_uri][google.cloud.discoveryengine.v1alpha.SearchResponse.redirect_uri]
    /// and
    /// [attribution_token][google.cloud.discoveryengine.v1alpha.SearchResponse.attribution_token]
    /// are set in the response.
    #[prost(string, tag = "12")]
    pub redirect_uri: ::prost::alloc::string::String,
    /// A token that can be sent as
    /// [SearchRequest.page_token][google.cloud.discoveryengine.v1alpha.SearchRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "5")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Contains the spell corrected query, if found. If the spell correction type
    /// is AUTOMATIC, then the search results are based on corrected_query.
    /// Otherwise the original query is used for search.
    #[prost(string, tag = "7")]
    pub corrected_query: ::prost::alloc::string::String,
    /// A summary as part of the search results.
    /// This field is only returned if
    /// [SearchRequest.ContentSearchSpec.summary_spec][google.cloud.discoveryengine.v1alpha.SearchRequest.ContentSearchSpec.summary_spec]
    /// is set.
    #[prost(message, optional, tag = "9")]
    pub summary: ::core::option::Option<search_response::Summary>,
    /// Controls applied as part of the Control service.
    #[prost(string, repeated, tag = "10")]
    pub applied_controls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "16")]
    pub geo_search_debug_info: ::prost::alloc::vec::Vec<
        search_response::GeoSearchDebugInfo,
    >,
    /// Query expansion information for the returned results.
    #[prost(message, optional, tag = "14")]
    pub query_expansion_info: ::core::option::Option<
        search_response::QueryExpansionInfo,
    >,
}
/// Nested message and enum types in `SearchResponse`.
pub mod search_response {
    /// Represents the search results.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SearchResult {
        /// [Document.id][google.cloud.discoveryengine.v1alpha.Document.id] of the
        /// searched [Document][google.cloud.discoveryengine.v1alpha.Document].
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The document data snippet in the search response. Only fields that are
        /// marked as retrievable are populated.
        #[prost(message, optional, tag = "2")]
        pub document: ::core::option::Option<super::Document>,
        /// Google provided available scores.
        #[prost(btree_map = "string, message", tag = "4")]
        pub model_scores: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            super::DoubleList,
        >,
    }
    /// A facet result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Facet {
        /// The key for this facet. E.g., "colors" or "price". It matches
        /// [SearchRequest.FacetSpec.FacetKey.key][google.cloud.discoveryengine.v1alpha.SearchRequest.FacetSpec.FacetKey.key].
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// The facet values for this field.
        #[prost(message, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<facet::FacetValue>,
        /// Whether the facet is dynamically generated.
        #[prost(bool, tag = "3")]
        pub dynamic_facet: bool,
    }
    /// Nested message and enum types in `Facet`.
    pub mod facet {
        /// A facet value which contains value names and their count.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FacetValue {
            /// Number of items that have this facet value.
            #[prost(int64, tag = "3")]
            pub count: i64,
            /// A facet value which contains values.
            #[prost(oneof = "facet_value::FacetValue", tags = "1, 2")]
            pub facet_value: ::core::option::Option<facet_value::FacetValue>,
        }
        /// Nested message and enum types in `FacetValue`.
        pub mod facet_value {
            /// A facet value which contains values.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum FacetValue {
                /// Text value of a facet, such as "Black" for facet "colors".
                #[prost(string, tag = "1")]
                Value(::prost::alloc::string::String),
                /// Interval value for a facet, such as [10, 20) for facet "price". It
                /// matches
                /// [SearchRequest.FacetSpec.FacetKey.intervals][google.cloud.discoveryengine.v1alpha.SearchRequest.FacetSpec.FacetKey.intervals].
                #[prost(message, tag = "2")]
                Interval(super::super::super::Interval),
            }
        }
    }
    /// Guided search result. The guided search helps user to refine the search
    /// results and narrow down to the real needs from a broaded search results.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GuidedSearchResult {
        /// A list of ranked refinement attributes.
        #[prost(message, repeated, tag = "1")]
        pub refinement_attributes: ::prost::alloc::vec::Vec<
            guided_search_result::RefinementAttribute,
        >,
        /// Suggested follow-up questions.
        #[prost(string, repeated, tag = "2")]
        pub follow_up_questions: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// Nested message and enum types in `GuidedSearchResult`.
    pub mod guided_search_result {
        /// Useful attribute for search result refinements.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RefinementAttribute {
            /// Attribute key used to refine the results e.g. 'movie_type'.
            #[prost(string, tag = "1")]
            pub attribute_key: ::prost::alloc::string::String,
            /// Attribute value used to refine the results e.g. 'drama'.
            #[prost(string, tag = "2")]
            pub attribute_value: ::prost::alloc::string::String,
        }
    }
    /// Summary of the top N search result specified by the summary spec.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Summary {
        /// The summary content.
        #[prost(string, tag = "1")]
        pub summary_text: ::prost::alloc::string::String,
        /// Additional summary-skipped reasons. This provides the reason for ignored
        /// cases. If nothing is skipped, this field is not set.
        #[prost(enumeration = "summary::SummarySkippedReason", repeated, tag = "2")]
        pub summary_skipped_reasons: ::prost::alloc::vec::Vec<i32>,
        /// A collection of Safety Attribute categories and their associated
        /// confidence scores.
        #[prost(message, optional, tag = "3")]
        pub safety_attributes: ::core::option::Option<summary::SafetyAttributes>,
        #[prost(message, optional, tag = "4")]
        pub summary_with_metadata: ::core::option::Option<summary::SummaryWithMetadata>,
    }
    /// Nested message and enum types in `Summary`.
    pub mod summary {
        /// Safety Attribute categories and their associated confidence scores.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SafetyAttributes {
            /// The display names of Safety Attribute categories associated with the
            /// generated content. Order matches the Scores.
            #[prost(string, repeated, tag = "1")]
            pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// The confidence scores of the each category, higher
            /// value means higher confidence. Order matches the Categories.
            #[prost(float, repeated, tag = "2")]
            pub scores: ::prost::alloc::vec::Vec<f32>,
        }
        /// Citation metadata.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CitationMetadata {
            /// Citations for segments.
            #[prost(message, repeated, tag = "1")]
            pub citations: ::prost::alloc::vec::Vec<Citation>,
        }
        /// Citation info for a segment.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Citation {
            /// Index indicates the start of the segment, measured in bytes/unicode.
            #[prost(int64, tag = "1")]
            pub start_index: i64,
            /// End of the attributed segment, exclusive.
            #[prost(int64, tag = "2")]
            pub end_index: i64,
            /// Citation sources for the attributed segment.
            #[prost(message, repeated, tag = "3")]
            pub sources: ::prost::alloc::vec::Vec<CitationSource>,
        }
        /// Citation source.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CitationSource {
            /// Document reference index from SummaryWithMetadata.references.
            /// It is 0-indexed and the value will be zero if the reference_index is
            /// not set explicitly.
            #[prost(int64, tag = "4")]
            pub reference_index: i64,
        }
        /// Document reference.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Reference {
            /// Title of the document.
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
            /// Required.
            /// [Document.name][google.cloud.discoveryengine.v1alpha.Document.name] of
            /// the document. Full resource name of the referenced document, in the
            /// format
            /// `projects/*/locations/*/collections/*/dataStores/*/branches/*/documents/*`.
            #[prost(string, tag = "2")]
            pub document: ::prost::alloc::string::String,
            /// GCS or HTTP uri for the document.
            #[prost(string, tag = "3")]
            pub uri: ::prost::alloc::string::String,
        }
        /// Summary with metadata information.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SummaryWithMetadata {
            /// Summary text with no citation information.
            #[prost(string, tag = "1")]
            pub summary: ::prost::alloc::string::String,
            /// Citation metadata for given summary.
            #[prost(message, optional, tag = "2")]
            pub citation_metadata: ::core::option::Option<CitationMetadata>,
            /// Document References.
            #[prost(message, repeated, tag = "3")]
            pub references: ::prost::alloc::vec::Vec<Reference>,
        }
        /// An Enum for summary-skipped reasons.
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
        pub enum SummarySkippedReason {
            /// Default value. The summary skipped reason is not specified.
            Unspecified = 0,
            /// The adversarial query ignored case.
            ///
            /// Only populated when
            /// [SummarySpec.ignore_adversarial_query][google.cloud.discoveryengine.v1alpha.SearchRequest.ContentSearchSpec.SummarySpec.ignore_adversarial_query]
            /// is set to `true`.
            AdversarialQueryIgnored = 1,
            /// The non-summary seeking query ignored case.
            ///
            /// Only populated when
            /// [SummarySpec.ignore_non_summary_seeking_query][google.cloud.discoveryengine.v1alpha.SearchRequest.ContentSearchSpec.SummarySpec.ignore_non_summary_seeking_query]
            /// is set to `true`.
            NonSummarySeekingQueryIgnored = 2,
            /// The out-of-domain query ignored case.
            ///
            /// Google skips the summary if there are no high-relevance search results.
            /// For example, the data store contains facts about company A but the
            /// user query is asking questions about company B.
            OutOfDomainQueryIgnored = 3,
            /// The potential policy violation case.
            ///
            /// Google skips the summary if there is a potential policy violation
            /// detected. This includes content that may be violent or toxic.
            PotentialPolicyViolation = 4,
            /// The LLM addon not enabled case.
            ///
            /// Google skips the summary if the LLM addon is not enabled.
            LlmAddonNotEnabled = 5,
        }
        impl SummarySkippedReason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SummarySkippedReason::Unspecified => {
                        "SUMMARY_SKIPPED_REASON_UNSPECIFIED"
                    }
                    SummarySkippedReason::AdversarialQueryIgnored => {
                        "ADVERSARIAL_QUERY_IGNORED"
                    }
                    SummarySkippedReason::NonSummarySeekingQueryIgnored => {
                        "NON_SUMMARY_SEEKING_QUERY_IGNORED"
                    }
                    SummarySkippedReason::OutOfDomainQueryIgnored => {
                        "OUT_OF_DOMAIN_QUERY_IGNORED"
                    }
                    SummarySkippedReason::PotentialPolicyViolation => {
                        "POTENTIAL_POLICY_VIOLATION"
                    }
                    SummarySkippedReason::LlmAddonNotEnabled => "LLM_ADDON_NOT_ENABLED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SUMMARY_SKIPPED_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "ADVERSARIAL_QUERY_IGNORED" => Some(Self::AdversarialQueryIgnored),
                    "NON_SUMMARY_SEEKING_QUERY_IGNORED" => {
                        Some(Self::NonSummarySeekingQueryIgnored)
                    }
                    "OUT_OF_DOMAIN_QUERY_IGNORED" => Some(Self::OutOfDomainQueryIgnored),
                    "POTENTIAL_POLICY_VIOLATION" => Some(Self::PotentialPolicyViolation),
                    "LLM_ADDON_NOT_ENABLED" => Some(Self::LlmAddonNotEnabled),
                    _ => None,
                }
            }
        }
    }
    /// Debug information specifically related to forward geocoding issues arising
    /// from Geolocation Search.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GeoSearchDebugInfo {
        /// The address from which forward geocoding ingestion produced issues.
        #[prost(string, tag = "1")]
        pub original_address_query: ::prost::alloc::string::String,
        /// The error produced.
        #[prost(string, tag = "2")]
        pub error_message: ::prost::alloc::string::String,
    }
    /// Information describing query expansion including whether expansion has
    /// occurred.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryExpansionInfo {
        /// Bool describing whether query expansion has occurred.
        #[prost(bool, tag = "1")]
        pub expanded_query: bool,
        /// Number of pinned results. This field will only be set when expansion
        /// happens and
        /// [SearchRequest.QueryExpansionSpec.pin_unexpanded_results][google.cloud.discoveryengine.v1alpha.SearchRequest.QueryExpansionSpec.pin_unexpanded_results]
        /// is set to true.
        #[prost(int64, tag = "2")]
        pub pinned_result_count: i64,
    }
}
/// Generated client implementations.
pub mod search_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for search.
    #[derive(Debug, Clone)]
    pub struct SearchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SearchServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SearchServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SearchServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Performs a search.
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SearchService/Search",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SearchService",
                        "Search",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// External conversation proto definition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Conversation {
    /// Immutable. Fully qualified name
    /// `project/*/locations/global/collections/{collection}/dataStore/*/conversations/*`
    /// or
    /// `project/*/locations/global/collections/{collection}/engines/*/conversations/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The state of the Conversation.
    #[prost(enumeration = "conversation::State", tag = "2")]
    pub state: i32,
    /// A unique identifier for tracking users.
    #[prost(string, tag = "3")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// Conversation messages.
    #[prost(message, repeated, tag = "4")]
    pub messages: ::prost::alloc::vec::Vec<ConversationMessage>,
    /// Output only. The time the conversation started.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the conversation finished.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Conversation`.
pub mod conversation {
    /// Enumeration of the state of the conversation.
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
        /// Unknown.
        Unspecified = 0,
        /// Conversation is currently open.
        InProgress = 1,
        /// Conversation has been completed.
        Completed = 2,
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
                State::Completed => "COMPLETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "IN_PROGRESS" => Some(Self::InProgress),
                "COMPLETED" => Some(Self::Completed),
                _ => None,
            }
        }
    }
}
/// Defines a reply message to user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reply {
    /// DEPRECATED: use `summary` instead.
    /// Text reply.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub reply: ::prost::alloc::string::String,
    /// References in the reply.
    #[deprecated]
    #[prost(message, repeated, tag = "2")]
    pub references: ::prost::alloc::vec::Vec<reply::Reference>,
    /// Summary based on search results.
    #[prost(message, optional, tag = "3")]
    pub summary: ::core::option::Option<search_response::Summary>,
}
/// Nested message and enum types in `Reply`.
pub mod reply {
    /// Defines reference in reply.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reference {
        /// URI link reference.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
        /// Anchor text.
        #[prost(string, tag = "2")]
        pub anchor_text: ::prost::alloc::string::String,
        /// Anchor text start index.
        #[prost(int32, tag = "3")]
        pub start: i32,
        /// Anchor text end index.
        #[prost(int32, tag = "4")]
        pub end: i32,
    }
}
/// Defines context of the conversation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationContext {
    /// The current list of documents the user is seeing.
    /// It contains the document resource references.
    #[prost(string, repeated, tag = "1")]
    pub context_documents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The current active document the user opened.
    /// It contains the document resource reference.
    #[prost(string, tag = "2")]
    pub active_document: ::prost::alloc::string::String,
}
/// Defines text input.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInput {
    /// Text input.
    #[prost(string, tag = "1")]
    pub input: ::prost::alloc::string::String,
    /// Conversation context of the input.
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<ConversationContext>,
}
/// Defines a conversation message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationMessage {
    /// Output only. Message creation timestamp.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "conversation_message::Message", tags = "1, 2")]
    pub message: ::core::option::Option<conversation_message::Message>,
}
/// Nested message and enum types in `ConversationMessage`.
pub mod conversation_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// User text input.
        #[prost(message, tag = "1")]
        UserInput(super::TextInput),
        /// Search reply.
        #[prost(message, tag = "2")]
        Reply(super::Reply),
    }
}
/// Request message for
/// [ConversationalSearchService.ConverseConversation][google.cloud.discoveryengine.v1alpha.ConversationalSearchService.ConverseConversation]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseConversationRequest {
    /// Required. The resource name of the Conversation to get. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/conversations/{conversation_id}`.
    /// Use
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/conversations/-`
    /// to activate auto session mode, which automatically creates a new
    /// conversation inside a ConverseConversation session.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Current user input.
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<TextInput>,
    /// The resource name of the Serving Config to use. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/servingConfigs/{serving_config_id}`
    /// If this is not set, the default serving config will be used.
    #[prost(string, tag = "3")]
    pub serving_config: ::prost::alloc::string::String,
    /// The conversation to be used by auto session only. The name field will be
    /// ignored as we automatically assign new name for the conversation in auto
    /// session.
    #[prost(message, optional, tag = "5")]
    pub conversation: ::core::option::Option<Conversation>,
    /// Whether to turn on safe search.
    #[prost(bool, tag = "6")]
    pub safe_search: bool,
    /// The user labels applied to a resource must meet the following requirements:
    ///
    /// * Each resource can have multiple labels, up to a maximum of 64.
    /// * Each label must be a key-value pair.
    /// * Keys have a minimum length of 1 character and a maximum length of 63
    ///    characters and cannot be empty. Values can be empty and have a maximum
    ///    length of 63 characters.
    /// * Keys and values can contain only lowercase letters, numeric characters,
    ///    underscores, and dashes. All characters must use UTF-8 encoding, and
    ///    international characters are allowed.
    /// * The key portion of a label must be unique. However, you can use the same
    ///    key with multiple resources.
    /// * Keys must start with a lowercase letter or international character.
    ///
    /// See [Google Cloud
    /// Document](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>)
    /// for more details.
    #[prost(btree_map = "string, string", tag = "7")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A specification for configuring the summary returned in the response.
    #[prost(message, optional, tag = "8")]
    pub summary_spec: ::core::option::Option<
        search_request::content_search_spec::SummarySpec,
    >,
    /// The filter syntax consists of an expression language for constructing a
    /// predicate from one or more fields of the documents being filtered. Filter
    /// expression is case-sensitive. This will be used to filter search results
    /// which may affect the summary response.
    ///
    /// If this field is unrecognizable, an  `INVALID_ARGUMENT`  is returned.
    ///
    /// Filtering in Vertex AI Search is done by mapping the LHS filter key to a
    /// key property defined in the Vertex AI Search backend -- this mapping is
    /// defined by the customer in their schema. For example a media customer might
    /// have a field 'name' in their schema. In this case the filter would look
    /// like this: filter --> name:'ANY("king kong")'
    ///
    /// For more information about filtering including syntax and filter
    /// operators, see
    /// [Filter](<https://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata>)
    #[prost(string, tag = "9")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for
/// [ConversationalSearchService.ConverseConversation][google.cloud.discoveryengine.v1alpha.ConversationalSearchService.ConverseConversation]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseConversationResponse {
    /// Answer to the current query.
    #[prost(message, optional, tag = "1")]
    pub reply: ::core::option::Option<Reply>,
    /// Updated conversation including the answer.
    #[prost(message, optional, tag = "2")]
    pub conversation: ::core::option::Option<Conversation>,
    /// Suggested related questions.
    #[prost(string, repeated, tag = "6")]
    pub related_questions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Search Results.
    #[prost(message, repeated, tag = "3")]
    pub search_results: ::prost::alloc::vec::Vec<search_response::SearchResult>,
}
/// Request for CreateConversation method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversationRequest {
    /// Required. Full resource name of parent data store. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The conversation to create.
    #[prost(message, optional, tag = "2")]
    pub conversation: ::core::option::Option<Conversation>,
}
/// Request for UpdateConversation method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConversationRequest {
    /// Required. The Conversation to update.
    #[prost(message, optional, tag = "1")]
    pub conversation: ::core::option::Option<Conversation>,
    /// Indicates which fields in the provided
    /// [Conversation][google.cloud.discoveryengine.v1alpha.Conversation] to
    /// update. The following are NOT supported:
    ///
    /// * [conversation.name][]
    ///
    /// If not set or empty, all supported fields are updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for DeleteConversation method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConversationRequest {
    /// Required. The resource name of the Conversation to delete. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/conversations/{conversation_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for GetConversation method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversationRequest {
    /// Required. The resource name of the Conversation to get. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}/conversations/{conversation_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for ListConversations method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationsRequest {
    /// Required. The data store resource name. Format:
    /// `projects/{project_number}/locations/{location_id}/collections/{collection}/dataStores/{data_store_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of results to return. If unspecified, defaults
    /// to 50. Max allowed value is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListConversations` call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter to apply on the list results. The supported features are:
    /// user_pseudo_id, state.
    ///
    /// Example:
    /// "user_pseudo_id = some_id"
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// A comma-separated list of fields to order by, sorted in ascending order.
    /// Use "desc" after a field name for descending.
    /// Supported fields:
    ///    * `update_time`
    ///    * `create_time`
    ///    * `conversation_name`
    ///
    /// Example:
    /// "update_time desc"
    /// "create_time"
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListConversations method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationsResponse {
    /// All the Conversations for a given data store.
    #[prost(message, repeated, tag = "1")]
    pub conversations: ::prost::alloc::vec::Vec<Conversation>,
    /// Pagination token, if not returned indicates the last page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod conversational_search_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for conversational search.
    #[derive(Debug, Clone)]
    pub struct ConversationalSearchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversationalSearchServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ConversationalSearchServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ConversationalSearchServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Converses a conversation.
        pub async fn converse_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::ConverseConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConverseConversationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.ConversationalSearchService/ConverseConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.ConversationalSearchService",
                        "ConverseConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Conversation.
        ///
        /// If the [Conversation][google.cloud.discoveryengine.v1alpha.Conversation] to
        /// create already exists, an ALREADY_EXISTS error is returned.
        pub async fn create_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.ConversationalSearchService/CreateConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.ConversationalSearchService",
                        "CreateConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Conversation.
        ///
        /// If the [Conversation][google.cloud.discoveryengine.v1alpha.Conversation] to
        /// delete does not exist, a NOT_FOUND error is returned.
        pub async fn delete_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConversationRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.ConversationalSearchService/DeleteConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.ConversationalSearchService",
                        "DeleteConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a Conversation.
        ///
        /// [Conversation][google.cloud.discoveryengine.v1alpha.Conversation] action
        /// type cannot be changed. If the
        /// [Conversation][google.cloud.discoveryengine.v1alpha.Conversation] to update
        /// does not exist, a NOT_FOUND error is returned.
        pub async fn update_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.ConversationalSearchService/UpdateConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.ConversationalSearchService",
                        "UpdateConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a Conversation.
        pub async fn get_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversationRequest>,
        ) -> std::result::Result<tonic::Response<super::Conversation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.ConversationalSearchService/GetConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.ConversationalSearchService",
                        "GetConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all Conversations by their parent
        /// [DataStore][google.cloud.discoveryengine.v1alpha.DataStore].
        pub async fn list_conversations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.ConversationalSearchService/ListConversations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.ConversationalSearchService",
                        "ListConversations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// SiteSearchEngine captures DataStore level site search persisting
/// configurations. It is a singleton value per data store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SiteSearchEngine {
    /// The fully qualified resource name of the site search engine.
    /// Format: `projects/*/locations/*/dataStores/*/siteSearchEngine`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A target site for the SiteSearchEngine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetSite {
    /// Output only. The fully qualified resource name of the target site.
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}`
    /// The `target_site_id` is system-generated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Input only. The user provided URI pattern from which the
    /// `generated_uri_pattern` is generated.
    #[prost(string, tag = "2")]
    pub provided_uri_pattern: ::prost::alloc::string::String,
    /// The type of the target site, e.g. whether the site is to be included or
    /// excluded.
    #[prost(enumeration = "target_site::Type", tag = "3")]
    pub r#type: i32,
    /// Input only. If set to false, a uri_pattern is generated to include all
    /// pages whose address contains the provided_uri_pattern. If set to true, an
    /// uri_pattern is generated to try to be an exact match of the
    /// provided_uri_pattern or just the specific page if the provided_uri_pattern
    /// is a specific one. provided_uri_pattern is always normalized to
    /// generate the URI pattern to be used by the search engine.
    #[prost(bool, tag = "6")]
    pub exact_match: bool,
    /// Output only. This is system-generated based on the provided_uri_pattern.
    #[prost(string, tag = "4")]
    pub generated_uri_pattern: ::prost::alloc::string::String,
    /// Output only. Site ownership and validity verification status.
    #[prost(message, optional, tag = "7")]
    pub site_verification_info: ::core::option::Option<SiteVerificationInfo>,
    /// Output only. Indexing status.
    #[prost(enumeration = "target_site::IndexingStatus", tag = "8")]
    pub indexing_status: i32,
    /// Output only. The target site's last updated time.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Failure reason.
    #[prost(message, optional, tag = "9")]
    pub failure_reason: ::core::option::Option<target_site::FailureReason>,
}
/// Nested message and enum types in `TargetSite`.
pub mod target_site {
    /// Site search indexing failure reasons.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FailureReason {
        #[prost(oneof = "failure_reason::Failure", tags = "1")]
        pub failure: ::core::option::Option<failure_reason::Failure>,
    }
    /// Nested message and enum types in `FailureReason`.
    pub mod failure_reason {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QuotaFailure {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Failure {
            /// Failed due to insufficient quota.
            #[prost(message, tag = "1")]
            QuotaFailure(QuotaFailure),
        }
    }
    /// Possible target site types.
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
    pub enum Type {
        /// This value is unused. In this case, server behavior defaults to
        /// [Type.INCLUDE][google.cloud.discoveryengine.v1alpha.TargetSite.Type.INCLUDE].
        Unspecified = 0,
        /// Include the target site.
        Include = 1,
        /// Exclude the target site.
        Exclude = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Include => "INCLUDE",
                Type::Exclude => "EXCLUDE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "INCLUDE" => Some(Self::Include),
                "EXCLUDE" => Some(Self::Exclude),
                _ => None,
            }
        }
    }
    /// Target site indexing status enumeration.
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
    pub enum IndexingStatus {
        /// Defaults to SUCCEEDED.
        Unspecified = 0,
        /// The target site is in the update queue and will be picked up by indexing
        /// pipeline.
        Pending = 1,
        /// The target site fails to be indexed.
        Failed = 2,
        /// The target site has been indexed.
        Succeeded = 3,
        /// The previously indexed target site has been marked to be deleted. This is
        /// a transitioning state which will resulted in either:
        /// 1. target site deleted if unindexing is successful;
        /// 2. state reverts to SUCCEEDED if the unindexing fails.
        Deleting = 4,
    }
    impl IndexingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IndexingStatus::Unspecified => "INDEXING_STATUS_UNSPECIFIED",
                IndexingStatus::Pending => "PENDING",
                IndexingStatus::Failed => "FAILED",
                IndexingStatus::Succeeded => "SUCCEEDED",
                IndexingStatus::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INDEXING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                "SUCCEEDED" => Some(Self::Succeeded),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// Verification information for target sites in advanced site search.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SiteVerificationInfo {
    /// Site verification state indicating the ownership and validity.
    #[prost(enumeration = "site_verification_info::SiteVerificationState", tag = "1")]
    pub site_verification_state: i32,
    /// Latest site verification time.
    #[prost(message, optional, tag = "2")]
    pub verify_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `SiteVerificationInfo`.
pub mod site_verification_info {
    /// Site verification state.
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
    pub enum SiteVerificationState {
        /// Defaults to VERIFIED.
        Unspecified = 0,
        /// Site ownership verified.
        Verified = 1,
        /// Site ownership pending verification or verification failed.
        Unverified = 2,
        /// Site exempt from verification, e.g. a public website that opens to all.
        Exempted = 3,
    }
    impl SiteVerificationState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SiteVerificationState::Unspecified => {
                    "SITE_VERIFICATION_STATE_UNSPECIFIED"
                }
                SiteVerificationState::Verified => "VERIFIED",
                SiteVerificationState::Unverified => "UNVERIFIED",
                SiteVerificationState::Exempted => "EXEMPTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SITE_VERIFICATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "VERIFIED" => Some(Self::Verified),
                "UNVERIFIED" => Some(Self::Unverified),
                "EXEMPTED" => Some(Self::Exempted),
                _ => None,
            }
        }
    }
}
/// Request message for
/// [SiteSearchEngineService.GetSiteSearchEngine][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.GetSiteSearchEngine]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSiteSearchEngineRequest {
    /// Required. Resource name of
    /// [SiteSearchEngine][google.cloud.discoveryengine.v1alpha.SiteSearchEngine],
    /// such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
    ///
    /// If the caller does not have permission to access the \[SiteSearchEngine\],
    /// regardless of whether or not it exists, a PERMISSION_DENIED error is
    /// returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [SiteSearchEngineService.CreateTargetSite][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.CreateTargetSite]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetSiteRequest {
    /// Required. Parent resource name of
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite], such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite]
    /// to create.
    #[prost(message, optional, tag = "2")]
    pub target_site: ::core::option::Option<TargetSite>,
}
/// Metadata related to the progress of the
/// [SiteSearchEngineService.CreateTargetSite][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.CreateTargetSite]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetSiteMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for [SiteSearchEngineService.s][] method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateTargetSitesRequest {
    /// Required. The parent resource shared by all TargetSites being created.
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
    /// The parent field in the CreateBookRequest messages must either be empty or
    /// match this field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message specifying the resources to create.
    /// A maximum of 20 TargetSites can be created in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<CreateTargetSiteRequest>,
}
/// Request message for
/// [SiteSearchEngineService.GetTargetSite][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.GetTargetSite]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetSiteRequest {
    /// Required. Full resource name of
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite], such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}`.
    ///
    /// If the caller does not have permission to access the
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite], regardless
    /// of whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the requested
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite] does not
    /// exist, a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [SiteSearchEngineService.UpdateTargetSite][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.UpdateTargetSite]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTargetSiteRequest {
    /// Required. The target site to update.
    /// If the caller does not have permission to update the
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite], regardless
    /// of whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite] to
    /// update does not exist, a NOT_FOUND error is returned.
    #[prost(message, optional, tag = "1")]
    pub target_site: ::core::option::Option<TargetSite>,
}
/// Metadata related to the progress of the
/// [SiteSearchEngineService.UpdateTargetSite][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.UpdateTargetSite]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTargetSiteMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// [SiteSearchEngineService.DeleteTargetSite][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.DeleteTargetSite]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTargetSiteRequest {
    /// Required. Full resource name of
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite], such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine/targetSites/{target_site}`.
    ///
    /// If the caller does not have permission to access the
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite], regardless
    /// of whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the requested
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite] does not
    /// exist, a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata related to the progress of the
/// [SiteSearchEngineService.DeleteTargetSite][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.DeleteTargetSite]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTargetSiteMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// [SiteSearchEngineService.ListTargetSites][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.ListTargetSites]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetSitesRequest {
    /// Required. The parent site search engine resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
    ///
    /// If the caller does not have permission to list
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite]s under this
    /// site search engine, regardless of whether or not this branch exists, a
    /// PERMISSION_DENIED error is returned.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested. If
    /// unspecified, server will pick an appropriate default. The maximum value is
    /// 1000; values above 1000 will be coerced to 1000.
    ///
    /// If this field is negative, an INVALID_ARGUMENT error is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListTargetSites` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTargetSites`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [SiteSearchEngineService.ListTargetSites][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.ListTargetSites]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetSitesResponse {
    /// List of TargetSites.
    #[prost(message, repeated, tag = "1")]
    pub target_sites: ::prost::alloc::vec::Vec<TargetSite>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of items matching the request.
    /// This will always be populated in the response.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Metadata related to the progress of the
/// [SiteSearchEngineService.BatchCreateTargetSite][] operation. This will be
/// returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateTargetSiteMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response message for
/// [SiteSearchEngineService.BatchCreateTargetSites][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.BatchCreateTargetSites]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateTargetSitesResponse {
    /// TargetSites created.
    #[prost(message, repeated, tag = "1")]
    pub target_sites: ::prost::alloc::vec::Vec<TargetSite>,
}
/// Request message for
/// [SiteSearchEngineService.EnableAdvancedSiteSearch][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.EnableAdvancedSiteSearch]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableAdvancedSiteSearchRequest {
    /// Required. Full resource name of the
    /// [SiteSearchEngine][google.cloud.discoveryengine.v1alpha.SiteSearchEngine],
    /// such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store_id}/siteSearchEngine`.
    #[prost(string, tag = "1")]
    pub site_search_engine: ::prost::alloc::string::String,
}
/// Response message for
/// [SiteSearchEngineService.EnableAdvancedSiteSearch][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.EnableAdvancedSiteSearch]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableAdvancedSiteSearchResponse {}
/// Metadata related to the progress of the
/// [SiteSearchEngineService.EnableAdvancedSiteSearch][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.EnableAdvancedSiteSearch]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableAdvancedSiteSearchMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// [SiteSearchEngineService.DisableAdvancedSiteSearch][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.DisableAdvancedSiteSearch]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableAdvancedSiteSearchRequest {
    /// Required. Full resource name of the
    /// [SiteSearchEngine][google.cloud.discoveryengine.v1alpha.SiteSearchEngine],
    /// such as
    /// `projects/{project}/locations/{location}/dataStores/{data_store_id}/siteSearchEngine`.
    #[prost(string, tag = "1")]
    pub site_search_engine: ::prost::alloc::string::String,
}
/// Response message for
/// [SiteSearchEngineService.DisableAdvancedSiteSearch][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.DisableAdvancedSiteSearch]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableAdvancedSiteSearchResponse {}
/// Metadata related to the progress of the
/// [SiteSearchEngineService.DisableAdvancedSiteSearch][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.DisableAdvancedSiteSearch]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableAdvancedSiteSearchMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// [SiteSearchEngineService.RecrawlUris][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.RecrawlUris]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecrawlUrisRequest {
    /// Required. Full resource name of the
    /// [SiteSearchEngine][google.cloud.discoveryengine.v1alpha.SiteSearchEngine],
    /// such as
    /// `projects/*/locations/*/collections/*/dataStores/*/siteSearchEngine`.
    #[prost(string, tag = "1")]
    pub site_search_engine: ::prost::alloc::string::String,
    /// Required. List of URIs to crawl. At most 10K URIs are supported, otherwise
    /// an INVALID_ARGUMENT error is thrown. Each URI should match at least one
    /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite] in
    /// `site_search_engine`.
    #[prost(string, repeated, tag = "2")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for
/// [SiteSearchEngineService.RecrawlUris][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.RecrawlUris]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecrawlUrisResponse {
    /// Details for a sample of up to 10 `failed_uris`.
    #[prost(message, repeated, tag = "1")]
    pub failure_samples: ::prost::alloc::vec::Vec<recrawl_uris_response::FailureInfo>,
    /// URIs that were not crawled before the LRO terminated.
    #[prost(string, repeated, tag = "2")]
    pub failed_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `RecrawlUrisResponse`.
pub mod recrawl_uris_response {
    /// Details about why a particular URI failed to be crawled. Each FailureInfo
    /// contains one FailureReason per CorpusType.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FailureInfo {
        /// URI that failed to be crawled.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
        /// List of failure reasons by corpus type (e.g. desktop, mobile).
        #[prost(message, repeated, tag = "2")]
        pub failure_reasons: ::prost::alloc::vec::Vec<failure_info::FailureReason>,
    }
    /// Nested message and enum types in `FailureInfo`.
    pub mod failure_info {
        /// Details about why crawling failed for a particular CorpusType, e.g.
        /// DESKTOP and MOBILE crawling may fail for different reasons.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FailureReason {
            /// DESKTOP, MOBILE, or CORPUS_TYPE_UNSPECIFIED.
            #[prost(enumeration = "failure_reason::CorpusType", tag = "1")]
            pub corpus_type: i32,
            /// Reason why the URI was not crawled.
            #[prost(string, tag = "2")]
            pub error_message: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `FailureReason`.
        pub mod failure_reason {
            /// CorpusType for the failed crawling operation.
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
            pub enum CorpusType {
                /// Default value.
                Unspecified = 0,
                /// Denotes a crawling attempt for the desktop version of a page.
                Desktop = 1,
                /// Denotes a crawling attempt for the mobile version of a page.
                Mobile = 2,
            }
            impl CorpusType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        CorpusType::Unspecified => "CORPUS_TYPE_UNSPECIFIED",
                        CorpusType::Desktop => "DESKTOP",
                        CorpusType::Mobile => "MOBILE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "CORPUS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                        "DESKTOP" => Some(Self::Desktop),
                        "MOBILE" => Some(Self::Mobile),
                        _ => None,
                    }
                }
            }
        }
    }
}
/// Metadata related to the progress of the
/// [SiteSearchEngineService.RecrawlUris][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.RecrawlUris]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecrawlUrisMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Unique URIs in the request that don't match any TargetSite in the
    /// DataStore, only match TargetSites that haven't been fully indexed, or match
    /// a TargetSite with type EXCLUDE.
    #[prost(string, repeated, tag = "3")]
    pub invalid_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Total number of unique URIs in the request that are not in invalid_uris.
    #[prost(int32, tag = "4")]
    pub valid_uris_count: i32,
    /// Total number of URIs that have been crawled so far.
    #[prost(int32, tag = "5")]
    pub success_count: i32,
    /// Total number of URIs that have yet to be crawled.
    #[prost(int32, tag = "6")]
    pub pending_count: i32,
    /// Total number of URIs that were rejected due to insufficient indexing
    /// resources.
    #[prost(int32, tag = "7")]
    pub quota_exceeded_count: i32,
}
/// Request message for
/// [SiteSearchEngineService.BatchVerifyTargetSites][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.BatchVerifyTargetSites]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchVerifyTargetSitesRequest {
    /// Required. The parent resource shared by all TargetSites being verified.
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response message for
/// [SiteSearchEngineService.BatchVerifyTargetSites][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.BatchVerifyTargetSites]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchVerifyTargetSitesResponse {}
/// Metadata related to the progress of the
/// [SiteSearchEngineService.BatchVerifyTargetSites][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.BatchVerifyTargetSites]
/// operation. This will be returned by the google.longrunning.Operation.metadata
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchVerifyTargetSitesMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// [SiteSearchEngineService.FetchDomainVerificationStatus][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.FetchDomainVerificationStatus]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchDomainVerificationStatusRequest {
    /// Required. The site search engine resource under which we fetch all the
    /// domain verification status.
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/siteSearchEngine`.
    #[prost(string, tag = "1")]
    pub site_search_engine: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested. If
    /// unspecified, server will pick an appropriate default. The maximum value is
    /// 1000; values above 1000 will be coerced to 1000.
    ///
    /// If this field is negative, an INVALID_ARGUMENT error is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `FetchDomainVerificationStatus`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `FetchDomainVerificationStatus` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// [SiteSearchEngineService.FetchDomainVerificationStatus][google.cloud.discoveryengine.v1alpha.SiteSearchEngineService.FetchDomainVerificationStatus]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchDomainVerificationStatusResponse {
    /// List of TargetSites containing the site verification status.
    #[prost(message, repeated, tag = "1")]
    pub target_sites: ::prost::alloc::vec::Vec<TargetSite>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of items matching the request.
    /// This will always be populated in the response.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod site_search_engine_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for managing site search related resources.
    #[derive(Debug, Clone)]
    pub struct SiteSearchEngineServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SiteSearchEngineServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SiteSearchEngineServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SiteSearchEngineServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Gets the
        /// [SiteSearchEngine][google.cloud.discoveryengine.v1alpha.SiteSearchEngine].
        pub async fn get_site_search_engine(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSiteSearchEngineRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SiteSearchEngine>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/GetSiteSearchEngine",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "GetSiteSearchEngine",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite].
        pub async fn create_target_site(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTargetSiteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/CreateTargetSite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "CreateTargetSite",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite] in a
        /// batch.
        pub async fn batch_create_target_sites(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateTargetSitesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/BatchCreateTargetSites",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "BatchCreateTargetSites",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite].
        pub async fn get_target_site(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTargetSiteRequest>,
        ) -> std::result::Result<tonic::Response<super::TargetSite>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/GetTargetSite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "GetTargetSite",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite].
        pub async fn update_target_site(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTargetSiteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/UpdateTargetSite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "UpdateTargetSite",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite].
        pub async fn delete_target_site(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTargetSiteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/DeleteTargetSite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "DeleteTargetSite",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a list of
        /// [TargetSite][google.cloud.discoveryengine.v1alpha.TargetSite]s.
        pub async fn list_target_sites(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTargetSitesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTargetSitesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/ListTargetSites",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "ListTargetSites",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Upgrade from basic site search to advanced site search.
        pub async fn enable_advanced_site_search(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableAdvancedSiteSearchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/EnableAdvancedSiteSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "EnableAdvancedSiteSearch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Downgrade from advanced site search to basic site search.
        pub async fn disable_advanced_site_search(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableAdvancedSiteSearchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/DisableAdvancedSiteSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "DisableAdvancedSiteSearch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Request on-demand recrawl for a list of URIs.
        pub async fn recrawl_uris(
            &mut self,
            request: impl tonic::IntoRequest<super::RecrawlUrisRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/RecrawlUris",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "RecrawlUris",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Verify target sites' ownership and validity.
        /// This API sends all the target sites under site search engine for
        /// verification.
        pub async fn batch_verify_target_sites(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchVerifyTargetSitesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/BatchVerifyTargetSites",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "BatchVerifyTargetSites",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns list of target sites with its domain verification status.
        /// This method can only be called under data store with BASIC_SITE_SEARCH
        /// state at the moment.
        pub async fn fetch_domain_verification_status(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchDomainVerificationStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchDomainVerificationStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SiteSearchEngineService/FetchDomainVerificationStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SiteSearchEngineService",
                        "FetchDomainVerificationStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for
/// [SearchTuningService.TrainCustomModel][google.cloud.discoveryengine.v1alpha.SearchTuningService.TrainCustomModel]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainCustomModelRequest {
    /// Required. The resource name of the Data Store, such as
    /// `projects/*/locations/global/collections/default_collection/dataStores/default_data_store`.
    /// This field is used to identify the data store where to train the models.
    #[prost(string, tag = "1")]
    pub data_store: ::prost::alloc::string::String,
    /// Model to be trained. Supported values are:
    ///
    ///   * **search-tuning**: Fine tuning the search system based on data provided.
    #[prost(string, tag = "3")]
    pub model_type: ::prost::alloc::string::String,
    /// The desired location of errors incurred during the data ingestion and
    /// training.
    #[prost(message, optional, tag = "4")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
    /// Model training input.
    #[prost(oneof = "train_custom_model_request::TrainingInput", tags = "2")]
    pub training_input: ::core::option::Option<
        train_custom_model_request::TrainingInput,
    >,
}
/// Nested message and enum types in `TrainCustomModelRequest`.
pub mod train_custom_model_request {
    /// Gcs training data input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsTrainingInput {
        /// The gcs corpus data which could be associated in train data.
        /// The data path format is gs://<bucket_to_data>/<jsonl_file_name>.
        /// A newline delimited jsonl/ndjson file.
        ///   * For search-tuning model, each line should have the _id, title
        ///   and text. Example: {"_id": "doc1", title: "relevant doc", "text":
        ///   "relevant text"}
        #[prost(string, tag = "1")]
        pub corpus_data_path: ::prost::alloc::string::String,
        /// The gcs query data which could be associated in train data.
        /// The data path format is gs://<bucket_to_data>/<jsonl_file_name>.
        /// A newline delimited jsonl/ndjson file.
        ///   * For search-tuning model, each line should have the _id
        ///   and text. Example: {"_id": "query1",  "text": "example query"}
        #[prost(string, tag = "2")]
        pub query_data_path: ::prost::alloc::string::String,
        /// Gcs training data path whose format should be
        /// gs://<bucket_to_data>/<tsv_file_name>. The file should be in tsv format.
        /// Each line should have the doc_id and query_id and score (number).
        ///   * For search-tuning model, it should have the query-id corpus-id
        ///   score as tsv file header. The score should be a number in [0, inf+). The
        ///   larger the number is, the more relevant the pair is. Example:
        ///   query-id\tcorpus-id\tscore
        ///   query1\tdoc1\t1
        #[prost(string, tag = "3")]
        pub train_data_path: ::prost::alloc::string::String,
        /// Gcs test data. Same format as train_data_path. If not provided, a
        /// random 80/20 train/test split will be performed on train_data_path.
        #[prost(string, tag = "4")]
        pub test_data_path: ::prost::alloc::string::String,
    }
    /// Model training input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TrainingInput {
        /// Gcs training input.
        #[prost(message, tag = "2")]
        GcsTrainingInput(GcsTrainingInput),
    }
}
/// Response of the
/// [TrainCustomModelRequest][google.cloud.discoveryengine.v1alpha.TrainCustomModelRequest].
/// This message is returned by the google.longrunning.Operations.response field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainCustomModelResponse {
    /// A sample of errors encountered while processing the data.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag = "2")]
    pub error_config: ::core::option::Option<ImportErrorConfig>,
    /// The trained model status. Possible values are:
    ///
    ///   * **bad-data**: The training data quality is bad.
    ///   * **no-improvement**: Tuning didn't improve performance. Won't deploy.
    ///   * **in-progress**: Model training is in progress.
    ///   * **ready**: The model is ready for serving.
    #[prost(string, tag = "3")]
    pub model_status: ::prost::alloc::string::String,
}
/// Metadata related to the progress of the TrainCustomModel operation. This is
/// returned by the google.longrunning.Operation.metadata field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainCustomModelMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod search_tuning_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for search tuning.
    #[derive(Debug, Clone)]
    pub struct SearchTuningServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SearchTuningServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SearchTuningServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SearchTuningServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Trains a custom model.
        pub async fn train_custom_model(
            &mut self,
            request: impl tonic::IntoRequest<super::TrainCustomModelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.SearchTuningService/TrainCustomModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.SearchTuningService",
                        "TrainCustomModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for Recommend method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendRequest {
    /// Required. Full resource name of a
    /// [ServingConfig][google.cloud.discoveryengine.v1alpha.ServingConfig]:
    /// `projects/*/locations/global/collections/*/engines/*/servingConfigs/*`, or
    /// `projects/*/locations/global/collections/*/dataStores/*/servingConfigs/*`
    ///
    /// One default serving config is created along with your recommendation engine
    /// creation. The engine ID will be used as the ID of the default serving
    /// config. For example, for Engine
    /// `projects/*/locations/global/collections/*/engines/my-engine`, you can use
    /// `projects/*/locations/global/collections/*/engines/my-engine/servingConfigs/my-engine`
    /// for your [Recommend][] requests.
    #[prost(string, tag = "1")]
    pub serving_config: ::prost::alloc::string::String,
    /// Required. Context about the user, what they are looking at and what action
    /// they took to trigger the Recommend request. Note that this user event
    /// detail won't be ingested to userEvent logs. Thus, a separate userEvent
    /// write request is required for event logging.
    ///
    /// Don't set
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1alpha.UserEvent.user_pseudo_id]
    /// or
    /// [UserEvent.user_info.user_id][google.cloud.discoveryengine.v1alpha.UserInfo.user_id]
    /// to the same fixed ID for different users. If you are trying to receive
    /// non-personalized recommendations (not recommended; this can negatively
    /// impact model performance), instead set
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1alpha.UserEvent.user_pseudo_id]
    /// to a random unique ID and leave
    /// [UserEvent.user_info.user_id][google.cloud.discoveryengine.v1alpha.UserInfo.user_id]
    /// unset.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::core::option::Option<UserEvent>,
    /// Maximum number of results to return. Set this property
    /// to the number of recommendation results needed. If zero, the service will
    /// choose a reasonable default. The maximum allowed value is 100. Values
    /// above 100 will be coerced to 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Filter for restricting recommendation results with a length limit of 5,000
    /// characters. Currently, only filter expressions on the `filter_tags`
    /// attribute is supported.
    ///
    ///
    /// Examples:
    ///
    ///   * `(filter_tags: ANY("Red", "Blue") OR filter_tags: ANY("Hot", "Cold"))`
    ///   * `(filter_tags: ANY("Red", "Blue")) AND NOT (filter_tags: ANY("Green"))`
    ///
    /// If `attributeFilteringSyntax` is set to true under the `params` field, then
    /// attribute-based expressions are expected instead of the above described
    /// tag-based syntax. Examples:
    ///
    ///   * (launguage: ANY("en", "es")) AND NOT (categories: ANY("Movie"))
    ///   * (available: true) AND
    ///     (launguage: ANY("en", "es")) OR (categories: ANY("Movie"))
    ///
    /// If your filter blocks all results, the API will return generic
    /// (unfiltered) popular Documents. If you only want results strictly matching
    /// the filters, set `strictFiltering` to True in
    /// [RecommendRequest.params][google.cloud.discoveryengine.v1alpha.RecommendRequest.params]
    /// to receive empty results instead.
    ///
    /// Note that the API will never return
    /// [Document][google.cloud.discoveryengine.v1alpha.Document]s with
    /// `storageStatus` of `EXPIRED` or `DELETED` regardless of filter choices.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Use validate only mode for this recommendation query. If set to true, a
    /// fake model will be used that returns arbitrary Document IDs.
    /// Note that the validate only mode should only be used for testing the API,
    /// or if the model is not ready.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
    /// Additional domain specific parameters for the recommendations.
    ///
    /// Allowed values:
    ///
    /// * `returnDocument`: Boolean. If set to true, the associated Document
    ///     object will be returned in
    ///     [RecommendResponse.RecommendationResult.document][google.cloud.discoveryengine.v1alpha.RecommendResponse.RecommendationResult.document].
    /// * `returnScore`: Boolean. If set to true, the recommendation 'score'
    ///     corresponding to each returned Document will be set in
    ///     [RecommendResponse.RecommendationResult.metadata][google.cloud.discoveryengine.v1alpha.RecommendResponse.RecommendationResult.metadata].
    ///     The given 'score' indicates the probability of a Document conversion
    ///     given the user's context and history.
    /// * `strictFiltering`: Boolean. True by default. If set to false, the service
    ///     will return generic (unfiltered) popular Documents instead of empty if
    ///     your filter blocks all recommendation results.
    /// * `diversityLevel`: String. Default empty. If set to be non-empty, then
    ///     it needs to be one of:
    ///      *  `no-diversity`
    ///      *  `low-diversity`
    ///      *  `medium-diversity`
    ///      *  `high-diversity`
    ///      *  `auto-diversity`
    ///     This gives request-level control and adjusts recommendation results
    ///     based on Document category.
    /// * `attributeFilteringSyntax`: Boolean. False by default. If set to true,
    ///     the `filter` field is interpreted according to the new,
    ///     attribute-based syntax.
    #[prost(btree_map = "string, message", tag = "6")]
    pub params: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// The user labels applied to a resource must meet the following requirements:
    ///
    /// * Each resource can have multiple labels, up to a maximum of 64.
    /// * Each label must be a key-value pair.
    /// * Keys have a minimum length of 1 character and a maximum length of 63
    ///    characters and cannot be empty. Values can be empty and have a maximum
    ///    length of 63 characters.
    /// * Keys and values can contain only lowercase letters, numeric characters,
    ///    underscores, and dashes. All characters must use UTF-8 encoding, and
    ///    international characters are allowed.
    /// * The key portion of a label must be unique. However, you can use the same
    ///    key with multiple resources.
    /// * Keys must start with a lowercase letter or international character.
    ///
    /// See [Requirements for
    /// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements>)
    /// for more details.
    #[prost(btree_map = "string, string", tag = "8")]
    pub user_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Response message for Recommend method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendResponse {
    /// A list of recommended Documents. The order represents the ranking (from the
    /// most relevant Document to the least).
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<recommend_response::RecommendationResult>,
    /// A unique attribution token. This should be included in the
    /// [UserEvent][google.cloud.discoveryengine.v1alpha.UserEvent] logs resulting
    /// from this recommendation, which enables accurate attribution of
    /// recommendation model performance.
    #[prost(string, tag = "2")]
    pub attribution_token: ::prost::alloc::string::String,
    /// IDs of documents in the request that were missing from the default Branch
    /// associated with the requested ServingConfig.
    #[prost(string, repeated, tag = "3")]
    pub missing_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// True if
    /// [RecommendRequest.validate_only][google.cloud.discoveryengine.v1alpha.RecommendRequest.validate_only]
    /// was set.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Nested message and enum types in `RecommendResponse`.
pub mod recommend_response {
    /// RecommendationResult represents a generic recommendation result with
    /// associated metadata.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecommendationResult {
        /// Resource ID of the recommended Document.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Set if `returnDocument` is set to true in
        /// [RecommendRequest.params][google.cloud.discoveryengine.v1alpha.RecommendRequest.params].
        #[prost(message, optional, tag = "2")]
        pub document: ::core::option::Option<super::Document>,
        /// Additional Document metadata / annotations.
        ///
        /// Possible values:
        ///
        /// * `score`: Recommendation score in double value. Is set if
        ///    `returnScore` is set to true in
        ///    [RecommendRequest.params][google.cloud.discoveryengine.v1alpha.RecommendRequest.params].
        #[prost(btree_map = "string, message", tag = "3")]
        pub metadata: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            ::prost_types::Value,
        >,
    }
}
/// Generated client implementations.
pub mod recommendation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for making recommendations.
    #[derive(Debug, Clone)]
    pub struct RecommendationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RecommendationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RecommendationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            RecommendationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Makes a recommendation, which requires a contextual user event.
        pub async fn recommend(
            &mut self,
            request: impl tonic::IntoRequest<super::RecommendRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecommendResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.RecommendationService/Recommend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.RecommendationService",
                        "Recommend",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for
/// [CompletionService.CompleteQuery][google.cloud.discoveryengine.v1alpha.CompletionService.CompleteQuery]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryRequest {
    /// Required. The parent data store resource name for which the completion is
    /// performed, such as
    /// `projects/*/locations/global/collections/default_collection/dataStores/default_data_store`.
    #[prost(string, tag = "1")]
    pub data_store: ::prost::alloc::string::String,
    /// Required. The typeahead input used to fetch suggestions. Maximum length is
    /// 128 characters.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Selects data model of query suggestions for serving. Currently supported
    /// values:
    ///
    /// * `document` - Using suggestions generated from user-imported documents.
    /// * `search-history` - Using suggestions generated from the past history of
    /// [SearchService.Search][google.cloud.discoveryengine.v1alpha.SearchService.Search]
    /// API calls. Do not use it when there is no traffic for Search API.
    /// * `user-event` - Using suggestions generated from user-imported search
    /// events.
    /// * `document-completable` - Using suggestions taken directly from
    /// user-imported document fields marked as completable.
    ///
    /// Default values:
    ///
    /// * `document` is the default model for regular dataStores.
    /// * `search-history` is the default model for site search dataStores.
    #[prost(string, tag = "3")]
    pub query_model: ::prost::alloc::string::String,
    /// A unique identifier for tracking visitors. For example, this could be
    /// implemented with an HTTP cookie, which should be able to uniquely identify
    /// a visitor on a single device. This unique identifier should not change if
    /// the visitor logs in or out of the website.
    ///
    /// This field should NOT have a fixed value such as `unknown_visitor`.
    ///
    /// This should be the same identifier as
    /// [UserEvent.user_pseudo_id][google.cloud.discoveryengine.v1alpha.UserEvent.user_pseudo_id]
    /// and
    /// [SearchRequest.user_pseudo_id][google.cloud.discoveryengine.v1alpha.SearchRequest.user_pseudo_id].
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "4")]
    pub user_pseudo_id: ::prost::alloc::string::String,
    /// Indicates if tail suggestions should be returned if there are no
    /// suggestions that match the full query. Even if set to true, if there are
    /// suggestions that match the full query, those are returned and no
    /// tail suggestions are returned.
    #[prost(bool, tag = "5")]
    pub include_tail_suggestions: bool,
}
/// Response message for
/// [CompletionService.CompleteQuery][google.cloud.discoveryengine.v1alpha.CompletionService.CompleteQuery]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryResponse {
    /// Results of the matched query suggestions. The result list is ordered and
    /// the first result is a top suggestion.
    #[prost(message, repeated, tag = "1")]
    pub query_suggestions: ::prost::alloc::vec::Vec<
        complete_query_response::QuerySuggestion,
    >,
    /// True if the returned suggestions are all tail suggestions.
    ///
    /// For tail matching to be triggered, include_tail_suggestions in the request
    /// must be true and there must be no suggestions that match the full query.
    #[prost(bool, tag = "2")]
    pub tail_match_triggered: bool,
}
/// Nested message and enum types in `CompleteQueryResponse`.
pub mod complete_query_response {
    /// Suggestions as search queries.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QuerySuggestion {
        /// The suggestion for the query.
        #[prost(string, tag = "1")]
        pub suggestion: ::prost::alloc::string::String,
        /// The unique document field paths that serve as the source of this
        /// suggestion if it was generated from completable fields.
        ///
        /// This field is only populated for the document-completable model.
        #[prost(string, repeated, tag = "2")]
        pub completable_field_paths: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
}
/// Generated client implementations.
pub mod completion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for Auto-Completion.
    #[derive(Debug, Clone)]
    pub struct CompletionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CompletionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CompletionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CompletionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Completes the specified user input with keyword suggestions.
        pub async fn complete_query(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteQueryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.CompletionService/CompleteQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.CompletionService",
                        "CompleteQuery",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Request message for WriteUserEvent method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteUserEventRequest {
    /// Required. The parent DataStore resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User event to write.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::core::option::Option<UserEvent>,
}
/// Request message for CollectUserEvent method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectUserEventRequest {
    /// Required. The parent DataStore resource name, such as
    /// `projects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. URL encoded UserEvent proto with a length limit of 2,000,000
    /// characters.
    #[prost(string, tag = "2")]
    pub user_event: ::prost::alloc::string::String,
    /// The URL including cgi-parameters but excluding the hash fragment with a
    /// length limit of 5,000 characters. This is often more useful than the
    /// referer URL, because many browsers only send the domain for third-party
    /// requests.
    #[prost(string, optional, tag = "3")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    /// The event timestamp in milliseconds. This prevents browser caching of
    /// otherwise identical get requests. The name is abbreviated to reduce the
    /// payload bytes.
    #[prost(int64, optional, tag = "4")]
    pub ets: ::core::option::Option<i64>,
}
/// Generated client implementations.
pub mod user_event_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for ingesting end user actions on a website to Discovery Engine API.
    #[derive(Debug, Clone)]
    pub struct UserEventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserEventServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UserEventServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UserEventServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Writes a single user event.
        pub async fn write_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteUserEventRequest>,
        ) -> std::result::Result<tonic::Response<super::UserEvent>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.UserEventService/WriteUserEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.UserEventService",
                        "WriteUserEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Writes a single user event from the browser. This uses a GET request to
        /// due to browser restriction of POST-ing to a third-party domain.
        ///
        /// This method is used only by the Discovery Engine API JavaScript pixel and
        /// Google Tag Manager. Users should not call this method directly.
        pub async fn collect_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CollectUserEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::api::HttpBody>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.UserEventService/CollectUserEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.UserEventService",
                        "CollectUserEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes permanently all user events specified by the filter provided.
        /// Depending on the number of events specified by the filter, this operation
        /// could take hours or days to complete. To test a filter, use the list
        /// command first.
        pub async fn purge_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeUserEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.UserEventService/PurgeUserEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.UserEventService",
                        "PurgeUserEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Bulk import of User events. Request processing might be
        /// synchronous. Events that already exist are skipped.
        /// Use this method for backfilling historical user events.
        ///
        /// Operation.response is of type ImportResponse. Note that it is
        /// possible for a subset of the items to be successfully inserted.
        /// Operation.metadata is of type ImportMetadata.
        pub async fn import_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportUserEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.discoveryengine.v1alpha.UserEventService/ImportUserEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.discoveryengine.v1alpha.UserEventService",
                        "ImportUserEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
