use libc::c_uint;
use vm_types::integer_t;

pub const CPU_STATE_MAX: c_uint = 4;
pub const CPU_STATE_USER: c_uint = 0;
pub const CPU_STATE_SYSTEM: c_uint = 1;
pub const CPU_STATE_IDLE: c_uint = 2;
pub const CPU_STATE_NICE: c_uint = 3;

pub type cpu_type_t = integer_t;
pub type cpu_subtype_t = integer_t;
pub type cpu_threadtype_t = integer_t;
