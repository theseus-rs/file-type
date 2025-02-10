use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123349564: FileType = FileType {
    file_format: &FileFormat {
        id: 123_349_564,
        source_type: SourceType::Wikidata,
        name: "Clooz database file",
        extensions: &["clz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
