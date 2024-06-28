use inquire::Select;
use reqwest::ClientBuilder;
use serde::Deserialize;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;

    let resp = client
        .get(API_URL)
        .send()
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

    let assets = client
        .get(&resp[idx].assets_url)
        .send()
        .await?
        .json::<Vec<Asset>>()
        .await?;

    let os_choice = Select::new(
        "Which binary would you like to install?",
        assets.iter().map(|x| &x.name).collect(),
    );

    let os = os_choice.prompt()?;

    let idx = assets.iter().position(|x| &x.name == os).unwrap();

    let bin_url = assets[idx].browser_download_url;

    // TODO: write binary to file

    Ok(())
}
