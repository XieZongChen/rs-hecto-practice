use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        Editor {}
    }
    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program end!"),
            _ => (),
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

/*
将输入字符转换为对应的 ASCII 控制字符的索引
*/
// fn to_ctrl_byte(c: char) -> u8 {
//     let byte = c as u8;
//     /*
//      & 操作符用于执行位与运算。0b0001_1111 是一个二进制掩码，它的作用是保留 byte 的最低 5 位，这正好对应于 ASCII 控制字符的范围（0 到 31）
//      返回结果是一个 8 位整数，其最低 5 位与 ASCII 控制字符的编码相对应。这个值表示控制字符的索引。
//     */
//     byte & 0b0001_1111
// }
