## 针对大数据进行排序
> 对u32内范围的整数进行排序
> 使用并行处理
> 极致的速度

## 生成数据
> cargo run --release --bin gen_data -- 100000000

## 排序
> cargo run --release --bin sort -- sort_input.txt sort_output.txt


对于现代机器 100000000个数据 通常可以跑到 2s内
