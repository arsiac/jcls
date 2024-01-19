use std::io::Error;
use std::path::Path;
use std::{collections::VecDeque, fs};

use log::{debug, info};

pub trait Sweeper {
    /// 清理路径
    fn clean(&self, path: &Path) -> Result<(), Error>;
}

pub struct SweeperChain {
    sweepers: Vec<Box<dyn Sweeper>>,
}

impl SweeperChain {
    pub fn new() -> Self {
        Self {
            sweepers: Vec::new(),
        }
    }

    pub fn add_sweeper(&mut self, sweeper: Box<dyn Sweeper>) {
        self.sweepers.push(sweeper);
    }

    pub fn clean(&self, path: &Path) -> Result<(), Error> {
        for sweeper in &self.sweepers {
            sweeper.clean(path)?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct MavenSweeper {}

impl MavenSweeper {
    /// 是否是 Maven Module
    fn is_maven_module(path: &Path) -> bool {
        let pom = path.join("pom.xml");
        if !pom.exists() {
            debug!("Not a Maven module: {}", path.display());
            return false;
        }

        true
    }

    /// 清理 Maven Module
    fn clean_maven_module(path: &Path) -> Result<(), Error> {
        let target = path.join("target");
        if !target.exists() || target.is_file() {
            debug!("Maven module is clean: {}", path.display());
            return Ok(());
        }

        let target = target.as_path();
        info!("Remove folder '{}'", target.display());
        fs::remove_dir_all(target)
    }
}

impl Sweeper for MavenSweeper {
    fn clean(&self, path: &Path) -> Result<(), Error> {
        if Self::is_maven_module(path) {
            Self::clean_maven_module(path)?;
        }

        let mut folder_stack = VecDeque::new();
        folder_stack.push_back(path.to_path_buf());
        while let Some(path) = folder_stack.pop_front() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        if Self::is_maven_module(&entry_path) {
                            Self::clean_maven_module(&entry_path)?;
                        }

                        let filename = entry.file_name();
                        let filename = filename.to_str().unwrap();
                        if filename != "src" && filename != ".idea" && filename != ".git" {
                            folder_stack.push_back(entry_path);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct IdeaSweeper {}

impl IdeaSweeper {}

impl Sweeper for IdeaSweeper {
    fn clean(&self, path: &Path) -> Result<(), Error> {
        if path.is_file() {
            debug!("Not a folder: {}", path.display());
            return Ok(());
        }

        let mut folder_stack = VecDeque::new();
        folder_stack.push_back(path.to_path_buf());
        while let Some(path) = folder_stack.pop_front() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        folder_stack.push_back(entry_path);
                    } else {
                        let filename = entry.file_name();
                        let filename = filename.to_str().unwrap();
                        if filename.ends_with(".iml") {
                            info!("Remove file '{}'", entry_path.display());
                            std::fs::remove_file(&entry_path)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
