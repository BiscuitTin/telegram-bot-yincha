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

use directories::ProjectDirs;
use std::{fs, io::Write, path::Path};
use teloxide::{
    prelude::{AutoSend, Bot},
    requests::Requester,
    types::{ChatMember, Me},
    RequestError,
};

pub async fn display_bot_info(bot: &AutoSend<Bot>) {
    let me: Result<Me, RequestError> = bot.get_me().await;
    let me = match me {
        Ok(me) => me,
        Err(ref error) => {
            log::error!("AppError::api: API error, details: {:?}", error);
            me.unwrap()
        }
    };
    log::info!("Bot id:                 {}", me.user.id);
    log::info!("Bot name:               {}", me.user.first_name);
    log::info!("Bot username:           {}", me.user.username.unwrap());
    log::info!("Bot can join groups:    {}", me.can_join_groups);
}

#[allow(dead_code)]
pub(crate) async fn check_sender_is_admin(bot: &AutoSend<Bot>, chat_id: i64) -> bool {
    let chat_admin: Result<Vec<ChatMember>, RequestError> =
        bot.get_chat_administrators(chat_id).await;
    let chat_admin = match chat_admin {
        Ok(admin) => admin,
        Err(ref error) => {
            log::error!("AppError::api: API error, details: {:?}", error);
            chat_admin.unwrap()
        }
    };
    println!("{}", chat_admin.first().unwrap().user.id);
    return false;
}

pub(crate) fn get_setting_dir() -> ProjectDirs {
    let dirs =
        ProjectDirs::from("org", "BiscuitTin", "YinChaBot").expect("AppError::utils::directories");
    return dirs;
}

pub(crate) fn check_exists_and_create(p: &Path, buf: &[u8]) -> bool {
    if p.exists() {
        return true;
    }
    let name = p.file_name().expect("App::utils::create");
    let dir = String::from(p.clone().to_str().unwrap()).replace(name.clone().to_str().unwrap(), "");
    let dir_p = Path::new(&dir);
    if !dir_p.exists() {
        log::trace!("App::utils::create dir {}", dir);
        let _d = fs::create_dir_all(dir_p); // If dir not exist, create first.
    }
    log::trace!("App::utils::create file: {}", p.display());
    // File also create first.
    let mut _f = fs::File::create(p).expect("AppError::utils::create");
    _f.write_all(buf).expect("AppError::utils::writeAll");
    _f.sync_all().expect("AppError::utils::syncAll");
    return false;
}
