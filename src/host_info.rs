use libc::c_uint;
use vm_types::integer_t;
use vm_types::natural_t;
use machine::cpu_type_t;
use machine::cpu_threadtype_t;
use machine::cpu_subtype_t;
use machine::CPU_STATE_MAX;

pub const HOST_BASIC_INFO       : c_uint = 1;       /* basic info */
pub const HOST_SCHED_INFO       : c_uint = 3;       /* scheduling info */
pub const HOST_RESOURCE_SIZES   : c_uint = 4;       /* kernel struct sizes */
pub const HOST_PRIORITY_INFO    : c_uint = 5;       /* priority information */
pub const HOST_SEMAPHORE_TRAPS  : c_uint = 7;       /* Has semaphore traps */
pub const HOST_MACH_MSG_TRAP    : c_uint = 8;       /* Has mach_msg_trap */
pub const HOST_VM_PURGABLE      : c_uint = 9;       /* purg'e'able memory info */
pub const HOST_DEBUG_INFO_INTERNAL : c_uint = 10;   /* Used for kernel internal development tests only */
pub const HOST_CAN_HAS_DEBUGGER : c_uint = 11;
pub const HOST_PREFERRED_USER_ARCH : c_uint = 12;

pub const HOST_LOAD_INFO: c_uint = 1;
pub const HOST_VM_INFO: c_uint = 2;
pub const HOST_CPU_LOAD_INFO : c_uint = 3;

pub const HOST_INFO_MAX: c_uint = 1024;
pub type host_info_data_t = [integer_t; HOST_INFO_MAX as usize];


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct host_basic_info {
    max_cpus: integer_t,
    avail_cpus: integer_t,
    memory_size: natural_t,
    cpu_type: cpu_type_t,
    cpu_subtype: cpu_subtype_t,
    cpu_threadtype: cpu_threadtype_t,
    physical_cpu: integer_t,
    physical_cpu_max: integer_t,
    logical_cpu: integer_t,
    logical_cpu_max: integer_t,
    max_mem: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct host_cpu_load_info {
    cpu_ticks: [natural_t; CPU_STATE_MAX as usize],
}

pub type host_cpu_load_info_data_t = host_cpu_load_info;
