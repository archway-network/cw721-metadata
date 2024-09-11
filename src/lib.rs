use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Metadata {
    /// Name of the asset
    pub name: Option<String>,
    /// Description of the asset
    pub description: Option<String>,
    /// URI pointing to the asset's logo
    pub image: Option<String>,
    /// URI pointing to the asset's animation
    pub animation_url: Option<String>,
    /// URI pointing to an external URL defining the asset
    pub external_url: Option<String>,
    /// Array of attributes defining the characteristics of the asset
    pub attributes: Option<Vec<Attribute>>,
    /// Additional properties that define the asset
    pub properties: Option<Properties>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata::default()
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    pub fn with_animation_url(mut self, animation_url: impl Into<String>) -> Self {
        self.animation_url = Some(animation_url.into());
        self
    }

    pub fn with_external_url(mut self, external_url: impl Into<String>) -> Self {
        self.external_url = Some(external_url.into());
        self
    }

    pub fn with_attributes(mut self, attributes: Vec<Attribute>) -> Self {
        self.attributes = Some(attributes);
        self
    }

    pub fn with_properties(mut self, properties: Properties) -> Self {
        self.properties = Some(properties);
        self
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Attribute {
    /// The type of attribute
    pub trait_type: String,
    /// The value for that attribute
    pub value: String,
}

impl Attribute {
    pub fn new(trait_type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            trait_type: trait_type.into(),
            value: value.into(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Properties {
    /// Additional files to include with the asset
    pub files: Vec<AssetFile>,
    /// A media category for the asset
    pub category: String,
}

impl Properties {
    pub fn new(category: impl Into<String>, files: Vec<AssetFile>) -> Self {
        Self {
            category: category.into(),
            files,
        }
    }
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
    pub size: Option<u64>,
}

impl AssetFile {
    pub fn new(uri: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            file_type: file_type.into(),
            cdn: None,
            resolution: None,
            size: None,
        }
    }

    pub fn set_cdn(&mut self, cdn: bool) {
        self.cdn = Some(cdn)
    }

    pub fn with_cdn(mut self) -> Self {
        self.cdn = Some(true);
        self
    }

    pub fn with_resolution(mut self, resolution: impl Into<String>) -> Self {
        self.resolution = Some(resolution.into());
        self
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }
}
