# ESP32 Knight Rider LED Scanner with Web Visualization

This project combines **bare-metal Rust firmware** on the **ESP32** with a **Python web server** to visualize the Knight Rider (Larson Scanner) LED effect in real-time. It’s a learning-focused project for embedded Rust, GPIO control, UART communication, and web-based hardware visualization.

---

## 🔥 Features

- 🚦 Knight Rider LED effect on 4 LEDs  
- 🧱 Bare-metal Rust (`no_std`, `no_main`)  
- 📟 UART serial logging (LED state, cycle count, direction)  
- ⏱️ Timing control with blocking delays  
- 🛠️ Modular Rust code (`config.rs`, `sensor.rs`)  
- 🌡️ Virtual sensor module for testing (temperature & humidity)  
- 💻 Python web server (`web_server.py`) with WebSocket  
- 🖥️ Browser interface (`index.html`) for virtual LED control and monitoring  
- 🎛️ Start, Stop, Reset controls in browser  
- 🌐 Real-time LED sync between hardware and browser

---

## 📂 Project Structure

```text
knight_rider/
├── src/
│   ├── main.rs       # Main firmware logic (Knight Rider effect + UART)
│   ├── config.rs     # Board GPIO pin mapping
│   └── sensor.rs     # Virtual sensor (temperature & humidity)
├── web_server.py      # Python HTTP + WebSocket server for visualization
├── index.html         # Browser interface for LED visualization
├── README.md          # This file
└── .gitignore
