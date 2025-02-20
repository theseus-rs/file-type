use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3133352226: FileType = FileType {
    file_format: &FileFormat {
        id: 3_133_352_226,
        source_type: SourceType::Iana,
        name: "jpeg",
        extensions: &[],
        media_types: &["image/jpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
