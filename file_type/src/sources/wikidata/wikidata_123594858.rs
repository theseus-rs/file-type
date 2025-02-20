use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123594858: FileType = FileType {
    file_format: &FileFormat {
        id: 123_594_858,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Archive, version 4e",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
