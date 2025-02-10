use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_849097741: FileType = FileType {
    file_format: &FileFormat {
        id: 849_097_741,
        source_type: SourceType::Iana,
        name: "vnd.omads-file+xml",
        extensions: &[],
        media_types: &["application/vnd.omads-file+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
