pub mod banner;
pub mod command;
pub mod config;
pub mod errors;
pub mod history;
pub mod pipeline;
pub mod plugins;
pub mod prompt;
pub mod readline;

pub use banner::show_banner;
pub use config::Config;
pub use pipeline::{parse_command_chain, execute_chain};
pub use plugins::PluginManager;
pub use prompt::build_prompt;
pub use readline::ReadLine;
