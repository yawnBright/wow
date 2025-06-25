use crate::config::Config;
use std::{
    fs::{self, File},
    io::Write,
    process::Command,
    time::{Duration, SystemTime},
};
use termion::color;
use tokio::time::sleep;

pub struct Wow {
    args: Vec<String>,
    config: Config,
    working_space: String,
}

impl Wow {
    pub fn new() -> Self {
        Wow {
            args: vec![],
            config: Config::default(),
            working_space: String::new(),
        }
    }

    pub async fn run(&mut self) {
        let err = self.init_workspace();
        if !err.is_empty() {
            err.print_err();
            return;
        }

        let load_err = self.load_config();
        if !load_err.is_empty() {
            load_err.print_err();
        }

        let args: Vec<String> = std::env::args().collect();
        self.args = args;
        match self.args.get(1) {
            Some(a) => match a.as_str() {
                "help" => {
                    self.print_help();
                }
                "run" => {
                    self._run().await;
                }
                "stop" => {
                    self._stop();
                }
                "update" => {
                    self.try_update(true).await.print_err();
                }
                "freq" => {
                    self.set_update_frequance().print_err();
                }
                "from" => {
                    self.set_img_souce().print_err();
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
                self.try_update(false).await.print_err();
            }
        }
    }

    async fn _run(&mut self) {
        let config_path = self.working_space.clone() + "/wow.conf";
        self.config.load(&config_path);
        if self.config.working {
            println!("wow已在运行中");
            return;
        }

        self.config.ask_stop = false;
        self.config.working = true;
        let config_path = self.working_space.clone() + "/wow.conf";
        let mut err = self.config.flush(&config_path);
        if !err.is_empty() {
            err.print_err();
            self.config = Config::default();
            self.config.flush(&config_path);
            return;
        }

        loop {
            self.try_update(false).await.print_err();

            sleep(Duration::from_secs(30)).await;

            err = self.load_config();
            if !err.is_empty() {
                err.print_err();
                self.config = Config::default();
                self.config.flush(&config_path);
                return;
            }
            if self.config.ask_stop {
                println!("退出");
                break;
            }
        }
    }

    pub fn _stop(&mut self) {
        let config_path = self.working_space.clone() + "/wow.conf";
        self.config.ask_stop = true;
        self.config.working = false;
        let err = self.config.flush(&config_path);
        if !err.is_empty() {
            err.print_err();
            self.config = Config::default();
            self.config.flush(&config_path);
        }
    }

    fn init_workspace(&mut self) -> ErrInfo {
        match std::env::current_exe() {
            Err(e) => ErrInfo::new(&format!("can't access current working space:\n{}", e)),
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
                ErrInfo::empty()
            }
        }
    }

