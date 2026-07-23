use iced::widget::{button, container, row, stack, column, text, image, Space};

use iced::{Task, Subscription, Theme, Length, Element, Alignment, Color};
use iced::keyboard::{self, key::Named, Key};
use std::collections::{HashMap, HashSet};

use std::time::Duration;

use crate::components::sidebar::render_sidebar;
use crate::config::{AppConfig, Language};
use crate::lang::{get_lang, Lang};
use crate::system::{
    CpuInfo, DiskInfo, MemoryInfo, NetworkInterfaceInfo, ProcessItem, SystemMonitor, SystemOverview,
};
use crate::theme::ThemeColors;
use crate::views::disks::render_disks_view;
use crate::views::network::render_network_view;
use crate::views::overview::render_overview_view;
use crate::views::cpu::render_cpu_view; 
use crate::views::ram::render_ram_view;
use crate::views::process_info::render_process_info_modal;
use crate::views::processes::render_processes_view;
use crate::views::startup::render_startup_view;
use crate::views::details::render_details_view;
use crate::startup::{fetch_startup_items, StartupItem};
use crate::views::settings::render_settings_view;
use crate::views::system_info::render_system_info_view;

pub const HISTORY_LEN: usize = 60;
/// Ticks before flagging a memory leak (process mem growing for N consecutive ticks)
const LEAK_TICKS: usize = 15;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview,
    Cpu,
    Ram,
    Processes,
    Disks,
    Network,
    SystemInfo,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessesSubTab {
    Processes,
    Startup,
    Details,
    Anomalies,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Pid,
    Name,
    Cpu,
    Memory,
    Disk,
    Age,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Existing messages
    TickFast,
    TickSlow,
    FastDataUpdated(CpuInfo, MemoryInfo),
    SlowDataUpdated(Vec<ProcessItem>, Vec<DiskInfo>, Vec<NetworkInterfaceInfo>, SystemOverview),
    RefreshProcesses(Vec<ProcessItem>),
    TabSelected(Tab),
    SearchQueryChanged(String),
    SortBy(SortKey),
    ToggleSortOrder,
    SelectProcess(Option<u32>),
    SelectNextProcess,
    SelectPreviousProcess,
    ToggleProcessesExpanded,
    ProcessesSubTabSelected(ProcessesSubTab),
    SelectStartupItem(Option<usize>),
    OpenStartupLocation(String),
    RemoveStartupItem(usize),
    KillProcess(u32),
    KillProcessTree(u32),
    SuspendProcess(u32),
    ResumeProcess(u32),
    SearchProcessOnline(String),
    // Process Info modal
    OpenProcessInfo(u32),
    CloseProcessInfo,
    // Snapshot / Diff
    TakeSnapshot,
    ClearSnapshot,
    ToggleDiffMode,
    ToggleBoostMode,
    // Settings
    SetLanguage(Language),
    SetRefreshRate(f32),
    // Animation tick for loading screen
    AnimationTick,
    // Immediate process reload
    LoadProcessesNow,
}

// Thread-safe wrapper to share SystemMonitor across background threads
use std::sync::{Arc, Mutex};

pub struct TaskExplorerApp {
    current_tab: Tab,
    monitor: Arc<Mutex<SystemMonitor>>,
    config: AppConfig,
    lang: Lang,

    search_query: String,
    sort_key: SortKey,
    pub sort_ascending: bool,
    selected_pid: Option<u32>,
    status_message: Option<String>,
    show_process_info: bool,

    cpu: CpuInfo,
    memory: MemoryInfo,
    disks: Vec<DiskInfo>,
    networks: Vec<NetworkInterfaceInfo>,
    processes: Vec<ProcessItem>,
    filtered_processes: Vec<ProcessItem>,
    system_overview: SystemOverview,

    pub cpu_history: Vec<f32>,
    pub memory_history: Vec<f32>,

    // Per-process memory tracking for leak detection: pid -> last N memory values
    proc_mem_history: HashMap<u32, Vec<u64>>,
    /// PIDs currently flagged as memory leaks
    leak_pids: HashSet<u32>,

