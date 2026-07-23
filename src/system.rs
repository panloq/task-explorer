use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, Networks, Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub global_usage: f32,
    pub brand: String,
    pub frequency_mhz: u64,
    pub physical_core_count: usize,
    pub logical_core_count: usize,
    pub cores: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
}

impl MemoryInfo {
    pub fn used_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f64 / self.total_bytes as f64 * 100.0) as f32
        }
    }

    #[allow(dead_code)]
    pub fn swap_percent(&self) -> f32 {
        if self.swap_total_bytes == 0 {
            0.0
        } else {
            (self.swap_used_bytes as f64 / self.swap_total_bytes as f64 * 100.0) as f32
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub fs_type: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub is_removable: bool,
    pub disk_kind: String,
}

impl DiskInfo {
    pub fn used_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.available_bytes)
    }

    pub fn used_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes() as f64 / self.total_bytes as f64 * 100.0) as f32
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub rx_bytes_sec: u64,
    pub tx_bytes_sec: u64,
    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessItem {
    pub pid: u32,
    pub parent_pid: Option<u32>,
    pub name: String,
    pub exe_path: String,
    pub cmd_line: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub disk_read_bytes_sec: u64,
    pub disk_write_bytes_sec: u64,
    pub status: String,
    pub total_read_bytes: u64,
    pub total_written_bytes: u64,
    /// UNIX timestamp seconds when process started (0 = unknown)
    pub start_time: u64,
    pub cpu_history: Vec<f32>,
}

impl ProcessItem {
    /// Returns age in seconds since process started. 0 if start_time unknown.
    pub fn age_secs(&self) -> u64 {
        if self.start_time == 0 {
            return 0;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now.saturating_sub(self.start_time)
    }

    /// True if this process looks like a zombie (no CPU, no meaningful RAM, been running a while)
    pub fn is_zombie_like(&self) -> bool {
        self.cpu_usage < 0.01 && self.memory_bytes < 512 * 1024 && self.age_secs() > 120
            || self.status.to_lowercase().contains("zombie")
    }
}

#[derive(Debug, Clone)]
pub struct SystemOverview {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub host_name: String,
    pub uptime_secs: u64,
    pub gpu_name: String,
}

#[cfg(target_os = "windows")]
pub fn detect_gpu_name() -> String {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}";
    if let Ok(class_key) = hklm.open_subkey(class_path) {
        let mut gpus = Vec::new();
        for subkey_name in class_key.enum_keys().flatten() {
            if let Ok(gpu_key) = class_key.open_subkey(&subkey_name) {
                if let Ok(driver_desc) = gpu_key.get_value::<String, _>("DriverDesc") {
                    let desc = driver_desc.trim().to_string();
                    if !desc.is_empty() && !gpus.contains(&desc) {
                        gpus.push(desc);
                    }
                }
            }
        }
        if !gpus.is_empty() {
            return gpus.join(" / ");
        }
    }
    "Unknown GPU".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn detect_gpu_name() -> String {
    "Unknown GPU".to_string()
}

pub struct SystemMonitor {
    sys: System,
    disks: Disks,
    networks: Networks,
    last_update: Instant,
    prev_net_rx: HashMap<String, u64>,
    prev_net_tx: HashMap<String, u64>,
    proc_cpu_history: HashMap<u32, std::collections::VecDeque<f32>>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let refresh_kind = RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(ProcessRefreshKind::nothing()
                .with_cpu()
                .with_memory()
                .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
                .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet)
                .with_disk_usage()
            );

        let mut sys = System::new_with_specifics(refresh_kind);
        sys.refresh_all();

