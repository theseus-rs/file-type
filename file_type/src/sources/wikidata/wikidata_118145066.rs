use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118145066: FileType = FileType {
    file_format: &FileFormat {
        id: 118_145_066,
        source_type: SourceType::Wikidata,
        name: "Serenade Harmonica File",
        extensions: &["ckt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
