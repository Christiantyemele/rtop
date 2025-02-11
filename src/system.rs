use std::{
    io,
    sync::{Arc, RwLock},
};

use sysinfo::{self};

use crate::{config::CPU_UPDATE_INTERVAL, utils::SystemState};

#[derive(Default)]
pub struct CpuState {
    pub cpu_usage: f32,
    pub brand: String,
    pub name: String,
    pub frequency: u64,
    pub temperature: f32,
}

impl CpuState {
    pub fn cpu_info(&mut self, system_state: Arc<RwLock<SystemState>>) -> io::Result<()> {
        loop {
            let mut system = system_state.write().unwrap(); // Single write lock

            system.sys.refresh_cpu_all();
            for cpu in system.sys.cpus() {
                self.cpu_usage = cpu.cpu_usage();

                if self.brand.is_empty() {
                    self.brand = cpu.brand().to_owned();
                    self.name = cpu.name().to_owned();
                }

                self.frequency = cpu.frequency();
            }

            std::thread::sleep(CPU_UPDATE_INTERVAL);
        }
    }
}
