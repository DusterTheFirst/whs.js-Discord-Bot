use crate::config::{Config, StaticConfig};
use crate::grade::Grade;
use crate::platform::Platform;
use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::{channel::Message, guild::Member, id::UserId};
use serenity::prelude::*;
use serenity::utils::Colour;
use std::collections::HashSet;

group!({
    name: "General",
    options: {
        description: "General Commands",
    },
    commands: [grade, platform, beta],
});

#[command]
#[aliases("g")]
fn grade(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    let config: &Config = data.get::<StaticConfig>().unwrap();
    let mut member: Member = config.bot.guild.member(&ctx, msg.author.id)?;
    let grades_roles = &config.roles.grades;

    // If there was one arg passed, set the user's grade to that
    if args.len() == 1 {
        // Match the result of parsing it
        match args.parse::<Grade>() {
            // If it has a valid grade, set the user's grade to that
            Ok(grade) => {
                // Remove all other class roles
                member.remove_roles(
                    &ctx,
                    &[
                        grades_roles.freshman,
                        grades_roles.sophomore,
                        grades_roles.junior,
                        grades_roles.senior,
                        grades_roles.graduate,
                    ],
                )?;
                // Add the apropriate class role
                member.add_role(
                    &ctx,
                    match grade {
                        Grade::Freshman => grades_roles.freshman,
                        Grade::Sophomore => grades_roles.sophomore,
                        Grade::Junior => grades_roles.junior,
                        Grade::Senior => grades_roles.senior,
                        Grade::Graduate => grades_roles.graduate,
                    },
                )?;
                // Inform the user that their role has been set
                msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Set grade")
                            .description(format!("Your grade has now been set to `{:?}`", grade))
                            .color(Colour::DARK_GREEN)
                    })
                })?;
            }
            // If it could not be parsed, handle the error
            Err(_) => {
                // Inform the user of the valid inputs
                msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Invalid grade")
                            .description(
                                "The valid grades are: `Freshman` `Sophomore` `Junior` `Senior` and `Graduate`"
                            )
                            .color(Colour::DARK_RED)
                    })
                })?;
            }
        };
    // If there was no arguments passed, tell the user their grade
    } else if args.len() == 0 {
        // Get the user's grade
        if let Some(grade) = Grade::get(&member.roles, &config.roles.grades) {
            // Tell the user their grades
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Your Grade")
                        .description(format!("Your current grade level is `{:?}`", grade))
                        .color(Colour::DARK_GREEN)
                })
            })?;
        } else {
            // Tell the user they have no grade selected
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Your Grade")
                        .description("You have no grade specified, please specify one using this command and specifying the grade you are in. ex: !grade freshman")
                        .color(Colour::GOLD)
                })
            })?;
        }
    } else {
        // Tell the user that they have used the command wrong
        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Invalid amount of arguments")
                    .description("You can only specify one argument max")
                    .color(Colour::DARK_RED)
            })
        })?;
    }

    Ok(())
}

#[command]
#[aliases("p")]
fn platform(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    let config: &Config = data.get::<StaticConfig>().unwrap();
    let mut member: Member = config.bot.guild.member(&ctx, msg.author.id)?;
    let platforms_roles = &config.roles.platforms;

    // If there was one arg passed, set the user's platform to that
    if args.len() == 1 {
        // Match the result of parsing it
        match args.parse::<Platform>() {
            // If it has a valid platform, set the user's platform to that
            Ok(platform) => {
                // Remove all other platform roles
                member.remove_roles(&ctx, &[platforms_roles.ios, platforms_roles.android])?;
                // Add the apropriate platform role
                member.add_role(
                    &ctx,
                    match platform {
                        Platform::IOS => platforms_roles.ios,
                        Platform::Android => platforms_roles.android,
                    },
                )?;
                // Inform the user that their role has been set
                msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Set platform")
                            .description(format!(
                                "Your platform has now been set to `{:?}`",
                                platform
                            ))
                            .color(Colour::DARK_GREEN)
                    })
                })?;
            }
            // If it could not be parsed, handle the error
            Err(_) => {
                // Inform the user of the valid inputs
                msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Invalid platform")
                            .description("The valid platforms are: `iOS`, `Android`")
                            .color(Colour::DARK_RED)
                    })
                })?;
            }
        };
    // If there was no arguments passed, tell the user their platform
    } else if args.len() == 0 {
        // Get the user's platform
        if let Some(platform) = Platform::get(&member.roles, &config.roles.platforms) {
            // Tell the user their platform
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Your Platform")
                        .description(format!("Your current platform is `{:?}`", platform))
                        .color(Colour::DARK_GREEN)
                })
            })?;
        } else {
            // Tell the user they have no platform selected
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Your Platform")
                        .description("You have no platform specified, please specify one using this command and specifying the platform you are in. ex: !platform ios")
                        .color(Colour::GOLD)
                })
            })?;
        }
    } else {
        // Tell the user that they have used the command wrong
        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Invalid amount of arguments")
                    .description("You can only specify one argument max")
                    .color(Colour::DARK_RED)
            })
        })?;
    }

    Ok(())
}

