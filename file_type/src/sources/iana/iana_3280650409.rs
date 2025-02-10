use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3280650409: FileType = FileType {
    file_format: &FileFormat {
        id: 3_280_650_409,
        source_type: SourceType::Iana,
        name: "scvp-cv-response",
        extensions: &[],
        media_types: &["application/scvp-cv-response"],
        signatures: &[],
        related_formats: &[],
    },
};
