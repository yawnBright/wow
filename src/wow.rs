use crate::config::Config;
use std::{
    fs::File,
    io::Write,
    time::{Duration, SystemTime},
};
use termion::color;

pub struct Wow {
    config: Config,
    working_space: String,
}

impl Wow {
    pub fn new() -> Self {
        Wow {
            config: Config::default(),
            working_space: String::new(),
        }
    }

    pub async fn run(&mut self) {
        let load_err = self.load_config();
        if !load_err.is_empty() {
            load_err.print_err();
            return;
        }

        let args: Vec<String> = std::env::args().collect();
        match args.get(1) {
            Some(a) => match a.as_str() {
                "help" => {
                    self.print_help();
                }
                "freq" => {
                    self.set_update_frequance();
                }
                "from" => {
                    self.set_img_souce();
                }
                "tip" => {
                    self.show_tip_code();
                }
                "bye" => {
                    self.self_remove();
                }
                _ => {
                    self.print_help();
                }
            },
            None => {
                self.try_update().await.print_err();
            }
        }
    }

    fn load_config(&mut self) -> ErrInfo {
        match std::env::current_exe() {
            Err(e) => {
                return ErrInfo {
                    info: format!("can't access current working space:\n{}", e),
                };
            }
            Ok(e) => {
                let w_space = match e.parent() {
                    Some(p) => match p.to_str() {
                        Some(w) => w.to_string(),
                        None => {
                            return ErrInfo {
                                info: "can't access current working space".to_string(),
                            };
                        }
                    },
                    None => {
                        return ErrInfo {
                            info: "can't access current working space".to_string(),
                        };
                    }
                };
                self.working_space = w_space;
            }
        }
        let mut _config = Config::default();
        let conf_path = self.working_space.clone() + "/wow.conf";
        if _config.load(&conf_path) {
            self.config = _config;
        } else {
            // 如果加载配置失败，
            // 则重置配置
            return self.config.flush(&conf_path);
        }
        ErrInfo::empty()
    }

