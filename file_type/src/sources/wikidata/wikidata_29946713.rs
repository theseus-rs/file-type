use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29946713: FileType = FileType {
    file_format: &FileFormat {
        id: 29_946_713,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Exchange 1:2001",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
