use std::string::FromUtf8Error;

use serde::{ser::Serializer, Deserialize, Serialize};
use sled::{Db, IVec};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State,
};

fn serialize<T>(data: &T) -> Result<IVec>
where
    T: Serialize,
{
    Ok(IVec::from(serde_cbor::to_vec(data)?))
}

fn deserialize<'a, T>(data: &'a IVec) -> Result<T>
where
    T: Deserialize<'a>,
{
    Ok(serde_cbor::from_slice(data.as_ref())?)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub name: String,
    pub aliases: Vec<String>,
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum AssetType {
    Css,
    Html,
    ProjectData,
    Script,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub digest: Digest,
    pub asset_type: AssetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DigestType {
    Compressed,
    Uncompressed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DigestAlgo {
    SHA256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Digest {
    pub algo: DigestAlgo,
    pub digest_type: DigestType,
    pub value: String,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    DBError(#[from] sled::Error),
    #[error(transparent)]
    Serialization(#[from] serde_cbor::Error),
    #[error("no site with name {0} found in tree. Data corruption occured")]
    NoSiteWithNameFound(String),
    #[error("no file with digest {0} found in site {1}")]
    NoSuchFileInThisSite(String, String),
    #[error(transparent)]
    NotValidUTF8(#[from] FromUtf8Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[command]
async fn list_sites(state: State<'_, Db>) -> Result<Vec<String>> {
    let trees = state.tree_names();
    let site_names: Vec<String> = trees
        .into_iter()
        .map(|tree_name| String::from_utf8_lossy(tree_name.as_ref()).to_string())
        .filter(|name| !name.starts_with("__sled"))
        .collect();
    Ok(site_names)
}

#[command]
async fn create_site(state: State<'_, Db>, site_name: String) -> Result<Site> {
    let tree = state.open_tree(site_name.as_bytes())?;
    let site = Site {
        name: site_name,
        aliases: vec![],
        files: vec![],
    };
    tree.insert(b"site", serialize(&site)?)?;
    Ok(site)
}

#[command]
async fn load_site(state: State<'_, Db>, site_name: String) -> Result<Site> {
    let tree = state.open_tree(site_name.as_bytes())?;
    let site = tree.get(b"site")?;
    deserialize(&site.ok_or(Error::NoSiteWithNameFound(site_name))?)
}

#[command]
async fn save_site(state: State<'_, Db>, site: Site) -> Result<Site> {
    let tree = state.open_tree(site.name.as_bytes())?;
    tree.insert(b"site", serialize(&site)?)?;
    Ok(site)
}

#[command]
async fn save_file(
    state: State<'_, Db>,
    mut site: Site,
    file: String,
    asset_type: AssetType,
) -> Result<Site> {
    let tree = state.open_tree(site.name.as_bytes())?;
    let file_digest = sha256::digest(file.as_str());
    let mut b = sled::Batch::default();
    let f = File {
        digest: Digest {
            algo: DigestAlgo::SHA256,
            digest_type: DigestType::Uncompressed,
            value: file_digest.clone(),
        },
        asset_type: asset_type.clone(),
    };
    let existing_files: Vec<&File> = site
        .files
        .iter()
        .filter(|f| f.asset_type == asset_type)
        .collect();
    for ef in existing_files {
        b.remove(ef.digest.value.as_bytes());
    }
    site.files = site
        .files
        .into_iter()
        .filter(|f| f.asset_type != asset_type)
        .collect();
    site.files.push(f);
    b.insert(file_digest.as_bytes(), file.as_bytes());
    b.insert(b"site", serialize(&site)?);
    tree.apply_batch(b)?;
    Ok(site)
}

#[command]
async fn load_file(state: State<'_, Db>, site_name: String, file_digest: String) -> Result<String> {
    let tree = state.open_tree(site_name.as_bytes())?;
    let f = tree
        .get(file_digest.as_bytes())?
        .ok_or(Error::NoSuchFileInThisSite(
            file_digest.clone(),
            site_name.clone(),
        ))?;
    Ok(String::from_utf8(f.as_ref().to_vec())?)
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("site")
        .invoke_handler(tauri::generate_handler![
            list_sites,
            create_site,
            load_site,
            save_site,
            save_file,
            load_file,
        ])
        .setup(move |app| {
            let data_dir = app.path_resolver().app_data_dir().unwrap();
            let db = sled::open(data_dir.join("sites.db")).unwrap();
            app.manage(db);
            Ok(())
        })
        .build()
}
