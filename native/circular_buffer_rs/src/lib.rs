use std::collections::VecDeque;
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
    buffer: VecDeque<SupportedTerm>,
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

    pub fn push(&mut self, item: SupportedTerm) -> Atom {

        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item); // 将新元素添加到缓冲区的末尾
        atoms::ok()
    }

    pub fn get(&self, index: usize) -> Option<&SupportedTerm> {
        self.buffer.get(index)
    }

    pub fn last(&self, length: usize) -> Result<Vec<SupportedTerm>, Atom> {
        if length > self.buffer.len() {
            return Err(atoms::index_out_of_bounds());
        }
        let end = self.buffer.len();
        let start = end - length;
        let result: Vec<SupportedTerm> = self.buffer.range(start..end).cloned().collect();
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
fn push(resource: ResourceArc<CircularBufferResource>, term: Term) -> Atom {
    let item = match convert_to_supported_term(&term) {
        None => return atoms::unsupported_type(),
        Some(term) => term,
    };
    
    let mut buffer = match resource.0.try_lock() {
        Ok(buffer) => buffer,
        Err(_) => return atoms::lock_fail()
    };
    buffer.push(item) 
}

#[rustler::nif]
fn last(resource: ResourceArc<CircularBufferResource>, num: usize) -> Result<Vec<SupportedTerm>, Atom> {
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



fn convert_to_supported_term(term: &Term) -> Option<SupportedTerm> {
    if term.is_number() {
        match term.decode() {
            Ok(i) => Some(SupportedTerm::Integer(i)),
            Err(_) => None,
        }
    } else if term.is_atom() {
        match term.atom_to_string() {
            Ok(a) => Some(SupportedTerm::Atom(a)),
            Err(_) => None,
        }
    } else if term.is_tuple() {
        match get_tuple(*term) {
            Ok(t) => {
                let initial_length = t.len();
                let inner_terms: Vec<SupportedTerm> = t
                    .into_iter()
                    .filter_map(|i: Term| convert_to_supported_term(&i))
                    .collect();
                if initial_length == inner_terms.len() {
                    Some(SupportedTerm::Tuple(inner_terms))
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    } else if term.is_list() {
        match term.decode::<Vec<Term>>() {
            Ok(l) => {
                let initial_length = l.len();
                let inner_terms: Vec<SupportedTerm> = l
                    .into_iter()
                    .filter_map(|i: Term| convert_to_supported_term(&i))
                    .collect();
                if initial_length == inner_terms.len() {
                    Some(SupportedTerm::List(inner_terms))
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    } else if term.is_binary() {
        match term.decode() {
            Ok(b) => Some(SupportedTerm::Bitstring(b)),
            Err(_) => None,
        }
    } else {
        None
    }
}
