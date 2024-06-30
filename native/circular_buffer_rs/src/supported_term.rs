use rustler::types::atom::Atom;
use rustler::types::tuple::make_tuple;
use rustler::Encoder;
use rustler::Env;
use rustler::Term;

use crate::atoms;

/// SupportedTerm is an enum that covers all the Erlang / Elixir term types that can be stored in
/// a SortedSet.
///
/// There are a number of types that are not supported because of their complexity and the
/// difficulty of safely implementing their storage.
///
/// Types that are not supported
///   - Reference
///   - Function
///   - Port
///   - Pid
///
/// Types that are supported but not explicitly listed
///   - Boolean (Note that booleans in Erlang / Elixir are just atoms)
#[derive(Clone)]
pub enum SupportedTerm {
    Integer(i64),
    Atom(String),
    Tuple(Vec<SupportedTerm>),
    List(Vec<SupportedTerm>),
    Bitstring(String),
}

impl Encoder for SupportedTerm {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            SupportedTerm::Integer(inner) => inner.encode(env),
            SupportedTerm::Atom(inner) => match Atom::from_str(env, inner) {
                Ok(atom) => atom.encode(env),
                Err(_) => atoms::error().encode(env),
            },
            SupportedTerm::Tuple(inner) => {
                let terms: Vec<_> = inner.iter().map(|t| t.encode(env)).collect();
                make_tuple(env, terms.as_ref()).encode(env)
            }
            SupportedTerm::List(inner) => inner.encode(env),
            SupportedTerm::Bitstring(inner) => inner.encode(env),
        }
    }
}
