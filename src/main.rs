// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
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
            print!("Processing CSV file... to {}", output);
            process_csv(&opts.input, &output, opts.format)?
        }
    }
    Ok(())
}
