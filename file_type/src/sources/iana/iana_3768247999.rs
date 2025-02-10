use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3768247999: FileType = FileType {
    file_format: &FileFormat {
        id: 3_768_247_999,
        source_type: SourceType::Iana,
        name: "vnd.bekitzur-stech+json",
        extensions: &[],
        media_types: &["application/vnd.bekitzur-stech+json"],
        signatures: &[],
        related_formats: &[],
    },
};
