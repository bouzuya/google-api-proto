#[cfg(
    any(
        feature = "google-ads-googleads-v15-common",
        feature = "google-ads-googleads-v15-enums",
        feature = "google-ads-googleads-v15-errors",
        feature = "google-ads-googleads-v15-resources",
        feature = "google-ads-googleads-v15-services",
    )
)]
pub mod v15;
#[cfg(
    any(
        feature = "google-ads-googleads-v16-common",
        feature = "google-ads-googleads-v16-enums",
        feature = "google-ads-googleads-v16-errors",
        feature = "google-ads-googleads-v16-resources",
        feature = "google-ads-googleads-v16-services",
    )
)]
pub mod v16;
