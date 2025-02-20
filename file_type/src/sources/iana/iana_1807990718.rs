use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1807990718: FileType = FileType {
    file_format: &FileFormat {
        id: 1_807_990_718,
        source_type: SourceType::Iana,
        name: "mbms-register+xml",
        extensions: &[],
        media_types: &["application/mbms-register+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
