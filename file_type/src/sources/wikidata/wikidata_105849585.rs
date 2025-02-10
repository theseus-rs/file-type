use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_105849585: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_585,
        source_type: SourceType::Wikidata,
        name: "Construct 3 Project",
        extensions: &["c3p"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
