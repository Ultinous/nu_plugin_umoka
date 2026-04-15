use std::sync::{Arc, Mutex};

use nu_plugin::{Plugin, PluginCommand};

use crate::commands::{
    Clear, Delete, Get, GetOrPut, Has, Incr, Put, PutIfAbsent, Stats, Take, UMoka,
};
use crate::store::Store;

const DEFAULT_MAX_CAPACITY: u64 = 1024;

pub struct UMokaPlugin {
    store: Arc<Mutex<Store>>,
}

impl UMokaPlugin {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(Store::new(DEFAULT_MAX_CAPACITY))),
        }
    }

    pub(crate) fn store(&self) -> &Arc<Mutex<Store>> {
        &self.store
    }
}

impl Plugin for UMokaPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(UMoka),
            Box::new(Put),
            Box::new(PutIfAbsent),
            Box::new(Get),
            Box::new(GetOrPut),
            Box::new(Take),
            Box::new(Delete),
            Box::new(Has),
            Box::new(Clear),
            Box::new(Incr),
            Box::new(Stats),
        ]
    }
}
