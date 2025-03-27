use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29946845: FileType = FileType {
    file_format: &FileFormat {
        id: 29_946_845,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Exchange 3:2003",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
