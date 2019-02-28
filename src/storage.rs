use crate::stream::Error;

pub trait Storage {
    fn get(&self, i: u32) -> Result<u32, Error>;
    fn set(&mut self, i: u32, v: u32) -> Result<(), Error>;
    fn count(&self) -> usize;
}

impl Storage {
    pub fn iter<'a>(&'a self) -> StorageIterator<'a> {
        StorageIterator {
            mem: Box::new(self),
            pos: 0,
        }
    }
}

pub struct StorageIterator<'a> {
    mem: Box<&'a Storage>,
    pos: u32,
}

impl<'a> Iterator for StorageIterator<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(itm) = (*self.mem).get(self.pos).ok() {
            self.pos += 1;
            Some((self.pos - 1, itm))
        } else {
            None
        }
    }
}
