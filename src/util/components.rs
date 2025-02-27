use std::io::Read;

use flate2::read::GzDecoder;
use itertools::Itertools;
use lsp_types::{MarkupContent, MarkupKind};
use once_cell::sync::Lazy;
use serde::Deserialize;
use smol_str::SmolStr;

use crate::{
    db::{analysis::TexLinkKind, Document, Workspace},
    Db,
};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDatabase {
    pub components: Vec<Component>,
    pub metadata: Vec<ComponentMetadata>,
}

impl ComponentDatabase {
    pub fn find(&self, name: &str) -> Option<&Component> {
        self.components.iter().find(|component| {
            component
                .file_names
                .iter()
                .any(|file_name| file_name == name)
        })
    }

    pub fn linked_components(&self, db: &dyn Db, child: Document) -> Vec<&Component> {
        Workspace::get(db)
            .related(db, child)
            .iter()
            .filter_map(|document| document.parse(db).as_tex())
            .flat_map(|data| data.analyze(db).links(db))
            .filter_map(|link| match link.kind(db) {
                TexLinkKind::Sty => Some(format!("{}.sty", link.path(db).text(db))),
                TexLinkKind::Cls => Some(format!("{}.cls", link.path(db).text(db))),
                _ => None,
            })
            .filter_map(|name| self.find(&name))
            .chain(std::iter::once(self.kernel()))
            .flat_map(|comp| {
                comp.references
                    .iter()
                    .filter_map(|name| self.find(name))
                    .chain(std::iter::once(comp))
            })
            .unique_by(|comp| &comp.file_names)
            .collect()
    }

    pub fn kernel(&self) -> &Component {
        self.components
            .iter()
            .find(|component| component.file_names.is_empty())
            .unwrap()
    }

    pub fn documentation(&self, name: &str) -> Option<MarkupContent> {
        let metadata = self
            .metadata
            .iter()
            .find(|metadata| metadata.name == name)?;

        let desc = metadata.description.clone()?;
        Some(MarkupContent {
            kind: MarkupKind::PlainText,
            value: desc,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub file_names: Vec<SmolStr>,
    pub references: Vec<SmolStr>,
    pub commands: Vec<ComponentCommand>,
    pub environments: Vec<SmolStr>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCommand {
    pub name: SmolStr,
    pub image: Option<String>,
    pub glyph: Option<SmolStr>,
    pub parameters: Vec<ComponentParameter>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentParameter(pub Vec<ComponentArgument>);

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentArgument {
    pub name: SmolStr,
    pub image: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentMetadata {
    pub name: String,
    pub caption: Option<String>,
    pub description: Option<String>,
}

const JSON_GZ: &[u8] = include_bytes!("../../data/components.json.gz");

pub static COMPONENT_DATABASE: Lazy<ComponentDatabase> = Lazy::new(|| {
    let mut decoder = GzDecoder::new(JSON_GZ);
    let mut buf = String::new();
    decoder.read_to_string(&mut buf).unwrap();
    serde_json::from_str(&buf).unwrap()
});
