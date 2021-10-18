use crate::system::System;
use crate::EngineContextWithoutSystemManager;
use std::collections::BTreeMap;

pub struct SystemManager {
    systems: BTreeMap<isize, Vec<Box<dyn System>>>,
}

impl SystemManager {
    pub fn register_system<S: 'static + System>(&mut self, priority: isize, system: S) {
        self.systems
            .entry(priority)
            .or_default()
            .push(Box::new(system));
    }

    pub fn run(&mut self, context: &EngineContextWithoutSystemManager) {
        for (_, systems) in &mut self.systems {
            for system in systems {
                system.run(context);
            }
        }
    }
}

impl Default for SystemManager {
    fn default() -> Self {
        Self {
            systems: BTreeMap::new(),
        }
    }
}
