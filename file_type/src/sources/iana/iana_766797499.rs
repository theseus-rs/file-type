use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_766797499: FileType = FileType {
    file_format: &FileFormat {
        id: 766_797_499,
        source_type: SourceType::Iana,
        name: "vnd.ufdl",
        extensions: &[],
        media_types: &["application/vnd.ufdl"],
        signatures: &[],
        related_formats: &[],
    },
};
