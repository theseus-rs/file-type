use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2704290828: FileType = FileType {
    file_format: &FileFormat {
        id: 2_704_290_828,
        source_type: SourceType::Iana,
        name: "cpl+xml",
        extensions: &[],
        media_types: &["application/cpl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
