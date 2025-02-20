use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51837224: FileType = FileType {
    file_format: &FileFormat {
        id: 51_837_224,
        source_type: SourceType::Wikidata,
        name: "Paradox Database Table, version 7",
        extensions: &["db"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