    // Snapshot / Diff
    snapshot_pids: HashSet<u32>,
    snapshot_taken: bool,
    diff_mode: bool,

    suspended_pids: HashSet<u32>,
    boost_active: bool,
    boost_suspended_pids: HashSet<u32>,

    processes_subtab: ProcessesSubTab,
    processes_expanded: bool,
    startup_items: Vec<StartupItem>,
    selected_startup_index: Option<usize>,

    // UI loading indicator
    loading: bool,
    loading_start_time: std::time::Instant,
}

impl TaskExplorerApp {
    pub fn new() -> (Self, Task<Message>) {
        let config = AppConfig::load();
        let lang = get_lang(config.language);

        let monitor = SystemMonitor::new();
        let monitor_arc = Arc::new(Mutex::new(monitor));

        let app = Self {
            current_tab: Tab::Overview,
            monitor: monitor_arc,
            config,
            lang,

            search_query: String::new(),
            sort_key: SortKey::Cpu,
            sort_ascending: false,
            selected_pid: None,
            status_message: None,
            show_process_info: false,

            cpu: CpuInfo {
                global_usage: 0.0,
                brand: String::new(),
                frequency_mhz: 0,
                physical_core_count: 0,
                logical_core_count: 0,
                cores: Vec::new(),
            },
            memory: MemoryInfo {
                used_bytes: 0,
                total_bytes: 0,
                swap_used_bytes: 0,
                swap_total_bytes: 0,
            },
            disks: Vec::new(),
            networks: Vec::new(),
            processes: Vec::new(),
            filtered_processes: Vec::new(),
            system_overview: SystemOverview {
                os_name: String::new(),
                os_version: String::new(),
                kernel_version: String::new(),
                host_name: String::new(),
                uptime_secs: 0,
                gpu_name: String::new(),
            },

            cpu_history: vec![0.0; HISTORY_LEN],
            memory_history: vec![0.0; HISTORY_LEN],

            proc_mem_history: HashMap::new(),
            leak_pids: HashSet::new(),

            snapshot_pids: HashSet::new(),
            snapshot_taken: false,
            diff_mode: false,

            suspended_pids: HashSet::new(),
            boost_active: false,
            boost_suspended_pids: HashSet::new(),

            processes_subtab: ProcessesSubTab::Processes,
            processes_expanded: true,
            startup_items: fetch_startup_items(),
            selected_startup_index: None,

            loading: true,
            loading_start_time: std::time::Instant::now(),
        };

        let monitor_clone = Arc::clone(&app.monitor);
        let task = Task::perform(
            async move {
                // First pass to establish baseline measurements
                {
                    let mut mon = monitor_clone.lock().unwrap();
                    mon.refresh_fast();
                    mon.refresh_slow();
                }
                
                // Sleep for 5 seconds to show the loading screen and let CPU usage readings settle down
                tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
                
                // Second pass to compute actual rates and correct CPU percentages
                let mut mon = monitor_clone.lock().unwrap();
                mon.refresh_fast();
                mon.refresh_slow();
                (
                    mon.process_list(),
                    mon.disk_info_list(),
                    mon.network_info_list(),
                    mon.system_overview(),
                )
            },
            |(processes, disks, networks, overview)| {
                Message::SlowDataUpdated(processes, disks, networks, overview)
            }
        );

        (app, task)
    }

