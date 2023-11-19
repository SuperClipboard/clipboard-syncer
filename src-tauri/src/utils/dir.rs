use anyhow::Result;
use std::fs;
use std::path::PathBuf;

const APP_DIR: &str = "clipboard-syncer";

const CONFIG_FILE: &str = "config.json";

const SECRET_FILE: &str = "secret.txt";

/// get the app home dir
pub fn app_home_dir() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        use tauri::utils::platform::current_exe;
        let app_exe = current_exe()?;
        let app_exe = dunce::canonicalize(app_exe)?;
        let app_dir = app_exe
            .parent()
            .ok_or(anyhow::anyhow!("failed to get the portable app dir"))?;
        Ok(PathBuf::from(app_dir).join(".config").join(APP_DIR))
    }

    #[cfg(not(target_os = "windows"))]
    Ok(tauri::api::path::home_dir()
        .ok_or(anyhow::anyhow!("failed to get the app home dir"))?
        .join(".config")
        .join(APP_DIR))
}

/// logs dir
#[allow(unused)]
pub fn app_log_dir() -> Result<PathBuf> {
    let app_log_dir = app_home_dir()?.join("logs");
    fs::create_dir_all(&app_log_dir)?;
    Ok(app_log_dir)
}

pub fn config_path() -> Result<PathBuf> {
    Ok(app_home_dir()?.join(CONFIG_FILE))
}

pub fn secret_path() -> Result<PathBuf> {
    Ok(app_home_dir()?.join(SECRET_FILE))
}

pub fn app_data_dir() -> Result<PathBuf> {
    let app_data_dir = app_home_dir()?.join("data");
    fs::create_dir_all(&app_data_dir)?;
    Ok(app_data_dir)
}

pub fn app_data_img_dir() -> Result<PathBuf> {
    let app_data_img_dir = app_data_dir()?.join("img");
    fs::create_dir_all(&app_data_img_dir)?;
    Ok(app_data_img_dir)
}

#[test]
fn test() {
    println!("app_home_dir: {:?}", app_home_dir());
    println!("app_logs_dir: {:?}", app_log_dir());
    println!("config_path: {:?}", config_path());
    println!("secret_path: {:?}", secret_path());
    println!("app_data_dir: {:?}", app_data_dir());
}
