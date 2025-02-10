use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3987383493: FileType = FileType {
    file_format: &FileFormat {
        id: 3_987_383_493,
        source_type: SourceType::Iana,
        name: "jxr",
        extensions: &[],
        media_types: &["image/jxr"],
        signatures: &[],
        related_formats: &[],
    },
};
