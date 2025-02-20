use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_440186036: FileType = FileType {
    file_format: &FileFormat {
        id: 440_186_036,
        source_type: SourceType::Iana,
        name: "vnd.wap.wbmp",
        extensions: &[],
        media_types: &["image/vnd.wap.wbmp"],
        signatures: &[],
        related_formats: &[],
    },
};
