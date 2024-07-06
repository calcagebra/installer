mod setup;

use std::{env, fs::Permissions, os::unix::fs::PermissionsExt, path::PathBuf, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use inquire::Select;
use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use setup::setup;
use tokio::fs::set_permissions;

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    assets_url: String,
}

#[derive(Deserialize, Debug)]
struct Asset {
    browser_download_url: String,
    name: String,
    /// The size of the asset in bytes
    size: u64,
}

const API_URL: &str = "https://api.github.com/repos/calcagebra/calcagebra/releases";
const USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:101.0) Gecko/20100101 Firefox/101.0";

async fn wrap_spinner(
    client: &Client,
    url: &str,
    message: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"]),
    );
    pb.set_message(message.clone());

    let res = client.get(url).send().await;

    pb.finish_with_message(format!("{message} done!"));

    res
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _binname = args.next();
    match args.next() {
        Some(arg) => match arg.as_str() {
            "setup" => {
                setup(std::env::consts::OS)?;
                install_ver().await?;
                println!("Please add `. ~/.calcagebra/env` to your shell's rc file");

                Ok(())
            },
            _ => todo!()
        },
        _ => install_ver().await,
    }
}

async fn install_ver() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;

    let resp = wrap_spinner(&client, API_URL, "Fetching version list...".to_owned())
        .await?
        .json::<Vec<Release>>()
        .await?;

    let p_builder = Select::new(
        "What version would you like to install?",
        resp.iter().map(|x| &x.tag_name).collect(),
    );

    let answer = p_builder.prompt()?;

    // Unwrap is ok because the string comes from the vec
    let idx = resp.iter().position(|x| &x.tag_name == answer).unwrap();

    let assets = wrap_spinner(
        &client,
        &resp[idx].assets_url,
        "Fetching asset list...".to_owned(),
    )
    .await?
    .json::<Vec<Asset>>()
    .await?;

    let fname = match std::env::consts::OS {
        "windows" => "calcagebra.exe",
        "macos" => match std::env::consts::ARCH {
            "aarch64" => "calcagebra-aarch64-darwin",
            _ => "calcagebra-darwin",
        },
        _ => "calcagebra",
    };

    let idx = assets.iter().position(|x| x.name == fname).unwrap();

    let contents = wrap_spinner(
        &client,
        &assets[idx].browser_download_url,
        "Downloading binary...".to_owned(),
    )
    .await?
    .bytes()
    .await?;

    let data_var = match cfg!(windows) {
        true => "USERPROFILE",
        false => "HOME",
    };

    let mut data_dir = PathBuf::from(env::var(data_var)?);
    data_dir.push(".calcagebra");
    data_dir.push("bin");
    let _ = tokio::fs::create_dir_all(&data_dir).await;
    data_dir.push("calcagebra");

    tokio::fs::write(&data_dir, contents).await?;

    set_permissions(&data_dir, Permissions::from_mode(0o770)).await?;

    println!(
        "Wrote file `calcagebra` ({:.2}mb) to {}",
        assets[idx].size as f32 / 1e+6,
        data_dir
            .to_string_lossy()
            .replace(&env::var(data_var)?, "~")
    );

    Ok(())
}
