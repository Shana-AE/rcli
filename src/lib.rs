mod opts;
mod process;
mod utils;

use enum_dispatch::enum_dispatch;
pub use opts::{
    Base64SubCommand, CsvOpts, DecodeOpts, EncodeOpts, GenPassOpts, HttpServeOpts, HttpSubCommand,
    KeyGenerateOpts, Opts, SubCommand, TextSignFormat, TextSignOpts, TextSubCommand,
    TextVerifyOpts,
};
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
