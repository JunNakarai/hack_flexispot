use rppal::gpio::{Gpio, OutputPin};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::process;
use std::time::Duration;
use serialport::SerialPort;

// GPIOピン番号やシリアルポートの設定
const SERIAL_PORT: &str = "/dev/ttyAMA0";
const GPIO_PIN: u8 = 12;

// サポートされているコマンド
// こちらのリポジトリ(https://github.com/iMicknl/LoctekMotion_IoT)から引用させていただきました。
fn supported_commands() -> HashMap<&'static str, &'static [u8]> {
    HashMap::from([
        ("up", &b"\x9b\x06\x02\x01\x00\xfc\xa0\x9d"[..]),
        ("down", &b"\x9b\x06\x02\x02\x00\x0c\xa0\x9d"[..]),
        ("preset_1", &b"\x9b\x06\x02\x04\x00\xac\xa3\x9d"[..]),
        ("preset_2", &b"\x9b\x06\x02\x08\x00\xac\xa6\x9d"[..]),
        ("stand", &b"\x9b\x06\x02\x10\x00\xac\xac\x9d"[..]),
        ("sit", &b"\x9b\x06\x02\x00\x01\xac\x60\x9d"[..]),
    ])
}

struct HackFlexispot {
    serial: Box<dyn SerialPort>,
    pin: OutputPin,
}

impl HackFlexispot {
    /// インスタンスの初期化
    fn new(serial_port: &str, gpio_pin: u8) -> Result<Self, Box<dyn std::error::Error>> {
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
    fn execute_command(&mut self, command_name: &str) -> io::Result<()> {
        let commands = supported_commands();
        if let Some(command) = commands.get(command_name) {
            self.serial.write_all(command)?;
            println!("Command '{}' sent successfully.", command_name);
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unknown command: {}", command_name)));
        }
        Ok(())
    }
}

// メイン関数
fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [COMMAND]", args[0]);
        eprintln!("Supported Commands:");
        for command in supported_commands().keys() {
            eprintln!("\t{}", command);
        }
        process::exit(1);
    }

    let command = &args[1];

    // HackFlexispot を初期化してコマンドを実行
    match HackFlexispot::new(SERIAL_PORT, GPIO_PIN) {
        Ok(mut loctek) => {
            if let Err(e) = loctek.execute_command(command) {
                eprintln!("Error executing command '{}': {}", command, e);
            }
        }
        Err(e) => eprintln!("Initialization error: {}", e),
    }
}
