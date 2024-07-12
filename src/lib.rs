use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Metadata {
    /// Name of the asset
    pub name: String,
    /// Description of the asset
    pub description: String,
    /// URI pointing to the asset's logo
    pub image: String,
    /// URI pointing to the asset's animation
    pub animation_url: String,
    /// URI pointing to an external URL defining the asset
    pub external_url: String,
    /// Array of attributes defining the characteristics of the asset
    pub attributes: Vec<Attribute>,
    /// Additional properties that define the asset
    pub properties: Properties,
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Attribute {
    /// The type of attribute
    pub trait_type: String,
    /// The value for that attribute
    pub value: String,
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Properties {
    /// Additional files to include with the asset
    pub files: Vec<AssetFile>,
    /// A media category for the asset
    pub category: String,
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AssetFile {
    /// The file's URI
    pub uri: String,
    /// The file's type
    #[serde(rename = "type")]
    pub file_type: String,
    /// Whether the file is served from a CDN.
    pub cdn: Option<bool>,
    /// Defines the file's resolution if applicable
    pub resolution: Option<String>,
    /// The files size if applicable
    pub size: Option<u64>
}