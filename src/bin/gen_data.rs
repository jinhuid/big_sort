use std::{env, fs::File, io::{self, BufWriter, Write}};

use rand::Rng;

fn main() -> io::Result<()> {
    let data_size = env::args()
        .nth(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(100000000);

    let file = File::create("sort_input.txt").expect("创建文件失败");
    let mut writer = BufWriter::with_capacity(32 * 1024 * 1024, file);

    for _ in 0..data_size {
        writeln!(writer, "{}", random(0, 1e8 as u32))?;
    }   

    writer.flush()?;
    println!("已生成 {} 个随机数据到 sort_input.txt", data_size);
    Ok(())
}

fn random(min: u32, max: u32) -> u32 {
    rand::rng().random_range(min..max)
}
