use std::path::PathBuf;

use crate::cfg::Config;
use anyhow::Result;
use vipera::Configuration;

pub struct Mhw {
    cfg: Config,
}

impl Mhw {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    pub fn init(&mut self, exe_path: String) -> Result<()> {
        self.cfg.exe_path = Some(exe_path);
        self.cfg.write_config()?;
        Ok(())
    }

    pub fn list(&self) -> Result<()> {
        let root = self.cfg.mod_directory_path.parse::<PathBuf>()?;
        for (i, entry) in root.read_dir()?.enumerate() {
            let entry = entry?;
            let path = entry.path();
            println!("{}\t{:?}", i, path);
        }
        Ok(())
    }

    pub fn add(&self, path: String) -> Result<()> {
        let options = fs_extra::dir::CopyOptions::new().skip_exist(true);
        let root = &self.cfg.mod_directory_path;
        fs_extra::copy_items(&[path], root, &options)?;
        Ok(())
    }
}
