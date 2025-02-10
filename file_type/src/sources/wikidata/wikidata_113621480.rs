use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113621480: FileType = FileType {
    file_format: &FileFormat {
        id: 113_621_480,
        source_type: SourceType::Wikidata,
        name: "LoadRunner Analysis file",
        extensions: &["lra"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
