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

use rand::{rngs::ThreadRng, seq::IteratorRandom};
use std::{
    env,
    fs::{read_dir, ReadDir},
    path::PathBuf,
};

#[allow(dead_code)]
pub(crate) fn get_voice_dir() -> ReadDir {
    let dir_path = env::var("BOT_VOICE_DIR");
    let dir_path = match dir_path {
        Ok(path) => path,
        Err(error) => {
            log::error!("AppError::env: BOT_VOICE_DIR not set!");
            panic!("AppError::env: BOT_VOICE_DIR not set! Details: {:?}", error);
        }
    };

    let dir = read_dir(dir_path);
    let dir = match dir {
        Ok(dir) => dir,
        Err(error) => {
            log::error!("AppError::io: Bot voice dir read error!");
            panic!(
                "AppError::io: Bot voice dir read error! Details: {:?}",
                error
            );
        }
    };
    return dir;
}

#[allow(dead_code)]
pub(crate) fn get_random_voice(dir: ReadDir) -> PathBuf {
    let mut rng = rand::thread_rng();

    let choose = dir.choose::<ThreadRng>(&mut rng).expect("AppError::io");
    let file = choose
        .expect("AppError::io: choose voice file error")
        .path();

    return file;
}
