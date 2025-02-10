use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1919128391: FileType = FileType {
    file_format: &FileFormat {
        id: 1_919_128_391,
        source_type: SourceType::Iana,
        name: "vnd.microsoft.portable-executable",
        extensions: &[],
        media_types: &["application/vnd.microsoft.portable-executable"],
        signatures: &[],
        related_formats: &[],
    },
};
