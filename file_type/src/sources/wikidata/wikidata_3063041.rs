use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3063041: FileType = FileType {
    file_format: &FileFormat {
        id: 3_063_041,
        source_type: SourceType::Wikidata,
        name: "Filmbox",
        extensions: &["fbx"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
