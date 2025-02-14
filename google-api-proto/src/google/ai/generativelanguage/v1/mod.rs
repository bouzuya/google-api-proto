/// A collection of source attributions for a piece of content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CitationMetadata {
    /// Citations to sources for a specific response.
    #[prost(message, repeated, tag = "1")]
    pub citation_sources: ::prost::alloc::vec::Vec<CitationSource>,
}
/// A citation to a source for a portion of a specific response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CitationSource {
    /// Optional. Start of segment of the response that is attributed to this
    /// source.
    ///
    /// Index indicates the start of the segment, measured in bytes.
    #[prost(int32, optional, tag = "1")]
    pub start_index: ::core::option::Option<i32>,
    /// Optional. End of the attributed segment, exclusive.
    #[prost(int32, optional, tag = "2")]
    pub end_index: ::core::option::Option<i32>,
    /// Optional. URI that is attributed as a source for a portion of the text.
    #[prost(string, optional, tag = "3")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. License for the GitHub project that is attributed as a source for
    /// segment.
    ///
    /// License info is required for code citations.
    #[prost(string, optional, tag = "4")]
    pub license: ::core::option::Option<::prost::alloc::string::String>,
}
/// Safety rating for a piece of content.
///
/// The safety rating contains the category of harm and the
/// harm probability level in that category for a piece of content.
/// Content is classified for safety across a number of
/// harm categories and the probability of the harm classification is included
/// here.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafetyRating {
    /// Required. The category for this rating.
    #[prost(enumeration = "HarmCategory", tag = "3")]
    pub category: i32,
    /// Required. The probability of harm for this content.
    #[prost(enumeration = "safety_rating::HarmProbability", tag = "4")]
    pub probability: i32,
    /// Was this content blocked because of this rating?
    #[prost(bool, tag = "5")]
    pub blocked: bool,
}
/// Nested message and enum types in `SafetyRating`.
pub mod safety_rating {
    /// The probability that a piece of content is harmful.
    ///
    /// The classification system gives the probability of the content being
    /// unsafe. This does not indicate the severity of harm for a piece of content.
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
    pub enum HarmProbability {
        /// Probability is unspecified.
        Unspecified = 0,
        /// Content has a negligible chance of being unsafe.
        Negligible = 1,
        /// Content has a low chance of being unsafe.
        Low = 2,
        /// Content has a medium chance of being unsafe.
        Medium = 3,
        /// Content has a high chance of being unsafe.
        High = 4,
    }
    impl HarmProbability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HarmProbability::Unspecified => "HARM_PROBABILITY_UNSPECIFIED",
                HarmProbability::Negligible => "NEGLIGIBLE",
                HarmProbability::Low => "LOW",
                HarmProbability::Medium => "MEDIUM",
                HarmProbability::High => "HIGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HARM_PROBABILITY_UNSPECIFIED" => Some(Self::Unspecified),
                "NEGLIGIBLE" => Some(Self::Negligible),
                "LOW" => Some(Self::Low),
                "MEDIUM" => Some(Self::Medium),
                "HIGH" => Some(Self::High),
                _ => None,
            }
        }
    }
}
/// Safety setting, affecting the safety-blocking behavior.
///
/// Passing a safety setting for a category changes the allowed proability that
/// content is blocked.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafetySetting {
    /// Required. The category for this setting.
    #[prost(enumeration = "HarmCategory", tag = "3")]
    pub category: i32,
    /// Required. Controls the probability threshold at which harm is blocked.
    #[prost(enumeration = "safety_setting::HarmBlockThreshold", tag = "4")]
    pub threshold: i32,
}
/// Nested message and enum types in `SafetySetting`.
pub mod safety_setting {
    /// Block at and beyond a specified harm probability.
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
    pub enum HarmBlockThreshold {
        /// Threshold is unspecified.
        Unspecified = 0,
        /// Content with NEGLIGIBLE will be allowed.
        BlockLowAndAbove = 1,
        /// Content with NEGLIGIBLE and LOW will be allowed.
        BlockMediumAndAbove = 2,
        /// Content with NEGLIGIBLE, LOW, and MEDIUM will be allowed.
        BlockOnlyHigh = 3,
        /// All content will be allowed.
        BlockNone = 4,
    }
    impl HarmBlockThreshold {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HarmBlockThreshold::Unspecified => "HARM_BLOCK_THRESHOLD_UNSPECIFIED",
                HarmBlockThreshold::BlockLowAndAbove => "BLOCK_LOW_AND_ABOVE",
                HarmBlockThreshold::BlockMediumAndAbove => "BLOCK_MEDIUM_AND_ABOVE",
                HarmBlockThreshold::BlockOnlyHigh => "BLOCK_ONLY_HIGH",
                HarmBlockThreshold::BlockNone => "BLOCK_NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HARM_BLOCK_THRESHOLD_UNSPECIFIED" => Some(Self::Unspecified),
                "BLOCK_LOW_AND_ABOVE" => Some(Self::BlockLowAndAbove),
                "BLOCK_MEDIUM_AND_ABOVE" => Some(Self::BlockMediumAndAbove),
                "BLOCK_ONLY_HIGH" => Some(Self::BlockOnlyHigh),
                "BLOCK_NONE" => Some(Self::BlockNone),
                _ => None,
            }
        }
    }
}
/// The category of a rating.
///
/// These categories cover various kinds of harms that developers
/// may wish to adjust.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HarmCategory {
    /// Category is unspecified.
    Unspecified = 0,
    /// Negative or harmful comments targeting identity and/or protected attribute.
    Derogatory = 1,
    /// Content that is rude, disrepspectful, or profane.
    Toxicity = 2,
    /// Describes scenarios depictng violence against an individual or group, or
    /// general descriptions of gore.
    Violence = 3,
    /// Contains references to sexual acts or other lewd content.
    Sexual = 4,
    /// Promotes unchecked medical advice.
    Medical = 5,
    /// Dangerous content that promotes, facilitates, or encourages harmful acts.
    Dangerous = 6,
    /// Harasment content.
    Harassment = 7,
    /// Hate speech and content.
    HateSpeech = 8,
    /// Sexually explicit content.
    SexuallyExplicit = 9,
    /// Dangerous content.
    DangerousContent = 10,
}
impl HarmCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HarmCategory::Unspecified => "HARM_CATEGORY_UNSPECIFIED",
            HarmCategory::Derogatory => "HARM_CATEGORY_DEROGATORY",
            HarmCategory::Toxicity => "HARM_CATEGORY_TOXICITY",
            HarmCategory::Violence => "HARM_CATEGORY_VIOLENCE",
            HarmCategory::Sexual => "HARM_CATEGORY_SEXUAL",
            HarmCategory::Medical => "HARM_CATEGORY_MEDICAL",
            HarmCategory::Dangerous => "HARM_CATEGORY_DANGEROUS",
            HarmCategory::Harassment => "HARM_CATEGORY_HARASSMENT",
            HarmCategory::HateSpeech => "HARM_CATEGORY_HATE_SPEECH",
            HarmCategory::SexuallyExplicit => "HARM_CATEGORY_SEXUALLY_EXPLICIT",
            HarmCategory::DangerousContent => "HARM_CATEGORY_DANGEROUS_CONTENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HARM_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "HARM_CATEGORY_DEROGATORY" => Some(Self::Derogatory),
            "HARM_CATEGORY_TOXICITY" => Some(Self::Toxicity),
            "HARM_CATEGORY_VIOLENCE" => Some(Self::Violence),
            "HARM_CATEGORY_SEXUAL" => Some(Self::Sexual),
            "HARM_CATEGORY_MEDICAL" => Some(Self::Medical),
            "HARM_CATEGORY_DANGEROUS" => Some(Self::Dangerous),
            "HARM_CATEGORY_HARASSMENT" => Some(Self::Harassment),
            "HARM_CATEGORY_HATE_SPEECH" => Some(Self::HateSpeech),
            "HARM_CATEGORY_SEXUALLY_EXPLICIT" => Some(Self::SexuallyExplicit),
            "HARM_CATEGORY_DANGEROUS_CONTENT" => Some(Self::DangerousContent),
            _ => None,
        }
    }
}
/// Information about a Generative Language Model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Required. The resource name of the `Model`.
    ///
    /// Format: `models/{model}` with a `{model}` naming convention of:
    ///
    /// * "{base_model_id}-{version}"
    ///
    /// Examples:
    ///
    /// * `models/chat-bison-001`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the base model, pass this to the generation request.
    ///
    /// Examples:
    ///
    /// * `chat-bison`
    #[prost(string, tag = "2")]
    pub base_model_id: ::prost::alloc::string::String,
    /// Required. The version number of the model.
    ///
    /// This represents the major version
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// The human-readable name of the model. E.g. "Chat Bison".
    ///
    /// The name can be up to 128 characters long and can consist of any UTF-8
    /// characters.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// A short description of the model.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Maximum number of input tokens allowed for this model.
    #[prost(int32, tag = "6")]
    pub input_token_limit: i32,
    /// Maximum number of output tokens available for this model.
    #[prost(int32, tag = "7")]
    pub output_token_limit: i32,
    /// The model's supported generation methods.
    ///
    /// The method names are defined as Pascal case
    /// strings, such as `generateMessage` which correspond to API methods.
    #[prost(string, repeated, tag = "8")]
    pub supported_generation_methods: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Controls the randomness of the output.
    ///
    /// Values can range over `\[0.0,1.0\]`, inclusive. A value closer to `1.0` will
    /// produce responses that are more varied, while a value closer to `0.0` will
    /// typically result in less surprising responses from the model.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(float, optional, tag = "9")]
    pub temperature: ::core::option::Option<f32>,
    /// For Nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose probability
    /// sum is at least `top_p`.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(float, optional, tag = "10")]
    pub top_p: ::core::option::Option<f32>,
    /// For Top-k sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(int32, optional, tag = "11")]
    pub top_k: ::core::option::Option<i32>,
}
/// Request for getting information about a specific Model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. The resource name of the model.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing all Models.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// The maximum number of `Models` to return (per page).
    ///
    /// The service may return fewer models.
    /// If unspecified, at most 50 models will be returned per page.
    /// This method returns at most 1000 models per page, even if you pass a larger
    /// page_size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListModels` call.
    ///
    /// Provide the `page_token` returned by one request as an argument to the next
    /// request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListModels` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from `ListModel` containing a paginated list of Models.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// The returned Models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    ///
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for getting metadata information about Generative Models.
    #[derive(Debug, Clone)]
    pub struct ModelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ModelServiceClient<T>
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
        ) -> ModelServiceClient<InterceptedService<T, F>>
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
            ModelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets information about a specific Model.
        pub async fn get_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelRequest>,
        ) -> std::result::Result<tonic::Response<super::Model>, tonic::Status> {
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
                "/google.ai.generativelanguage.v1.ModelService/GetModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1.ModelService",
                        "GetModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists models available through the API.
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListModelsResponse>,
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
                "/google.ai.generativelanguage.v1.ModelService/ListModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1.ModelService",
                        "ListModels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The base structured datatype containing multi-part content of a message.
///
/// A `Content` includes a `role` field designating the producer of the `Content`
/// and a `parts` field containing multi-part data that contains the content of
/// the message turn.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    /// Ordered `Parts` that constitute a single message. Parts may have different
    /// MIME types.
    #[prost(message, repeated, tag = "1")]
    pub parts: ::prost::alloc::vec::Vec<Part>,
    /// Optional. The producer of the content. Must be either 'user' or 'model'.
    ///
    /// Useful to set for multi-turn conversations, otherwise can be left blank
    /// or unset.
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
}
/// A datatype containing media that is part of a multi-part `Content` message.
///
/// A `Part` consists of data which has an associated datatype. A `Part` can only
/// contain one of the accepted types in `Part.data`.
///
/// A `Part` must have a fixed IANA MIME type identifying the type and subtype
/// of the media if the `inline_data` field is filled with raw bytes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(oneof = "part::Data", tags = "2, 3")]
    pub data: ::core::option::Option<part::Data>,
}
/// Nested message and enum types in `Part`.
pub mod part {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Inline text.
        #[prost(string, tag = "2")]
        Text(::prost::alloc::string::String),
        /// Inline media bytes.
        #[prost(message, tag = "3")]
        InlineData(super::Blob),
    }
}
/// Raw media bytes.
///
/// Text should not be sent as raw bytes, use the 'text' field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    /// The IANA standard MIME type of the source data.
    /// Accepted types include: "image/png", "image/jpeg", "image/heic",
    /// "image/heif", "image/webp".
    #[prost(string, tag = "1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Raw bytes for media formats.
    #[prost(bytes = "bytes", tag = "2")]
    pub data: ::prost::bytes::Bytes,
}
/// Request to generate a completion from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContentRequest {
    /// Required. The name of the `Model` to use for generating the completion.
    ///
    /// Format: `name=models/{model}`.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The content of the current conversation with the model.
    ///
    /// For single-turn queries, this is a single instance. For multi-turn queries,
    /// this is a repeated field that contains conversation history + latest
    /// request.
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
    /// Optional. A list of unique `SafetySetting` instances for blocking unsafe
    /// content.
    ///
    /// This will be enforced on the `GenerateContentRequest.contents` and
    /// `GenerateContentResponse.candidates`. There should not be more than one
    /// setting for each `SafetyCategory` type. The API will block any contents and
    /// responses that fail to meet the thresholds set by these settings. This list
    /// overrides the default settings for each `SafetyCategory` specified in the
    /// safety_settings. If there is no `SafetySetting` for a given
    /// `SafetyCategory` provided in the list, the API will use the default safety
    /// setting for that category. Harm categories HARM_CATEGORY_HATE_SPEECH,
    /// HARM_CATEGORY_SEXUALLY_EXPLICIT, HARM_CATEGORY_DANGEROUS_CONTENT,
    /// HARM_CATEGORY_HARASSMENT are supported.
    #[prost(message, repeated, tag = "3")]
    pub safety_settings: ::prost::alloc::vec::Vec<SafetySetting>,
    /// Optional. Configuration options for model generation and outputs.
    #[prost(message, optional, tag = "4")]
    pub generation_config: ::core::option::Option<GenerationConfig>,
}
/// Configuration options for model generation and outputs. Not all parameters
/// may be configurable for every model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationConfig {
    /// Optional. Number of generated responses to return.
    ///
    /// This value must be between \[1, 8\], inclusive. If unset, this will default
    /// to 1.
    #[prost(int32, optional, tag = "1")]
    pub candidate_count: ::core::option::Option<i32>,
    /// Optional. The set of character sequences (up to 5) that will stop output
    /// generation. If specified, the API will stop at the first appearance of a
    /// stop sequence. The stop sequence will not be included as part of the
    /// response.
    #[prost(string, repeated, tag = "2")]
    pub stop_sequences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The maximum number of tokens to include in a candidate.
    ///
    /// If unset, this will default to output_token_limit specified in the `Model`
    /// specification.
    #[prost(int32, optional, tag = "4")]
    pub max_output_tokens: ::core::option::Option<i32>,
    /// Optional. Controls the randomness of the output.
    /// Note: The default value varies by model, see the `Model.temperature`
    /// attribute of the `Model` returned the `getModel` function.
    ///
    /// Values can range from \[0.0,1.0\],
    /// inclusive. A value closer to 1.0 will produce responses that are more
    /// varied and creative, while a value closer to 0.0 will typically result in
    /// more straightforward responses from the model.
    #[prost(float, optional, tag = "5")]
    pub temperature: ::core::option::Option<f32>,
    /// Optional. The maximum cumulative probability of tokens to consider when
    /// sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Tokens are sorted based on their assigned probabilities so that only the
    /// most likely tokens are considered. Top-k sampling directly limits the
    /// maximum number of tokens to consider, while Nucleus sampling limits number
    /// of tokens based on the cumulative probability.
    ///
    /// Note: The default value varies by model, see the `Model.top_p`
    /// attribute of the `Model` returned the `getModel` function.
    #[prost(float, optional, tag = "6")]
    pub top_p: ::core::option::Option<f32>,
    /// Optional. The maximum number of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// Defaults to 40.
    ///
    /// Note: The default value varies by model, see the `Model.top_k`
    /// attribute of the `Model` returned the `getModel` function.
    #[prost(int32, optional, tag = "7")]
    pub top_k: ::core::option::Option<i32>,
}
/// Response from the model supporting multiple candidates.
///
/// Note on safety ratings and content filtering. They are reported for both
/// prompt in `GenerateContentResponse.prompt_feedback` and for each candidate
/// in `finish_reason` and in `safety_ratings`. The API contract is that:
///   - either all requested candidates are returned or no candidates at all
///   - no candidates are returned only if there was something wrong with the
///     prompt (see `prompt_feedback`)
///   - feedback on each candidate is reported on `finish_reason` and
///     `safety_ratings`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContentResponse {
    /// Candidate responses from the model.
    #[prost(message, repeated, tag = "1")]
    pub candidates: ::prost::alloc::vec::Vec<Candidate>,
    /// Returns the prompt's feedback related to the content filters.
    #[prost(message, optional, tag = "2")]
    pub prompt_feedback: ::core::option::Option<
        generate_content_response::PromptFeedback,
    >,
}
/// Nested message and enum types in `GenerateContentResponse`.
pub mod generate_content_response {
    /// A set of the feedback metadata the prompt specified in
    /// `GenerateContentRequest.content`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PromptFeedback {
        /// Optional. If set, the prompt was blocked and no candidates are returned.
        /// Rephrase your prompt.
        #[prost(enumeration = "prompt_feedback::BlockReason", tag = "1")]
        pub block_reason: i32,
        /// Ratings for safety of the prompt.
        /// There is at most one rating per category.
        #[prost(message, repeated, tag = "2")]
        pub safety_ratings: ::prost::alloc::vec::Vec<super::SafetyRating>,
    }
    /// Nested message and enum types in `PromptFeedback`.
    pub mod prompt_feedback {
        /// Specifies what was the reason why prompt was blocked.
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
        pub enum BlockReason {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// Prompt was blocked due to safety reasons. You can inspect
            /// `safety_ratings` to understand which safety category blocked it.
            Safety = 1,
            /// Prompt was blocked due to unknown reaasons.
            Other = 2,
        }
        impl BlockReason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    BlockReason::Unspecified => "BLOCK_REASON_UNSPECIFIED",
                    BlockReason::Safety => "SAFETY",
                    BlockReason::Other => "OTHER",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "BLOCK_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "SAFETY" => Some(Self::Safety),
                    "OTHER" => Some(Self::Other),
                    _ => None,
                }
            }
        }
    }
}
/// A response candidate generated from the model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candidate {
    /// Output only. Index of the candidate in the list of candidates.
    #[prost(int32, optional, tag = "3")]
    pub index: ::core::option::Option<i32>,
    /// Output only. Generated content returned from the model.
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Output only. The reason why the model stopped generating tokens.
    ///
    /// If empty, the model has not stopped generating the tokens.
    #[prost(enumeration = "candidate::FinishReason", tag = "2")]
    pub finish_reason: i32,
    /// List of ratings for the safety of a response candidate.
    ///
    /// There is at most one rating per category.
    #[prost(message, repeated, tag = "5")]
    pub safety_ratings: ::prost::alloc::vec::Vec<SafetyRating>,
    /// Output only. Citation information for model-generated candidate.
    ///
    /// This field may be populated with recitation information for any text
    /// included in the `content`. These are passages that are "recited" from
    /// copyrighted material in the foundational LLM's training data.
    #[prost(message, optional, tag = "6")]
    pub citation_metadata: ::core::option::Option<CitationMetadata>,
    /// Output only. Token count for this candidate.
    #[prost(int32, tag = "7")]
    pub token_count: i32,
}
/// Nested message and enum types in `Candidate`.
pub mod candidate {
    /// Defines the reason why the model stopped generating tokens.
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
    pub enum FinishReason {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Natural stop point of the model or provided stop sequence.
        Stop = 1,
        /// The maximum number of tokens as specified in the request was reached.
        MaxTokens = 2,
        /// The candidate content was flagged for safety reasons.
        Safety = 3,
        /// The candidate content was flagged for recitation reasons.
        Recitation = 4,
        /// Unknown reason.
        Other = 5,
    }
    impl FinishReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FinishReason::Unspecified => "FINISH_REASON_UNSPECIFIED",
                FinishReason::Stop => "STOP",
                FinishReason::MaxTokens => "MAX_TOKENS",
                FinishReason::Safety => "SAFETY",
                FinishReason::Recitation => "RECITATION",
                FinishReason::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FINISH_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "STOP" => Some(Self::Stop),
                "MAX_TOKENS" => Some(Self::MaxTokens),
                "SAFETY" => Some(Self::Safety),
                "RECITATION" => Some(Self::Recitation),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
/// Request containing the `Content` for the model to embed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedContentRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The content to embed. Only the `parts.text` fields will be
    /// counted.
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Optional task type for which the embeddings will be used. Can
    /// only be set for `models/embedding-001`.
    #[prost(enumeration = "TaskType", optional, tag = "3")]
    pub task_type: ::core::option::Option<i32>,
    /// Optional. An optional title for the text. Only applicable when TaskType is
    /// `RETRIEVAL_DOCUMENT`.
    ///
    /// Note: Specifying a `title` for `RETRIEVAL_DOCUMENT` provides better quality
    /// embeddings for retrieval.
    #[prost(string, optional, tag = "4")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
}
/// A list of floats representing an embedding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentEmbedding {
    /// The embedding values.
    #[prost(float, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
