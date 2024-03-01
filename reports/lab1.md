参考 https://cloud.tsinghua.edu.cn/f/17a7c9d9b57f4838ae5f/ 完成此次专题
```
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM], /// This will trigger some BUG, using [u32; MAX_SYSCALL_NUM] instead
    ...
}
```
由于变更了 syscall_times 大小，因此在传入 syscall_id 时需要重新 mapping 到 syscall_times 中