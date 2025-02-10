use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4132604366: FileType = FileType {
    file_format: &FileFormat {
        id: 4_132_604_366,
        source_type: SourceType::Iana,
        name: "t38",
        extensions: &[],
        media_types: &["image/t38"],
        signatures: &[],
        related_formats: &[],
    },
};
