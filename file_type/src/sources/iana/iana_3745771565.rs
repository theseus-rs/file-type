use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3745771565: FileType = FileType {
    file_format: &FileFormat {
        id: 3_745_771_565,
        source_type: SourceType::Iana,
        name: "BMPEG",
        extensions: &[],
        media_types: &["video/BMPEG"],
        signatures: &[],
        related_formats: &[],
    },
};
