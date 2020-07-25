#[derive(Default)]
pub struct Mapping {
    page_tables: Vec<PageTableTracker>,
    root_ppn: PhysicalPageNumber,
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}

impl Mapping {
    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }
    pub fn find_entry(&mut self,vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry>{
        let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                *entry = PageTableEntry::new(Some(new_ppn), Flags::VALID);
                self.page_tables.push(new_table);
            }
            entry = &mut entry.get_next_table().entries[*vpn_slice];
        }
        Ok(entry)
    }
    fn map_one(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: Option<PhysicalPageNumber>,
        flags: Flags,
    ) -> MemoryResult<()>{
        let entry = self.find_entry(vpn)?;
        assert!(entry.is_empty(), "virtual address is already mapped");
        *entry  = PageTableEntry::new(ppn,flags);
        Ok(())
    }
}