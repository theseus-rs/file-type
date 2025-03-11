use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4670672: FileType = FileType {
    file_format: &FileFormat {
        id: 4_670_672,
        source_type: SourceType::Wikidata,
        name: "Abuse Reporting Format",
        extensions: &[],
        media_types: &["message/feedback-report"],
        signatures: &[],
        related_formats: &[],
    },
};
