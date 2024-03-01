//!Implementation of [`TaskManager`]
use super::{TaskControlBlock, current_task, TaskStatus};
use crate::mm::{VirtAddr, MapPermission};
use crate::sync::UPSafeCell;
use crate::syscall::process::TaskInfo;
use crate::timer::get_time_us;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
use crate::syscall::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
    ///
    fn syscall_table_mapping(syscall_id: usize)->usize{
        match syscall_id {
            SYSCALL_WRITE => 0,
            SYSCALL_EXIT => 1,
            SYSCALL_YIELD => 2,
            SYSCALL_GET_TIME => 3,
            SYSCALL_TASK_INFO => 4,
            SYSCALL_MMAP => 5,
            SYSCALL_MUNMAP => 6,
            SYSCALL_SBRK => 7,
            SYSCALL_READ => 8,
            SYSCALL_SET_PRIORITY => 9,
            SYSCALL_FORK => 10,
            SYSCALL_EXEC => 11,
            SYSCALL_WAITPID => 12,
            SYSCALL_GETPID => 13,
            SYSCALL_SPAWN => 14,
            _ => 999
        }
    }
    ///
    fn get_task_info(&self, ti: *mut TaskInfo) {
        // let mut inner = self.fetch().unwrap().inner_exclusive_access();
        let current_tcb = current_task().take().unwrap();
        let current_time = get_time_us() / 1000;
        let syscall_map = TaskManager::syscall_table_mapping;
        unsafe {
            (*ti).time = current_time - current_tcb.start_time;
            (*ti).syscall_times[SYSCALL_WRITE] = current_tcb.syscall_times[syscall_map(SYSCALL_WRITE)];
            (*ti).syscall_times[SYSCALL_EXIT] = current_tcb.syscall_times[syscall_map(SYSCALL_EXIT)];
            (*ti).syscall_times[SYSCALL_YIELD] = current_tcb.syscall_times[syscall_map(SYSCALL_YIELD)];
            (*ti).syscall_times[SYSCALL_GET_TIME] = current_tcb.syscall_times[syscall_map(SYSCALL_GET_TIME)];
            (*ti).syscall_times[SYSCALL_TASK_INFO] = current_tcb.syscall_times[syscall_map(SYSCALL_TASK_INFO)];
            (*ti).syscall_times[SYSCALL_MMAP] = current_tcb.syscall_times[syscall_map(SYSCALL_MMAP)];
            (*ti).syscall_times[SYSCALL_MUNMAP] = current_tcb.syscall_times[syscall_map(SYSCALL_MUNMAP)];
            (*ti).syscall_times[SYSCALL_SBRK] = current_tcb.syscall_times[syscall_map(SYSCALL_SBRK)];
            (*ti).syscall_times[SYSCALL_READ] = current_tcb.syscall_times[syscall_map(SYSCALL_READ)];
            (*ti).syscall_times[SYSCALL_SET_PRIORITY] = current_tcb.syscall_times[syscall_map(SYSCALL_SET_PRIORITY)];
            (*ti).syscall_times[SYSCALL_FORK] = current_tcb.syscall_times[syscall_map(SYSCALL_FORK)];
            (*ti).syscall_times[SYSCALL_EXEC] = current_tcb.syscall_times[syscall_map(SYSCALL_EXEC)];
            (*ti).syscall_times[SYSCALL_WAITPID] = current_tcb.syscall_times[syscall_map(SYSCALL_WAITPID)];
            (*ti).syscall_times[SYSCALL_GETPID] = current_tcb.syscall_times[syscall_map(SYSCALL_GETPID)];
            (*ti).syscall_times[SYSCALL_SPAWN] = current_tcb.syscall_times[syscall_map(SYSCALL_SPAWN)];
            (*ti).status = TaskStatus::Running;
        }
        drop(current_tcb);
    }
     ///
     fn insert_current_map_frame(&self,start_va: VirtAddr,end_va: VirtAddr,permission: MapPermission) {
        // let mut inner = self.inner.exclusive_access();
        // let current = inner.current_task;
        // inner.tasks[current]
        //     .memory_set
        //     .insert_framed_area(start_va, end_va, permission);
        let current_tcb = current_task().unwrap();
        current_tcb.inner_exclusive_access().memory_set.insert_framed_area(start_va, end_va, permission);
    }
    ///
    fn remove_current_map_frame(&self,start_va: VirtAddr,end_va: VirtAddr) {
        // let mut inner = self.inner.exclusive_access();
        // let current = inner.current_task;
        // inner.tasks[current]
        //     .memory_set
        //     .remove_framed_area(start_va, end_va);
        let current_tcb = current_task().unwrap();
        current_tcb.inner_exclusive_access().memory_set.remove_framed_area(start_va, end_va);
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
///
pub fn get_task_info(ti: *mut TaskInfo) {
    TASK_MANAGER.exclusive_access().get_task_info(ti)
}
///
pub fn insert_current_map_frame(start_va: VirtAddr,end_va: VirtAddr,permission: MapPermission) {
    TASK_MANAGER.exclusive_access().insert_current_map_frame(start_va, end_va, permission)
}
///
pub fn remove_current_map_frame(start_va: VirtAddr,end_va: VirtAddr) {
    TASK_MANAGER.exclusive_access().remove_current_map_frame(start_va, end_va)
}