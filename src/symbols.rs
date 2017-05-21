use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::path::{Path, PathBuf};

pub struct Symbols {
    symbol_map: HashMap<u16, String>,
    file_path: PathBuf,
    can_save: bool,
}

impl Default for Symbols {
    fn default() -> Self {
        Symbols {
            symbol_map: HashMap::new(),
            file_path: PathBuf::new(),
            can_save: false,
        }
    }
}

impl Symbols {
    pub fn new<P: AsRef<Path>>(file_name: P) -> Self {
        Symbols {
            symbol_map: HashMap::new(),
            file_path: file_name.as_ref().to_path_buf(),
            can_save: true,
        }
    }

    pub fn load<P: AsRef<Path>>(file_name: P) -> io::Result<Symbols> {
        let file = File::open(file_name.as_ref().clone());
        let file = match file {
            Ok(f) => f,
            Err(_) => return Ok(Self::new(file_name)),
        };

        let reader = BufReader::new(&file);

        let mut symbol_map = HashMap::new();
        let mut in_symbols = false;

        for line in reader.lines() {
            let line = line?;
            if line.trim() == "[labels]" {
                in_symbols = true;
                continue;
            }

            if in_symbols {
                let mut parts = line.trim().split_whitespace();
                let mut addr_parts = parts.nth(0).unwrap().split(':');

                let (bank, addr) = (addr_parts.nth(0).unwrap(), addr_parts.nth(0).unwrap());

                // TODO - take into account rom/ram banking
                #[allow(unused_variables)]
                let bank = bank.parse::<u8>().unwrap();
                let addr = addr.parse::<u16>().unwrap();

                if symbol_map.contains_key(&addr) {
                    panic!("Duplicate addresses for symbols: {}", addr);
                }
                symbol_map.insert(addr, parts.nth(0).unwrap().to_owned());
            }
        }



        Ok(Symbols {
            symbol_map: symbol_map,
            file_path: file_name.as_ref().to_path_buf(),
            can_save: true,
        })
    }

    pub fn save(&self) -> io::Result<()> {
        if !self.can_save {
            return Ok(());
        }

        let mut file = File::create(&self.file_path)?;

        write!(file, "[labels]\n")?;

        for (k, v) in &self.symbol_map {
            write!(file, "00:{} {}\n", k, v)?;
        }

        Ok(())
    }

    pub fn get(&self, addr: u16) -> Option<&str> {
        match self.symbol_map.get(&addr) {
            Some(s) => Some(s),
            None => None,
        }
    }

    pub fn reverse_get(&self, symbol: &str) -> Option<u16> {
        for (k, v) in &self.symbol_map {
            if v == symbol {
                return Some(*k);
            }
        }

        None
    }

    pub fn insert(&mut self, addr: u16, symbol: &str) {
        if self.symbol_map.contains_key(&addr) {
            // TODO - better error handling
            panic!("Duplicate symbol for addr: {}", addr);
        }

        self.symbol_map.insert(addr, symbol.to_owned());
    }

    pub fn remove(&mut self, addr: u16) {
        self.symbol_map.remove(&addr);
    }
}
