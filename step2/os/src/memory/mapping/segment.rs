#[derive(Debug)]
pub enum MapType {
    Linear,
    Framed,
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Segment{
    pub map_type:MapType,
    pub range: Range<VirtualAddress>,
    pub flags:Flags,
}

impl Segment{
    pub fn iter_mapped(&self) -> Option<>
}