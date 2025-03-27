use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_817650: FileType = FileType {
    file_format: &FileFormat {
        id: 817_650,
        source_type: SourceType::Wikidata,
        name: "COM",
        extensions: &["com"],
        media_types: &["application/x-dosexec"],
        signatures: &[],
        related_formats: &[],
    },
};
