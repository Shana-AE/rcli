// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    process_text_verify, Base64SubCommand, Opts, SubCommand, TextSubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                if output.ends_with(&opts.format.to_string()) {
                    output
                } else {
                    format!("{}.{}", output, opts.format)
                }
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?
        }
        SubCommand::GenPass(genpass) => process_genpass(
            genpass.length,
            genpass.uppercase,
            genpass.lowercase,
            genpass.number,
            genpass.symbol,
        )?,
        SubCommand::Base64SubCommand(b64) => match b64 {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcommand) => match subcommand {
            TextSubCommand::Sign(opts) => {
                process_text_sign(&opts.input, &opts.key, opts.format)?;
            }
            TextSubCommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
            }
        },
    }
    Ok(())
}
