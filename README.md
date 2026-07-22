# TaskExplorer 🚀

**TaskExplorer** is a modern, blazingly fast, and lightweight process manager and system monitor written in **Rust** using the **Iced GUI** framework. It is designed to be a more powerful, advanced, and safer alternative to the default Windows Task Manager.

Powered by low-level system access (`sysinfo` and native Windows APIs), TaskExplorer operates with zero latency, consumes a fraction of the system resources, and offers out-of-the-box rescue and diagnostic features that standard tools lack.

## 🔥 Why TaskExplorer is Better (Key Features)

* 🕵️ **Advanced Process Analytics & Anomaly Detection:**
  * **Memory Leak Detection:** Automatically flags processes that continuously increase their RAM usage over time.
  * **Zombie Process Identification:** Detects suspended or dead processes (consuming 0% CPU and RAM) that can be safely terminated.
  * **Snapshots & Diff Mode:** Take a "snapshot" of currently running processes and enable "Diff Mode" later to instantly see exactly which new background processes were spawned and which were closed.
* 💾 **SSD Wear Guard:** A unique feature in the Disk Manager that tracks the total amount of data written by a process since it started. This allows you to instantly detect rogue applications that are "hammering" your SSD with continuous gigabytes of background writes.
* 🚀 **Startup Apps Manager:** Built-in startup manager that reads directly from the Windows Registry (HKCU/HKLM) and system folders, making it easy to locate and manage hidden autostart programs.
* ⚙️ **Full Hardware & Disk Control:** Accurate statistics for CPU, RAM, network traffic, and an advanced Disks view (differentiating between HDD/SSD and tracking real-time I/O operations). Includes GPU model detection.
* ⚡ **Extreme Optimization:** Written entirely in Rust. The interface switches tabs with **0 ms latency**. Smooth animations and full control over the auto-refresh rate protect your CPU from unnecessary load.
* 🌍 **Multi-Language Support:** Fully translated into English, Polish, and Russian.

## 📸 Screenshots
*(Add screenshots of the Overview, Processes, and Disk Manager here)*

## 🛠️ Technologies Used
* **Rust** - Ensures memory safety and top-tier performance.
* **Iced GUI** - A modern, cross-platform, and smooth user interface framework.
* **Sysinfo / Winreg / Winres** - For low-level Windows system integration and privilege management.

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
   *(Note: TaskExplorer requests Administrator privileges on Windows to properly manage system processes and read registry keys).*

## 🤝 Contributing
Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

## 📝 License
This project is open-source and available under the [MIT License](LICENSE).
