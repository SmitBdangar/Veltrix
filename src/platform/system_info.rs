//! System information retrieval (RAM, CPU, OS).

use sysinfo::System;

/// Retrieves real-time hardware and OS metrics.
pub struct SystemInfo {
    sys: System,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo {
    pub fn new() -> Self {
        Self { sys: System::new_all() }
    }

    /// Refresh hardware data (should be called periodically, not every frame).
    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    /// Total system memory in bytes.
    pub fn total_memory(&self) -> u64 {
        self.sys.total_memory()
    }

    /// Currently used system memory in bytes.
    pub fn used_memory(&self) -> u64 {
        self.sys.used_memory()
    }

    /// Number of physical CPU cores.
    pub fn physical_core_count(&self) -> Option<usize> {
        self.sys.physical_core_count()
    }

    /// CPU name/model.
    pub fn cpu_name(&self) -> String {
        self.sys.cpus()[0].brand().to_string()
    }

    /// Host OS name (e.g., Windows, Linux, macOS).
    pub fn os_name(&self) -> Option<String> {
        sysinfo::System::name()
    }

    /// Host OS kernel version.
    pub fn os_version(&self) -> Option<String> {
        sysinfo::System::os_version()
    }
}
