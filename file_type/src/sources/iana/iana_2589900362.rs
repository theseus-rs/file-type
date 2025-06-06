use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2589900362: FileType = FileType {
    file_format: &FileFormat {
        id: 2_589_900_362,
        source_type: SourceType::Iana,
        name: "ktx2",
        extensions: &[],
        media_types: &["image/ktx2"],
        signatures: &[],
        related_formats: &[],
    },
};
