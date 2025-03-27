use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29946141: FileType = FileType {
    file_format: &FileFormat {
        id: 29_946_141,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Variable Data and Transactional Printing, streamed delivery variant",
        extensions: &["pdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
