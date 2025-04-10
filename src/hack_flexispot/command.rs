use std::collections::HashMap;

/// サポートされているコマンドを返す関数
/// こちらのリポジトリ(https://github.com/iMicknl/LoctekMotion_IoT)から引用
pub fn supported_commands() -> HashMap<&'static str, &'static [u8]> {
    HashMap::from([
        ("up",       &b"\x9b\x06\x02\x01\x00\xfc\xa0\x9d"[..]),
        ("down",     &b"\x9b\x06\x02\x02\x00\x0c\xa0\x9d"[..]),
        ("preset_1", &b"\x9b\x06\x02\x04\x00\xac\xa3\x9d"[..]),
        ("preset_2", &b"\x9b\x06\x02\x08\x00\xac\xa6\x9d"[..]),
        ("stand",    &b"\x9b\x06\x02\x10\x00\xac\xac\x9d"[..]),
        ("sit",      &b"\x9b\x06\x02\x00\x01\xac\x60\x9d"[..]),
    ])
}