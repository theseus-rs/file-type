use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3235921983: FileType = FileType {
    file_format: &FileFormat {
        id: 3_235_921_983,
        source_type: SourceType::Iana,
        name: "kbl+xml",
        extensions: &[],
        media_types: &["application/kbl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
