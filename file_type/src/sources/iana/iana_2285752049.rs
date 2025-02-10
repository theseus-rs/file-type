use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2285752049: FileType = FileType {
    file_format: &FileFormat {
        id: 2_285_752_049,
        source_type: SourceType::Iana,
        name: "tnauthlist",
        extensions: &[],
        media_types: &["application/tnauthlist"],
        signatures: &[],
        related_formats: &[],
    },
};
