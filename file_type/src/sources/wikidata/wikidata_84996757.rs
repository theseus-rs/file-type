use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_84996757: FileType = FileType {
    file_format: &FileFormat {
        id: 84_996_757,
        source_type: SourceType::Wikidata,
        name: "HP Photo Album",
        extensions: &["albm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
