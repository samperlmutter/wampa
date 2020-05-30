use std::env;

use serenity::prelude::*;
use serenity::model::{ channel::Message, id::RoleId };

use crate::event_handler::WampaError;

pub enum Command {
    Name(String)
}

impl Command {
    pub fn from_str(s: &str) -> Result<Self, WampaError> {
        let msg = s.chars()
            .skip(1)
            .collect::<String>();
        let params = msg.split_whitespace()
            .collect::<Vec<&str>>();
        match params[0] {
            "name" => Ok(Command::Name(params[1].to_string())),
            _ => Err(WampaError::InvalidCmd("Invalid command".to_string()))
        }
    }

    pub fn exec(&self, ctx: Context, msg: Message) -> Result<(), WampaError> {
        match &self {
            Command::Name(name) => {
                let guild_id  = msg.guild_id.ok_or(WampaError::InternalServerError("Error finding guild id".to_string()))?;
                let guild = ctx.cache.read()
                    .guild(guild_id)
                    .ok_or(WampaError::InternalServerError("Error finding guild".to_string()))?;
                guild.read().edit_member(&ctx.http, msg.author.id, |m| m.nickname(name))?;

                if msg.channel_id.0 == env::var("WELCOME_CHANNEL_ID")?.parse::<u64>()? {
                    let mut msgs = msg.channel_id.messages(&ctx.http, |ret| ret.before(msg.id))?;
                    msgs.push(msg.clone());
                    msg.channel_id.delete_messages(&ctx.http, msgs.iter().filter(|m| m.id != env::var("WELCOME_MESSAGE_ID").unwrap().parse::<u64>().unwrap()))?;

                    let roles = vec![RoleId(env::var("MEMBER_ROLE_ID")?.parse::<u64>()?)];
                    guild.read().edit_member(&ctx.http, msg.author.id, |m| m.roles(roles))?;
                }
            }
        }

        Ok(())
    }
}
