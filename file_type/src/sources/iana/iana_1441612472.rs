use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
