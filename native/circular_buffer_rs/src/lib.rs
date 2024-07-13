use rustler::{Atom, Env, Term};
use rustler::types::tuple::get_tuple;
use rustler::resource::ResourceArc;
use std::sync::Mutex;
mod supported_term;
use crate::supported_term::SupportedTerm;

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
    buffer: Box<[Option<SupportedTerm>]>,
    capacity: usize,
    size: usize,
    start: usize,
    end: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> CircularBuffer {
        let buffer = vec![None; capacity].into_boxed_slice();
        CircularBuffer {
            buffer,
            capacity,
            size: 0,
            start: 0,
            end: 0,
        }
    }

    pub fn push(&mut self, item: SupportedTerm) -> Atom {
        self.buffer[self.end] = Some(item);
        self.end = (self.end + 1) % self.capacity;
        if self.size < self.capacity {
            self.size += 1;
        } else {
            self.start = (self.start + 1) % self.capacity;
        }
        atoms::ok()
    }

    pub fn get(&self, index: usize) -> Option<&SupportedTerm> {
        if index >= self.size {
            return None;
        }
        self.buffer[(self.start + index) % self.capacity].as_ref()
    }

    pub fn last(&self, length: usize) -> Result<Vec<SupportedTerm>, Atom> {
        if length > self.size {
            return Err(atoms::index_out_of_bounds());
        }
        let mut result = Vec::with_capacity(length);
        for i in 0..length {
            let idx = (self.end + self.capacity - length + i) % self.capacity;
            if let Some(ref item) = self.buffer[idx] {
                result.push(item.clone());
            }
        }
        Ok(result)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
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
fn push(resource: CircularBufferArc, term: Term) -> Atom {
    let item = match convert_to_supported_term(&term) {
        None => return atoms::unsupported_type(),
        Some(term) => term,
    };

    with_locked_resource(resource, |buffer| buffer.push(item))
}

#[rustler::nif]
fn last(resource: CircularBufferArc, num: usize) -> Result<Vec<SupportedTerm>, Atom> {
    with_locked_resource(resource, |buffer| buffer.last(num))
}

#[rustler::nif]
fn size(resource: CircularBufferArc) -> Result<usize, Atom> {
    with_locked_resource(resource, |buffer| Ok(buffer.capacity()))
}

fn with_locked_resource<F, R>(resource: CircularBufferArc, f: F) -> R
where
    F: FnOnce(&mut CircularBuffer) -> R,
{
    match resource.0.lock() {
        Ok(mut buffer) => f(&mut buffer),
        Err(_) => panic!("Failed to lock the buffer"), // Or you can use a custom error handling
    }
}

fn convert_to_supported_term(term: &Term) -> Option<SupportedTerm> {
    if term.is_number() {
        term.decode().ok().map(SupportedTerm::Integer)
    } else if term.is_atom() {
        term.atom_to_string().ok().map(SupportedTerm::Atom)
    } else if term.is_tuple() {
        get_tuple(*term)
            .ok()
            .and_then(|t| {
                let initial_length = t.len();
                let inner_terms: Vec<SupportedTerm> = t
                    .into_iter()
                    .filter_map(|i: Term| convert_to_supported_term(&i))
                    .collect();
                (initial_length == inner_terms.len()).then_some(SupportedTerm::Tuple(inner_terms))
            })
    } else if term.is_list() {
        term.decode::<Vec<Term>>()
            .ok()
            .and_then(|l| {
                let initial_length = l.len();
                let inner_terms: Vec<SupportedTerm> = l
                    .into_iter()
                    .filter_map(|i: Term| convert_to_supported_term(&i))
                    .collect();
                (initial_length == inner_terms.len()).then_some(SupportedTerm::List(inner_terms))
            })
    } else if term.is_binary() {
        term.decode().ok().map(SupportedTerm::Bitstring)
    } else {
        None
    }
}
