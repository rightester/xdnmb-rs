use serde::{Deserialize, Serialize};


pub type CdnPathList = Vec<CdnPath>;


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CdnPath {
    pub rate: f32,
    pub url: String,
}
