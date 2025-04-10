use rppal::gpio::{Gpio, OutputPin};
use serialport::SerialPort;
use std::io::{self, Write};
use std::time::Duration;

/// HackFlexispot デバイスを表す構造体
pub struct HackFlexispot {
    pub serial: Box<dyn SerialPort>,
    pub pin: OutputPin,
}

impl HackFlexispot {
    /// インスタンスの初期化
    pub fn new(serial_port: &str, gpio_pin: u8) -> Result<Self, Box<dyn std::error::Error>> {
        // シリアルポートを開く
        let serial = serialport::new(serial_port, 9600)
            .timeout(Duration::from_millis(500))
            .open()
            .map_err(|e| format!("Failed to open serial port: {}", e))?;

        // GPIOピンを初期化
        let gpio = Gpio::new()?;
        let mut pin = gpio.get(gpio_pin)?.into_output();
        pin.set_high();

        Ok(Self { serial, pin })
    }

    /// コマンドを実行
    pub fn execute_command(&mut self, command: &[u8], command_name: &str) -> io::Result<()> {
        self.serial.write_all(command)?;
        println!("Command '{}' sent successfully.", command_name);
        Ok(())
    }
}