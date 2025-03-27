use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29946771: FileType = FileType {
    file_format: &FileFormat {
        id: 29_946_771,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Exchange 1a:2003",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
