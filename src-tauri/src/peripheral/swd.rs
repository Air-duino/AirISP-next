use std::error::Error;

use colored::{Color, Colorize};
use probe_rs::flashing::DownloadOptions;
use probe_rs::{flashing, Lister, MemoryInterface, Permissions, Session};
use rust_i18n::t;

use crate::log::LOG;
use crate::peripheral;
use crate::peripheral::Pp;
use crate::AirISP;

use crate::peripheral::{chip_info, ChipFamily, ChipInfo, CHIPS};

pub struct Swd<'a> {
    air_isp: &'a AirISP::AirISP,
    info: ChipInfo,
}

impl Swd<'_> {
    pub fn new(air_isp: &AirISP::AirISP) -> Swd {
        let mut swd =
        Swd {
            air_isp,
            info: ChipInfo { // 用于初始化，在后面会被修改
                name: "",
                debug_idcode_reg: 0,
                pid: 0,
                flash_size_reg: 0,
                flash_size: 0,
                ram_size: 0,
                uid_reg: 0,
            },
        };
        match swd.get_chip_info() {
            Ok(chip_info) => {
                swd.info = chip_info.clone();
            }
            Err(_) => {
                std::process::exit(AirISP::ExitCode::NoMatchChip as i32);
            }
        }
        swd
    }

    fn get_chip_session_name(&mut self, chip_name: &str) -> Result<Session, Box<dyn Error>> {
        let session;
        let chip_name = chip_name.to_lowercase();
        let mut speed = self.air_isp.get_baud();
        if speed == 0 {
            speed = 200; // 默认速度200k
        }

        // session = Session::auto_attach(chip_name, Permissions::default())?;
        if self.air_isp.get_port() == "auto" {
            session = Session::auto_attach(chip_name, Permissions::default())?;
            return Ok(session);
        } else {
            let lister = Lister::new();
            let probe_list = lister.list_all();
            for i in probe_list {
                // 输入的端口名称和扫描到的名称子串匹配
                if i.identifier
                    .to_lowercase()
                    .contains(self.air_isp.get_port().as_str())
                {
                    let mut probe = i.open(&lister)?;
                    probe.set_speed(speed)?;
                    session = probe.attach(chip_name, Permissions::default())?;
                    return Ok(session);
                }
            }
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "get probe fail",
        )))
    }

    fn get_chip_session(&mut self) -> Result<Session, Box<dyn Error>> {
        let chip_name = self.info.name;
        self.get_chip_session_name(chip_name)
    }
}

impl chip_info for Swd<'_> {
    fn get_chip_info(&mut self) -> Result<&peripheral::ChipInfo, Box<dyn Error>> {
        // 自动判断芯片型号
        if self.air_isp.get_chip() == "auto" {
            for chip in CHIPS.iter() {
                for i in chip.info.iter() {
                    // 0xFFFFFFFF 证明暂且未知，因此假设就是这个芯片，不进行进一步的判断
                    return if i.debug_idcode_reg == 0xFFFFFFFF {
                        Ok(i)
                    } else {
                        let mut session =
                            Session::auto_attach("cortex-m0", Permissions::default())?; // 默认使用m0去连接，一般可以连上
                        let mut core = session.core(0)?;
                        let pid = core.read_word_32(i.debug_idcode_reg as u64)?;
                        if pid != i.pid as u32 {
                            continue;
                        }
                        Ok(i)
                    };
                }
            }

        }
        //if self.air_isp.get_chip() == "auto"
        else
        // 有具体的型号
        {
            let chip_name = self.air_isp.get_chip();
            for chip in CHIPS.iter() {
                for i in chip.info.iter() {
                    // 全部转换成小写，然后进行匹配
                    if i.name.to_lowercase() == chip_name.to_lowercase() {
                        if i.debug_idcode_reg == 0xFFFFFFFF {
                            LOG.warn(
                                t!("swd_pid_not_match_unknown_no_pid_help").as_str(),
                            );
                            return Ok(i);
                        }

                        let mut session = self.get_chip_session_name(chip_name.as_str())?;
                        let mut core = session.core(0)?;
                        let pid = match core.read_word_32(i.debug_idcode_reg as u64) {
                            Ok(pid) => pid,
                            Err(_) => {
                                LOG.warn(
                                    t!("swd_read_debug_idcode_fail_help",
                                        "addr" => format!("{:#010x}", i.debug_idcode_reg as u32)
                                    )
                                    .as_str(),
                                );
                                continue;
                            }
                        };
                        if pid != i.pid as u32 {
                            LOG.warn(t!("swd_pid_not_match_help",
                                "set_pid" => format!("{:#04x} {:#04x}", (i.pid >> 8) & 0xFF, i.pid & 0xFF),
                                "read_pid" => format!("{:#04x} {:#04x}", (pid >> 8) & 0xFF, pid & 0xFF),
                            ).as_str());
                        }
                        return Ok(i);
                    }
                }
            }
        };

        // 都没有找到，返回错误
        LOG.error(
            t!("swd_pid_not_match_unknown_help").as_str()
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "no match chip",
        )));
    }
    fn get_chip_pid(&mut self) -> Result<u32, Box<dyn Error>> {
        let print_pid = |pid: u16| {
            LOG.info(
                t!("get_chip_success_help",
                    "chip_id" => format!("{:#04x} {:#04x}", (pid >> 8) & 0xFF, pid & 0xFF),
                )
                .as_str(),
                Color::BrightBlue,
            );
        };
        let pid = self.get_chip_info()?.pid;
        print_pid(pid);
        Ok(pid as u32)
    }
}

impl Pp for Swd<'_> {
    fn write_flash(
        &mut self,
        address: u32,
        data: &[u8],
        progress: AirISP::Progress,
    ) -> Result<(), Box<dyn Error>> {
        let mut session = self.get_chip_session()?;
        let mut loader = session.target().flash_loader();

        LOG.info(t!("write_flash_file_help").as_str(), Color::BrightBlue);
        let now_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        loader.add_data(address as u64, data)?;
        loader.commit(&mut session, DownloadOptions::default())?;

        LOG.info(t!("write_flash_success_help",
                    "time" => format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() - now_time),
                    "addr" => format!("{:#010x}", address as u32),
                    "size" => format!("{}", data.len())
                ).as_str(), Color::Green);
        Ok(())
    }
    fn reset_bootloader(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn get_chip_id(&mut self) -> Result<(), Box<dyn Error>> {
        self.get_chip_pid()?;
        Ok(())
    }

    fn erase_all(&mut self) -> Result<(), Box<dyn Error>> {
        println!("{}", format!("{}", t!("erase_all_help")).bright_blue());
        let now_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut session = self.get_chip_session()?;
        flashing::erase_all(&mut session, None)?;

        LOG.info(t!("erase_all_success_help",
                    "time" => format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() - now_time)
                ).as_str(), Color::Green);
        Ok(())
    }
    fn reset_app(&mut self) -> Result<(), Box<dyn Error>> {
        LOG.info(t!("leaving_help").as_str(), Color::Blue);
        let mut session = self.get_chip_session()?;
        session.core(0)?.reset()?;
        Ok(())
    }
}
