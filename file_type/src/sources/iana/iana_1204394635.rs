use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1204394635: FileType = FileType {
    file_format: &FileFormat {
        id: 1_204_394_635,
        source_type: SourceType::Iana,
        name: "vnd.ms-fontobject",
        extensions: &[],
        media_types: &["application/vnd.ms-fontobject"],
        signatures: &[],
        related_formats: &[],
    },
};
