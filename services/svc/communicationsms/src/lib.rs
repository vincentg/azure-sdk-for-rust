#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-sms-2021-03-07")]
pub mod package_sms_2021_03_07;
#[cfg(all(feature = "package-sms-2021-03-07", not(feature = "no-default-tag")))]
pub use package_sms_2021_03_07::*;
#[cfg(feature = "package-2020-07-20-preview1")]
pub mod package_2020_07_20_preview1;
#[cfg(all(feature = "package-2020-07-20-preview1", not(feature = "no-default-tag")))]
pub use package_2020_07_20_preview1::*;
