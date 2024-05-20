use clap::Args;

use crate::{process_genpass, CmdExecutor};
use zxcvbn::zxcvbn;

#[derive(Debug, Args)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    // the default action of bool value is SetTrue
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    pub number: bool,
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", password);

        let estimate = zxcvbn(&password, &[]).unwrap();
        eprintln!("password strength score: {}", estimate.score());
        Ok(())
    }
}
