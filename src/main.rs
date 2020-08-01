use structopt::StructOpt;
use serde::{Serialize, Deserialize};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// figma api root
    #[structopt(long, env="FIGMA_API_ROOT", default_value="https://api.figma.com")]
    api_root: String,

    /// figma api token
    #[structopt(long, env="FIGMA_ACCESS_TOKEN")]
    access_token: String,

    /// figma project to be displayes
    #[structopt(short, long, env="FIGMA_PROJECT_ID")]
    project_id: u64,
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

fn default_layers() -> Option<Vec<LayerDetails>> { Default::default() }


#[derive(Debug, Serialize, Deserialize)]
struct FileDetails {
     key: String,
     name: String,
     pages: Option<Vec<PageDetails>>,
}

impl Default for FileDetails {
     fn default() -> Self {
         Self {key: "".to_string(), name: "".to_string(), pages: Default::default() }
     }
}

fn main() {
    let opt = Opt::from_args();  
    println!("Hello, {:#?}", opt);
    println!("FileDetails {:#?}", FileDetails::default())
}
