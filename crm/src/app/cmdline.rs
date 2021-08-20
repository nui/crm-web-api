use std::path::PathBuf;

use once_cell::sync::Lazy;
use structopt::StructOpt;

fn get_version_with_commit_id() -> String {
    match option_env!("BUILD_COMMIT_ID") {
        Some(v) => format!("#{}", v),
        None => String::new(),
    }
}

struct HelpText {
    pub version: String,
}

impl HelpText {
    fn new() -> Self {
        HelpText {
            version: get_version_with_commit_id(),
        }
    }
}

static HELP_TEXT: Lazy<HelpText> = Lazy::new(HelpText::new);

#[derive(Debug, StructOpt)]
#[structopt(name = "crm", version = HELP_TEXT.version.as_str())]
pub struct Cmdline {
    #[structopt(
        long,
        env = "CRM_CONFIG",
        help = "Override app configuration file (e.g., default.toml)"
    )]
    pub config: Option<PathBuf>,
    #[structopt(long, help = "Print all configuration")]
    pub print_config: bool,
}

/// Parse command line arguments
pub fn from_args() -> Cmdline {
    Cmdline::from_args()
}
