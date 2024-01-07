use std::error::Error;

#[derive(Clone)]
pub struct Bin {
    pub address: u32,
    pub data: Vec<u8>,
}


pub fn hex_to_bin(hex: &str) -> Result<Vec<Bin>, Box<dyn Error>> {
    const FILL_BYTE: u8 = 0xFF;
    let mut vec_bin: Vec<Bin> = Vec::new();

    let mut start_address = String::from("-1");
    let mut current_address = 0;

    let mut bin = Bin {
        address: 0,
        data: Vec::new(),
    };

    let mut is_first = true;

    for line in hex.lines() {
        let hex_line = line.replace(":", "").replace(" ", "");
        let data_length = usize::from_str_radix(&hex_line[0..2], 16)?;
        let offset_address = usize::from_str_radix(&hex_line[2..6], 16)?;
        let record_type = usize::from_str_radix(&hex_line[6..8], 16)?;

        match record_type {
            0 => { // 数据记录
                if start_address != "-1" {
                    if current_address < offset_address {
                        // 如果当前地址小于偏移地址，用 0xFF 填充间隙
                        bin.data.resize(offset_address, FILL_BYTE);
                    }
                    // 转换数据区的内容为二进制并存入 bin
                    for i in 0..data_length {
                        let data_byte = u8::from_str_radix(&hex_line[8 + i * 2..10 + i * 2], 16)?;
                        bin.data.push(data_byte);
                        current_address += 1;
                    }
                }
            }
            1 => { // 文件结束记录
                break;
            }
            4 => { // 扩展线性地址记录
                // 获取高16位地址并更新起始地址
                if is_first { // 第一次进入，不需要添加
                    is_first = false;
                } else {
                    vec_bin.push(bin.clone());
                }
                let high_address = &hex_line[8..12];
                start_address = format!("0x{}0000", high_address);
                current_address = 0;
                bin.address = u32::from_str_radix(&start_address[2..], 16)?;
                bin.data.clear();
            }
            _ => { // 其他记录类型
                // 忽略不处理
            }
        }
    }
    vec_bin.push(bin);
    Ok(vec_bin)
}