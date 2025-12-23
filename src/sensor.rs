// src/sensor.rs
pub struct VirtualSensor {
    temperature: f32,
    humidity: f32,
}

impl VirtualSensor {
    pub fn new() -> Self {
        Self {
            temperature: 22.5,
            humidity: 50.0,
        }
    }
    
    pub fn read(&mut self) -> (f32, f32) {
        // Simple simulated sensor reading
        self.temperature += 0.1;
        if self.temperature > 30.0 {
            self.temperature = 20.0;
        }
        
        self.humidity += 0.2;
        if self.humidity > 70.0 {
            self.humidity = 40.0;
        }
        
        (self.temperature, self.humidity)
    }
}