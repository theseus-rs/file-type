use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_72959401: FileType = FileType {
    file_format: &FileFormat {
        id: 72_959_401,
        source_type: SourceType::Wikidata,
        name: "Panorama database",
        extensions: &["pan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
