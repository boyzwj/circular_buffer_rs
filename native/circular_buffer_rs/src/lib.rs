use std::collections::VecDeque;
use dashmap::DashMap;
use std::sync::Arc;
use rustler::Atom;
use rustler::types::binary::Binary;
use std::sync::RwLock;
use lazy_static::lazy_static;

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


pub struct CircularBuffer {
    buffer: RwLock<VecDeque<Vec<u8>>>,
    capacity: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: RwLock::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }

    pub fn push(&self, item: Vec<u8>) -> Result<Atom, Atom> {
        let mut write_guard = self.buffer.write().map_err(|_| atoms::lock_fail())?;
        if write_guard.len() == self.capacity {
            write_guard.pop_front(); // 如果缓冲区已满，移除最旧的元素
        }
        write_guard.push_back(item); // 将新元素添加到缓冲区的末尾
        Ok(atoms::ok())
    }

    pub fn get(&self, index: usize) -> Option<Vec<u8>> {
        self.buffer.read().unwrap().get(index).cloned()
    }

    pub fn last(&self, length: usize) -> Result<Vec<Vec<u8>>, Atom> {
        let read_guard = self.buffer.read().map_err(|_| atoms::lock_fail())?;
        if length > read_guard.len() {
            return Err(atoms::index_out_of_bounds());
        }
        let end = read_guard.len();
        let start = end - length;
        let result = read_guard.range(start..end).cloned().collect();
        Ok(result)
    }


    pub fn len(&self) -> usize {
        self.buffer.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.read().unwrap().is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}




struct PlayerRingBuffer {
    buffers: Arc<DashMap<u64, CircularBuffer>>,
}

impl PlayerRingBuffer {
    fn new() -> Self {
        PlayerRingBuffer {
            buffers:  Arc::new(DashMap::new()),
        }
    }

    fn create_buffer(&self, player_id: u64, capacity: usize) {
        self.buffers.insert(player_id, CircularBuffer::new(capacity));
    }


}

lazy_static! {
    static ref PLAYER_RING_BUFFER: PlayerRingBuffer = PlayerRingBuffer::new();
}


#[rustler::nif]
fn new(uid: u64, capacity: usize) -> Result<Atom, Atom> {
    PLAYER_RING_BUFFER.create_buffer(uid, capacity);
    Ok(atoms::ok())
}

#[rustler::nif]
fn remove(uid: u64) -> Result<Atom, Atom> {
    PLAYER_RING_BUFFER.buffers.remove(&uid);
    Ok(atoms::ok())
}

#[rustler::nif]
fn push(uid: u64, message: Binary) -> Result<Atom, Atom> {
    let message = message.as_slice().to_vec();
    match PLAYER_RING_BUFFER.buffers.get(&uid) {
        Some(buffer) =>  buffer.push(message),
        None => Err(atoms::not_found())
    }
}

#[rustler::nif]
fn last(uid: u64, num: usize) -> Result<Vec<Vec<u8>>, Atom> {
    if num == 0 {
        return Ok(Vec::new());
    }
    match PLAYER_RING_BUFFER.buffers.get(&uid) {
        Some(buffer) => {
            buffer.last(num)
        }
        None => Err(atoms::not_found())
    }
}

rustler::init!("Elixir.CircularBufferRs.Native", [new,remove,push,last]);

