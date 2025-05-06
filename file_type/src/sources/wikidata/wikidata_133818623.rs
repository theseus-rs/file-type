use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133818623: FileType = FileType {
    file_format: &FileFormat {
        id: 133_818_623,
        source_type: SourceType::Wikidata,
        name: "MapLibre style",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
