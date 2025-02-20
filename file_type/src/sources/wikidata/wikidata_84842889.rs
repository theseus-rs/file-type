use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_84842889: FileType = FileType {
    file_format: &FileFormat {
        id: 84_842_889,
        source_type: SourceType::Wikidata,
        name: "GL Transmission Format, version 2 (text)",
        extensions: &["gltf"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
