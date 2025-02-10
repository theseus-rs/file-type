use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1443796901: FileType = FileType {
    file_format: &FileFormat {
        id: 1_443_796_901,
        source_type: SourceType::Iana,
        name: "vnd.wfa.wsc",
        extensions: &[],
        media_types: &["application/vnd.wfa.wsc"],
        signatures: &[],
        related_formats: &[],
    },
};
