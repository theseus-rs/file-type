use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4037192836: FileType = FileType {
    file_format: &FileFormat {
        id: 4_037_192_836,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
