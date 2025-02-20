use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000609: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_609,
        source_type: SourceType::Wikidata,
        name: "Java Card CAP",
        extensions: &["cap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
