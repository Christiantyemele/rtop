use sysinfo::System;

#[derive(Default)]
pub struct SystemState {
    pub sys: System,
}

impl SystemState {
    pub fn new() -> Self {
        let sys = System::new_all();
        Self { sys }
    }
}
