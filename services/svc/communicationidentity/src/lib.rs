#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2021-10")]
pub mod package_preview_2021_10;
#[cfg(all(feature = "package-preview-2021-10", not(feature = "no-default-tag")))]
pub use package_preview_2021_10::*;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "no-default-tag")))]
pub use package_2022_10::*;
#[cfg(feature = "package-2022-06")]
pub mod package_2022_06;
#[cfg(all(feature = "package-2022-06", not(feature = "no-default-tag")))]
pub use package_2022_06::*;
#[cfg(feature = "package-2021-03-31-preview1")]
pub mod package_2021_03_31_preview1;
#[cfg(all(feature = "package-2021-03-31-preview1", not(feature = "no-default-tag")))]
pub use package_2021_03_31_preview1::*;
#[cfg(feature = "package-2021-03-07")]
pub mod package_2021_03_07;
#[cfg(all(feature = "package-2021-03-07", not(feature = "no-default-tag")))]
pub use package_2021_03_07::*;
