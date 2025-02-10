use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1251956507: FileType = FileType {
    file_format: &FileFormat {
        id: 1_251_956_507,
        source_type: SourceType::Iana,
        name: "3gppHalForms+json",
        extensions: &[],
        media_types: &["application/3gppHalForms+json"],
        signatures: &[],
        related_formats: &[],
    },
};
