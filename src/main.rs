use log::{info, trace};
use serenity::framework::standard::StandardFramework;
use serenity::utils::Colour;
use serenity::Client;
use std::env;

mod commands;
mod config;
mod error;
mod grade;
mod platform;
mod handler;

use commands::{GENERAL_GROUP, HELP};
use config::{Config, StaticConfig};
use handler::Handler;

fn main() {
    setup_env();

    let discord_token = env::var("DISCORD_TOKEN").expect("Env var DISCORD_TOKEN missing");
    let config = Config::load();

    let mut client = Client::new(discord_token, Handler).expect("Failed to create client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(&config.bot.prefix))
            .group(&GENERAL_GROUP)
            .help(&HELP)
            .after(|ctx, msg, cmd_name, error| {
                //  Print out an error if it happened
                if let Err(why) = error {
                    println!("Error in {}: {:?}", cmd_name, why);
                    msg.channel_id
                        .send_message(&ctx, |m| {
                            m.embed(|e| {
                                e.title(format!("Error in `{}`", cmd_name))
                                    .description(format!(
                                        "Encountered an error when executing `{}`.\n```rs\n{:?}```",
                                        cmd_name, why
                                    ))
                                    .color(Colour::DARK_RED)
                            })
                        })
                        .unwrap();
                }
            }),
    );

    // Persist database connection and config
    {
        let mut data = client.data.write();
        data.insert::<StaticConfig>(config);
    }

    info!("Starting Bot");
    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn setup_env() {
    if cfg!(debug_assertions) {
        print!("Loading environment variables from dev.env ... ");
        match dotenv::from_filename("dev.env").ok() {
            Some(f) => println!("Found {:?}", f.file_name().unwrap_or_default()),
            None => println!("No file found"),
        }
    }

    print!("Loading environment variables from .env ... ");
    match dotenv::dotenv().ok() {
        Some(f) => println!("Found {:?}", f.file_name().unwrap_or_default()),
        None => println!("No file found"),
    }

    print!("Loading secrets from secrets.env ... ");
    match dotenv::from_filename("secrets.env").ok() {
        Some(f) => println!("Found {:?}", f.file_name().unwrap_or_default()),
        None => println!("No file found"),
    }

    print!("Setting up logger ... ");
    pretty_env_logger::init();
    println!("Done");

    trace!("env logger loaded");
}
