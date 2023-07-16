#[cfg(
    any(
        feature = "google-ads-googleads-v12-common",
        feature = "google-ads-googleads-v12-enums",
        feature = "google-ads-googleads-v12-errors",
        feature = "google-ads-googleads-v12-resources",
        feature = "google-ads-googleads-v12-services",
    )
)]
pub mod v12;
#[cfg(
    any(
        feature = "google-ads-googleads-v13-common",
        feature = "google-ads-googleads-v13-enums",
        feature = "google-ads-googleads-v13-errors",
        feature = "google-ads-googleads-v13-resources",
        feature = "google-ads-googleads-v13-services",
    )
)]
pub mod v13;
#[cfg(
    any(
        feature = "google-ads-googleads-v14-common",
        feature = "google-ads-googleads-v14-enums",
        feature = "google-ads-googleads-v14-errors",
        feature = "google-ads-googleads-v14-resources",
        feature = "google-ads-googleads-v14-services",
    )
)]
pub mod v14;
