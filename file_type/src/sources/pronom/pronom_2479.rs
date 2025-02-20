use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2479: FileType = FileType {
    file_format: &FileFormat {
        id: 2_479,
        source_type: SourceType::Pronom,
        name: "Typescript",
        extensions: &["ts", "tsx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
