use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_921628903: FileType = FileType {
    file_format: &FileFormat {
        id: 921_628_903,
        source_type: SourceType::Iana,
        name: "vnd.ms-works",
        extensions: &[],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
