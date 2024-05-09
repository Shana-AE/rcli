mod opts;
mod process;

pub use opts::{Base64SubCommand, Opts, SubCommand};
pub use process::{process_csv, process_genpass};
