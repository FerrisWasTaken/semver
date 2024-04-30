mod err;
mod parsers;
mod serde_impl;

pub use serde::{Serialize, Deserialize};

pub(crate) mod comparator;
pub(crate) mod version;
pub(crate) mod versionreq;

pub use crate::{comparator::Comparator, version::Version, versionreq::VersionReq};