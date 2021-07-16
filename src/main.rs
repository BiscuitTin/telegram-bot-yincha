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

use dotenv::dotenv;
use std::env;
use teloxide::{prelude::Bot, requests::RequesterExt};
use yinchabot::{handler::message_handler, listener::polling_listener, utils::display_bot_info};

#[tokio::main]
async fn main() {
    dotenv().expect("AppError::env");
    yinchabot::enable_logging!();
    log::trace!("Environment set, app starting...");
    run_bot().await;
}

async fn run_bot() {
    log::trace!("Bot starting...");
    let bot = Bot::from_env();
    log::trace!("Bot created! Current api url: {}.", bot.api_url().as_str());
    let bot_inst = bot.auto_send();
    log::trace!("Bot auto send enabled!");

    display_bot_info(&bot_inst).await;

    let listener = polling_listener(bot_inst.clone());

    teloxide::repl_with_listener(bot_inst.clone(), message_handler, listener).await;
}
