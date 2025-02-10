use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3513742129: FileType = FileType {
    file_format: &FileFormat {
        id: 3_513_742_129,
        source_type: SourceType::Iana,
        name: "vnd.japannet-directory-service",
        extensions: &[],
        media_types: &["application/vnd.japannet-directory-service"],
        signatures: &[],
        related_formats: &[],
    },
};
