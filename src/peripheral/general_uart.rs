use crate::peripheral::Pp;
use std::error::Error;
use std::ffi::c_float;
use std::time::Duration;
use colored::Colorize;
use serialport::{available_ports, SerialPort, SerialPortType};
use crate::AirISP;
use rust_i18n::t;
use crate::AirISP::air_isp;
use tokio::runtime::Runtime;
use tokio::time::{sleep};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{Read, Write};

#[repr(u8)]
enum Command {
    Get = 0x00,
    GetVersion = 0x01,
    GetID = 0x02,
    GetDeviceID = 0x03,
    ReadMemory = 0x11,
    Go = 0x21,
    WriteMemory = 0x31,
    ExtendedErase = 0x44,
    WriteProtect = 0x63,
    WriteUnprotect = 0x73,
    ReadProtect = 0x82,
    ReadUnprotect = 0x92,
}

#[repr(u8)]
enum Ack {
    Ack = 0x79,
    Nack = 0x1F,
}

pub struct GeneralUart<'a> {
    air_isp: &'a AirISP::AirISP,

    handle: Box<dyn SerialPort>,
}

impl GeneralUart<'_> {
    pub fn new(air_isp: &AirISP::AirISP) -> GeneralUart {
        let port = serialport::new(air_isp.get_port(), air_isp.get_baud())
            .timeout(std::time::Duration::from_millis(2000))
            .parity(serialport::Parity::Even)
            .open()
            // 显示错误信息，并退出程序
            .unwrap_or_else(|e| {
                eprintln!("{}", t!("open_serial_fail_help", "TTY" => air_isp.get_port(), "error" => e));
                std::process::exit(1);
            });

        println!("{}", format!("{}", t!("open_serial_success_help", "TTY" => air_isp.get_port())).green());

        GeneralUart {
            air_isp,
            // handle 初始化为null
            handle: port,
        }
    }

    /**
     * 检查是否有返回ACK
     */
    pub fn get_ack(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = [0u8; 1];
        let s = self.handle.read(&mut buf)?;
        if buf[0] == Ack::Ack as u8 {
            Ok(())
        } else {
            self.handle.clear(serialport::ClearBuffer::All)?; // 清空缓冲区
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "ack error")))
        }
    }
}

