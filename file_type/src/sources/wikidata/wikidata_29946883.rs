use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29946883: FileType = FileType {
    file_format: &FileFormat {
        id: 29_946_883,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Exchange 4p",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
