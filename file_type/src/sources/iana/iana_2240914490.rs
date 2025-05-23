use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2240914490: FileType = FileType {
    file_format: &FileFormat {
        id: 2_240_914_490,
        source_type: SourceType::Iana,
        name: "parallel",
        extensions: &[],
        media_types: &["multipart/parallel"],
        signatures: &[],
        related_formats: &[],
    },
};
