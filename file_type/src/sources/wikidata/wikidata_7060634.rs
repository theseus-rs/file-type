use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7060634: FileType = FileType {
    file_format: &FileFormat {
        id: 7_060_634,
        source_type: SourceType::Wikidata,
        name: "Norton Guides",
        extensions: &["ng"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
