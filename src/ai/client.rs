use crate::ai::prompt::build_prompt;
use std::process::Command;

fn find_ai_translate() -> std::path::PathBuf {
    let local = std::path::PathBuf::from(if cfg!(windows) {
        "ai_translate.exe"
    } else {
        "ai_translate"
    });

    if local.exists() {
        return local;
    }

    let mut exe_dir = std::env::current_exe()
        .expect("Cannot find current executable")
        .parent()
        .expect("Cannot find exe directory")
        .to_path_buf();

    if cfg!(windows) {
        exe_dir.push("ai_translate.exe");
    } else {
        exe_dir.push("ai_translate");
    }

    exe_dir
}

fn check_env() -> Result<(), String> {
    let env = std::fs::read_to_string(".env")
        .map_err(|_| ".env file not found — create one with HACKCLUB_API_KEY=your_token_here".to_string())?;

    let has_token = env.lines().any(|l| {
        l.starts_with("HACKCLUB_API_KEY=") && l.len() > "HACKCLUB_API_KEY=".len()
    });

    if !has_token {
        return Err("HACKCLUB_API_KEY not found or empty in .env".to_string());
    }

    Ok(())
}

pub fn translate(source: String) -> Result<String, String> {
    check_env()?;

    let prompt = build_prompt(&source);

    std::fs::write("_prompt_tmp.txt", &prompt)
        .map_err(|e| format!("Failed to write temp prompt: {e}"))?;

    let ai_path = find_ai_translate();

    let output = if ai_path.exists() {
        Command::new(&ai_path)
            .output()
            .map_err(|e| format!("Failed to run ai_translate: {e}"))?
    } else {
        Command::new("python")
            .args(["ai_translate.py"])
            .output()
            .or_else(|_| Command::new("python3").args(["ai_translate.py"]).output())
            .map_err(|_| "Python not found. Install Python or build ai_translate.".to_string())?
    };

    let _ = std::fs::remove_file("_prompt_tmp.txt");

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("AI translation failed:\n{err}"));
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result.is_empty() {
        return Err("AI returned empty response".to_string());
    }

    Ok(result)
}