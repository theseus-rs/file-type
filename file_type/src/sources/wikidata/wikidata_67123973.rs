use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67123973: FileType = FileType {
    file_format: &FileFormat {
        id: 67_123_973,
        source_type: SourceType::Wikidata,
        name: "Print Artist certificate file format",
        extensions: &["cer"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
