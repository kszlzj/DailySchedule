#[derive(Copy,CLone,Default)]
pub struct PageTable(usize);

const FlAG_RANGE: core::ops::Range<usize> = 0..8;

const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;

impl pageTableEntry{
    pub fn new(page_number:Option<PhysicalPageNumber>,mut flag: Flages)-> Self {
        flags.set(Flags::VALID,page_number.is_some());
    }
    pub fn update_page_number(&mut self, ppn: Option<PhysicalPageNumber>) {
        if let Some(ppn) = ppn {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() | Flags::VALID).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, ppn.into());
        } else {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() - Flags::VALID).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, 0);
        }
    }
    
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }
    
    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }
    
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(..8) as u8) }
    }
    
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}


impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const VALID =       1 << 0;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXECUTABLE =  1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
    }
}

#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8],
}

impl PageTable {
    pub fn zero_init(&mut self) {
        self.entries = [Default::default(); PAGE_SIZE / 8];
    }
}

pub struct PageTableTracker(pub FrameTracker);

impl PageTableTracker {
    pub fn new(frame: FrameTracker) -> Self {
        let mut page_table = Self(frame);
        page_table.zero_init();
        page_table
    }
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0.page_number()
    }
}