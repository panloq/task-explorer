use crate::config::Language;
use serde::{Deserialize, Serialize};

/// All user-visible strings, available in EN and PL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lang {
    // Sidebar
    pub tab_overview: String,
    pub tab_processes: String,
    pub tab_disks: String,
    pub tab_network: String,
    pub tab_system_info: String,
    pub tab_settings: String,

    // Overview
    pub overview_title: String,
    pub overview_cpu: String,
    pub overview_ram: String,
    pub overview_disk: String,
    pub overview_network: String,
    pub overview_cores: String,
    pub overview_cpu_chart: String,
    pub overview_ram_chart: String,
    pub overview_now: String,
    pub overview_partitions: String,
    pub overview_interfaces: String,

    // RAM Forecast
    pub forecast_stable: String,
    pub forecast_full_in: String,
    pub forecast_minutes: String,
    pub forecast_hours: String,
    pub forecast_title: String,

    // Memory Leak
    pub leak_title: String,
    pub leak_growing: String,
    pub leak_badge: String,

    // Processes
    pub proc_title: String,
    pub proc_subtab_processes: String,
    pub proc_subtab_autostart: String,
    pub proc_subtab_details: String,
    pub startup_title: String,
    pub startup_col_name: String,
    pub startup_col_cmd: String,
    pub startup_col_location: String,
    pub startup_col_status: String,
    pub startup_enabled: String,
    pub startup_disabled: String,
    pub startup_open_location: String,
    pub proc_search_placeholder: String,
    pub proc_count: String,
    pub proc_sort_asc: String,
    pub proc_sort_desc: String,
    pub proc_col_pid: String,
    pub proc_col_name: String,
    pub proc_col_cpu: String,
    pub proc_col_ram: String,
    pub proc_col_disk: String,
    pub proc_col_status: String,
    pub proc_col_age: String,
    pub proc_kill: String,
    pub proc_info: String,
    pub proc_search_online: String,
    pub proc_select_hint: String,
    pub proc_snapshot_take: String,
    pub proc_snapshot_diff: String,
    pub proc_snapshot_clear: String,
    pub proc_snapshot_active: String,
    pub proc_diff_new: String,
    pub proc_diff_gone: String,
    pub proc_kill_tree: String,
    pub proc_suspend: String,
    pub proc_resume: String,
    pub proc_suspended_badge: String,
    pub info_children: String,

    // Zombie
    pub zombie_tooltip: String,

    // Process Info (modal)
    pub info_title: String,
    pub info_pid: String,
    pub info_name: String,
    pub info_status: String,
    pub info_age: String,
    pub info_path: String,
    pub info_parent: String,
    pub info_cmd: String,
    pub info_cpu: String,
    pub info_ram: String,
    pub info_disk_read: String,
    pub info_disk_write: String,
    pub info_total_read: String,
    pub info_total_written: String,
    pub info_close: String,
    pub info_kill: String,

    // Settings
    pub settings_title: String,
    pub settings_language: String,
    pub settings_lang_en: String,
    pub settings_lang_pl: String,
    pub settings_lang_ru: String,
    pub settings_saved: String,
    pub settings_refresh_rate: String,

    // System Info
    pub sysinfo_title: String,
    pub sysinfo_os: String,
    pub sysinfo_kernel: String,
    pub sysinfo_hostname: String,
    pub sysinfo_uptime: String,
    pub sysinfo_cpu_model: String,
    pub sysinfo_gpu_model: String,
    pub sysinfo_cpu_cores: String,
    pub sysinfo_ram_total: String,

    // Disks
    pub disk_title: String,
    pub disk_fs: String,
    pub disk_total: String,
    pub disk_used: String,
    pub disk_free: String,
    pub disk_removable: String,
    pub disk_read_rate: String,
    pub disk_write_rate: String,
    pub disk_wear_guard: String,
    pub disk_header: String,
    pub disk_label: String,
    pub disk_type: String,
    pub disk_fixed: String,
    pub disk_fill: String,
    pub disk_io_activity: String,
    pub disk_active_procs: String,
    pub disk_no_io: String,
    pub disk_high_write: String,
    pub disk_no_wear_data: String,
    pub disk_page_title: String,

    // Network
    pub net_title: String,
    pub net_download: String,
    pub net_upload: String,
    pub net_total_rx: String,
    pub net_total_tx: String,
    pub net_interface: String,
    pub net_total: String,

    // System Info
    pub sysinfo_sys_id: String,
    pub sysinfo_hw_spec: String,
    pub sysinfo_page_title: String,
    pub sysinfo_phys: String,
    pub sysinfo_logical: String,
    pub sysinfo_base_freq: String,
    pub sysinfo_ram_installed: String,

    // General
    pub unknown: String,
    pub none: String,
    pub auto_refresh_label: String,
}

pub fn get_lang(language: Language) -> Lang {
    let data = match language {
        Language::English => include_str!("../locales/en.json"),
        Language::Polish => include_str!("../locales/pl.json"),
        Language::Russian => include_str!("../locales/ru.json"),
    };
    serde_json::from_str(data).expect("Failed to parse language locale JSON file")
}
