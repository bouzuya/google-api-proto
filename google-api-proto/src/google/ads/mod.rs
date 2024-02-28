#[cfg(any(feature = "google-ads-admob-v1"))]
pub mod admob;
#[cfg(
    any(
        feature = "google-ads-googleads-v14-common",
        feature = "google-ads-googleads-v14-enums",
        feature = "google-ads-googleads-v14-errors",
        feature = "google-ads-googleads-v14-resources",
        feature = "google-ads-googleads-v14-services",
        feature = "google-ads-googleads-v15-common",
        feature = "google-ads-googleads-v15-enums",
        feature = "google-ads-googleads-v15-errors",
        feature = "google-ads-googleads-v15-resources",
        feature = "google-ads-googleads-v15-services",
        feature = "google-ads-googleads-v16-common",
        feature = "google-ads-googleads-v16-enums",
        feature = "google-ads-googleads-v16-errors",
        feature = "google-ads-googleads-v16-resources",
        feature = "google-ads-googleads-v16-services",
    )
)]
pub mod googleads;
#[cfg(
    any(
        feature = "google-ads-searchads360-v0-common",
        feature = "google-ads-searchads360-v0-enums",
        feature = "google-ads-searchads360-v0-resources",
        feature = "google-ads-searchads360-v0-services",
    )
)]
pub mod searchads360;
