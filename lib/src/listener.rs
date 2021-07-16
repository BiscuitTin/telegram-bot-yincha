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

use crate::settings::Settings;
use futures::{
    future::{ready, Either},
    stream::{self, Stream, StreamExt},
};
use std::{convert::TryInto, time::Duration};
use teloxide::{
    dispatching::{
        stop_token::{AsyncStopFlag, AsyncStopToken},
        update_listeners::{StatefulListener, UpdateListener},
    },
    payloads::GetUpdates,
    requests::{HasPayload, Request, Requester},
    types::{
        AllowedUpdate, Chat, ChatKind, ChatPublic, ForwardKind, ForwardOrigin, MediaKind,
        MediaText, Message, MessageCommon, MessageKind, PublicChatGroup, PublicChatKind,
        SemiparsedVec, Update, UpdateKind,
    },
};

fn mock_update_chat(chat_id: i64) -> Update {
    Update {
        id: 0,
        kind: UpdateKind::Message(Message {
            id: 0,
            date: 0,
            chat: Chat {
                id: chat_id,
                kind: ChatKind::Public(ChatPublic {
                    title: None,
                    kind: PublicChatKind::Group(PublicChatGroup { permissions: None }),
                    description: None,
                    invite_link: None,
                }),
                photo: None,
                pinned_message: None,
                message_auto_delete_time: None,
            },
            via_bot: None,
            kind: MessageKind::Common(MessageCommon {
                from: None,
                sender_chat: None,
                author_signature: None,
                forward_kind: ForwardKind::Origin(ForwardOrigin {
                    reply_to_message: None,
                }),
                edit_date: None,
                media_kind: MediaKind::Text(MediaText {
                    text: "".to_string(),
                    entities: vec![],
                }),
                reply_markup: None,
            }),
        }),
    }
}

struct State<B: Requester> {
    bot: B,
    timeout: Option<u32>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
    offset: i32,
    flag: AsyncStopFlag,
    token: AsyncStopToken,
}

fn stream<B>(st: &mut State<B>) -> impl Stream<Item = Result<Update, B::Err>> + '_
where
    B: Requester,
{
    stream::unfold(st, move |state| async move {
        let State {
            timeout,
            limit,
            allowed_updates,
            bot,
            offset,
            flag,
            ..
        } = &mut *state;

        if flag.is_stopped() {
            let mut req = bot.get_updates_fault_tolerant();

            req.payload_mut().0 = GetUpdates {
                offset: Some(*offset),
                timeout: Some(0),
                limit: Some(1),
                allowed_updates: allowed_updates.take(),
            };

            return match req.send().await {
                Ok(_) => None,
                Err(err) => Some((Either::Left(stream::once(ready(Err(err)))), state)),
            };
        }

        let mut req = bot.get_updates_fault_tolerant();
        req.payload_mut().0 = GetUpdates {
            offset: Some(*offset),
            timeout: *timeout,
            limit: *limit,
            allowed_updates: allowed_updates.take(),
        };

        let updates = match req.send().await {
            Err(err) => return Some((Either::Left(stream::once(ready(Err(err)))), state)),
            Ok(SemiparsedVec(updates)) => {
                // Set offset to the last update's id + 1
                if let Some(upd) = updates.last() {
                    let id: i32 = match upd {
                        Ok(ok) => ok.id,
                        Err((value, _)) => value["update_id"]
                            .as_i64()
                            .expect("The 'update_id' field must always exist in Update")
                            .try_into()
                            .expect("update_id must be i32"),
                    };

                    *offset = id + 1;
                }

                for update in &updates {
                    if let Err((value, e)) = update {
                        log::error!(
                            "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                            This is a bug in teloxide-core, please open an issue here: \
                            https://github.com/teloxide/teloxide-core/issues.",
                            e,
                            value
                        );
                    }
                }

                updates.into_iter().filter_map(Result::ok).map(Ok)
            }
        };

        let set = Settings::new();
        let subs = set.subscribe;
        let sub_updates = subs
            .into_iter()
            .map(|sub| mock_update_chat(sub.chat_id))
            .map(Ok);

        let all_updates = updates.chain(sub_updates);

        Some((Either::Right(stream::iter(all_updates)), state))
    })
    .flatten()
}

fn polling<R>(
    requester: R,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
) -> impl UpdateListener<R::Err>
where
    R: Requester + 'static,
    <R as Requester>::GetUpdatesFaultTolerant: Send,
{
    let (token, flag) = AsyncStopToken::new_pair();
    let state = State {
        bot: requester,
        timeout: timeout.map(|t| t.as_secs().try_into().expect("timeout is too big")),
        limit,
        allowed_updates,
        offset: 0,
        flag,
        token,
    };
    let stop_token = |st: &mut State<_>| st.token.clone();

    StatefulListener::new(state, stream, stop_token)
}

pub fn polling_listener<R>(requester: R) -> impl UpdateListener<R::Err>
where
    R: Requester + 'static,
    <R as Requester>::GetUpdatesFaultTolerant: Send,
{
    // delete_webhook_if_setup(&requester).await;
    polling(requester, Some(Duration::from_secs(10)), None, None)
}
