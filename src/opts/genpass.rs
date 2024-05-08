use clap::Args;

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
