use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2126647565: FileType = FileType {
    file_format: &FileFormat {
        id: 2_126_647_565,
        source_type: SourceType::Iana,
        name: "vnd.xiff",
        extensions: &[],
        media_types: &["image/vnd.xiff"],
        signatures: &[],
        related_formats: &[],
    },
};
