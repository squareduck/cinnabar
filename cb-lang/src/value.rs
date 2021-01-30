use bimap::BiMap;
use num::Integer;
use ordered_float::OrderedFloat;
use rpds::{List, RedBlackTreeMap, Vector};
use std::hash::Hash;
use std::rc::Rc;

pub(crate) type KeywordId = usize;
pub(crate) type SymbolId = usize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Value {
    Int(i64),
    Float(OrderedFloat<f64>),
    Bool(bool),
    Char(char),
    String(String),
    Keyword(KeywordId),
    Symbol(SymbolId),
    List(List<RcValue>),
    Vector(Vector<RcValue>),
    Map(RedBlackTreeMap<RcValue, RcValue>),
}

pub(crate) type RcValue = Rc<Value>;

#[derive(Default)]
pub(crate) struct UniqueValueStore {
    counter: usize,
    map: BiMap<usize, String>,
}

impl UniqueValueStore {
    fn new() -> Self {
        Self {
            counter: Default::default(),
            map: BiMap::new(),
        }
    }

    pub fn put(&mut self, name: &str) -> usize {
        match self.map.get_by_right(name) {
            Some(id) => *id,
            None => {
                let id = self.counter;
                self.map.insert(id, name.to_string());
                self.counter += 1;
                id
            }
        }
    }

    pub fn get_by_id(&mut self, id: usize) -> Option<&String> {
        self.map.get_by_left(&id)
    }

    pub fn get_by_name(&mut self, name: &str) -> Option<usize> {
        match self.map.get_by_right(name) {
            Some(id) => Some(*id),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context() {
        let mut ctx = UniqueValueStore::new();

        let id_a = ctx.put("a");
        let id_b = ctx.put("b");

        assert_eq!(ctx.get_by_id(id_a).unwrap(), "a");
        assert_eq!(ctx.get_by_name("a").unwrap(), id_a);

        assert_eq!(ctx.get_by_id(id_b).unwrap(), "b");
        assert_eq!(ctx.get_by_name("b").unwrap(), id_b);

        assert!(id_a != id_b);
    }
}
