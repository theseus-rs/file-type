use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1623329905: FileType = FileType {
    file_format: &FileFormat {
        id: 1_623_329_905,
        source_type: SourceType::Iana,
        name: "vnd.wap.si",
        extensions: &[],
        media_types: &["text/vnd.wap.si"],
        signatures: &[],
        related_formats: &[],
    },
};
