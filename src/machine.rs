use vm_types::integer_t;
use libc::c_uint;

pub const CPU_STATE_MAX: c_uint = 4;

pub type cpu_type_t = integer_t;
pub type cpu_subtype_t = integer_t;
pub type cpu_threadtype_t = integer_t;
