use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::num::NonZeroU64;

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct ResourcesMeta {
    pub version: u32,
    pub resources: Vec<Resource>,
    pub resource_uuids: BTreeMap<ResourceUUID, u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Resource {
    pub parent: Option<ResourceUUID>,
    pub uuid: ResourceUUID,
    pub ty: String,
    pub hash: String,
    pub meta: Option<ResourceMeta>,
    pub deps: Vec<ResourceUUID>,
    pub subs: Vec<ResourceUUID>,
}

pub type ResourceUUID = NonZeroU64;

pub type ResourceMeta = BTreeMap<String, ResourceMetaValue>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceMetaValue {
    Boolean(bool),
    Integer(i64),
    String(String),
}
