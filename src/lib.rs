use num_cpus;
use rayon::prelude::*;

pub struct Parser;

impl Parser {
    fn split_to_chunks(data: &[u8], chunks_num: usize) -> Vec<&[u8]> {
        let mut chunks = Vec::with_capacity(chunks_num);
        let chunk_size = data.len() / chunks_num;
        let mut start = 0;
        let mut end = chunk_size;
        for _ in 1..chunks_num {
            let mut skip = false;
            let chunk = loop {
                while !data[end].is_ascii_digit() {
                    end += 1;
                    skip = true;
                }
                if skip {
                    break &data[start..end];
                }
                end += 1;
            };
            chunks.push(chunk);
            start = end;
            end += chunk_size;
        }
        chunks.push(&data[start..]);
        chunks
    }

    fn chunk_to_int(chunk: &[u8]) -> Vec<u32> {
        // 吧一个数字当初8个字节来算
        let mut parsed_chunk = Vec::with_capacity(chunk.len() / 8 as usize);
        let mut current_number: u32 = 0;
        let mut parsing = false;
        for &byte in chunk {
            if byte.is_ascii_digit() {
                current_number = current_number * 10 + (byte - b'0') as u32;
                parsing = true;
            } else if parsing {
                parsed_chunk.push(current_number);
                current_number = 0;
                parsing = false;
            }
        }
        if parsing {
            parsed_chunk.push(current_number);
        }
        parsed_chunk
    }

    pub fn parse_int(data: &[u8]) -> Vec<u32> {
        let thread_num = num_cpus::get().max(1);

        Self::split_to_chunks(data, thread_num)
            .par_iter()
            .flat_map(|&chunk| Self::chunk_to_int(chunk))
            .collect()
    }

    // 将整数转换为字符串并写入缓冲区
    #[inline]
    fn fast_write_int(num: u32, buf: &mut Vec<u8>) {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(num);
        buf.extend_from_slice(s.as_bytes());
        buf.push(b'\n');
    }

    pub fn int_to_buffers(sorted: &[u32]) -> Vec<Vec<u8>> {
        let thread_count = num_cpus::get().max(1);
        let chunk_size = sorted.len() / thread_count + 1;

        sorted
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut buffer = Vec::with_capacity(chunk.len() * 10); // 预估每个数字平均10个字节
                chunk.into_iter().for_each(|&num| {
                    // println!("num: {}", num);
                    Self::fast_write_int(num, &mut buffer);
                });
                buffer
            })
            .collect()
    }
}
