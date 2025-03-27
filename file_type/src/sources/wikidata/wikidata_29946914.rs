use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29946914: FileType = FileType {
    file_format: &FileFormat {
        id: 29_946_914,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Exchange 5n",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
