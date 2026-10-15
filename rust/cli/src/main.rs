use anyhow::Result;
use clap::{
    Parser, Subcommand, ValueEnum,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};
use time::macros::format_description;
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

/// {{project-name}} — A CLI application template built with clap.
#[derive(Parser, Debug)]
#[command(name = "{{project-name}}", version, about, long_about = None, styles = STYLES)]
struct Cli {
    /// Verbosity level (can be used multiple times: -v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Suppress all output except errors
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    quiet: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Greet someone
    Greet {
        /// Name to greet
        #[arg(default_value = "World")]
        name: String,

        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u32,

        /// Greeting style
        #[arg(short, long, value_enum, default_value_t = GreetingStyle::Friendly)]
        style: GreetingStyle,
    },

    /// Show configuration
    Config {
        /// Show as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum GreetingStyle {
    /// Friendly greeting
    Friendly,
    /// Formal greeting
    Formal,
    /// Pirate-style greeting
    Pirate,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(cli.verbose, cli.quiet);
    tracing::debug!("CLI arguments: {:?}", cli);

    match cli.command {
        Command::Greet { name, count, style } => {
            for i in 0..count {
                let greeting = match style {
                    GreetingStyle::Friendly => format!("Hello, {}!", name),
                    GreetingStyle::Formal => format!("Good day to you, {}.", name),
                    GreetingStyle::Pirate => format!("Ahoy, {}! Arr!", name),
                };
                if count > 1 {
                    println!("[{}] {}", i + 1, greeting);
                } else {
                    println!("{}", greeting);
                }
            }
        }
        Command::Config { json } => {
            let name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            if json {
                let lb = '{';
                let rb = '}';
                println!("{lb}\"name\": \"{name}\", \"version\": \"{version}\"{rb}");
            } else {
                println!("Configuration for {name}:");
                println!("  Version: {version}");
            }
        }
    }

    Ok(())
}

/// Initialize tracing subscriber based on verbosity flags.
fn init_logging(verbose: u8, quiet: bool) {
    let filter = if quiet {
        "error"
    } else {
        match verbose {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        }
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter)),
        )
        .with_writer(std::io::stderr)
        .with_timer(LocalTime::new(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        )))
        .init();
}
