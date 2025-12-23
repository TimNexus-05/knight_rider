# knight_rider
A bare-metal Rust project for the ESP32 demonstrating a Knight Rider–style LED scanner with UART serial logging. Built using esp-hal in a no_std environment, the project showcases GPIO control, delays, structured board configuration, and a simulated sensor module for embedded systems learning

## Features

- Hardware LED scanning on ESP32C3 (GPIOs 2, 3, 4, 5)
- Serial output for debugging
- Virtual web-based visualization
- Real-time LED animation simulation

## Hardware Setup

Connect LEDs to the following GPIO pins on ESP32C3:
- LED 1: GPIO 2
- LED 2: GPIO 3
- LED 3: GPIO 4
- LED 4: GPIO 5

## Building and Running

### Hardware (ESP32C3)

1. Build the firmware:
   ```bash
   cargo build --bin knight_rider
   ```

2. Flash to ESP32C3 (using your preferred method, e.g., espflash)

### Virtual Visualization

#### Option 1: Manual Sync (Simple)
1. Start the web server:
   ```bash
   python3 web_server.py
   ```

2. Open http://localhost:8000 in your browser

3. Start your ESP32C3 simulation in Wokwi (or flash to hardware)

4. Click the "Start" button on the web page at the same time you start the hardware simulation

#### Option 2: Real-Time Sync (Advanced)
For true real-time synchronization with physical hardware:

1. Install dependencies:
   ```bash
   pip install websockets pyserial
   ```

2. Connect your ESP32C3 to USB and note the serial port (e.g., `/dev/ttyUSB0`)

3. Edit `web_server.py` and change `SERIAL_PORT` to your port

4. Start the enhanced web server:
   ```bash
   python3 web_server.py
   ```

5. Open http://localhost:8000 - it will automatically sync with your hardware UART output

The web interface provides:
- Visual representation of the 4 LEDs
- Start/Stop/Reset controls (for manual mode)
- Real-time status updates
- Automatic animation mimicking the hardware behavior