impl Pp for GeneralUart<'_> {

    fn write_flash(&mut self, address: u32, data: &[u8], progress: AirISP::Progress) -> Result<(), Box<dyn Error>> {
        let cmd = [Command::WriteMemory as u8, !(Command::WriteMemory as u8)];
        // 一次最多写256个字节
        for i in 0..data.len() / 256 {
            let now_address = address + (i * 256) as u32;
            let mut addr = [0u8; 5]; //小端模式
            addr[0] = (now_address >> 24) as u8;
            addr[1] = (now_address >> 16) as u8;
            addr[2] = (now_address >> 8) as u8;
            addr[3] = now_address as u8;
            addr[4] = addr[0] ^ addr[1] ^ addr[2] ^ addr[3];            // 校验和
            self.handle.write(&cmd)?;
            self.get_ack()?;
            self.handle.write(&addr)?;
            self.get_ack()?;

            // 发送需要写入的数据，最多256个字节，4个字节为一组
            let mut data_len = data.len() - i * 256;
            if data.len() % 4 != 0 {
                data_len += 4 - data.len() % 4;
            }
            data_len += 1; // 此处加1是因为长度字节也要算进去，但是在后面的buf中是使用校验位作为最后一位
            let mut data_buf = vec![0u8; data_len];
            // 填充数据
            for j in 0..data_len - 1 {
                if j < data.len() {
                    data_buf[j] = data[i * 256 + j];
                } else {
                    data_buf[j] = 0xFF;
                }
            }
            // 计算校验和
            for j in 0..data_len - 1 {
                data_buf[data_len - 1] ^= data_buf[j];
            }

            let len = [data_len as u8];
            self.handle.write(&len)?;
            self.handle.write(&data_buf)?;
            std::thread::sleep(Duration::from_millis(10));
            self.get_ack()?;

            match progress {
                AirISP::Progress::None => {}
                AirISP::Progress::Percent => {
                    let percent = (i * 256) as c_float / data.len() as c_float * 100.0;
                    let addr = address + i as u32 * 256;
                    println!("{}", t!("write_flash_file_percent","addr" => addr, "percent" => percent));
                }
                AirISP::Progress::Bar => {
                    todo!()
                }
            }
        }
        Ok(())
    }

    fn get_chip_id(&mut self) -> Result<(), Box<dyn Error>> {
        let cmd = [Command::GetID as u8, !(Command::GetID as u8)];
        self.handle.write(&cmd)?;
        std::thread::sleep(Duration::from_millis(10));

        self.get_ack()?;
        let mut buf = [0u8; 1]; // 先取出字节数大小
        self.handle.read(&mut buf)?;
        let mut data_len = 0;
        data_len = buf[0] as usize + 1;
        let mut data_buf = vec![0u8; data_len + 1];
        self.handle.read(&mut data_buf)?;

        let mut chip_id : String = Default::default() ;
        for i in 0..data_len {
            chip_id.push_str(&format!("{:#04x} ", data_buf[i]));
        }
        println!("{}", format!("{}", t!("get_chip_success_help","chip_id" => chip_id).cyan()));
        Ok(())
    }

    fn reset_bootloader(&mut self) -> Result<(), Box<dyn Error>> {
        print!("{}", t!("connect_help"));

        // 打印进度条
        let runtime = Runtime::new().unwrap();

        runtime.block_on(async {
            let is_cancelled = Arc::new(AtomicBool::new(false));
            let is_cancelled_for_task = Arc::clone(&is_cancelled);

            let log_task = tokio::spawn(async move {
                let mut count = 0;
                let mut write_flag = true;

                loop {
                    if is_cancelled_for_task.load(Ordering::SeqCst) {
                        break;
                    }

                    if count >= 3 {
                        write_flag = !write_flag;
                        count = 0;
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }

                    if write_flag {
                        print!(".");
                    } else {
                        print!("_");
                    }
                    std::io::stdout().flush().unwrap();

                    count += 1;
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
            });

            for i in 0..self.air_isp.get_connect_attempts() {
                match self.air_isp.get_before().as_str() {
                    // 使用异或电路
                    "default_reset" => {
                        // write_request_to_send是RTS，write_data_terminal_ready是DTR
                        //防止之前没退出复位状态
                        self.handle.write_request_to_send(false).unwrap();
                        self.handle.write_data_terminal_ready(false).unwrap();
                        tokio::time::sleep(Duration::from_millis(50)).await;

                        self.handle.write_data_terminal_ready(true).unwrap();
                        self.handle.write_request_to_send(false).unwrap();
                        tokio::time::sleep(Duration::from_millis(20)).await;

                        self.handle.write_request_to_send(true).unwrap();
                        self.handle.write_data_terminal_ready(false).unwrap();
                        tokio::time::sleep(Duration::from_millis(5)).await;

                        self.handle.write_request_to_send(false).unwrap();
                        self.handle.write_data_terminal_ready(true).unwrap();

                        tokio::time::sleep(Duration::from_millis(5)).await;

                        self.handle.write_data_terminal_ready(false).unwrap();
                    },
                    // 使用直连电路
                    "direct_connect" => {
                        self.handle.write_data_terminal_ready(true).unwrap();
                        self.handle.write_request_to_send(false).unwrap();
                        tokio::time::sleep(Duration::from_millis(20)).await;

                        self.handle.write_request_to_send(true).unwrap();
                        self.handle.write_data_terminal_ready(false).unwrap();
                        // std::thread::sleep(Duration::from_millis(5));

                        self.handle.write_request_to_send(false).unwrap();
                        self.handle.write_data_terminal_ready(true).unwrap();
                        tokio::time::sleep(Duration::from_millis(5)).await;

                        self.handle.write_data_terminal_ready(false).unwrap();
                    }
                    _ => {
                        todo!()
                    }
                }

                let data = [0x7F as u8];
                self.handle.write(&data).unwrap();
                tokio::time::sleep(Duration::from_millis(100)).await;
                match self.get_ack() {
                    Ok(_) => {
                        // println!("{}", t!("connect_success_help"));
                        break;
                    }
                    Err(_) => {
                        if i == self.air_isp.get_connect_attempts() - 1 {
                            println!(""); // 换行
                            println!("{}", format!("{}", t!("connect_fail_help")).red());
                            std::process::exit(1);
                        }
                        tokio::time::sleep(Duration::from_millis(200)).await;
                    }
                }
            }

            // 取消任务
            is_cancelled.store(true, Ordering::SeqCst);
            log_task.await.unwrap();
        });
        println!(""); // 换行

        // 读取Chip ID
        let retry = 5;
        for i in 0..retry {
            match self.get_chip_id() {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    if i == retry - 1 {
                        println!("{}", format!("{}", t!("get_chip_id_fail_help")).red());
                        std::process::exit(1);
                    }
                    std::thread::sleep(Duration::from_millis(100));
                    //也许你看到这行代码的时候会感觉疑惑，这看起来是一个非常愚蠢的行为，让人无法理解。
                    //但是事实并不是这样，经过逻辑分析仪的抓取，我们发现使用CDC驱动的USB转串口，在Windows下使能RTS或者DTR似乎会发出一个奇怪的字节，
                    //这个字节可能是0x7F或者0xFD等，暂时还没找到什么规律。但是正因为串入了这个字节，因此mcu接收到的第一个字节就不是我们发送的用来握手的0x7F，
                    //这样后续的整个指令将会完全乱掉，因此我们额外添加了一个字节去处理，假如GetID操作失败的话，很有可能就是因为发送的指令乱掉了，那么我们手动
                    //加入一个字节来补全，并尝试重试3次。
                    let data = [0x7F as u8];
                    self.handle.write(&data).unwrap();
                    std::thread::sleep(Duration::from_millis(5));
                    // 取出串口缓冲区的数据
                    self.handle.clear(serialport::ClearBuffer::All).unwrap();
                }
            }
        }

        Ok(())
    }

}
