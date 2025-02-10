use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1370014260: FileType = FileType {
    file_format: &FileFormat {
        id: 1_370_014_260,
        source_type: SourceType::Iana,
        name: "vnd.oma.cab-feature-handler+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.cab-feature-handler+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
