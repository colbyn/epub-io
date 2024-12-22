use std::{collections::{BTreeMap, HashSet}, path::{Path, PathBuf}};

use itertools::Itertools;

pub type ResourceIdentifier = String;
pub type MimeType = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NavigationPoint {
    /// the title of this navpoint
    pub label: String,
    /// the resource path
    pub content: PathBuf,
    /// nested navpoints
    pub children: Vec<NavigationPoint>,
    /// the order in the toc
    pub play_order: usize,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceInfo {
    pub file_path: PathBuf,
    pub mime_type: MimeType,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    /// EPUB Spine IDs
    pub spine: Vec<ResourceIdentifier>,
    /// Table Of Content, list of `NavigationPoint` in the `toc.ncx`.
    pub toc: Vec<NavigationPoint>,
    /// resource id -> (path, mime)
    pub resources: BTreeMap<ResourceIdentifier, ResourceInfo>,
    /// The EPUB metadata stored as key -> value
    pub metadata: BTreeMap<String, Vec<String>>,
    /// Custom css list to inject in every xhtml file
    pub extra_css: Vec<String>,
    /// unique identifier
    pub unique_identifier: Option<String>,
    /// The id of the cover, if any
    pub cover_id: Option<String>,
}

impl Manifest {
    pub fn provision_for(epub: &epub::doc::EpubDoc<std::io::BufReader<std::fs::File>>) -> Self {
        let toc = epub.toc
            .iter()
            .map(|x| NavigationPoint::from_raw(x))
            .collect_vec();
        let resources = epub.resources
            .iter()
            .map(|(id, (path, mime))| {
                let id = id.to_owned();
                let value = ResourceInfo { file_path: path.to_owned(), mime_type: mime.to_owned() };
                (id, value)
            })
            .collect::<BTreeMap<_, _>>();
        let metadata = epub.metadata.clone().into_iter().collect::<BTreeMap<_, _>>();
        Self {
            spine: epub.spine.clone(),
            toc,
            resources,
            metadata,
            extra_css: epub.extra_css.clone(),
            unique_identifier: epub.unique_identifier.clone(),
            cover_id: epub.cover_id.clone(),
        }
    }
    /// If this fails the process will terminate with an error message printed to STDERR. 
    pub fn unsafe_open(file_path: impl AsRef<Path>) -> Manifest {
        let file_path = file_path.as_ref();
        assert!(file_path.extension().unwrap() == "toml");
        let file_content = std::fs::read_to_string(file_path).expect("Failed to read manifest file");
        let manifest = ::toml::from_str::<Manifest>(&file_content).expect("Failed to parse/deserialize manifest file");
        manifest
    }
    /// If this fails the process will terminate with an error message printed to STDERR. 
    pub fn unsafe_write(&self, file_path: impl AsRef<Path>) {
        let file_path = file_path.as_ref();
        assert!(file_path.extension().unwrap() == "toml");
        let file_content = ::toml::to_string_pretty(self).unwrap();
        std::fs::write(file_path, file_content)
            .expect("Failed to write the output manifest of the EPUB content files.");
    }
    pub fn all_file_paths(&self) -> HashSet<PathBuf> {
        (&self.resources)
            .into_iter()
            .map(|(_, ResourceInfo { file_path, mime_type: _ })| file_path.to_owned())
            .collect::<HashSet<_>>()
    }
    // pub fn update_file_path(&mut self, old: impl AsRef<Path>, new: impl AsRef<Path>) {
    //     let old = old.as_ref();
    //     let new = new.as_ref();
    //     for (_, ResourceInfo { file_path, .. }) in self.resources.iter_mut() {
    //         if file_path == old {
    //             *file_path = new.to_owned();
    //         }
    //     }
    // }
}

impl NavigationPoint {
    fn from_raw(nav_point: &epub::doc::NavPoint) -> Self {
        Self {
            label: nav_point.label.clone(),
            content: nav_point.content.clone(),
            children: nav_point.children
                .iter()
                .map(|x| Self::from_raw(x))
                .collect_vec(),
            play_order: nav_point.play_order,
        }
    }
}
