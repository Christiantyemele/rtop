use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, RwLock}};

use crate::{config::CPU_UPDATE_INTERVAL, utils::SystemState};

#[derive(Default, Clone, Debug)]
pub struct CpuState {
    pub cpu_usage: f32,
    pub brand: String,
    pub name: String,
    pub frequency: u64,
    pub temperature: f32,
    pub num_cpus: usize,
}

impl CpuState {
    pub fn cpu_info(
        &mut self,
        system_state: Arc<RwLock<SystemState>>,
        tx: std::sync::mpsc::Sender<HashMap<String, CpuState>>,
    ) -> Result<(), String> {
        let state = RefCell::new(HashMap::new());
        let mut guard = system_state.write().unwrap();
        let num_cpus = num_cpus::get();
        loop {
            guard.sys.refresh_all();
            for cpu in guard.sys.cpus() {
                let new = CpuState {
                    brand: cpu.brand().to_owned(),
                    cpu_usage: cpu.cpu_usage(),
                    frequency: cpu.frequency(),
                    name: cpu.name().to_owned(),
                    temperature: f32::default(),
                    num_cpus: num_cpus,
                };
                
                // update the state
                state.borrow_mut().insert(cpu.name().to_string(), new);
                tx.send(state.borrow().clone()).map_err(|e| format!("error: {}", e.to_string()))?;
            }

            std::thread::sleep(CPU_UPDATE_INTERVAL);
        }
    }
}
