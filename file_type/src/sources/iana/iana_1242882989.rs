use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1242882989: FileType = FileType {
    file_format: &FileFormat {
        id: 1_242_882_989,
        source_type: SourceType::Iana,
        name: "jose+json",
        extensions: &[],
        media_types: &["application/jose+json"],
        signatures: &[],
        related_formats: &[],
    },
};
