use super::wow::ErrInfo;
use bincode::{Decode, Encode};
use std::fs::File;
use std::time::SystemTime;

const BING_PAPER_EVERYDAY_URL: &str = "https://bing.img.run/uhd.php";
const BING_PAPER_RANDOM_URL: &str = "https://bing.img.run/rand_uhd.php";

#[derive(Encode, Decode)]
pub struct Config {
    source: i8,
    freq: i8,
    update_at: SystemTime,
}

impl Config {
    pub fn default() -> Self {
        Config {
            source: 1,
            freq: 12,
            update_at: SystemTime::UNIX_EPOCH,
        }
    }
    /// 从`path`加载配置文件，
    /// 如果发生错误，
    /// 外层将重置配置
    pub fn load(&mut self, path: &str) -> bool {
        // let mut _config = Config::default();
        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(_) => {
                return false;
            }
        };
        match bincode::decode_from_slice::<Config, bincode::config::Configuration>(
            &data,
            bincode::config::standard(),
        ) {
            Ok((c, _)) => {
                *self = c;
                true
            }
            Err(_) => false,
        }
    }

    pub fn get_url(&self) -> &'static str {
        match self.source {
            1 => BING_PAPER_RANDOM_URL,
            2 => BING_PAPER_EVERYDAY_URL,
            _ => BING_PAPER_RANDOM_URL,
        }
    }

    pub fn get_freq(&self) -> i8 {
        self.freq
    }

    pub fn get_update_at(&self) -> SystemTime {
        self.update_at
    }
    pub fn set_update_at(&mut self, t: SystemTime) {
        self.update_at = t;
    }

    pub fn flush(&self, path: &str) -> ErrInfo {
        match File::create(path) {
            Ok(mut fs) => {
                match bincode::encode_into_std_write(self, &mut fs, bincode::config::standard()) {
                    Ok(_) => ErrInfo::empty(),
                    Err(e) => ErrInfo::new(&format!("{}", e)),
                }
            }
            Err(e) => ErrInfo::new(&format!("{}", e)),
        }
    }

    // pub fn get_img_save_path(&self) -> String {
    //     return self.img_save_path.clone();
    // }
}
