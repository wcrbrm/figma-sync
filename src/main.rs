use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use structopt::StructOpt;

#[derive(Clone, StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// figma api root
    #[structopt(long, env = "FIGMA_API_ROOT", default_value = "https://api.figma.com")]
    api_root: String,

    /// figma api token
    #[structopt(long, env = "FIGMA_ACCESS_TOKEN")]
    access_token: String,

    /// figma project to be displayed
    #[structopt(short, long, env = "FIGMA_PROJECT_ID", default_value = "5027923")]
    project_id: u64,
}

fn figma_headers(opt: Opt) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Figma-Token", opt.access_token.parse().unwrap());
    headers
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ExportOptions {
    directory: Option<String>,
    format: Option<String>,
    scale: Option<u32>,
    first_page_only: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LayerDetails {
    id: String,
    name: Option<String>,
    image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PageDetails {
    id: String,
    name: String,

    #[serde(default = "default_layers")]
    layers: Option<Vec<LayerDetails>>,
}

fn default_layers() -> Option<Vec<LayerDetails>> {
    Default::default()
}

#[derive(Debug, Serialize, Deserialize)]
struct FileDetails {
    key: String,
    name: String,
    pages: Option<Vec<PageDetails>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    key: String,
    name: String,
    thumbnail_url: Option<String>,
    last_modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectDetails {
    name: String,
    files: Vec<FileInfo>,
}

async fn read_project_details(opt: Opt) -> Result<ProjectDetails, Box<dyn Error>> {
    let url = format!("{}/v1/projects/{}/files", opt.api_root, opt.project_id);
    let response = reqwest::Client::new()
        .get(&url)
        .headers(figma_headers(opt))
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    println!("Hello, {:#?}", opt);

    let response = read_project_details(opt.clone()).await?;
    println!("{:#?}", response);
    Ok(())
}
