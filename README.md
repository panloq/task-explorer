# TaskExplorer 🚀

**TaskExplorer** is a modern, blazingly fast, and lightweight process manager and system monitor written in **Rust** using the **Iced GUI** framework. It is designed to be a more powerful, advanced, and safer alternative to the default Windows Task Manager.

Powered by low-level system access (`sysinfo` and native Windows APIs), TaskExplorer operates with zero latency, consumes a fraction of the system resources, and offers out-of-the-box rescue and diagnostic features that standard tools lack.

## 🔥 Why TaskExplorer is Better (Key Features)

* 🕵️ **Advanced Process Analytics & Anomaly Detection:**
  * **Heuristic Anomaly Scanner:** Built-in scanner that looks for suspicious behavior, such as crypto-miners (high CPU without a parent process), fake system processes, or programs executing from temporary directories.
  * **Memory Leak Detection:** Automatically flags processes that continuously increase their RAM usage over time.
  * **Zombie Process Identification:** Detects suspended or dead processes (consuming 0% CPU and RAM) that can be safely terminated.
  * **Snapshots & Diff Mode:** Take a "snapshot" of currently running processes and enable "Diff Mode" later to instantly see exactly which new background processes were spawned and which were closed.
* 🚀 **Boost Mode (Gaming Mode):** Instantly suspend resource-heavy background apps (like browsers, Discord, or Steam) with a single click to free up CPU cycles for your games. Easily resume them all when you are done.
* 📈 **Live Mini-Charts (Sparklines):** Displays real-time CPU history graphs directly in the process list for every single process, giving you an immediate visual cue of a program's behavior.
* 💾 **SSD Wear Guard:** A unique feature in the Disk Manager that tracks the total amount of data written by a process since it started. This allows you to instantly detect rogue applications that are "hammering" your SSD with continuous background writes.
* ⚡ **Startup Apps Manager:** Built-in startup manager that reads directly from the Windows Registry (HKCU/HKLM) and system folders. Easily toggle apps on/off or **completely delete** them from autostart.
* ⚙️ **Full Hardware & Disk Control:** Accurate statistics for CPU, RAM, network traffic, and an advanced Disks view (differentiating between HDD/SSD and tracking real-time I/O operations). Includes GPU model detection.
* 🛡️ **Smart Privilege Management:** Runs smoothly as a standard user without annoying UAC prompts on every startup. It only requests Administrator privileges (via PowerShell RunAs) when performing advanced actions like terminating protected process trees.
* 🌍 **Multi-Language Support:** Fully translated into English, Polish, and Russian.

## 📸 Screenshots

### System Performance Overview
![System Performance Overview](./screenshots/overview.png)

### Process Manager (with live CPU mini-charts)
<img width="994" height="669" alt="{D775C8E6-5C57-4D80-ACEB-8455B56AFA0D}" src="https://github.com/user-attachments/assets/5b2c7d93-983c-4126-8ebc-afce8b528bc0" />


## 🛠️ Technologies Used
* **Rust** - Ensures memory safety and top-tier performance.
* **Iced GUI** - A modern, cross-platform, and smooth user interface framework.
* **Sysinfo / Winreg** - For low-level Windows system integration and registry management.

## 🚀 How to Run

1. Make sure you have [Rust and Cargo](https://rustup.rs/) installed on your system.
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/task_explorer.git
   cd task_explorer
   ```
3. Run the application:
   ```bash
   cargo run --release
   ```

## 🤝 Contributing
Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

## 📝 License
This project is open-source and available under the [MIT License](LICENSE).
