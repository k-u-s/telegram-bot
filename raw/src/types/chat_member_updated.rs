use crate::types::*;
use serde::Deserialize;

/// This object represents a chat member update, directly mapped.
/// Introduced by https://core.telegram.org/bots/api-changelog#march-9-2021
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: Integer,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    pub invite_link: Option<ChatInviteLink>,
}