    fn load_config(&mut self) -> ErrInfo {
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
        println!("{}命令列表", color::Fg(color::LightGreen),);
        println!(
            "  {}help{}    - 显示此帮助信息",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}run{}     - 开启自动更新",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}stop{}    - 关闭自动更新",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}update{}  - 更新壁纸",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}freq{}    - 设置壁纸更新频率",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}from{}    - 选择壁纸图片来源",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}bye{}     - 卸载程序",
            color::Fg(color::Yellow),
            color::Fg(color::LightMagenta)
        );
        println!(
            "  {}tip{}     - 赞赏",
            color::Fg(color::Yellow),
            color::Fg(color::Rgb(53, 92, 125))
        );
        println!();

        println!(
            "{}当前配置{}",
            color::Fg(color::LightGreen),
            color::Fg(color::Reset)
        );
        println!(
            "  图片来源: {}{}{}",
            color::Fg(color::LightCyan),
            match self.config.get_url() {
                "https://bing.img.run/rand_uhd.php" => "必应随机历史图片",
                "https://bing.img.run/uhd.php" => "必应每日图片",
                _ => "未知来源｜unknow",
            },
            color::Fg(color::Reset)
        );
        println!(
            "  更新频率: {}每{}小时{}分{}",
            color::Fg(color::LightCyan),
            self.config.get_freq() / 3600,
            (self.config.get_freq() % 3600) / 60,
            color::Fg(color::Reset)
        );
        println!("{}", color::Fg(color::Reset));
    }

    fn set_update_frequance(&mut self) -> ErrInfo {
        let mut print_info = || {
            let time_now = SystemTime::now();
            let time_updated = self.config.get_update_at();
            match SystemTime::now().duration_since(time_updated) {
                Ok(d) => {
                    let freq_in_config = self.config.get_freq(); // 秒

                    let dif = freq_in_config as isize - d.as_secs() as isize;
                    let mut left_secs = freq_in_config - d.as_secs() as usize;
                    if dif < 0 {
                        left_secs = 0;
                    }

                    let hour = left_secs / 3600;
                    let left_secs = left_secs % 3600;
                    let min = left_secs / 60;
                    let sec = left_secs % 60;
                    println!(
                        "当前更新频率:  {}每{}小时{}",
                        color::Fg(color::LightBlue),
                        self.config.get_freq() as f32 / 3600.0,
                        color::Fg(color::Reset)
                    );
                    println!(
                        "下次更新在 {}{}小时{}分钟{}秒 {}后",
                        color::Fg(color::LightBlue),
                        hour,
                        min,
                        sec,
                        color::Fg(color::Reset)
                    );
                    if dif < 0 {
                        println!(":( 错过了更新\n将在下次更新时再次尝试");
                    }
                    println!("使用`wow update`手动更新");
                    ErrInfo::empty()
                }
                Err(e) => {
                    self.config.set_update_at(time_now);
                    self.config
                        .flush(&(self.working_space.clone() + "/wow.conf"));
                    ErrInfo::new(&format!(
                        "{}\n{}已重置时间{}",
                        e,
                        color::Fg(color::LightRed),
                        color::Fg(color::Reset)
                    ))
                }
            }
        };
        let mut print_help = || {
            println!(
                "{}设置壁纸更新频率{}",
                color::Fg(color::LightGreen),
                color::Fg(color::Reset),
            );
            println!(
                "{}usage:{} wow freq x    --  x > 0",
                color::Fg(color::LightRed),
                color::Fg(color::Reset)
            );
            println!(
                "{}eg:{}    wow freq 2.5  --  设置更新频率为每2.5小时",
                color::Fg(color::LightRed),
                color::Fg(color::Reset)
            );

            println!();

            print_info()
        };
        match self.args.get(2) {
            Some(a) => {
                if self.args.get(3).is_some() {
                    print_help()
                } else {
                    let freq = match a.parse::<f32>() {
                        Ok(f) => f,
                        Err(e) => {
                            println!("{}", e);
                            return print_help();
                        }
                    };
                    // FIXME: 限制范围
                    if freq > 0.0 {
                        let err = self._set_update_frequance(freq);
                        if err.is_empty() {
                            println!("设置成功");
                        }
                        err
                    } else {
                        print_help()
                    }
                }
            }
            None => print_help(),
        }
    }

    fn _set_update_frequance(&mut self, f: f32) -> ErrInfo {
        let f = f * 3600.0;
        self.config.set_freq(f as usize);
        self.config
            .flush(&(self.working_space.clone() + "/wow.conf"))
    }

    fn set_img_souce(&mut self) -> ErrInfo {
        let print_help = || {
            println!(
                "{}设置壁纸图片来源{}",
                color::Fg(color::LightGreen),
                color::Fg(color::Reset),
            );
            println!(
                "{}usage:{} wow from x    --  x = 1 or 2",
                color::Fg(color::LightRed),
                color::Fg(color::Reset)
            );
            println!("1  -> 必应随机历史图片");
            println!("2  -> 必应每日图片");
        };
        match self.args.get(2) {
            Some(s) => {
                if self.args.get(3).is_some() {
                    print_help();
                    ErrInfo::empty()
                } else {
                    let s = match s.parse::<u8>() {
                        Ok(s) => s,
                        Err(e) => {
                            // println!("{}", e);
                            print_help();
                            return ErrInfo::new(&format!("{}", e));
                        }
                    };
                    self.config.set_url(s);
                    self.config
                        .flush(&(self.working_space.clone() + "/wow.conf"))
                }
            }
            None => {
                print_help();
                ErrInfo::empty()
            }
        }
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
        println!("TODO");
        println!("请手动删除{}", self.working_space);
    }

    async fn try_update(&mut self, anyway: bool) -> ErrInfo {
        println!("准备更新");
        // 读取配置
        // 检查更新时间
        let time_now = SystemTime::now();
        let time_update = self.config.get_update_at();
        match time_now.duration_since(time_update) {
            Ok(d) => {
                if anyway || d >= Duration::from_secs(self.config.get_freq() as u64) {
                    println!("更新中...");
                    let res = self.update_paper(time_now).await;
                    if res.is_empty() {
                        // sync_update_time(time_now);
                        self.config.set_update_at(time_now);
                        self.config
                            .flush(&(self.working_space.clone() + "/wow.conf"));
                        return ErrInfo::new("壁纸已更新");
                    }
                    res
                } else {
                    ErrInfo::new("未到更新时间\n使用`wow update`手动更新\n使用`wow help`获取帮助")
                }
            }
            Err(e) => {
                self.config.set_update_at(time_now);
                self.config
                    .flush(&(self.working_space.clone() + "/wow.conf"));
                ErrInfo::new(&format!("{}\n已重置时间", e))
            }
        }
    }

    /// 根据配置的图片源，尝试更新图片
    /// 返回失败的原因
    ///
    /// `当前支持`
    /// - 必应每日图片
    /// - 必应随机历史图片
    async fn update_paper(&mut self, t: SystemTime) -> ErrInfo {
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

        let stamp = t
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(114514))
            .as_secs()
            .to_string();

        let save_path = self.working_space.clone() + "/" + &stamp + ".jpg";
        match client.get(real_url).send().await {
            Err(e) => ErrInfo::new(&format!("fetch image error:\n{}", e)),
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
                    Err(e) => ErrInfo::new(&format!("error when read image data:\n{}", e)),
                    Ok(data) => match fs.write_all(&data) {
                        Ok(_) => {
                            self.delete_pre_img();
                            self.config.set_cur_img(&save_path);
                            match Command::new(self.working_space.clone() + "/updater")
                                .arg(save_path)
                                .status()
                            {
                                Ok(_) => ErrInfo::empty(),
                                Err(e) => ErrInfo::new(&format!("{}\n设置壁纸失败", e)),
                            }
                        }
                        Err(e) => ErrInfo::new(&format!("error when saving image:\n{}", e)),
                    },
                }
            }
        }
    }

    fn delete_pre_img(&self) {
        if fs::remove_file(self.config.get_cur_img()).is_err() {}
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
