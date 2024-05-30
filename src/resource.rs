use std::error::Error;

pub trait Read {
    fn read() -> Result<Vec<Self>, Box<dyn Error>>
    where
        Self: Sized;
}

pub trait Save {
    fn save() -> Result<(), Box<dyn Error>>;
}
