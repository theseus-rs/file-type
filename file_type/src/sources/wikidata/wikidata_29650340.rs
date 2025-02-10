use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650340: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_340,
        source_type: SourceType::Wikidata,
        name: "PES",
        extensions: &["pes"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
