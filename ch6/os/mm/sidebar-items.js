window.SIDEBAR_ITEMS = {"fn":[["frame_alloc","Allocate a physical page frame in FrameTracker style"],["frame_dealloc","Deallocate a physical page frame with a given ppn"],["init","initiate heap allocator, frame allocator and kernel space"],["kernel_token","the kernel token"],["remap_test","remap test in kernel space"],["translated_byte_buffer","Translate&Copy a ptr[u8] array with LENGTH len to a mutable u8 Vec through page table"],["translated_ref","Translate a ptr[u8] array through page table and return a reference of T"],["translated_refmut","Translate a ptr[u8] array through page table and return a mutable reference of T"],["translated_str","Translate&Copy a ptr[u8] array end with `\\0` to a `String` Vec through page table"]],"mod":[["address","Implementation of physical and virtual address and page number."],["frame_allocator","Implementation of [`FrameAllocator`] which controls all the frames in the operating system."],["heap_allocator","The global allocator"],["memory_set","Implementation of [`MapArea`] and [`MemorySet`]."],["page_table","Implementation of [`PageTableEntry`] and [`PageTable`]."]],"struct":[["FrameTracker","tracker for physical page frame allocation and deallocation"],["KERNEL_SPACE","The kernel’s initial memory mapping(kernel address space)"],["MapPermission","map permission corresponding to that in pte: `R W X U`"],["MemorySet","address space"],["PageTable","page table structure"],["PageTableEntry","page table entry structure"],["PhysAddr","Definitions"],["PhysPageNum","Physical Page Number PPN phiscal page number"],["UserBuffer","An abstraction over a buffer passed from user space to kernel space"],["UserBufferIterator","An iterator over a UserBuffer"],["VirtAddr","Virtual Address virtual address"],["VirtPageNum","Virtual Page Number VPN"]],"trait":[["StepByOne","iterator for phy/virt page number"]]};