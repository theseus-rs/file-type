use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125691821: FileType = FileType {
    file_format: &FileFormat {
        id: 125_691_821,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Master Document",
        extensions: &["odm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
