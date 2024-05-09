// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{process_csv, process_genpass, Base64SubCommand, Opts, SubCommand};

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
                println!("encode opts {:?}", opts);
            }
            Base64SubCommand::Decode(opts) => {
                println!("decode opts {:?}", opts);
            }
        },
    }
    Ok(())
}
