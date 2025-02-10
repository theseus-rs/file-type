use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1565024009: FileType = FileType {
    file_format: &FileFormat {
        id: 1_565_024_009,
        source_type: SourceType::Iana,
        name: "rtploopback",
        extensions: &[],
        media_types: &["application/rtploopback"],
        signatures: &[],
        related_formats: &[],
    },
};
