use std::io::{self, Write};

use super::Command;
use crate::err::Error;
use async_trait::async_trait;
use clap::{
    Arg, ArgAction, ArgMatches, Command as ClapCommand, CommandFactory, Parser, Subcommand,
};

pub struct LeaderboardCommand;

#[derive(Parser, Debug)]
#[command(name = "leaderboard", about = "Interact with Leeterboard")]
struct LeaderboardCommandArgs {
    #[command(subcommand)]
    subcommand: LeaderboardSubcommand,
}

#[derive(Subcommand, Debug)]
enum LeaderboardSubcommand {
    #[command(name = "login", about = "Login to Leeterboard")]
    Login,
}

#[async_trait]
impl Command for LeaderboardCommand {
    fn usage() -> ClapCommand {
        LeaderboardCommandArgs::command()
    }

    async fn handler(m: &ArgMatches) -> Result<(), Error> {
        match m.subcommand() {
            Some(("login", sub_m)) => Ok(LeaderboardLoginCommand::handler(sub_m).await?),
            _ => Err(Error::MatchError),
        }
    }
}

pub struct LeaderboardLoginCommand;

#[async_trait]
impl Command for LeaderboardLoginCommand {
    fn usage() -> ClapCommand {
        // We never actually use this, it's only used for top-level commands
        unimplemented!()
    }

    async fn handler(m: &ArgMatches) -> Result<(), Error> {
        // python input() I miss you
        print!("Username: ");
        io::stdout().flush()?;
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        username = username.trim_end().to_string();

        let password = rpassword::prompt_password("Password: ")?;

        unimplemented!()
    }
}
