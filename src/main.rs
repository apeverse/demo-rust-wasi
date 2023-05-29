use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {

    // 打开要读取的文件
    let mut file = File::open("input.txt")?;

    // 创建一个缓冲区来存储文件内容
    let mut buffer = Vec::new();

    // 读取文件内容到缓冲区
    file.read_to_end(&mut buffer)?;

    // 向屏幕输出文件内容
    io::stdout().write_all(&buffer)?;

    let message = "Hello, WASI!\n";

    // 向屏幕输出消息：Hello, WASI
    io::stdout().write_all(message.as_bytes())?;

    // 创建一个新文件来写入数据
    let mut output_file = File::create("output.txt")?;

    // 写入数据到文件
    output_file.write_all(message.as_bytes())?;

    Ok(())
}

