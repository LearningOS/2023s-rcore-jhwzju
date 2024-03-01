//! Process management syscalls

use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, get_task_info
    }, timer::get_time_us,
    mm::{translated_physical_address, VirtAddr, MapPermission, mmap_page, unmap_page}
};

#[repr(C)]
#[derive(Debug)]
///
pub struct TimeVal {
    ///
    pub sec: usize,
    ///
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let ts = translated_physical_address(current_user_token(), _ts as *const u8) as *mut TimeVal;
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    // trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    trace!("kernel: sys_task_info");
    let ti = translated_physical_address(current_user_token(), _ti as *const u8) as *mut TaskInfo;
    get_task_info(ti);
    0
}

/// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    // trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    trace!("kernel: sys_map");
    if len == 0 {
        return 0;
    }
    if port & !0b111 != 0 || port & 0b111 == 0 {
        return -1;
    }
    let mut permission = MapPermission::U;

    if (port & 0b001) != 0 {
        permission |= MapPermission::R;
    }
    if (port & 0b010) != 0 {
        permission |= MapPermission::W;
    }
    if (port & 0b100) != 0 {
        permission |= MapPermission::X;
    }
    let start_vaddress: VirtAddr = start.into();
    if !start_vaddress.aligned() {
        debug!("Mapping address failed cause didn't aligned");
        return -1;
    }
    mmap_page(current_user_token(), start, len, permission)
}

/// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let start_vaddress: VirtAddr = start.into();
    if !start_vaddress.aligned() {
        debug!("Mapping address failed cause didn't aligned");
        return -1;
    }
    unmap_page(current_user_token(), start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
