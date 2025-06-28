use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1495751075: FileType = FileType {
    file_format: &FileFormat {
        id: 1_495_751_075,
        source_type: SourceType::Iana,
        name: "jpeg2000-scl",
        extensions: &[],
        media_types: &["video/jpeg2000-scl"],
        signatures: &[],
        related_formats: &[],
    },
};
