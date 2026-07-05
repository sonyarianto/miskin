mod analytics;
mod cli;
mod config;
mod filters;
mod hooks;
mod prompt;
mod tee;

use clap::Parser;
use cli::Commands;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    let env_filter = match cli.verbose {
        0 => EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        1 => EnvFilter::new("info"),
        2 => EnvFilter::new("debug"),
        _ => EnvFilter::new("trace"),
    };
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    if let Some(command) = cli.command {
        match command {
            Commands::Init(args) => cli::init::run(
                &args.agent,
                args.global,
                args.uninstall,
                args.hook_only,
                args.auto_patch,
                args.show,
                args.dry_run,
            )?,
            Commands::Stats(args) => {
                cli::stats::run(args.graph, args.history, args.daily, args.all, args.json)?
            }
            Commands::Config(args) => match args.action.unwrap_or(cli::ConfigAction::Show) {
                cli::ConfigAction::Show => cli::config_cmd::run()?,
                cli::ConfigAction::Set { key, value } => cli::config_cmd::set(&key, &value)?,
                cli::ConfigAction::Reset => cli::config_cmd::reset()?,
            },
            Commands::Compress => run_compress(cli.ultra_compact)?,
            Commands::Discover(args) => cli::discover::run(args.all, args.since)?,
            Commands::Session(args) => cli::session::run(args.json)?,
            Commands::Proxy(args) => run_proxy(&args.command).await?,
            Commands::Err(args) => run_err(&args.command).await?,
            Commands::Completions(args) => run_completions(args)?,
        }
    } else if !cli.passthrough.is_empty() {
        run_passthrough(&cli.passthrough, cli.ultra_compact).await?;
    }

    Ok(())
}

async fn run_passthrough(args: &[String], ultra_compact: bool) -> anyhow::Result<()> {
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::process::Command;

    let config = config::MiskinConfig::load()?;
    let command_name = &args[0];
    let subcommand_args = &args[1..];

    if !config.general.enabled {
        let status = Command::new(command_name)
            .args(subcommand_args)
            .status()
            .await?;
        std::process::exit(status.code().unwrap_or(1));
    }

    if config.general.exclude_commands.contains(command_name) {
        let status = Command::new(command_name)
            .args(subcommand_args)
            .status()
            .await?;
        std::process::exit(status.code().unwrap_or(1));
    }

    let mut child = Command::new(command_name)
        .args(subcommand_args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let mut child_stdout = child.stdout.take().expect("stdout not piped");
    let mut child_stderr = child.stderr.take().expect("stderr not piped");

    let stdout_task = {
        let mut buf = Vec::new();
        async move {
            child_stdout.read_to_end(&mut buf).await?;
            Ok::<Vec<u8>, std::io::Error>(buf)
        }
    };

    let stderr_task = {
        let mut buf = Vec::new();
        async move {
            let mut chunk = [0u8; 8192];
            loop {
                let n = child_stderr.read(&mut chunk).await?;
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&chunk[..n]);
                let _ = tokio::io::stderr().write_all(&chunk[..n]).await;
            }
            Ok::<Vec<u8>, std::io::Error>(buf)
        }
    };

    let (stdout_result, stderr_result, status) =
        tokio::join!(stdout_task, stderr_task, child.wait());

    let stdout_buf = stdout_result?;
    let stderr_buf = stderr_result?;
    let exit_code = status?.code();

    let stdout = String::from_utf8_lossy(&stdout_buf).to_string();
    let stderr = String::from_utf8_lossy(&stderr_buf).to_string();
    let full_output = format!("{}{}", stdout, stderr);

    let registry = filters::FilterRegistry::default();
    let original_tokens = analytics::counter::count_tokens(&full_output);

    let mut generic_truncated = false;
    let filtered = if let Some(filter) = registry.get(command_name) {
        match filter.filter(subcommand_args, &full_output, exit_code) {
            filters::FilterResult::Filtered(s) => s,
            filters::FilterResult::PassThrough(s) => s,
            filters::FilterResult::Silent => String::new(),
        }
    } else if let Some(filter) = registry.get(
        subcommand_args
            .first()
            .map(|s| s.as_str())
            .unwrap_or(command_name),
    ) {
        match filter.filter(&subcommand_args[1..], &full_output, exit_code) {
            filters::FilterResult::Filtered(s) => s,
            filters::FilterResult::PassThrough(s) => s,
            filters::FilterResult::Silent => String::new(),
        }
    } else {
        let max = config.filters.max_lines;
        let line_count = full_output.lines().count();
        generic_truncated = line_count > max;
        if config.filters.deduplicate {
            filters::generic::deduplicate_lines(&filters::generic::truncate_lines(
                &full_output,
                max,
            ))
        } else {
            filters::generic::truncate_lines(&full_output, max)
        }
    };

    let final_output = if ultra_compact {
        ultra_compact_format(&filtered)
    } else {
        filtered.clone()
    };

    let filtered_tokens = analytics::counter::count_tokens(&final_output);

    let should_tee = exit_code != Some(0) || !stderr.is_empty() || generic_truncated;
    if should_tee
        && let Ok(tee_path) =
            tee::save_raw(&config.analytics.data_dir, &args.join(" "), &full_output)
    {
        eprintln!("[full output: {}]", tee_path.display());
    }

    print!("{}", final_output);

    if config.analytics.enabled {
        let mut store =
            analytics::AnalyticsStore::load(&config.analytics.data_dir).unwrap_or_default();
        store.record(
            &args.join(" "),
            original_tokens as u64,
            filtered_tokens as u64,
        );
        let _ = store.save(&config.analytics.data_dir);
    }

    std::process::exit(exit_code.unwrap_or(0));
}

