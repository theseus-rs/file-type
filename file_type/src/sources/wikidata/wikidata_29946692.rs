use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29946692: FileType = FileType {
    file_format: &FileFormat {
        id: 29_946_692,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Exchange 1:1999",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
