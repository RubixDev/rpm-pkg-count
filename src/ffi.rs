#![allow(non_camel_case_types, non_upper_case_globals)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rpmdbMatchIterator_s {
    _unused: [u8; 0],
}
pub type rpmdbMatchIterator = *mut rpmdbMatchIterator_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rpmts_s {
    _unused: [u8; 0],
}
pub type rpmts = *mut rpmts_s;

pub type rpm_tag_t = i32;
pub type rpmDbiTagVal = rpm_tag_t;
pub type size_t = ::std::os::raw::c_ulong;
pub type rpmDbiTag_e = ::std::os::raw::c_uint;

pub const rpmDbiTag_e_RPMDBI_LABEL: rpmDbiTag_e = 2;

extern "C" {
    pub fn rpmReadConfigFiles(
        file: *const ::std::os::raw::c_char,
        target: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn rpmdbFreeIterator(mi: rpmdbMatchIterator) -> rpmdbMatchIterator;
    pub fn rpmdbGetIteratorCount(mi: rpmdbMatchIterator) -> ::std::os::raw::c_int;
    pub fn rpmtsCreate() -> rpmts;
    pub fn rpmtsFree(ts: rpmts) -> rpmts;
    pub fn rpmtsInitIterator(
        ts: rpmts,
        rpmtag: rpmDbiTagVal,
        keyp: *const ::std::os::raw::c_void,
        keylen: size_t,
    ) -> rpmdbMatchIterator;
}
