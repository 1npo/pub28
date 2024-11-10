//
// 

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use pub28::settings::{Settings, DEFAULT_CONFIG_PATH};
use pub28::*;

const HELP_TEMPLATE_STR: &str = r#"{name} v{version}

{about}

{author}

{usage-heading} {usage}
{positionals}
{all-args}
"#;

#[derive(Parser)]
#[command(version, author, help_template(HELP_TEMPLATE_STR), about, long_about = None)]
struct Cli {
    /// Get configuration options from a file (default: use CLI options)
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn on debugging messages
    #[arg(short, long)]
    debug: bool,

    /// Specify which subcommand to use
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse street addresses from the command-line
    Parse(ParseArgs),

    /// Start pub28 as a REST API server
    Api(ApiArgs),

    /// List all the specific standards that pub28 applies to given street addresses
    List, 
}

#[derive(Args)]
struct ParseArgs {
    /// Full street address to parse, or the path to a file contianing street addresses
    /// (the "-f"/"--from-file" flag must be included when VALUE is a file path)
    value: Option<String>,

    #[command(flatten)]
    input: ParseArgsInputSource,

    #[command(flatten)]
    output_type: ParseArgsOutputType,

    #[command(flatten)]
    output_dest: ParseArgsOutputDestination,

    /// Don't apply certain Publication 28 standards when standardizing the address(es)
    #[arg(short, long, value_name = "LIST", value_delimiter = ',', num_args = 1..)]
    ignore: Option<Vec<String>>,

    /// Return full address string(s) (default: return segmented address elements)
    #[arg(short, long)]
    dont_segment_address: bool,

    /// Combine address_1 and address_2 into one address field when returning segmented
    /// address elements
    #[arg(short = '1', long)]
    one_address_field: bool,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct ParseArgsInputSource {
    /// Get street address(es) from a file (default: from CLI argument)
    #[arg(short = 'f', long)]
    from_file: bool,

    /// Get street address(es) from STDIN (default: from CLI argument)
    #[arg(short = 's', long)]
    from_stdin: bool,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct ParseArgsOutputDestination {
    /// Write addresses to a file (default: to STDOUT)
    #[arg(short = 'o', long)]
    to_file: Option<PathBuf>,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct ParseArgsOutputType {
    /// Output addresses as JSON (can print full addresses or segmented address elements) 
    /// (default: as plain text)
    #[arg(short = 'j', long)]
    as_json: bool,

    /// Output addresses as CSV (can print full addresses or segmented address elements)
    /// (default: as plain text)
    #[arg(short = 'c', long)]
    as_csv: bool,
}

#[derive(Args)]
struct ApiArgs {
    /// Bind API server to this hostname
    #[arg(short = 'H', long, value_name = "HOST")]
    host: Option<String>,

    /// Bind API server to this port
    #[arg(short = 'P', long, value_name = "PORT")]
    port: Option<u16>,
}

fn main() {
    let cli = Cli::parse();
    let config_path = PathBuf::from(DEFAULT_CONFIG_PATH);

    if let Some(config) = cli.config.as_deref() {
        let config_path = config;
        println!("config_path = {}", config_path.display());
    } else {
        println!("config_path = {}", config_path.display());
    }

    let mut s = Settings::new(config_path).unwrap();

    match &cli.command {
        Commands::Parse(parse_args) => {
            println!("value = {:?}", parse_args.value);
            println!("from_file = {:?}", parse_args.input.from_file);
            println!("from_stdin = {:?}", parse_args.input.from_stdin);
            println!("as_json = {:?}", parse_args.output_type.as_json);
            println!("as_csv = {:?}", parse_args.output_type.as_csv);
            println!("to_file = {:?}", parse_args.output_dest.to_file);
            println!("ignore = {:?}", parse_args.ignore);
            println!("dont_segment_address = {:?}", parse_args.dont_segment_address);
            println!("one_address_field = {:?}", parse_args.one_address_field);

            if parse_args.input.from_file {
                s = s.set_override("parse.input_source", "file").unwrap();
            }
            if parse_args.input.from_stdin {
                s = s.set_override("parse.input_source", "stdin").unwrap();
            }
            if parse_args.output_type.as_json {
                s = s.set_override("parse.output_type", "json").unwrap();
            }
            if parse_args.output_type.as_csv {
                s = s.set_override("parse.output_type", "csv").unwrap();
            }
            if parse_args.output_dest.to_file.is_some() {
                s = s.set_override("parse.output_destination",
                    parse_args.output_dest.to_file.clone()
                        .unwrap()
                        .as_os_str()
                        .to_str()
                        .unwrap())
                    .unwrap();
            }
            if parse_args.ignore.is_some() {
                s = s.set_override("parse.ignore_standards", parse_args.ignore.clone())
                    .unwrap();
            }
            if parse_args.dont_segment_address {
                s = s.set_override("parse.dont_segment_address", true).unwrap();
            }
            if parse_args.one_address_field {
                s = s.set_override("parse.one_address_field", true).unwrap();
            }
        },
        Commands::Api(api_args) => {
            println!("host = {:?}", api_args.host);
            println!("port = {:?}", api_args.port);

            if api_args.host.is_some() {
                s = s.set_override("api.host", api_args.host.clone()).unwrap();
            }
            if api_args.port.is_some() {
                s = s.set_override("api.port", api_args.port.clone()).unwrap();
            }
        },
        Commands::List => {
            println!("pub28 will apply all the following standards unless instructed \
                otherwise:\n");
            println!("Std. #\tDescription of Standard");
            println!("------\t-----------------------");
            for standard in PUB28_STANDARDS_DESC {
                println!("{}\t{}", standard.0, standard.1);
            }
        }
    }

    let settings = s.build().unwrap().try_deserialize::<Settings>().unwrap();

    println!("These are the settings: {:?}", settings);
}
