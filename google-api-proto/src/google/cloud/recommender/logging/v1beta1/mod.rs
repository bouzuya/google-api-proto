/// Log content of an action on a recommendation. This includes Mark* actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionLog {
    /// Required. User that executed this action. Eg, foo@gmail.com
    #[prost(string, tag = "1")]
    pub actor: ::prost::alloc::string::String,
    /// Required. State change that was made by the actor. Eg, SUCCEEDED.
    #[prost(
        enumeration = "super::super::v1beta1::recommendation_state_info::State",
        tag = "2"
    )]
    pub state: i32,
    /// Optional. Metadata that was included with the action that was taken.
    #[prost(btree_map = "string, string", tag = "3")]
    pub state_metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Name of the recommendation which was acted on. Eg, :
    /// 'projects/foo/locations/global/recommenders/roleReco/recommendations/r1'
    #[prost(string, tag = "4")]
    pub recommendation_name: ::prost::alloc::string::String,
}
/// Log content of an action on an insight. This includes Mark* actions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsightActionLog {
    /// Required. User that executed this action. Eg, foo@gmail.com
    #[prost(string, tag = "1")]
    pub actor: ::prost::alloc::string::String,
    /// Required. State change that was made by the actor. Eg, ACCEPTED.
    #[prost(enumeration = "super::super::v1beta1::insight_state_info::State", tag = "2")]
    pub state: i32,
    /// Optional. Metadata that was included with the action that was taken.
    #[prost(btree_map = "string, string", tag = "3")]
    pub state_metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Name of the insight which was acted on. Eg, :
    /// 'projects/foo/locations/global/insightTypes/roleInsight/insights/i1'
    #[prost(string, tag = "4")]
    pub insight: ::prost::alloc::string::String,
}
