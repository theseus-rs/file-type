use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126823474: FileType = FileType {
    file_format: &FileFormat {
        id: 126_823_474,
        source_type: SourceType::Wikidata,
        name: "Visual F# Script File",
        extensions: &["fsx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