    fn print_help(&self) {
        println!("{}命令列表/command list", color::Fg(color::LightGreen),);
        println!(
            "  {}help{}  - 显示此帮助信息｜show help info",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}freq{}  - 设置壁纸更新频率｜set update frequency",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}from{}  - 设置壁纸图片来源｜set the source of wallpaper",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}bye{}   - 卸载程序｜safely delete this app",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}tip{}   - 打赏｜encourage",
            color::Fg(color::Yellow),
            color::Fg(color::Rgb(149, 225, 221))
        );
        println!();

        println!(
            "{}当前配置｜current config{}",
            color::Fg(color::LightGreen),
            color::Fg(color::Reset)
        );
        println!(
            "  图片来源(image source): {}{}{}",
            color::Fg(color::LightCyan),
            match self.config.get_url() {
                "https://bing.img.run/rand_uhd.php" => "    必应随机历史图片｜bing random image",
                "https://bing.img.run/uhd.php" => "必应每日图片｜bing every-day image",
                _ => "未知来源｜unknow",
            },
            color::Fg(color::Reset)
        );
        println!(
            "  更新频率(update frequency): {}每{}小时｜every {} hours{}",
            color::Fg(color::LightCyan),
            self.config.get_freq(),
            self.config.get_freq(),
            color::Fg(color::Reset)
        );
        println!("{}", color::Fg(color::Reset));
    }

    fn set_update_frequance(&self) -> ErrInfo {
        println!("set freq");
        ErrInfo::empty()
    }

    fn set_img_souce(&self) -> ErrInfo {
        println!("set source");
        ErrInfo::empty()
    }

    fn show_tip_code(&self) {
        let colors = [
            [246, 114, 128],
            [53, 92, 125],
            [108, 91, 123],
            [240, 138, 93],
            [252, 186, 211],
            [224, 249, 181],
        ];
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let index = (now % colors.len() as u64) as usize;
        // let mut rng = rand::thread_rng();
        // let index = rng.gen_range(0..colors.len());
        print!(
            "{}
         █▀▀▀▀▀▀▀█▀████▀█▀▀█▀▀██▀███▀▀▀▀▀▀▀█
         █ █▀▀▀█ █ ▀█▄ █ ▀  █▄  ██▀█ █▀▀▀█ █
         █ █   █ █▄  █▀▀▄ ▀▀  █ █▀██ █   █ █
         █ ▀▀▀▀▀ █ ▄ █▀▄ █ █ ▄ ▄ █▀█ ▀▀▀▀▀ █
         █▀▀██▀▀▀███ ██▀█▀█▄▄ ▀ █▄▀██▀█▀▀▀▀█
         █ ▄█▄▄▀▀█▀▀▀ ▀▀ ▀▄▄ ▄██ ▀▀█▀█▀▄▀▄██
         █▀▀▀ ▄▄▀▄▄▄█▀▀█▀█▀ ▄▄▀    ▄▀  ▄█▀▄█
         █▄██ ▄ ▀▀▄▀▄ █▄▀ ▄█ ██ █▀█▀██▀▄▀███
         █▀▄▄  ▀▀█▀█▀▄ █▀ █  ▄█ ▀█  ▀█▄██▀▀█
         █ █ ██▄▀▄██▄ ███ ██▀█▀▀ ▀██▀▄█  ▀██
         █▄▀▄█▄▀▀▄▄█▄▄████▀▄██   ▄██▀██ █▀██
         █▄▄▄▄  ▀█▀█▄▀█▀ ▀▄█▀██ ▀ ▄▀█ █▀ ▀██
         █▀▀▀▀██▀▀█▄▄█▀█▄     ▀▄▀█▀▀▀ ▀██▄ █
         █▀▀▀▀▀▀▀█▄▄▄▀███ ███▄██ ▄ █▀█   ▀██
         █ █▀▀▀█ █  ▄▄ ▀▄██ ▀ ▄ ▀█ ▀▀▀  █▀▄█
         █ █   █ ██▄  ▀██▀▄▀███ █  ▀▄▄▀▀▄ ▄█
         █ ▀▀▀▀▀ █   ▀▀██▄▀ ▄▀▄ ▄▀▀▀█▀█▀██▀█
         ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
{}",
            color::Fg(color::Rgb(
                colors[index][0],
                colors[index][1],
                colors[index][2]
            )),
            color::Fg(color::Reset)
        );
    }

    fn self_remove(&self) {
        println!("byyyyyyye");
    }

    async fn try_update(&mut self) -> ErrInfo {
        // 读取配置
        //
        // 检查更新时间
        let time_now = SystemTime::now();
        let time_update = self.config.get_update_at();
        match time_now.duration_since(time_update) {
            Ok(d) => {
                if d >= Duration::from_secs(self.config.get_freq() as u64 * 60 * 60) {
                    let res = self.update_paper().await;
                    if res.is_empty() {
                        self.config.set_update_at(time_now);
                        self.config
                            .flush(&(self.working_space.clone() + "/wow.conf"));
                    }
                    return res;
                } else {
                    println!("not the time");
                }
            }
            Err(e) => {
                return ErrInfo::new(&format!("{}", e));
            }
        }
        // 是否需要更新
        //
        ErrInfo::empty()
    }

    /// 根据配置的图片源，尝试更新图片
    /// 返回失败的原因
    ///
    /// `当前支持`
    /// - 必应每日图片
    /// - 必应随机历史图片
    async fn update_paper(&self) -> ErrInfo {
        let client = reqwest::Client::new();

        let img_url = self.config.get_url();
        let real_url = match client.get(img_url).send().await {
            Err(e) => {
                return ErrInfo {
                    info: format!("fetch image error:\n{}", e),
                };
            }
            Ok(resp) => resp.url().clone().to_string(),
        };

        let save_path = self.working_space.clone() + "/today.jpg";
        match client.get(real_url).send().await {
            Err(e) => {
                return ErrInfo {
                    info: format!("fetch image error:\n{}", e),
                };
            }
            Ok(resp) => {
                let mut fs = match File::create(&save_path) {
                    Ok(f) => f,
                    Err(e) => {
                        return ErrInfo {
                            info: format!("can't open the file to save image:\n{}", e),
                        };
                    }
                };
                match resp.bytes().await {
                    Err(e) => {
                        return ErrInfo {
                            info: format!("error when read image data:\n{}", e),
                        };
                    }
                    Ok(data) => match fs.write_all(&data) {
                        Ok(_) => {}
                        Err(e) => {
                            return ErrInfo {
                                info: format!("error when saving image:\n{}", e),
                            };
                        }
                    },
                }
            }
        }
        ErrInfo::empty()
    }
}

pub struct ErrInfo {
    info: String,
}
impl ErrInfo {
    pub fn empty() -> Self {
        ErrInfo {
            info: String::new(),
        }
    }
    pub fn new(info: &str) -> Self {
        Self {
            info: info.to_string(),
        }
    }
    fn print_err(&self) {
        if self.info.is_empty() {
            return;
        }
        println!("{}", self.info);
    }
    fn is_empty(&self) -> bool {
        self.info.is_empty()
    }
}
