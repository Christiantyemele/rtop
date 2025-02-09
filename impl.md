### **Logic Behind Its Implementation**
*btop* works by continuously collecting system metrics and rendering them in a terminal UI with smooth animations. Here's a breakdown of its key components:

1. **System Monitoring:**
   - Uses system APIs (`/proc` on Linux, `sysctl` on macOS/BSD, and Windows APIs) to gather CPU, memory, disk, and network usage.
   - Retrieves running processes and their stats (CPU/memory usage, priority, etc.).

2. **Terminal UI Rendering:**
   - Uses **ncurses** for efficient and responsive UI updates.
   - Draws colorful, animated graphs and tables in the terminal.
   - Supports mouse and keyboard interactions.

3. **Asynchronous Data Collection:**
   - Runs monitoring tasks in separate threads for smooth updates.
   - Uses efficient polling intervals to reduce CPU usage.

4. **Configuration & Customization:**
   - Stores user preferences in a config file.
   - Allows color themes and keybindings customization.

### **Rewriting in Rust**
Rust is a great choice for this because it offers performance similar to C++ but with better memory safety. Here’s how you might approach it:

- Use **sysinfo** or **heim** crates for system metrics.
- Use **crossterm** or **ratatui** for a terminal UI.
- Use **tokio** or **async-std** for async data collection.

Would you like help structuring the project or picking libraries? 🚀

# features
- monitor running processes 
- monitor mem usage, disk usage, cache, cpu usage
- set maximuse mem usage for processes
- alert when max mem or/and cpu reached