use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3264258890: FileType = FileType {
    file_format: &FileFormat {
        id: 3_264_258_890,
        source_type: SourceType::Iana,
        name: "vnd.yamaha.smaf-audio",
        extensions: &[],
        media_types: &["application/vnd.yamaha.smaf-audio"],
        signatures: &[],
        related_formats: &[],
    },
};
