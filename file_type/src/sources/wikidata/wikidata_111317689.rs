use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111317689: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_689,
        source_type: SourceType::Wikidata,
        name: "Miles Sound System DLS 1 + XMI file",
        extensions: &["mss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
