#!/usr/bin/env python3
import asyncio
import http.server
import socketserver
import os
import websockets
import serial
import threading
import time

PORT = 8000
WS_PORT = 8765
SERIAL_PORT = '/dev/ttyUSB0'  # Change this to your ESP32C3 serial port
BAUD_RATE = 115200

connected_clients = set()

class Handler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/':
            self.path = '/index.html'
        return super().do_GET()

    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        super().end_headers()

async def websocket_handler(websocket, path):
    connected_clients.add(websocket)
    try:
        await websocket.wait_closed()
    finally:
        connected_clients.remove(websocket)

async def broadcast_message(message):
    if connected_clients:
        await asyncio.gather(*[client.send(message) for client in connected_clients])

def serial_reader():
    try:
        ser = serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=1)
        print(f"Connected to serial port {SERIAL_PORT}")
        while True:
            if ser.in_waiting > 0:
                line = ser.readline().decode('utf-8').strip()
                print(f"Serial: {line}")
                # Parse LED states from UART output
                if "LED" in line and "ON" in line:
                    led_num = int(line.split()[1])
                    asyncio.run(broadcast_message(f"led_on:{led_num}"))
                elif "LED" in line and "OFF" in line:
                    led_num = int(line.split()[1])
                    asyncio.run(broadcast_message(f"led_off:{led_num}"))
                elif "Cycle" in line:
                    cycle = line.split('#')[1].split()[0]
                    asyncio.run(broadcast_message(f"cycle:{cycle}"))
    except serial.SerialException as e:
        print(f"Serial error: {e}")
    except Exception as e:
        print(f"Error: {e}")

async def main():
    # Start serial reader in a thread
    serial_thread = threading.Thread(target=serial_reader, daemon=True)
    serial_thread.start()

    # Start WebSocket server
    ws_server = await websockets.serve(websocket_handler, "localhost", WS_PORT)
    print(f"WebSocket server running on ws://localhost:{WS_PORT}")

    # Start HTTP server
    os.chdir('/home/tim/Desktop/knight_rider')
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"HTTP server running at http://localhost:{PORT}")
        print("Open the URL in your browser to see the virtual LED visualization")
        print(f"WebSocket enabled for real-time updates from {SERIAL_PORT}")

        # Run both servers
        await asyncio.get_event_loop().run_in_executor(None, httpd.serve_forever)

if __name__ == "__main__":
    asyncio.run(main())