use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3959179: FileType = FileType {
    file_format: &FileFormat {
        id: 3_959_179,
        source_type: SourceType::Wikidata,
        name: "shar",
        extensions: &["sha", "shar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