        Self {
            sys,
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            last_update: Instant::now(),
            prev_net_rx: HashMap::new(),
            prev_net_tx: HashMap::new(),
            proc_cpu_history: HashMap::new(),
        }
    }

    pub fn refresh_fast(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
    }

    pub fn refresh_processes_only(&mut self) {
        self.sys.refresh_processes(ProcessesToUpdate::All, true);
    }

    pub fn refresh_slow(&mut self) {
        let now = Instant::now();
        let elapsed_secs = now.duration_since(self.last_update).as_secs_f64();
        let _dt = if elapsed_secs > 0.001 { elapsed_secs } else { 1.0 };
        self.last_update = now;

        self.sys.refresh_processes(ProcessesToUpdate::All, true);
        self.disks.refresh(false);
        self.networks.refresh(false);

        for (interface_name, net) in self.networks.iter() {
            let rx = net.total_received();
            let tx = net.total_transmitted();
            self.prev_net_rx.insert(interface_name.clone(), rx);
            self.prev_net_tx.insert(interface_name.clone(), tx);
        }
    }

    pub fn cpu_info(&self) -> CpuInfo {
        let cpus = self.sys.cpus();
        let brand = cpus.first().map(|c| c.brand().to_string()).unwrap_or_else(|| "Unknown CPU".to_string());
        let freq = cpus.first().map(|c| c.frequency()).unwrap_or(0);
        let cores = cpus.iter().map(|c| c.cpu_usage().min(100.0)).collect();

        CpuInfo {
            global_usage: self.sys.global_cpu_usage().min(100.0),
            brand,
            frequency_mhz: freq,
            physical_core_count: self.sys.physical_core_count().unwrap_or(cpus.len()),
            logical_core_count: cpus.len(),
            cores,
        }
    }

    pub fn memory_info(&self) -> MemoryInfo {
        MemoryInfo {
            used_bytes: self.sys.used_memory(),
            total_bytes: self.sys.total_memory(),
            swap_used_bytes: self.sys.used_swap(),
            swap_total_bytes: self.sys.total_swap(),
        }
    }

    pub fn disk_info_list(&self) -> Vec<DiskInfo> {
        self.disks
            .list()
            .iter()
            .map(|disk| {
                let kind_str = match disk.kind() {
                    sysinfo::DiskKind::SSD => "SSD".to_string(),
                    sysinfo::DiskKind::HDD => "HDD".to_string(),
                    _ => "Disk".to_string(),
                };
                DiskInfo {
                    name: disk.name().to_string_lossy().into_owned(),
                    mount_point: disk.mount_point().to_string_lossy().into_owned(),
                    fs_type: disk.file_system().to_string_lossy().into_owned(),
                    total_bytes: disk.total_space(),
                    available_bytes: disk.available_space(),
                    is_removable: disk.is_removable(),
                    disk_kind: kind_str,
                }
            })
            .collect()
    }

    pub fn network_info_list(&self) -> Vec<NetworkInterfaceInfo> {
        self.networks
            .iter()
            .map(|(name, net)| NetworkInterfaceInfo {
                name: name.clone(),
                rx_bytes_sec: net.received(),
                tx_bytes_sec: net.transmitted(),
                total_rx_bytes: net.total_received(),
                total_tx_bytes: net.total_transmitted(),
            })
            .collect()
    }

    pub fn process_list(&mut self) -> Vec<ProcessItem> {
        const SPARKLINE_LEN: usize = 20;
        let num_cpus = self.sys.cpus().len().max(1) as f32;
        
        let mut new_history: HashMap<u32, std::collections::VecDeque<f32>> = HashMap::new();
        
        let mut results = Vec::new();

        for (&pid, process) in self.sys.processes() {
            let pid_u32 = pid.as_u32();
            let disk_usage = process.disk_usage();
            let exe_path = process
                .exe()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default();
            
            let cmd_line = if !process.cmd().is_empty() {
                process.cmd().join(std::ffi::OsStr::new(" ")).to_string_lossy().into_owned()
            } else {
                String::new()
            };

            let parent_pid = process.parent().map(|p| p.as_u32());

            // sysinfo reports cpu_usage per-core (e.g. 800% on 8 cores).
            // Normalize to 0-100% range by dividing by number of logical CPUs.
            let raw_cpu = process.cpu_usage();
            let normalized_cpu = (raw_cpu / num_cpus).min(100.0);

            // Update sparkline history
            let mut history = self.proc_cpu_history.remove(&pid_u32).unwrap_or_default();
            history.push_back(normalized_cpu);
            if history.len() > SPARKLINE_LEN {
                history.pop_front();
            }
            let cpu_history_vec: Vec<f32> = history.iter().cloned().collect();
            new_history.insert(pid_u32, history);

            results.push(ProcessItem {
                pid: pid_u32,
                parent_pid,
                name: process.name().to_string_lossy().into_owned(),
                exe_path,
                cmd_line,
                cpu_usage: normalized_cpu,
                memory_bytes: process.memory(),
                disk_read_bytes_sec: disk_usage.read_bytes,
                disk_write_bytes_sec: disk_usage.written_bytes,
                status: format!("{:?}", process.status()),
                total_read_bytes: disk_usage.total_read_bytes,
                total_written_bytes: disk_usage.total_written_bytes,
                start_time: process.start_time(),
                cpu_history: cpu_history_vec,
            });
        }
        
        self.proc_cpu_history = new_history;
        results
    }

    pub fn system_overview(&self) -> SystemOverview {
        SystemOverview {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            host_name: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            uptime_secs: System::uptime(),
            gpu_name: detect_gpu_name(),
        }
    }

    pub fn kill_process(&mut self, pid_u32: u32) -> Result<(), String> {
        let sysinfo_pid = Pid::from_u32(pid_u32);
        if let Some(process) = self.sys.process(sysinfo_pid) {
            if process.kill() {
                return Ok(());
            }
        }

        #[cfg(target_os = "windows")]
        {
            let status = std::process::Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    &format!("Start-Process taskkill -ArgumentList '/F /PID {}' -Verb RunAs -WindowStyle Hidden", pid_u32),
                ])
                .status();

            if let Ok(s) = status {
                if s.success() {
                    return Ok(());
                }
            }
        }

        Err(format!("Failed to terminate process (PID: {})", pid_u32))
    }

    pub fn kill_process_tree(&mut self, root_pid: u32) -> Result<usize, String> {
        let mut pids_to_kill = Vec::new();
        let mut stack = vec![root_pid];

        while let Some(parent) = stack.pop() {
            pids_to_kill.push(parent);
            for (pid, process) in self.sys.processes() {
                if let Some(ppid) = process.parent() {
                    if ppid.as_u32() == parent {
                        stack.push(pid.as_u32());
                    }
                }
            }
        }

        let total_nodes = pids_to_kill.len();
        let mut killed_count = 0;
        for pid in &pids_to_kill {
            if self.kill_process(*pid).is_ok() {
                killed_count += 1;
            }
        }

        if killed_count == total_nodes && killed_count > 0 {
            return Ok(killed_count);
        }

        #[cfg(target_os = "windows")]
        {
            let status = std::process::Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    &format!("Start-Process taskkill -ArgumentList '/F /T /PID {}' -Verb RunAs -WindowStyle Hidden", root_pid),
                ])
                .status();

            if let Ok(s) = status {
                if s.success() {
                    return Ok(total_nodes);
                }
            }
        }

        if killed_count > 0 {
            Ok(killed_count)
        } else {
            Err(format!("Failed to kill process tree (PID: {})", root_pid))
        }
    }



    pub fn boost_mode_suspend_hogs(&mut self) -> Result<Vec<u32>, String> {
        const HOG_APPS: &[&str] = &[
            "chrome.exe", "msedge.exe", "firefox.exe", "brave.exe", "opera.exe",
            "discord.exe", "slack.exe", "teams.exe", "skype.exe", "spotify.exe",
            "onedrive.exe", "googledrivefs.exe", "dropbox.exe", "steamwebhelper.exe",
            "epicgameslauncher.exe", "battlenet.exe"
        ];
        
        let mut suspended_pids = Vec::new();
        // Zbieramy PIDy znanych pożeraczy zasobów
        let pids_to_suspend: Vec<u32> = self.sys.processes().iter()
            .filter_map(|(&pid, process)| {
                let name = process.name().to_string_lossy().to_lowercase();
                if HOG_APPS.contains(&name.as_str()) {
                    Some(pid.as_u32())
                } else {
                    None
                }
            }).collect();
            
        for pid in pids_to_suspend {
            if self.suspend_process(pid).is_ok() {
                suspended_pids.push(pid);
            }
        }
        
        Ok(suspended_pids)
    }

    pub fn suspend_process(&mut self, pid: u32) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            type HANDLE = *mut std::ffi::c_void;
            const PROCESS_SUSPEND_RESUME: u32 = 0x0800;

            unsafe extern "system" {
                fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> HANDLE;
                fn CloseHandle(hObject: HANDLE) -> i32;
                fn GetModuleHandleA(lpModuleName: *const u8) -> HANDLE;
                fn GetProcAddress(hModule: HANDLE, lpProcName: *const u8) -> *const std::ffi::c_void;
            }

            unsafe {
                let handle = OpenProcess(PROCESS_SUSPEND_RESUME, 0, pid);
                if handle.is_null() {
                    return Err(format!("Failed to open process PID {}", pid));
                }
                let h_ntdll = GetModuleHandleA(b"ntdll.dll\0".as_ptr());
                if h_ntdll.is_null() {
                    CloseHandle(handle);
                    return Err("Failed to load ntdll.dll".to_string());
                }
                let proc_addr = GetProcAddress(h_ntdll, b"NtSuspendProcess\0".as_ptr());
                if proc_addr.is_null() {
                    CloseHandle(handle);
                    return Err("NtSuspendProcess not found".to_string());
                }
                let func: extern "system" fn(HANDLE) -> i32 = std::mem::transmute(proc_addr);
                let res = func(handle);
                CloseHandle(handle);
                if res == 0 {
                    Ok(())
                } else {
                    Err(format!("NtSuspendProcess returned error 0x{:X}", res))
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let status = std::process::Command::new("kill")
                .arg("-STOP")
                .arg(pid.to_string())
                .status();
            match status {
                Ok(s) if s.success() => Ok(()),
                _ => Err(format!("Failed to suspend process PID {}", pid)),
            }
        }
    }

    pub fn resume_process(&mut self, pid: u32) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            type HANDLE = *mut std::ffi::c_void;
            const PROCESS_SUSPEND_RESUME: u32 = 0x0800;

            unsafe extern "system" {
                fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> HANDLE;
                fn CloseHandle(hObject: HANDLE) -> i32;
                fn GetModuleHandleA(lpModuleName: *const u8) -> HANDLE;
                fn GetProcAddress(hModule: HANDLE, lpProcName: *const u8) -> *const std::ffi::c_void;
            }

            unsafe {
                let handle = OpenProcess(PROCESS_SUSPEND_RESUME, 0, pid);
                if handle.is_null() {
                    return Err(format!("Failed to open process PID {}", pid));
                }
                let h_ntdll = GetModuleHandleA(b"ntdll.dll\0".as_ptr());
                if h_ntdll.is_null() {
                    CloseHandle(handle);
                    return Err("Failed to load ntdll.dll".to_string());
                }
                let proc_addr = GetProcAddress(h_ntdll, b"NtResumeProcess\0".as_ptr());
                if proc_addr.is_null() {
                    CloseHandle(handle);
                    return Err("NtResumeProcess not found".to_string());
                }
                let func: extern "system" fn(HANDLE) -> i32 = std::mem::transmute(proc_addr);
                let res = func(handle);
                CloseHandle(handle);
                if res == 0 {
                    Ok(())
                } else {
                    Err(format!("NtResumeProcess returned error 0x{:X}", res))
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let status = std::process::Command::new("kill")
                .arg("-CONT")
                .arg(pid.to_string())
                .status();
            match status {
                Ok(s) if s.success() => Ok(()),
                _ => Err(format!("Failed to resume process PID {}", pid)),
            }
        }
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_rate(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}

pub fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, mins, secs)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}
