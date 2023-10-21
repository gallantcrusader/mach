use libc::c_uint;
use machine::cpu_subtype_t;
use machine::cpu_threadtype_t;
use machine::cpu_type_t;
use machine::CPU_STATE_MAX;
use vm_types::integer_t;
use vm_types::natural_t;
use core::mem::size_of;

use message::mach_msg_type_number_t;

pub const HOST_BASIC_INFO: c_uint = 1; /* basic info */
pub const HOST_SCHED_INFO: c_uint = 3; /* scheduling info */
pub const HOST_RESOURCE_SIZES: c_uint = 4; /* kernel struct sizes */
pub const HOST_PRIORITY_INFO: c_uint = 5; /* priority information */
pub const HOST_SEMAPHORE_TRAPS: c_uint = 7; /* Has semaphore traps */
pub const HOST_MACH_MSG_TRAP: c_uint = 8; /* Has mach_msg_trap */
pub const HOST_VM_PURGABLE: c_uint = 9; /* purg'e'able memory info */
pub const HOST_DEBUG_INFO_INTERNAL: c_uint = 10; /* Used for kernel internal development tests only */
pub const HOST_CAN_HAS_DEBUGGER: c_uint = 11;
pub const HOST_PREFERRED_USER_ARCH: c_uint = 12;

pub const HOST_LOAD_INFO: c_uint = 1;
pub const HOST_VM_INFO: c_uint = 2;
pub const HOST_CPU_LOAD_INFO: c_uint = 3;


pub static HOST_CPU_LOAD_INFO_COUNT: mach_msg_type_number_t = (size_of::<host_cpu_load_info_data_t>() / size_of::<integer_t>()) as mach_msg_type_number_t;


pub const HOST_INFO_MAX: c_uint = 1024;
pub type host_info_data_t = [integer_t; HOST_INFO_MAX as usize];

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct host_basic_info {
    pub max_cpus: integer_t,
    pub avail_cpus: integer_t,
    pub memory_size: natural_t,
    pub cpu_type: cpu_type_t,
    pub cpu_subtype: cpu_subtype_t,
    pub cpu_threadtype: cpu_threadtype_t,
    pub physical_cpu: integer_t,
    pub physical_cpu_max: integer_t,
    pub logical_cpu: integer_t,
    pub logical_cpu_max: integer_t,
    pub max_mem: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct host_cpu_load_info {
    pub cpu_ticks: [natural_t; CPU_STATE_MAX as usize],
}

pub type host_cpu_load_info_data_t = host_cpu_load_info;
pub type host_info_t = *mut integer_t;
