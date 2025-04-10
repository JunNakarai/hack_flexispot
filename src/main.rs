use std::env;
use std::process;

mod hack_flexispot;
use hack_flexispot::command::supported_commands;
use hack_flexispot::device::HackFlexispot;

const SERIAL_PORT: &str = "/dev/serial0";
const GPIO_PIN: u8 = 12;

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

    let command_name = &args[1];

    // HackFlexispot を初期化してコマンドを実行
    match HackFlexispot::new(SERIAL_PORT, GPIO_PIN) {
        Ok(mut device) => {
            let commands = supported_commands();
            if let Some(cmd_bytes) = commands.get(command_name.as_str()) {
                if let Err(e) = device.execute_command(cmd_bytes, command_name) {
                    eprintln!("Error executing command '{}': {}", command_name, e);
                }
            } else {
                eprintln!("Unknown command: {}", command_name);
            }
        }
        Err(e) => eprintln!("Initialization error: {}", e),
    }
}
