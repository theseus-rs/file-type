use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2877491882: FileType = FileType {
    file_format: &FileFormat {
        id: 2_877_491_882,
        source_type: SourceType::Iana,
        name: "A2L",
        extensions: &[],
        media_types: &["application/A2L"],
        signatures: &[],
        related_formats: &[],
    },
};