    pub fn title(&self) -> String {
        "TaskExplorer".to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TickFast => {
                let monitor = Arc::clone(&self.monitor);
                return Task::perform(
                    async move {
                        let mut mon = monitor.lock().unwrap();
                        mon.refresh_fast();
                        (mon.cpu_info(), mon.memory_info())
                    },
                    |(cpu, mem)| Message::FastDataUpdated(cpu, mem),
                );
            },
            
            Message::AnimationTick => {
                // Ticking state forces iced to redraw and query view()
                Task::none()
            }
            Message::FastDataUpdated(cpu, memory) => {
                self.cpu = cpu;
                self.memory = memory;

                // Rolling history
                if self.cpu_history.len() >= HISTORY_LEN { self.cpu_history.remove(0); }
                self.cpu_history.push(self.cpu.global_usage);
                if self.memory_history.len() >= HISTORY_LEN { self.memory_history.remove(0); }
                self.memory_history.push(self.memory.used_percent());
                Task::none()
            },

            


            Message::SlowDataUpdated(processes, disks, networks, overview) => {
                self.processes = processes;
                self.disks = disks;
                self.networks = networks;
                self.system_overview = overview;
                self.loading = false;

                // Update per-process memory history & detect leaks
                self.update_leak_detection();
                self.filter_and_sort_processes();
                Task::none()
            }
            Message::TabSelected(tab) => {
                self.current_tab = tab;
                self.show_process_info = false;
                if tab == Tab::Processes {
                    self.processes_expanded = true;
                    return self.update(Message::LoadProcessesNow);
                }
                Task::none()
            },
            Message::ToggleProcessesExpanded => {
                self.processes_expanded = !self.processes_expanded;
                self.current_tab = Tab::Processes;
                self.show_process_info = false;
                Task::none()
            },
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                self.filter_and_sort_processes();
                Task::none()
            }
            Message::SortBy(key) => {
                if self.sort_key == key {
                    self.sort_ascending = !self.sort_ascending;
                } else {
                    self.sort_key = key;
                    self.sort_ascending = false;
                }
                self.filter_and_sort_processes();
                Task::none()
            }
            Message::ToggleSortOrder => {
                self.sort_ascending = !self.sort_ascending;
                self.filter_and_sort_processes();
                Task::none()
            }
            Message::SelectProcess(pid) => {
                // Instantly update selected PID and avoid any artificial delays
                self.selected_pid = pid;
                self.status_message = None;
                self.show_process_info = false;
                self.loading = false;
                Task::none()
            },
            Message::SelectNextProcess => {
                if self.filtered_processes.is_empty() {
                    return Task::none();
                }
                let next_pid = match self.selected_pid {
                    None => self.filtered_processes.first().map(|p| p.pid),
                    Some(current_pid) => {
                        if let Some(idx) = self.filtered_processes.iter().position(|p| p.pid == current_pid) {
                            let next_idx = (idx + 1).min(self.filtered_processes.len() - 1);
                            Some(self.filtered_processes[next_idx].pid)
                        } else {
                            self.filtered_processes.first().map(|p| p.pid)
                        }
                    }
                };
                self.selected_pid = next_pid;
                Task::none()
            },
            Message::SelectPreviousProcess => {
                if self.filtered_processes.is_empty() {
                    return Task::none();
                }
                let prev_pid = match self.selected_pid {
                    None => self.filtered_processes.last().map(|p| p.pid),
                    Some(current_pid) => {
                        if let Some(idx) = self.filtered_processes.iter().position(|p| p.pid == current_pid) {
                            let prev_idx = idx.saturating_sub(1);
                            Some(self.filtered_processes[prev_idx].pid)
                        } else {
                            self.filtered_processes.last().map(|p| p.pid)
                        }
                    }
                };
                self.selected_pid = prev_pid;
                Task::none()
            },
            Message::ProcessesSubTabSelected(subtab) => {
                self.current_tab = Tab::Processes;
                self.processes_subtab = subtab;
                self.show_process_info = false;
                if subtab == ProcessesSubTab::Startup {
                    self.startup_items = fetch_startup_items();
                }
                Task::none()
            },
            Message::SelectStartupItem(idx) => {
                self.selected_startup_index = idx;
                Task::none()
            },
            Message::OpenStartupLocation(cmd) => {
                crate::startup::open_file_location(&cmd);
                Task::none()
            },
            Message::RemoveStartupItem(idx) => {
                if let Some(item) = self.startup_items.get(idx).cloned() {
                    match crate::startup::remove_startup_item(&item) {
                        Ok(_) => {
                            self.status_message = Some(format!("Removed '{}' from startup.", item.name));
                            self.startup_items = crate::startup::fetch_startup_items();
                            self.selected_startup_index = None;
                        }
                        Err(err) => {
                            self.status_message = Some(err);
                        }
                    }
                }
                Task::none()
            },

