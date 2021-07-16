/*
 * Copyright 2021 Garfield Lee<opensource@550.moe>, Biscuit Tin
 *
 * The 3-Clause BSD License
 *
 * Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use crate::utils::*;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{
    fmt::{Display, Formatter},
    fs::{File, OpenOptions},
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscribe {
    pub chat_id: i64,
    pub timezone: String,
}

#[allow(dead_code)]
impl Subscribe {
    pub fn new(id: i64) -> Self {
        Subscribe {
            chat_id: id,
            timezone: String::from("UTC+8"),
        }
    }
    pub(crate) fn update(&mut self, id: i64, tz: &str) {
        self.chat_id = id;
        self.timezone = String::from(tz);
    }
    pub(crate) fn update_tz(&mut self, tz: &str) {
        self.timezone = String::from(tz);
    }
}

impl PartialEq for Subscribe {
    fn eq(&self, o: &Self) -> bool {
        self.chat_id == o.chat_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub subscribe: Vec<Subscribe>,
    file_path: String,
}

#[allow(dead_code)]
impl Settings {
    pub fn new() -> Self {
        let s = Self::make();
        s.save();
        s
    }
    pub(crate) fn save(&self) {
        let path = self.get_file_path().expect("AppError::Settings::save");
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .expect("AppError::Settings::save");
        serde_json::to_writer(file, &self).expect("AppError::Settings::save");
    }
    pub(crate) fn add_sub(&mut self, val: Subscribe) -> &Self {
        if !self.subscribe.contains(&val) {
            self.subscribe.push(val)
        }
        self
    }
    #[allow(unused_must_use)]
    pub(crate) fn get_file_path(&self) -> core::result::Result<&str, SettingError> {
        if self.file_path == "" {
            Err::<&str, SettingError>(SettingError);
        }
        Ok(self.file_path.as_str())
    }
    fn make() -> Self {
        let name = "Settings.json";
        let dirs = get_setting_dir();
        let dir = dirs.config_dir();
        let path = format!("{}/{}", dir.display(), name);
        let is_exists = check_exists_and_create(Path::new(&path.clone()), b"{}");

        if is_exists {
            let mut f = File::open(path).unwrap();
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).unwrap();
            let settings: Settings = serde_json::from_str(&buffer).unwrap();
            settings
        } else {
            Settings {
                subscribe: vec![],
                file_path: path.clone(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SettingError;

impl Display for SettingError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Setting file path does not exist, ensure call 'Settings::new()' first."
        )
    }
}
