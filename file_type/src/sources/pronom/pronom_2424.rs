use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2424: FileType = FileType {
    file_format: &FileFormat {
        id: 2_424,
        source_type: SourceType::Pronom,
        name: "PageMaker Template File",
        extensions: &["pt5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
