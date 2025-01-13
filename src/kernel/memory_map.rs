use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct MemoryFlags: u8 {
        /// Controls whether writes to the mapped frames are allowed.
        ///
        /// If this bit is unset in a level 1 page table entry, the mapped frame is read-only.
        /// If this bit is unset in a higher level page table entry the complete range of mapped
        /// pages is read-only.
        const WRITABLE =        1 << 0;
        /// Controls whether accesses from userspace (i.e. ring 3) are permitted.
        const USER_ACCESSIBLE = 1 << 1;
        /// If this bit is set, a “write-through” policy is used for the cache, else a “write-back”
        /// policy is used.
        const WRITE_THROUGH =   1 << 2;
        /// Disables caching for the pointed entry is cacheable.
        const NO_CACHE =        1 << 3;
        /// Forbid code execution from the mapped frames.
        const NO_EXECUTE =      1 << 4;
    }
}
impl Default for MemoryFlags {
    fn default() -> Self {
        MemoryFlags::WRITABLE | MemoryFlags::USER_ACCESSIBLE
    }
}

pub unsafe trait MemoryMap: Send {
    unsafe fn map_memory(&mut self, from: usize, to: usize, flags: MemoryFlags) -> bool;
    unsafe fn unmap_memory(&mut self, from: usize) -> bool;
    unsafe fn load_memory_map(&self);
}
