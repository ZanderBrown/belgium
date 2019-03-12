use crate::storage::Storage;
use crate::stream::Error;
use std::rc::Weak;

#[derive(Clone)]
pub struct ChangeEvent {
    pub idx: u32,
    pub val: u32,
}

pub trait Observer<T> {
    fn notify(&self, evt: T);
}

impl Memory {
    pub fn add_observer(&mut self, obs: Weak<dyn Observer<ChangeEvent>>) {
        self.listeners.push(obs);
    }

    pub fn emit(&self, evt: &ChangeEvent) {
        for l in &self.listeners.clone() {
            if let Some(ref l) = l.upgrade() {
                l.notify(evt.clone());
            }
        }
    }

    pub fn create(name: String, count: usize) -> Self {
        let mut backing = Vec::with_capacity(count);
        for _ in 0..count {
            backing.push(0);
        }
        Self {
            name,
            backing,
            listeners: vec![],
        }
    }
}

pub struct Memory {
    name: String,
    backing: Vec<u32>,
    listeners: Vec<Weak<Observer<ChangeEvent>>>,
}

impl Storage for Memory {
    fn get(&self, i: u32) -> Result<u32, Error> {
        if (i as usize) >= self.backing.len() {
            Err(Error::new(format!("Invalid {} {}", i, self.name), None))
        } else {
            Ok(self.backing[i as usize])
        }
    }

    fn set(&mut self, i: u32, v: u32) -> Result<(), Error> {
        if (i as usize) >= self.backing.len() {
            Err(Error::new(format!("Invalid {} {}", self.name, i), None))
        } else {
            self.backing[i as usize] = v;
            self.emit(&ChangeEvent { idx: i, val: v });
            Ok(())
        }
    }

    fn count(&self) -> usize {
        self.backing.len()
    }
}
