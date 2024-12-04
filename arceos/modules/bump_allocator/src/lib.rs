#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator {
    start: usize, // 内存起始地址
    end: usize,   // 内存结束地址
    b_pos: usize, // 当前字节分配位置
    p_pos: usize, // 当前页分配位置
    count: usize, // 字节分配次数
}

impl EarlyAllocator {
    /// 创建一个新的 EarlyAllocator
    pub const fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            b_pos: start,
            p_pos: end,
            count: 0,
        }
    }
}

/// 实现 BaseAllocator
impl BaseAllocator for EarlyAllocator {
    /// 初始化 EarlyAllocator，指定内存范围
    fn init(&mut self, start: usize, end: usize) {
        self.start = start;
        self.end = end;
        self.b_pos = start;
        self.p_pos = end;
        self.count = 0;
    }

    /// 添加内存区域到分配器
    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        allocator::AllocResult::Ok(())
    }
 }


/// 实现 ByteAllocator
impl ByteAllocator for EarlyAllocator {
    /// 分配字节
    fn alloc(&mut self, layout: Layout) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let offset=self.b_pos%layout.align();
        if offset > 0 {
            self.b_pos += layout.align() - offset;
        }
        if self.b_pos > self.p_pos {
            return allocator::AllocResult::Err(allocator::AllocError::NoMemory);
        }
        let addr=self.b_pos;
        self.b_pos += layout.size();

        Ok(core::ptr::NonNull::new(addr as *mut u8).unwrap())
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: Layout) {
        let pos=pos.as_ptr() as usize;
        if pos + layout.size() == self.b_pos {
            self.b_pos=pos;
        }
        else {

        }
    }

    /// 返回总字节数
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    /// 返回已使用字节数
    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    /// 返回可用字节数
    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }

}
/// 实现 PageAllocator
impl PageAllocator for EarlyAllocator {
    const PAGE_SIZE: usize = 4096; // 定义页大小为 4KB

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> allocator::AllocResult<usize> {
        // 对齐要求，2 的幂次
        let align = 1 << align_pow2;
        let align_mask = align - 1;

        // 计算对齐后的起始位置
        let mut new_pos = (self.p_pos - num_pages * Self::PAGE_SIZE) & !align_mask;

        // 检查是否与 b_pos 冲突
        if new_pos < self.b_pos {
            return Err(allocator::AllocError::NoMemory);
        }

        // 分配成功，更新 p_pos 并返回页的起始地址
        self.p_pos = new_pos;
        Ok(new_pos)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        // EarlyAllocator 假设页面分配是单向的，页面释放可能不会重新分配。
        // 因此，我们可以选择忽略释放操作，或者仅记录日志。
        // 这里为了简单，直接忽略释放。
    }

    /// 返回总页数
    fn total_pages(&self) -> usize {
        (self.end - self.start) / Self::PAGE_SIZE
    }

    /// 返回已使用页数
    fn used_pages(&self) -> usize {
        (self.end - self.p_pos) / Self::PAGE_SIZE
    }

    /// 返回可用页数
    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / Self::PAGE_SIZE
    }
}

