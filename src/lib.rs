mod opts;
mod process;
mod utils;

pub use opts::{
    Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
};
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
