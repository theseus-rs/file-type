use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51034969: FileType = FileType {
    file_format: &FileFormat {
        id: 51_034_969,
        source_type: SourceType::Wikidata,
        name: "Paradox Database Table format, version 3",
        extensions: &["db"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
