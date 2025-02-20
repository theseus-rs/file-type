use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_51249193: FileType = FileType {
    file_format: &FileFormat {
        id: 51_249_193,
        source_type: SourceType::Iana,
        name: "t38",
        extensions: &[],
        media_types: &["audio/t38"],
        signatures: &[],
        related_formats: &[],
    },
};
