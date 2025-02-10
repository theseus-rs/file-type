use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3767960255: FileType = FileType {
    file_format: &FileFormat {
        id: 3_767_960_255,
        source_type: SourceType::Iana,
        name: "vnd.ms-excel",
        extensions: &[],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[],
    },
};
