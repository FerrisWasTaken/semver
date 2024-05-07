mod err;
mod parsers;
mod serde_impl;

pub use serde::{Deserialize, Serialize};

pub(crate) mod comparator;
pub(crate) mod version;
pub(crate) mod versionreq;

pub use crate::{comparator::Comparator, version::Version, versionreq::VersionReq};
