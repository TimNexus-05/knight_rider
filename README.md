# knight_rider
A bare-metal Rust project for the ESP32 demonstrating a Knight Rider–style LED scanner with UART serial logging. Built using esp-hal in a no_std environment, the project showcases GPIO control, delays, structured board configuration, and a simulated sensor module for embedded systems learning
# ESP32 Knight Rider LED Scanner (Rust)

This project is a **bare-metal Rust application** for the **ESP32** that implements a classic **Knight Rider (Larson Scanner) LED effect** using GPIO pins, with detailed **UART serial logging** for real-time visibility.

It runs in a `#![no_std]`, `#![no_main]` environment using `esp-hal` and is designed as a **learning-focused embedded Rust project**, demonstrating GPIO control, UART communication, delays, modular configuration, and simulated sensor data.

---

## ✨ Features

- 🚦 Knight Rider (left-to-right & right-to-left) LED scanning effect
- 🔌 GPIO output control using `esp-hal`
- 📟 UART serial logging (cycle count, LED states, direction)
- ⏱️ Timing control using blocking delays
- 🧱 `no_std`, bare-metal Rust (no OS, no allocator)
- 🧩 Modular project structure (`config`, `sensor`)
- 🌡️ Virtual temperature & humidity sensor (simulated)

---

## 🧠 What This Project Demonstrates

- Embedded Rust without the standard library
- ESP32 peripheral initialization with `esp-hal`
- GPIO pin configuration and state control
- UART serial output using `core::fmt::Write`
- Timing using blocking delays
- Clean separation of hardware configuration
- Simulated sensor data for testing logic without hardware

---

## 📂 Project Structure

```text
src/
├── main.rs        # LED scanner logic + UART logging
├── config.rs      # Board pin configuration
└── sensor.rs      # Virtual temperature & humidity sensor
