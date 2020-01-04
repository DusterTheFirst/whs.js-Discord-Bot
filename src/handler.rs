use crate::config::{Config, StaticConfig};
use serenity::model::{guild::Member, id::GuildId, gateway::Ready};
use serenity::prelude::{Context, EventHandler};
use serenity::utils::Colour;
use log::info;

pub struct Handler;

impl EventHandler for Handler {
    fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, member: Member) {
        let data = ctx.data.read();
        let config: &Config = data.get::<StaticConfig>().unwrap();

        if config.bot.guild != guild_id {
            return;
        }

        member.user.read().dm(&ctx, |m| {
            m.embed(|e| {
                e.title("Welcome!")
                    .description(format!(
                        "Welcome to the `{}` discord server!\nIn order to be able to provide you with better support and assistance, we need some information from you first.\nPlease use the `{prefix}platform` and `{prefix}grade` commands to provide us with the information about your device and your school grade",
                        config.bot.guild.to_partial_guild(&ctx).expect("Failed to get guild").name, prefix=config.bot.prefix
                    ))
                    .color(Colour::DARK_GREEN)
            })
        }).unwrap();
    }
}
