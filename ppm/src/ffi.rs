use libc::{c_int, c_void};
#[link(name = "ppma_io")]
extern "C" {
    pub fn ppma_write_test ( file_out_name: *const u8) -> c_int;
    pub fn ppma_read_test ( inputname: *const u8 ) -> c_void;
}