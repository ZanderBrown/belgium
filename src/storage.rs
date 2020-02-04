use crate::stream::Error;

pub trait Storage {
    fn get(&self, i: u32) -> Result<u32, Error>;
    fn set(&mut self, i: u32, v: u32) -> Result<(), Error>;
    fn count(&self) -> usize;
}

impl dyn Storage {
    pub fn iter(&self) -> Iter {
        Iter {
            mem: Box::new(self),
            pos: 0,
        }
    }
}

pub struct Iter<'a> {
    mem: Box<&'a dyn Storage>,
    pos: u32,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(itm) = (*self.mem).get(self.pos) {
            self.pos += 1;
            Some((self.pos - 1, itm))
        } else {
            None
        }
    }
}
