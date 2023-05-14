use crate::ffi_types::*;

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
