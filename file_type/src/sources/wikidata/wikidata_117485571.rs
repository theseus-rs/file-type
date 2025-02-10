use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117485571: FileType = FileType {
    file_format: &FileFormat {
        id: 117_485_571,
        source_type: SourceType::Wikidata,
        name: "Audacity Audio Block File",
        extensions: &["au"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
