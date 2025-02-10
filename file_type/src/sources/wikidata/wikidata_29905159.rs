use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29905159: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_159,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System transport file",
        extensions: &["stx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
