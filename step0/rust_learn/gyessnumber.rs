pub struct Guess {
    value:u32,
}
impl Guess{
    pub fn new(value:u32)->Guess{
        if value<1 || value>100{
            panic!("Guess value must between 1 and 100mgot {}.",value);
        }
        Guess{
            value
        }
    }
    pub fn value(&self)->u32{
        self.value
    }
}