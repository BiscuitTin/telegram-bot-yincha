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

use crate::{
    media::{get_random_voice, get_voice_dir},
    settings::{Settings, Subscribe},
};
use chrono::Timelike;
use teloxide::{
    prelude::{AutoSend, UpdateWithCx},
    requests::{Requester, ResponseResult},
    types::{InputFile, Message},
    Bot,
};

pub async fn message_handler(msg: UpdateWithCx<AutoSend<Bot>, Message>) -> ResponseResult<()> {
    let chat = &msg.update.chat;
    let t_now = chrono::Utc::now();
    let t_hour = t_now.hour() + 8; // UTC+8
    let t_minute = t_now.minute();
    let t_second = t_now.second();
    if msg.update.id == 0 {
        log::trace!("Current time is: {}:{}:{}.", t_hour, t_minute, t_second);
        if t_hour == 15 && t_minute == 00 && t_second < 11 {
            voice_handler(&msg.requester, chat.id)
                .await
                .expect("AppError::sendVoice");
        }
    } else {
        let text = msg.update.text().unwrap();
        let from = msg.update.from().unwrap();
        // let is_admin = check_sender_is_admin(&msg.requester, chat.id).await; // TODO: Check sender is admin.

        if chat.is_group() && text.eq("/subscribe @yinchabot") {
            let mut set = Settings::new();
            set.add_sub(Subscribe::new(chat.id)).save();
            msg.reply_to::<&str>("Successful subscribe this group!")
                .await
                .expect("AppError::sendReply");
            log::info!(
                "Bot successful subscribe group id: {}, message sender: {}, user id: {}.",
                chat.id,
                from.first_name,
                from.id
            );
        }

        #[cfg(debug_assertions)]
        if text.contains("/test") {
            voice_handler(&msg.requester, chat.id)
                .await
                .expect("AppError::sendVoice");
        }
    }

    teloxide::respond(())
}

pub async fn voice_handler(bot: &AutoSend<Bot>, chat_id: i64) -> ResponseResult<()> {
    let voice_dir = get_voice_dir();
    let voice = get_random_voice(voice_dir);
    let voice_clone = voice.clone();
    bot.send_voice(chat_id, InputFile::file(voice))
        .await
        .expect("AppError::sendVoice");
    log::info!(
        "Bot successful send voice, chat id: {}, voice: {}.",
        chat_id,
        voice_clone.file_name().unwrap().to_str().unwrap()
    );
    teloxide::respond(())
}
