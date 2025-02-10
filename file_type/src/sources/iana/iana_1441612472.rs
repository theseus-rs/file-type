use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1441612472: FileType = FileType {
    file_format: &FileFormat {
        id: 1_441_612_472,
        source_type: SourceType::Iana,
        name: "vnd.eszigno3+xml",
        extensions: &[],
        media_types: &["application/vnd.eszigno3+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
