// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// at http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Helper functions to be used with Serde serializer.
//! Currently this includes adapters allowing to (de)serialize PathBuf using
//! Serde, but retaining format used by rustc_serialize
//! (https://github.com/rust-lang-deprecated/rustc-serialize/blob/master/src/serialize.rs#L1358-L1409)

use std::ffi::OsString;
use std::path::PathBuf;

use serde::de::{self, Deserialize, Deserializer};

#[cfg(windows)]
type PathByteWidth = u16;
#[cfg(not(windows))]
type PathByteWidth = u8;

#[cfg(windows)]
fn osstring_from_bytes(bytes: Vec<PathByteWidth>) -> Result<OsString, ()> {
    use std::os::windows::ffi::OsStringExt;

    Ok(OsStringExt::from_wide(&bytes))
}

#[cfg(unix)]
fn osstring_from_bytes(bytes: Vec<PathByteWidth>) -> Result<OsString, ()> {
    use std::os::unix::ffi::OsStringExt;

    Ok(OsStringExt::from_vec(bytes))
}

pub fn decode_pathbuf<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where D: Deserializer<'de>
{
    let bytes = Vec::<PathByteWidth>::deserialize(deserializer)?;
    let osstring = osstring_from_bytes(bytes.clone())
        .map_err(|_| de::Error::custom("invalid bytes for OsString type"))?;

    Ok(PathBuf::from(osstring))
}
