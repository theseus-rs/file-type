use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2812998300: FileType = FileType {
    file_format: &FileFormat {
        id: 2_812_998_300,
        source_type: SourceType::Iana,
        name: "x-x509-ca-cert",
        extensions: &[],
        media_types: &["application/x-x509-ca-cert"],
        signatures: &[],
        related_formats: &[],
    },
};
