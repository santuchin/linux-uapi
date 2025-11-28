
pub type off_t = core::ffi::c_longlong;

pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct io_uring_params {
    pub sq_entries: __u32,
    pub cq_entries: __u32,
    pub flags: __u32,
    pub sq_thread_cpu: __u32,
    pub sq_thread_idle: __u32,
    pub features: __u32,
    pub wq_fd: __u32,
    pub resv: [__u32; 3usize],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct io_sqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub flags: __u32,
    pub dropped: __u32,
    pub array: __u32,
    pub resv1: __u32,
    pub resv2: __u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct io_cqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub overflow: __u32,
    pub cqes: __u32,
    pub resv1: __u64,
    pub resv2: __u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct io_uring_cqe {
	pub user_data: u64,
	pub res: i32,
	pub flags: u32,
	pub big_cqe: *mut u64,
}
