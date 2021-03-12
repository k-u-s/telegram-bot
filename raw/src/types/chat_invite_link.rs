use crate::types::*;
use serde::Deserialize;

/// This object represents a chat member update, directly mapped.
/// Introduced by https://core.telegram.org/bots/api-changelog#march-9-2021
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub chat: Chat,
    pub creator: User,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub expire_date: Option<Integer>,
    pub member_limit: Option<Integer>,
}