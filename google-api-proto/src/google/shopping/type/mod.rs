/// The price represented as a number and currency.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// The price represented as a number in micros (1 million micros is an
    /// equivalent to one's currency standard unit, for example, 1 USD = 1000000
    /// micros).
    #[prost(int64, optional, tag = "1")]
    pub amount_micros: ::core::option::Option<i64>,
    /// The currency of the price using three-letter acronyms according to [ISO
    /// 4217](<http://en.wikipedia.org/wiki/ISO_4217>).
    #[prost(string, optional, tag = "2")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message that represents custom attributes. Exactly one of `value` or
/// `group_values` must not be empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The name of the attribute.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The value of the attribute. If `value` is not empty, `group_values` must be
    /// empty.
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Subattributes within this attribute group.  If
    /// `group_values` is not empty, `value` must be empty.
    #[prost(message, repeated, tag = "3")]
    pub group_values: ::prost::alloc::vec::Vec<CustomAttribute>,
}