            Message::KillProcess(pid) => {
                self.show_process_info = false;
                let monitor = Arc::clone(&self.monitor);
                let kill_res = monitor.lock().unwrap().kill_process(pid);
                match kill_res {
                    Ok(_) => {
                        self.status_message = Some(format!("PID {} terminated.", pid));
                        self.selected_pid = None;
                        // Refresh process list immediately without artificial delay
                        let monitor_clone = Arc::clone(&self.monitor);
                        return Task::perform(
                            async move {
                                let mut mon = monitor_clone.lock().unwrap();
                                mon.refresh_slow();
                                mon.process_list()
                            },
                            |procs| Message::SlowDataUpdated(
                                procs,
                                vec![],
                                vec![],
                                SystemOverview {
                                    os_name: String::new(),
                                    os_version: String::new(),
                                    kernel_version: String::new(),
                                    host_name: String::new(),
                                    uptime_secs: 0,
                                    gpu_name: String::new(),
                                }
                            )
                        );
                    }
                    Err(err) => { self.status_message = Some(err); }
                }
                self.loading = false;
                Task::none()
            }
            Message::KillProcessTree(pid) => {
                self.show_process_info = false;
                let monitor = Arc::clone(&self.monitor);
                let kill_res = monitor.lock().unwrap().kill_process_tree(pid);
                match kill_res {
                    Ok(count) => {
                        self.status_message = Some(format!("Terminated process tree for PID {} ({} processes).", pid, count));
                        self.selected_pid = None;
                        let monitor_clone = Arc::clone(&self.monitor);
                        return Task::perform(
                            async move {
                                let mut mon = monitor_clone.lock().unwrap();
                                mon.refresh_slow();
                                mon.process_list()
                            },
                            |procs| Message::SlowDataUpdated(
                                procs,
                                vec![],
                                vec![],
                                SystemOverview {
                                    os_name: String::new(),
                                    os_version: String::new(),
                                    kernel_version: String::new(),
                                    host_name: String::new(),
                                    uptime_secs: 0,
                                    gpu_name: String::new(),
                                }
                            )
                        );
                    }
                    Err(err) => { self.status_message = Some(err); }
                }
                self.loading = false;
                Task::none()
            }
            Message::SuspendProcess(pid) => {
                let monitor = Arc::clone(&self.monitor);
                let res = monitor.lock().unwrap().suspend_process(pid);
                match res {
                    Ok(_) => {
                        self.suspended_pids.insert(pid);
                        self.status_message = Some(format!("Process PID {} suspended.", pid));
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }
                Task::none()
            }
            Message::ResumeProcess(pid) => {
                let monitor = Arc::clone(&self.monitor);
                let res = monitor.lock().unwrap().resume_process(pid);
                match res {
                    Ok(_) => {
                        self.suspended_pids.remove(&pid);
                        self.status_message = Some(format!("Process PID {} resumed.", pid));
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }
                Task::none()
            }

            Message::SearchProcessOnline(name) => {
                let url = format!("https://www.google.com/search?q={}+process+security+info", name);
                let _ = std::process::Command::new("cmd").args(["/C", "start", &url]).spawn();
                Task::none()
            }
            Message::OpenProcessInfo(pid) => {
                self.selected_pid = Some(pid);
                self.show_process_info = true;
                Task::none()
            }
            Message::CloseProcessInfo => {
                self.show_process_info = false;
                Task::none()
            }
            Message::TakeSnapshot => {
                self.snapshot_pids = self.filtered_processes.iter().map(|p| p.pid).collect();
                self.snapshot_taken = true;
                self.diff_mode = false;
                Task::none()
            }
            Message::ClearSnapshot => {
                self.snapshot_pids.clear();
                self.snapshot_taken = false;
                self.diff_mode = false;
                Task::none()
            }
            Message::ToggleDiffMode => {
                self.diff_mode = !self.diff_mode;
                Task::none()
            }
            Message::ToggleBoostMode => {
                if self.boost_active {
                    // Restore suspended pids
                    self.boost_active = false;
                    let pids_to_resume: Vec<u32> = self.boost_suspended_pids.iter().cloned().collect();
                    let monitor = Arc::clone(&self.monitor);
                    let mut resumed_count = 0;
                    {
                        let mut mon = monitor.lock().unwrap();
                        for pid in pids_to_resume {
                            if mon.resume_process(pid).is_ok() {
                                resumed_count += 1;
                            }
                        }
                    }
                    self.suspended_pids.retain(|pid| !self.boost_suspended_pids.contains(pid));
                    self.boost_suspended_pids.clear();
                    self.status_message = Some(format!("Boost Mode disabled. Resumed {} apps.", resumed_count));
                } else {
                    // Activate boost mode
                    let monitor = Arc::clone(&self.monitor);
                    let mut mon = monitor.lock().unwrap();
                    match mon.boost_mode_suspend_hogs() {
                        Ok(suspended) => {
                            self.boost_active = true;
                            self.boost_suspended_pids = suspended.iter().cloned().collect();
                            for pid in suspended {
                                self.suspended_pids.insert(pid);
                            }
                            self.status_message = Some(format!("Boost Mode enabled. Suspended {} background apps.", self.boost_suspended_pids.len()));
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Failed to activate Boost Mode: {}", e));
                        }
                    }
                }
                // Force UI update
                return self.update(Message::TickSlow);
            }
            Message::RefreshProcesses(processes) => {
                if processes.is_empty() {
                    let monitor = Arc::clone(&self.monitor);
                    return Task::perform(
                        async move {
                            let mut mon = monitor.lock().unwrap();
                            mon.refresh_processes_only();
                            mon.process_list()
                        },
                        |procs| Message::RefreshProcesses(procs),
                    );
                }
                self.processes = processes;
                self.loading = false;
                // Re‑apply filtering and sorting after refresh
                self.filter_and_sort_processes();
                Task::none()
            },
            // Handle slow tick for disks, network, overview
            Message::TickSlow => {
                let monitor = Arc::clone(&self.monitor);
                return Task::perform(
                    async move {
                        let mut mon = monitor.lock().unwrap();
                        mon.refresh_slow();
                        (
                            mon.process_list(),
                            mon.disk_info_list(),
                            mon.network_info_list(),
                            mon.system_overview(),
                        )
                    },
                    |(processes, disks, networks, overview)| {
                        Message::SlowDataUpdated(processes, disks, networks, overview)
                    },
                );
            },
            Message::SetLanguage(lang) => {
                self.lang = get_lang(lang);
                self.config.language = lang;
                self.config.save();
                Task::none()
            },
            Message::SetRefreshRate(rate) => {
                self.config.refresh_rate = rate;
                self.config.save();
                Task::none()
            },
            Message::LoadProcessesNow => {
                // Trigger immediate data refresh (processes, disks, network, overview)
                return self.update(Message::TickSlow);
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![];
        if self.loading && self.processes.is_empty() {
            // High frequency tick (e.g. 30fps = 33ms) for smooth progress animation during loading screen
            subs.push(iced::time::every(Duration::from_millis(33)).map(|_| Message::AnimationTick));
        } else {
            let refresh_millis = (self.config.refresh_rate * 1000.0) as u64;
            // Fast updates for CPU/memory (always needed) - slower frequency to save CPU/GPU resources
            subs.push(iced::time::every(Duration::from_millis(refresh_millis)).map(|_| Message::TickFast));
            // Process refresh when on Processes tab - slower frequency to avoid flickering/jumping items too often
            let proc_refresh = if self.current_tab == Tab::Processes {
                subs.push(keyboard::on_key_press(|key, _modifiers| {
                    match key {
                        Key::Named(Named::ArrowDown) => Some(Message::SelectNextProcess),
                        Key::Named(Named::ArrowUp) => Some(Message::SelectPreviousProcess),
                        _ => None,
                    }
                }));
                iced::time::every(Duration::from_millis(refresh_millis)).map(|_| Message::RefreshProcesses(Vec::new()))
            } else {
                iced::time::every(Duration::from_secs(86400)).map(|_| Message::RefreshProcesses(Vec::new()))
            };
            subs.push(proc_refresh);
            // Slow updates for non‑process data (disks, network, overview) regardless of tab
            subs.push(iced::time::every(Duration::from_millis(90000)).map(|_| Message::TickSlow));
        }
        Subscription::batch(subs)
    }



    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.loading && self.processes.is_empty() {
            // Smooth progress animation over 5 seconds (5000ms)
            let elapsed_ms = self.loading_start_time.elapsed().as_millis() as f32;
            let percent = (elapsed_ms / 5000.0 * 100.0).min(100.0) as u32;

            // Load icon from embedded bytes instead of reading from disk on every render
            static APP_ICON_BYTES: &[u8] = include_bytes!("../assets/app_icon.png");
            let icon_image = image(iced::widget::image::Handle::from_bytes(APP_ICON_BYTES))
                .width(Length::Fixed(180.0))
                .height(Length::Fixed(180.0));

            let loading_label = text("LOADING")
                .size(24)
                .width(Length::Shrink)
                .style(|_| text::Style { color: Some(Color::from_rgb(0.96, 0.62, 0.04)) }); // Amber/Orange

            let percent_label = text(format!("{}%", percent))
                .size(16)
                .style(|_| text::Style { color: Some(Color::from_rgb(0.02, 0.71, 0.83)) });

            // Custom native progress bar using row inside rounded containers
            // Background bar container width = 450px
            let bar_width = 450.0;
            
            // Build the green filled portion smoothly matching the percentage
            let bar_inner = if percent > 0 {
                container(Space::new(Length::Fill, Length::Fill))
                    .width(Length::FillPortion(percent as u16))
                    .style(|_| container::Style {
                        background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.8, 0.4))), // Custom green
                        border: iced::Border {
                            radius: 10.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
            } else {
                container(Space::new(Length::Fixed(0.0), Length::Fill))
            };

            let bar_empty = if percent < 100 {
                container(Space::new(Length::Fill, Length::Fill))
                    .width(Length::FillPortion((100 - percent) as u16))
            } else {
                container(Space::new(Length::Fixed(0.0), Length::Fill))
            };

            let custom_progress_bar = container(
                row![bar_inner, bar_empty].width(Length::Fill).height(Length::Fill)
            )
            .width(Length::Fixed(bar_width))
            .height(Length::Fixed(20.0))
            .style(|_| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.06, 0.09, 0.16))),
                border: iced::Border {
                    color: Color::from_rgb(0.2, 0.4, 0.8),
                    width: 2.0,
                    radius: 10.0.into(),
                },
                ..Default::default()
            });

            let loading_screen = container(
                column![
                    icon_image,
                    Space::with_height(Length::Fixed(16.0)),
                    loading_label,
                    Space::with_height(Length::Fixed(12.0)),
                    custom_progress_bar,
                    Space::with_height(Length::Fixed(12.0)),
                    percent_label,
                ]
                .align_x(Alignment::Center)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(ThemeColors::BACKGROUND)),
                ..Default::default()
            });

            return loading_screen.into();
        }

        let sidebar = render_sidebar(
            self.current_tab,
            self.processes_subtab,
            self.processes_expanded,
            self.processes.len(),
            &self.lang,
            self.config.refresh_rate,
        );

