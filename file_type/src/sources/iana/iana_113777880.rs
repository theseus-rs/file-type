use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_113777880: FileType = FileType {
    file_format: &FileFormat {
        id: 113_777_880,
        source_type: SourceType::Iana,
        name: "vnd.infotech.project+xml",
        extensions: &[],
        media_types: &["application/vnd.infotech.project+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
