use std::{
    env, fs,
    io::{self, BufWriter, Write},
    time::Instant,
};

use memmap2::MmapOptions;
use rayon::prelude::*;

use sort::Parser;

fn main() -> io::Result<()> {
    // 获取参数
    let args: Vec<String> = env::args().collect();
    let (input_file, output_file) = if args.len() >= 3 {
        (args[1].as_str(), args[2].as_str())
    } else {
        ("sort_input.txt", "sort_output.txt")
    };

    let total_start = Instant::now();

    let mmap = get_file_mmap(input_file)?;
    let mut parsed_int = parse_int(&mmap);
    let sorted_int = sort_integers(&mut parsed_int);
    write_to_file(&sorted_int, output_file)?;
    println!("总耗时: {:.3}s", total_start.elapsed().as_secs_f32());
    Ok(())
}

fn get_file_mmap(file_path: &str) -> io::Result<memmap2::Mmap> {
    let read_start = Instant::now();
    let file = fs::File::open(file_path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    println!("数据读取耗时: {:.3}s", read_start.elapsed().as_millis());
    Ok(mmap)
}

fn parse_int(data: &[u8]) -> Vec<u32> {
    let parse_start = Instant::now();
    let parsed_int = Parser::parse_int(data);
    println!(
        "解析完成: {} 个整数, 耗时: {:.3}s",
        parsed_int.len(),
        parse_start.elapsed().as_secs_f32()
    );
    parsed_int
}

fn sort_integers(nums: &mut [u32]) -> Vec<u32> {
    let sort_start = Instant::now();
    nums.par_sort_unstable();
    println!("排序完成: 耗时: {:.3}s", sort_start.elapsed().as_secs_f32());
    nums.to_vec()
}

fn write_to_file(sorted: &[u32], output_path: &str) -> io::Result<()> {
    let write_start = Instant::now();
    let file = fs::File::create(output_path)?;
    let mut writer = BufWriter::with_capacity(32 * 1024 * 1024, file);
    let buffers = Parser::int_to_buffers(sorted);
    // 然后按顺序写入所有缓冲区，确保维持排序顺序
    for buffer in buffers {
        writer.write_all(&buffer)?;
    }
    // 确保所有数据都已写入
    writer.flush()?;
    println!("写入耗时: {:.3}s", write_start.elapsed().as_secs_f32());
    Ok(())
}
