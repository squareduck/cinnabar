use crate::value::{UniqueValueStore, Value};
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Env {
    symbols: UniqueValueStore,
    keywords: UniqueValueStore,
    modules: HashMap<String, Vec<Value>>,
}
