use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub fn get_ram_info() -> (u64, u64) {
    let refresh_kind = RefreshKind::everything().with_memory(MemoryRefreshKind::everything());

    let mut sys = System::new_with_specifics(refresh_kind);

    sys.refresh_memory();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    (total_memory, used_memory)
}
