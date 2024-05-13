// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
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
            TextSubCommand::Sign(opts) => match opts.format {
                TextSignFormat::Blake3 => {
                    process_text_sign(&opts.input, &opts.key, opts.format)?;
                }
                TextSignFormat::Ed25519 => {
                    println!("Sign with Ed25519")
                }
            },
            TextSubCommand::Verify(opts) => {
                println!("Verify: {:?}", opts);
            }
        },
    }
    Ok(())
}
