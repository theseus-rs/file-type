use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2234: FileType = FileType {
    file_format: &FileFormat {
        id: 2_234,
        source_type: SourceType::Pronom,
        name: "GST Publisher File",
        extensions: &["dtp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
