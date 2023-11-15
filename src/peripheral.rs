use std::error::Error;

pub trait Pp {
    fn begin(&self) -> Result<(), Box<dyn Error>>;
    fn end(&self) -> Result<(), Box<dyn Error>>;
    fn write(&self, data: &[u8]) -> Result<(), Box<dyn Error>>;
    fn read(&self, data: &mut [u8]) -> Result<(), Box<dyn Error>>;
}