#[command]
#[aliases("b")]
fn beta(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    let config: &Config = data.get::<StaticConfig>().unwrap();
    let mut member: Member = config.bot.guild.member(&ctx, msg.author.id)?;
    let beta_role = &config.roles.beta;

    if args.len() == 0 {
        // Get the user's platform
        if member
            .user
            .read()
            .has_role(&ctx, config.bot.guild, beta_role)?
        {
            // Tell the user their platform
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Beta Status")
                        .description("You are already a member of the beta program. To leave, use `!beta leave`.")
                        .color(Colour::DARK_GREEN)
                })
            })?;
        } else {
            // Tell the user they have no platform selected
            msg.channel_id.send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Beta Status")
                        .description("You are not a member of the beta program. To join, use `!beta join`.")
                        .color(Colour::GOLD)
                })
            })?;
        }
    } else if args.len() == 1 {
        match args.parse::<String>()?.to_lowercase().as_str() {
            "join" => {
                member.add_role(&ctx, beta_role)?;

                let platform = Platform::get(&member.roles, &config.roles.platforms);

                // Inform the user of their new status, aswell as how to join the beta program
                match &platform {
                    Some(platform) => {
                        msg.channel_id.send_message(&ctx, |m| {
                            m.embed(|e| {
                                e.title("You are now in the Beta program")
                                    .description(format!(
                                        "Welcome to the beta program, I have detected that you are on the `{:?}` platform.\nTo join, use this link: {}",
                                        platform,
                                        match platform {
                                            Platform::IOS => &config.links.beta.ios,
                                            Platform::Android => &config.links.beta.android
                                        }
                                    ))
                                    .color(Colour::DARK_GREEN)
                            })
                        })?;
                    }
                    None => {
                        msg.channel_id.send_message(&ctx, |m| {
                            m.embed(|e| {
                                e.title("You are now in the Beta program")
                                    .description(format!(
                                        "Welcome to the beta program, You have not specified which platform you are on, please do so using `!platform`.\nTo sign up on android, please use this link: {}\nTo sign up on ios, please use this link: {}",
                                        config.links.beta.ios,
                                        config.links.beta.android
                                    ))
                                    .color(Colour::DARK_GREEN)
                            })
                        })?;
                    }
                }
            }
            "leave" => {
                member.remove_role(&ctx, beta_role)?;

                // Inform the user of their new status, aswell as how to leave the beta program
                msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("You have left the beta program")
                            .description("To leave the beta app releases, redowload the app from the app store or play store")
                            .color(Colour::GOLD)
                    })
                })?;
            }
            _ => {
                // Inform the user of the valid inputs
                msg.channel_id.send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Invalid action")
                            .description("The valid actions are: `join` and `leave`")
                            .color(Colour::DARK_RED)
                    })
                })?;
            }
        }
    } else {
        // Tell the user that they have used the command wrong
        msg.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Invalid amount of arguments")
                    .description("You can only specify one argument max")
                    .color(Colour::DARK_RED)
            })
        })?;
    }

    Ok(())
}

#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
#[strikethrough_commands_tip_in_guild("")]
#[strikethrough_commands_tip_in_dm("")]
fn help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}