/// The response to an `EmbedContentRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedContentResponse {
    /// Output only. The embedding generated from the input content.
    #[prost(message, optional, tag = "1")]
    pub embedding: ::core::option::Option<ContentEmbedding>,
}
/// Batch request to get embeddings from the model for a list of prompts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEmbedContentsRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. Embed requests for the batch. The model in each of these requests
    /// must match the model specified `BatchEmbedContentsRequest.model`.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<EmbedContentRequest>,
}
/// The response to a `BatchEmbedContentsRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEmbedContentsResponse {
    /// Output only. The embeddings for each request, in the same order as provided
    /// in the batch request.
    #[prost(message, repeated, tag = "1")]
    pub embeddings: ::prost::alloc::vec::Vec<ContentEmbedding>,
}
/// Counts the number of tokens in the `prompt` sent to a model.
///
/// Models may tokenize text differently, so each model may return a different
/// `token_count`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountTokensRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The input given to the model as a prompt.
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
}
/// A response from `CountTokens`.
///
/// It returns the model's `token_count` for the `prompt`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountTokensResponse {
    /// The number of tokens that the `model` tokenizes the `prompt` into.
    ///
    /// Always non-negative.
    #[prost(int32, tag = "1")]
    pub total_tokens: i32,
}
/// Type of task for which the embedding will be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    /// Unset value, which will default to one of the other enum values.
    Unspecified = 0,
    /// Specifies the given text is a query in a search/retrieval setting.
    RetrievalQuery = 1,
    /// Specifies the given text is a document from the corpus being searched.
    RetrievalDocument = 2,
    /// Specifies the given text will be used for STS.
    SemanticSimilarity = 3,
    /// Specifies that the given text will be classified.
    Classification = 4,
    /// Specifies that the embeddings will be used for clustering.
    Clustering = 5,
}
impl TaskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaskType::Unspecified => "TASK_TYPE_UNSPECIFIED",
            TaskType::RetrievalQuery => "RETRIEVAL_QUERY",
            TaskType::RetrievalDocument => "RETRIEVAL_DOCUMENT",
            TaskType::SemanticSimilarity => "SEMANTIC_SIMILARITY",
            TaskType::Classification => "CLASSIFICATION",
            TaskType::Clustering => "CLUSTERING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TASK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RETRIEVAL_QUERY" => Some(Self::RetrievalQuery),
            "RETRIEVAL_DOCUMENT" => Some(Self::RetrievalDocument),
            "SEMANTIC_SIMILARITY" => Some(Self::SemanticSimilarity),
            "CLASSIFICATION" => Some(Self::Classification),
            "CLUSTERING" => Some(Self::Clustering),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod generative_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API for using Large Models that generate multimodal content and have
    /// additional capabilities beyond text generation.
    #[derive(Debug, Clone)]
    pub struct GenerativeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GenerativeServiceClient<T>
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
        ) -> GenerativeServiceClient<InterceptedService<T, F>>
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
            GenerativeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Generates a response from the model given an input
        /// `GenerateContentRequest`.
        pub async fn generate_content(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateContentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateContentResponse>,
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
                "/google.ai.generativelanguage.v1.GenerativeService/GenerateContent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1.GenerativeService",
                        "GenerateContent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates a streamed response from the model given an input
        /// `GenerateContentRequest`.
        pub async fn stream_generate_content(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateContentRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::GenerateContentResponse>>,
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
                "/google.ai.generativelanguage.v1.GenerativeService/StreamGenerateContent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1.GenerativeService",
                        "StreamGenerateContent",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Generates an embedding from the model given an input `Content`.
        pub async fn embed_content(
            &mut self,
            request: impl tonic::IntoRequest<super::EmbedContentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EmbedContentResponse>,
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
                "/google.ai.generativelanguage.v1.GenerativeService/EmbedContent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1.GenerativeService",
                        "EmbedContent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Generates multiple embeddings from the model given input text in a
        /// synchronous call.
        pub async fn batch_embed_contents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchEmbedContentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchEmbedContentsResponse>,
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
                "/google.ai.generativelanguage.v1.GenerativeService/BatchEmbedContents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1.GenerativeService",
                        "BatchEmbedContents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Runs a model's tokenizer on input content and returns the token count.
        pub async fn count_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::CountTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CountTokensResponse>,
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
                "/google.ai.generativelanguage.v1.GenerativeService/CountTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ai.generativelanguage.v1.GenerativeService",
                        "CountTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
