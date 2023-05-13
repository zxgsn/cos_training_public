#![no_std]
#![feature(asm_const)]

use riscv::register::satp;

pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;

const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;

#[cfg(feature = "sv39")]
#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_SV39: [u64; 512] = [0; 512];

#[cfg(feature = "sv48")]
#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_ROOT: [u64; 512] = [0; 512];

#[cfg(feature = "sv48")]
#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_LEAF1: [u64; 512] = [0; 512];

#[cfg(feature = "sv48")]
#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_LEAF2: [u64; 512] = [0; 512];

#[cfg(feature = "sv39")]
pub unsafe fn pre_mmu() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    // ppn
    BOOT_PT_SV39[2] = (0x80000 << 10) | 0xef;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    // 0x102 = 258
    
    BOOT_PT_SV39[0x102] = (0x80000 << 10) | 0xef;

    // 0xffff_ffff_c000_0000..highest, VRWX_GAD, 1G block
    // 0x1ff = 511 = 2^9 - 1
    BOOT_PT_SV39[0x1ff] = (0x80000 << 10) | 0xef;
}

#[cfg(feature = "sv48")]
pub unsafe fn pre_mmu() {
    // 根据老师课上讲的思路，建了一个根页表和两个叶子页表，进行映射
    // leaf1
    let leaf1_page_id = (BOOT_PT_LEAF1.as_ptr() as u64) >> 12;
    BOOT_PT_ROOT[0] = (leaf1_page_id << 10) | 0x1;
    // leaf2
    let leaf2_page_id = (BOOT_PT_LEAF2.as_ptr() as u64) >> 12;
    BOOT_PT_ROOT[0x1ff] = (leaf2_page_id << 10) | 0x1;
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    BOOT_PT_LEAF1[2] = (0x80000 << 10) | 0xef;
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    // 0x102 = 258 
    BOOT_PT_LEAF2[0x102] = (0x80000 << 10) | 0xef;

    // 0xffff_ffff_c000_0000..highest, VRWX_GAD, 1G block
    // 0x1ff = 511 = 2^9 - 1
    BOOT_PT_LEAF2[0x1ff] = (0x80000 << 10) | 0xef;
}

#[cfg(feature = "sv39")]
pub unsafe fn enable_mmu() {
    let page_table_root = BOOT_PT_SV39.as_ptr() as usize;
    satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

#[cfg(feature = "sv48")]
pub unsafe fn enable_mmu() {
    let page_table_root = BOOT_PT_ROOT.as_ptr() as usize;
    satp::set(satp::Mode::Sv48, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

// #[cfg(feature = "sv39")]
pub unsafe fn post_mmu() {
    core::arch::asm!("
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0
        add     ra, ra, t0
        ret     ",
    phys_virt_offset = const PHYS_VIRT_OFFSET,
    )
}
/*
#[cfg(feature = "sv48")]
pub unsafe fn post_mmu() {}
*/
