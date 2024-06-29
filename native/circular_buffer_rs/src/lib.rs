use std::collections::VecDeque;
use rustler::{Atom, Env, Term};
use rustler::resource::ResourceArc;
use rustler::types::binary::Binary;
use std::sync::Mutex;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;



#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL_ALLOCATOR: Jemalloc = Jemalloc;
mod atoms {
    rustler::atoms! {
        // Common Atoms
        ok,
        error,

        // Resource Atoms
        bad_reference,
        lock_fail,

        // Success Atoms
        added,
        duplicate,
        removed,

        // Error Atoms
        unsupported_type,
        not_found,
        index_out_of_bounds,
        max_size_exceeded,
    }
}


pub struct CircularBufferResource(Mutex<CircularBuffer>);

type CircularBufferArc = ResourceArc<CircularBufferResource>;


rustler::init!("Elixir.CircularBufferRs.Native", [new, push, last, size],
    load = load);

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(CircularBufferResource, env);
    true
}

pub struct CircularBuffer {
    buffer: VecDeque<Vec<u8>>,
    capacity: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> CircularBuffer {
        let buffer = VecDeque::with_capacity(capacity);
        CircularBuffer {
            buffer,
            capacity,
        }
    }

    pub fn push(&mut self, item: Vec<u8>) -> Result<Atom, Atom> {

        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item); // 将新元素添加到缓冲区的末尾
        Ok(atoms::ok())
    }

    pub fn get(&self, index: usize) -> Option<Vec<u8>> {
        self.buffer.get(index).cloned()
    }

    pub fn last(&self, length: usize) -> Result<Vec<Vec<u8>>, Atom> {
        if length > self.buffer.len() {
            return Err(atoms::index_out_of_bounds());
        }
        let end = self.buffer.len();
        let start = end - length;
        let result = self.buffer.range(start..end).cloned().collect();
        Ok(result)
    }


    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}



#[rustler::nif]
fn new(capacity: usize) -> (Atom, CircularBufferArc) {
    let resource = ResourceArc::new(CircularBufferResource(Mutex::new(CircularBuffer::new(capacity))));
    (atoms::ok(), resource)
}

#[rustler::nif]
fn push(resource: ResourceArc<CircularBufferResource>, message: Binary) -> Result<Atom, Atom> {
    let mut buffer = match resource.0.try_lock() {
        Ok(buffer) => buffer,
        Err(_) => return Err(atoms::lock_fail())
    };

    match buffer.push(message.as_slice().to_vec()) {
        Ok(_) => Ok(atoms::ok()),
        Err(e) => Err(e)
    }

}

#[rustler::nif]
fn last(resource: ResourceArc<CircularBufferResource>, num: usize) -> Result<Vec<Vec<u8>>, Atom> {
    let buffer = match resource.0.try_lock() {
        Ok(buffer) => buffer,
        Err(_) => return Err(atoms::lock_fail())
    };

    match buffer.last(num) {
        Ok(result) => Ok(result),
        Err(e) => Err(e)
    }
}


#[rustler::nif]
fn size(resource: ResourceArc<CircularBufferResource>) -> Result<usize, Atom> {
    let buffer = match resource.0.try_lock() {
        Ok(buffer) => buffer,
        Err(_) => return Err(atoms::lock_fail())
    };
    Ok(buffer.capacity())
}

