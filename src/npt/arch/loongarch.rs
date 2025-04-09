use page_table_entry::loongarch64::LA64PTE;
use page_table_multiarch::{PageTable64, loongarch64::LA64PageTable};

use crate::GuestPhysAddr;

pub type NestedPageTable<H> = PageTable64<LA64PageTable<GuestPhysAddr>, LA64PTE, H>;
