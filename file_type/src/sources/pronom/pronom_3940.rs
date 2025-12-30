use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_3940: FileType = FileType {
    file_format: &FileFormat {
        id: 3_940,
        source_type: SourceType::Pronom,
        name: "TOML",
        extensions: &["toml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
