use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1665161502: FileType = FileType {
    file_format: &FileFormat {
        id: 1_665_161_502,
        source_type: SourceType::Iana,
        name: "dash+xml",
        extensions: &[],
        media_types: &["application/dash+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
