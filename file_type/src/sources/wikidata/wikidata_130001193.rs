use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130001193: FileType = FileType {
    file_format: &FileFormat {
        id: 130_001_193,
        source_type: SourceType::Wikidata,
        name: "Jsonnet source code file",
        extensions: &["jsonnet"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
