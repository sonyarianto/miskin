pub mod config_cmd;
pub mod discover;
pub mod init;
pub mod session;
pub mod stats;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "miskin",
    version,
    about = "Save AI tokens across providers. CLI proxy that compresses command output + injects caveman-mode prompts."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Ultra-compact mode: ASCII icons, inline format (extra savings)
    #[arg(short = 'u', long, global = true)]
    pub ultra_compact: bool,

    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short = 'v', long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Command to run and filter (e.g. `miskin git status`)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub passthrough: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install hooks for AI tools (Claude Code, Copilot, Cursor, etc.)
    Init(InitArgs),

    /// Show token savings statistics (alias: gain)
    #[command(alias = "gain")]
    Stats(StatsArgs),

    /// Show or edit configuration
    Config(ConfigArgs),

    /// Compress text from stdin (caveman-mode prompt)
    Compress,

    /// Find missed savings opportunities (commands not yet filtered)
    Discover(DiscoverArgs),

    /// Show Miskin adoption across recent sessions
    Session(SessionArgs),

    /// Run command with raw passthrough + token tracking (no filtering)
    Proxy(ProxyArgs),

    /// Filter errors only from command output
    Err(ErrArgs),

    /// Generate shell completions (bash, zsh, fish)
    Completions(CompletionsArgs),
}

// ── Init ────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct InitArgs {
    /// AI tool to install hooks for
    #[arg(long, short, default_value = "claude")]
    pub agent: String,

    /// Install globally (all projects)
    #[arg(long, short = 'g')]
    pub global: bool,

    /// Uninstall hooks
    #[arg(long)]
    pub uninstall: bool,

    /// Hook only, skip prompt/markdown injection
    #[arg(long)]
    pub hook_only: bool,

    /// Non-interactive mode (CI/CD)
    #[arg(long)]
    pub auto_patch: bool,

    /// Verify installation
    #[arg(long)]
    pub show: bool,
}

// ── Stats ───────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct StatsArgs {
    /// Show ASCII bar graph (last 20 commands)
    #[arg(long)]
    pub graph: bool,

    /// Show recent command history
    #[arg(long)]
    pub history: bool,

    /// Day-by-day breakdown
    #[arg(long)]
    pub daily: bool,

    /// All-time stats (not just recent)
    #[arg(long)]
    pub all: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

// ── Config ──────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: Option<ConfigAction>,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,

    /// Set a configuration value
    Set { key: String, value: String },

    /// Reset configuration to defaults
    Reset,
}

// ── Discover ────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct DiscoverArgs {
    /// Show all projects
    #[arg(long)]
    pub all: bool,

    /// Look back N days (default: 7)
    #[arg(long, default_value = "7")]
    pub since: u32,
}

// ── Session ─────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct SessionArgs {
    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

// ── Proxy ───────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct ProxyArgs {
    /// Command and arguments to run (raw passthrough + tracking)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
    pub command: Vec<String>,
}

// ── Err ─────────────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct ErrArgs {
    /// Command and arguments to run, showing errors only
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 1..)]
    pub command: Vec<String>,
}

// ── Completions ─────────────────────────────────────────────────────

#[derive(clap::Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: clap_complete::Shell,
}
