use std::{
    io,
    sync::{
        mpsc::{Sender, SyncSender},
        Arc, RwLock,
    },
};

use sysinfo::{self};

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
        tx: SyncSender<CpuState>,
    ) -> io::Result<()> {
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
                    num_cpus: num_cpus, // should be constructed just once
                };

                tx.send(new).unwrap();
            }

            std::thread::sleep(CPU_UPDATE_INTERVAL);
        }
    }
}