// Loading indicator removed; no progress bar displayed

        let main_view: Element<Message> = match self.current_tab {
            Tab::Overview => render_overview_view(
                &self.cpu,
                &self.memory,
                &self.disks,
                &self.networks,
                &self.cpu_history,
                &self.memory_history,
                &self.lang,
                &self.leak_pids,
                &self.processes,
            ),
            Tab::Processes => {
                let subtab_btn = |subtab: ProcessesSubTab, label: &str| -> Element<'_, Message> {
                    let is_active = self.processes_subtab == subtab;
                    button(text(label.to_string()).size(13).style(move |_| text::Style {
                        color: Some(if is_active { Color::WHITE } else { ThemeColors::TEXT_MUTED }),
                    }))
                    .padding([8, 16])
                    .on_press(Message::ProcessesSubTabSelected(subtab))
                    .style(move |_, status| button::Style {
                        background: Some(iced::Background::Color(if is_active {
                            ThemeColors::CPU_ACCENT
                        } else if status == button::Status::Hovered {
                            ThemeColors::CARD_HOVER
                        } else {
                            Color::TRANSPARENT
                        })),
                        text_color: if is_active { Color::WHITE } else { ThemeColors::TEXT_MUTED },
                        border: iced::Border {
                            color: if is_active { ThemeColors::CPU_ACCENT } else { ThemeColors::CARD_BORDER },
                            width: 1.0,
                            radius: 6.0.into(),
                        },
                        shadow: Default::default(),
                    })
                    .into()
                };

                let subtabs_bar = container(
                    row![
                        subtab_btn(ProcessesSubTab::Processes, &self.lang.proc_subtab_processes),
                        Space::with_width(Length::Fixed(8.0)),
                        subtab_btn(ProcessesSubTab::Startup, &self.lang.proc_subtab_autostart),
                        Space::with_width(Length::Fixed(8.0)),
                        subtab_btn(ProcessesSubTab::Details, &self.lang.proc_subtab_details),
                        Space::with_width(Length::Fixed(8.0)),
                        subtab_btn(ProcessesSubTab::Anomalies, &self.lang.proc_subtab_anomalies),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding([12, 24]);

                let subview: Element<Message> = match self.processes_subtab {
                    ProcessesSubTab::Processes => render_processes_view(
                        &self.filtered_processes,
                        &self.search_query,
                        self.sort_key,
                        self.sort_ascending,
                        self.selected_pid,
                        self.snapshot_taken,
                        self.diff_mode,
                        &self.snapshot_pids,
                        &self.leak_pids,
                        &self.suspended_pids,
                        self.boost_active,
                        &self.lang,
                    ),
                    ProcessesSubTab::Startup => render_startup_view(
                        &self.startup_items,
                        &self.search_query,
                        self.selected_startup_index,
                        &self.lang,
                    ),
                    ProcessesSubTab::Details => render_details_view(
                        &self.filtered_processes,
                        &self.search_query,
                        self.sort_key,
                        self.sort_ascending,
                        self.selected_pid,
                        &self.lang,
                    ),
                    ProcessesSubTab::Anomalies => crate::views::anomalies::render_anomalies_view(
                        &self.filtered_processes,
                        &self.lang,
                    ),
                };

                let base = column![
                    subtabs_bar,
                    subview,
                ]
                .into();

                // Modal overlay on top
                if self.show_process_info {
                    if let Some(pid) = self.selected_pid {
                        if let Some(proc) = self.filtered_processes.iter()
                            .find(|p| p.pid == pid)
                            .or_else(|| self.processes.iter().find(|p| p.pid == pid))
                        {
                            let is_susp = self.suspended_pids.contains(&proc.pid);
                            let modal = render_process_info_modal(proc, is_susp, &self.lang);
                            return stack![base, modal].into();
                        }
                    }
                }
                base
            }
            Tab::Cpu => render_cpu_view(&self.cpu, &self.cpu_history, &self.lang),
            Tab::Ram => render_ram_view(&self.memory, &self.memory_history, &self.lang),
            Tab::Disks => render_disks_view(&self.disks, &self.processes, &self.lang),
            Tab::Network => render_network_view(&self.networks, &self.lang),
            Tab::SystemInfo => render_system_info_view(&self.system_overview, &self.cpu, &self.memory, &self.lang),
            Tab::Settings => render_settings_view(&self.lang, self.config.language, self.config.refresh_rate),
        };

        let content_container = container(main_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(ThemeColors::BACKGROUND)),
                ..Default::default()
            });

        let mut main_col = column![content_container].width(Length::Fill).height(Length::Fill);

        if let Some(msg) = &self.status_message {
            let is_error = msg.starts_with("Error") || msg.starts_with("Failed");
            let color = if is_error { ThemeColors::DANGER } else { ThemeColors::TEXT_PRIMARY };
            
            let status_bar = container(text(msg).size(12).style(move |_| text::Style { color: Some(color) }))
                .width(Length::Fill)
                .padding([4, 16])
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
                    border: iced::Border {
                        color: ThemeColors::CARD_BORDER,
                        width: 1.0,
                        radius: 0.0.into()
                    },
                    ..Default::default()
                });
                
            main_col = main_col.push(status_bar);
        }

        row![sidebar, main_col]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }


    fn filter_and_sort_processes(&mut self) {
        let q = self.search_query.trim().to_lowercase();
        let ascending = self.sort_ascending;

        let mut filtered: Vec<ProcessItem> = self.processes.iter()
            .filter(|p| {
                if q.is_empty() {
                    return true;
                }
                p.name.to_ascii_lowercase().contains(&q)
                    || p.pid.to_string().contains(&q)
            })
            .cloned()
            .collect();

        match self.sort_key {
            SortKey::Pid => filtered.sort_by_key(|p| p.pid),
            SortKey::Name => filtered.sort_by_key(|p| p.name.to_ascii_lowercase()),
            SortKey::Cpu => filtered.sort_by(|a, b|
                a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
            SortKey::Memory => filtered.sort_by_key(|p| p.memory_bytes),
            SortKey::Disk => filtered.sort_by_key(|p| p.disk_read_bytes_sec + p.disk_write_bytes_sec),
            SortKey::Age => filtered.sort_by_key(|p| p.age_secs()),
        }
        if !ascending { filtered.reverse(); }
        self.filtered_processes = filtered;
        // Limit number of displayed processes to improve UI rendering speed
        const MAX_DISPLAY: usize = 100;
        if self.filtered_processes.len() > MAX_DISPLAY {
            self.filtered_processes.truncate(MAX_DISPLAY);
        }
    }

    fn update_leak_detection(&mut self) {
        for proc in &self.processes {
            let history = self.proc_mem_history.entry(proc.pid).or_insert_with(Vec::new);
            history.push(proc.memory_bytes);
            if history.len() > LEAK_TICKS + 5 {
                let excess = history.len() - LEAK_TICKS;
                history.drain(0..excess);
            }
        }

        // Remove stale PIDs
        let alive_pids: HashSet<u32> = self.processes.iter().map(|p| p.pid).collect();
        self.proc_mem_history.retain(|pid, _| alive_pids.contains(pid));

        // Detect leaks: last LEAK_TICKS values all increasing AND total growth > 15%
        self.leak_pids.clear();
        for (pid, history) in &self.proc_mem_history {
            if history.len() < LEAK_TICKS { continue; }
            let tail = &history[history.len() - LEAK_TICKS..];
            let all_increasing = tail.windows(2).all(|w| w[1] >= w[0]);
            let first = tail[0];
            let last = *tail.last().unwrap();
            let growth_pct = if first > 0 { (last as f64 - first as f64) / first as f64 * 100.0 } else { 0.0 };
            if all_increasing && growth_pct > 15.0 && last > 1_000_000 { // > 1 MB baseline
                self.leak_pids.insert(*pid);
            }
        }
    }

    /// Linear regression slope of memory_history for RAM forecast.
    /// Returns MB/tick (positive = growing).
    #[allow(dead_code)]
    pub fn memory_trend_mb_per_tick(&self) -> f64 {
        let n = self.memory_history.len() as f64;
        if n < 10.0 { return 0.0; }
        let x_mean = (n - 1.0) / 2.0;
        let y_mean: f64 = self.memory_history.iter().map(|&v| v as f64).sum::<f64>() / n;
        let mut num = 0.0_f64;
        let mut den = 0.0_f64;
        for (i, &y) in self.memory_history.iter().enumerate() {
            let x = i as f64 - x_mean;
            num += x * (y as f64 - y_mean);
            den += x * x;
        }
        if den == 0.0 { 0.0 } else { num / den }
    }
}
