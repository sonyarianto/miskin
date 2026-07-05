use crate::hooks;

const SUPPORTED_AGENTS: &[&str] = &[
    "claude", "copilot", "cursor", "gemini", "codex", "opencode", "windsurf",
];

pub fn run(
    agent: &str,
    global: bool,
    uninstall: bool,
    hook_only: bool,
    auto_patch: bool,
    show: bool,
) -> anyhow::Result<()> {
    if show {
        verify_installation(agent, global)?;
        return Ok(());
    }

    if !SUPPORTED_AGENTS.contains(&agent) {
        anyhow::bail!(
            "Unknown agent '{}'. Supported: {}",
            agent,
            SUPPORTED_AGENTS.join(", ")
        );
    }

    if uninstall {
        uninstall_hooks(agent, global)?;
        println!("Hooks uninstalled for {}", agent);
        return Ok(());
    }

    if auto_patch {
        println!("Auto-patch mode: installing hooks for {}", agent);
    }

    install_hooks(agent, global, hook_only)?;
    Ok(())
}

fn install_hooks(agent: &str, global: bool, hook_only: bool) -> anyhow::Result<()> {
    match agent {
        "claude" => hooks::claude::install(global)?,
        "copilot" => hooks::copilot::install(global)?,
        "cursor" => hooks::cursor::install(global)?,
        "gemini" => hooks::gemini::install(global)?,
        "codex" => hooks::codex::install(global)?,
        "opencode" => hooks::opencode::install(global)?,
        "windsurf" => hooks::windsurf::install(global)?,
        _ => unreachable!(),
    }

    if !hook_only {
        install_prompt_injection(agent, global)?;
    }

    println!("Miskin hooks installed for {}", agent);
    println!("Restart your AI tool for changes to take effect.");
    Ok(())
}

fn uninstall_hooks(agent: &str, global: bool) -> anyhow::Result<()> {
    match agent {
        "claude" => hooks::claude::uninstall(global)?,
        "copilot" => hooks::copilot::uninstall(global)?,
        "cursor" => hooks::cursor::uninstall(global)?,
        "gemini" => hooks::gemini::uninstall(global)?,
        "codex" => hooks::codex::uninstall(global)?,
        "opencode" => hooks::opencode::uninstall(global)?,
        "windsurf" => hooks::windsurf::uninstall(global)?,
        _ => unreachable!(),
    }
    Ok(())
}

fn verify_installation(agent: &str, global: bool) -> anyhow::Result<()> {
    println!("Verifying Miskin installation for {}...", agent);
    let paths = hooks::paths_for_agent(agent, global);

    for path in &paths {
        if path.exists() {
            println!("  ✓ {}", path.display());
        } else {
            println!("  ✗ {} (missing)", path.display());
        }
    }

    if paths.iter().all(|p| p.exists()) {
        println!("All hooks installed correctly.");
    } else {
        println!("Some hooks missing. Run 'miskin init' to reinstall.");
    }

    Ok(())
}

fn install_prompt_injection(agent: &str, global: bool) -> anyhow::Result<()> {
    use crate::config::CavemanLevel;
    use crate::prompt;

    let prompt = prompt::caveman_system_prompt(&CavemanLevel::Full);

    let prompt_file_content = format!(
        "# MISKIN ACTIVE\n\n{}\n\n**Commands automatically rewritten**: \
         git, cargo, npm, pnpm, yarn, docker, pytest, jest, curl, ls, cat, find, ...\n\n\
         Command output is compressed. Use `miskin stats` to see savings.\n",
        prompt
    );

    let prompt_paths = hooks::prompt_paths_for_agent(agent, global);
    for path in &prompt_paths {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, &prompt_file_content)?;
        println!("  Installed prompt: {}", path.display());
    }

    Ok(())
}
