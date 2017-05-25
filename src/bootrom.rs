extern crate app_dirs;

use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::env;
use config::app_info::APP_INFO;
use config::model::{Model, DEFAULT_MODEL_PRIORITY};
use self::app_dirs::{AppDataType, get_app_dir};

pub struct Bootrom {
    data: Box<[u8]>,
}

impl Bootrom {
    pub fn load(file_name: &Path) -> io::Result<Bootrom> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(Bootrom::from_bytes(&buffer))
    }

    pub fn from_bytes(bytes: &[u8]) -> Bootrom {
        Bootrom {
            data: bytes.to_vec().into_boxed_slice(),
        }
    }

    pub fn lookup(models: &[Model]) -> Self {
        let mut cands = Vec::new();
        let models = if models.is_empty() { &DEFAULT_MODEL_PRIORITY } else { models };

        if let Ok(dir) = get_app_dir(AppDataType::UserData, &APP_INFO, "bootroms") {
            for model in models {
                cands.push((model, dir.join(model.bootrom_name())));
            }
        }

        if let Ok(cwd) = env::current_dir() {
            for model in models {
                cands.push((model, cwd.join(model.bootrom_name())));
            }
        }

        for (model, path) in cands {
            let path_str = path.to_string_lossy();
            println!("Scanning {} for a boot ROM", path_str);
            match Bootrom::load(&path) {
                Err(e) => println!("Warning: Boot rom \"{}\" ({})", path_str, e),
                Ok(bootrom) => {
                    println!("Using {} boot ROM from {}", model, path_str);
                    return bootrom;
                }
            }
        }

        panic!("No valid bootroms found");
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }
}
