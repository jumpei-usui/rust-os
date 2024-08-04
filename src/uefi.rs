use core::ffi::c_void;

#[repr(usize)]
pub enum EfiStatus {
    Success = 0,
}

#[repr(C)]
pub struct EfiHandle(*const c_void);

#[repr(C)]
struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct EfiSystemTable {
    hdr: EfiTableHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    con_in: usize,
    console_out_handle: EfiHandle,
    con_out: *const EfiSimpleTextOutputProtocol,
}

impl EfiSystemTable {
    pub fn con_out(&self) -> &EfiSimpleTextOutputProtocol {
        unsafe { &(*self.con_out) }
    }
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    reset: unsafe fn(this: &Self, extended_verification: bool) -> EfiStatus,
    output_string: unsafe fn(this: &Self, string: *const u16) -> EfiStatus,
}

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiStatus {
        unsafe { (self.reset)(self, extended_verification) }
    }

    pub fn output_string(&self, string: *const u16) -> EfiStatus {
        unsafe { (self.output_string)(self, string) }
    }
}