async fn run_proxy(args: &[String]) -> anyhow::Result<()> {
    use tokio::process::Command;
    let config = config::MiskinConfig::load()?;
    let output = Command::new(&args[0]).args(&args[1..]).output().await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    print!("{}{}", stdout, stderr);

    if config.analytics.enabled {
        let output_str = format!("{}{}", stdout, stderr);
        let tokens = analytics::counter::count_tokens(&output_str);
        let mut store =
            analytics::AnalyticsStore::load(&config.analytics.data_dir).unwrap_or_default();
        store.record(
            &format!("[proxy] {}", args.join(" ")),
            tokens as u64,
            tokens as u64,
        );
        let _ = store.save(&config.analytics.data_dir);
    }

    std::process::exit(output.status.code().unwrap_or(0));
}

async fn run_err(args: &[String]) -> anyhow::Result<()> {
    use tokio::process::Command;
    let output = Command::new(&args[0]).args(&args[1..]).output().await?;
    let stderr = String::from_utf8_lossy(&output.stderr);

    if stderr.trim().is_empty() {
        println!("no errors");
    } else {
        print!("{}", stderr);
    }

    std::process::exit(output.status.code().unwrap_or(0));
}

fn run_completions(args: cli::CompletionsArgs) -> anyhow::Result<()> {
    use clap::CommandFactory;
    clap_complete::generate(
        args.shell,
        &mut cli::Cli::command(),
        "miskin",
        &mut std::io::stdout(),
    );
    Ok(())
}

fn run_compress(ultra_compact: bool) -> anyhow::Result<()> {
    let config = config::MiskinConfig::load()?;
    let level = if ultra_compact {
        &config::CavemanLevel::Aggressive
    } else {
        &config.caveman.level
    };
    let prompt = prompt::caveman_system_prompt(level);
    println!("{}", prompt);
    Ok(())
}

fn ultra_compact_format(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() <= 1 {
        return input.to_string();
    }

    lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.replace("pass", "✓")
                .replace("fail", "✗")
                .replace("PASS", "✓")
                .replace("FAIL", "✗")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
