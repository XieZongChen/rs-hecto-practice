use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

/**
 * 将输入字符转换为对应的 ASCII 控制字符的索引
 */
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    /*
     & 操作符用于执行位与运算。0b0001_1111 是一个二进制掩码，它的作用是保留 byte 的最低 5 位，这正好对应于 ASCII 控制字符的范围（0 到 31）
     返回结果是一个 8 位整数，其最低 5 位与 ASCII 控制字符的编码相对应。这个值表示控制字符的索引。
    */
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    }
}
