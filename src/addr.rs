use memory_addr::{AddrRange, PhysAddr, VirtAddr, def_usize_addr, def_usize_addr_formatter};

/// Host virtual address.
pub type HostVirtAddr = VirtAddr;
/// Host physical address.
pub type HostPhysAddr = PhysAddr;

def_usize_addr! {
    /// Guest virtual address.
    pub type GuestVirtAddr;
    /// Guest physical address.
    pub type GuestPhysAddr;
}

def_usize_addr_formatter! {
    GuestVirtAddr = "GVA:{}";
    GuestPhysAddr = "GPA:{}";
}

/// Guest virtual address range.
pub type GuestVirtAddrRange = AddrRange<GuestVirtAddr>;
/// Guest physical address range.
pub type GuestPhysAddrRange = AddrRange<GuestPhysAddr>;

//loongarch64 
// #[cfg(target_arch = "loongarch64")]
// impl page_table_multiarch::loongarch64::SvVirtAddr for GuestPhysAddr {
//     fn flush_tlb(vaddr: Option<Self>) {
//         unsafe {
//             match vaddr {
//                 Some(addr) => {
//                     // 刷新某一个虚拟地址的 TLB 映射
//                     core::arch::asm!(
//                         "invtlb {type}, {asid}, {vaddr}",
//                         type = const 0,         // 通常 0 表示按地址失效
//                         asid = in(reg) 0,       // ASID（地址空间 ID），这里默认写死为 0，可按实际情况修改
//                         vaddr = in(reg) addr.0, // addr.0 提取 u64 数值（假设 GuestPhysAddr 是个新类型）
//                         options(nostack, preserves_flags)
//                     );
//                 }
//                 None => {
//                     // 刷新整个 TLB
//                     core::arch::asm!(
//                         "invtlb {type}, {asid}, $zero",
//                         type = const 1, // 通常 1 表示全局失效
//                         asid = in(reg) 0,
//                         options(nostack, preserves_flags)
//                     );
//                 }
//             }
//         }
//     }
// }

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
impl page_table_multiarch::riscv::SvVirtAddr for GuestPhysAddr {
    fn flush_tlb(_vaddr: Option<Self>) {
        todo!()
    }
}
