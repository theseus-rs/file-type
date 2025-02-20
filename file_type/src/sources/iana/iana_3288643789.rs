use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3288643789: FileType = FileType {
    file_format: &FileFormat {
        id: 3_288_643_789,
        source_type: SourceType::Iana,
        name: "x-pki-message",
        extensions: &[],
        media_types: &["application/x-pki-message"],
        signatures: &[],
        related_formats: &[],
    },
};
