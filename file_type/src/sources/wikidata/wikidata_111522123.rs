use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111522123: FileType = FileType {
    file_format: &FileFormat {
        id: 111_522_123,
        source_type: SourceType::Wikidata,
        name: "exFAT (extensible File Allocation Table) disc image",
        extensions: &["img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
