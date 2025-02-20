use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3161118794: FileType = FileType {
    file_format: &FileFormat {
        id: 3_161_118_794,
        source_type: SourceType::Iana,
        name: "cwl+json",
        extensions: &[],
        media_types: &["application/cwl+json"],
        signatures: &[],
        related_formats: &[],
    },
};
