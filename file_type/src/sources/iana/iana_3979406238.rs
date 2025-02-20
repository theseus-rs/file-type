use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3979406238: FileType = FileType {
    file_format: &FileFormat {
        id: 3_979_406_238,
        source_type: SourceType::Iana,
        name: "jxrS",
        extensions: &[],
        media_types: &["image/jxrS"],
        signatures: &[],
        related_formats: &[],
    },
};
