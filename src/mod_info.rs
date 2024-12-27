use std::{
    ffi::{CStr, CString},
    fmt::{self, Display, Formatter},
};

use crate::scotland2_raw::CModInfo;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModInfoBuf {
    pub id: String,
    pub version: String,
    pub version_long: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModInfo<'a> {
    pub id: &'a CStr,
    pub version: &'a CStr,
    pub version_long: usize,
}

impl ModInfoBuf {
    pub fn new(id: String, version: String, version_long: usize) -> Self {
        Self {
            id,
            version,
            version_long,
        }
    }
}

// Implementing From trait for conversion from ModInfoBuf to CModInfo
impl From<ModInfoBuf> for CModInfo {
    fn from(val: ModInfoBuf) -> Self {
        CModInfo {
            id: CString::new(val.id).unwrap().into_raw(),
            version: CString::new(val.version).unwrap().into_raw(),
            version_long: val.version_long as u64,
        }
    }
}
impl From<CModInfo> for ModInfoBuf {
    fn from(val: CModInfo) -> Self {
        Self {
            id: unsafe { CStr::from_ptr(val.id) }
                .to_string_lossy()
                .to_string(),
            version: unsafe { CStr::from_ptr(val.version) }
                .to_string_lossy()
                .to_string(),
            version_long: val.version_long as usize,
        }
    }
}

// Implementing From trait for conversion from CModInfo to ModInfo
impl From<CModInfo> for ModInfo<'_> {
    fn from(val: CModInfo) -> Self {
        Self {
            id: unsafe { CStr::from_ptr(val.id) },
            version: unsafe { CStr::from_ptr(val.version) },
            version_long: val.version_long as usize,
        }
    }
}

// display

impl Display for ModInfoBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ModInfoBuf {{ id: {}, version: {}, version_long: {} }}",
            self.id, self.version, self.version_long
        )
    }
}

impl Display for ModInfo<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ModInfo {{ id: {}, version: {}, version_long: {} }}",
            self.id.to_string_lossy(),
            self.version.to_string_lossy(),
            self.version_long
        )
    }
}

impl Display for CModInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CModInfo {{ id: {}, version: {}, version_long: {} }}",
            unsafe { CStr::from_ptr(self.id) }.to_string_lossy(),
            unsafe { CStr::from_ptr(self.version) }.to_string_lossy(),
            self.version_long
        )
    }
}
