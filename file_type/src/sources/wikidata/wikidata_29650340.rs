use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
