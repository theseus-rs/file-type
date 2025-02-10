use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_808: FileType = FileType {
    file_format: &FileFormat {
        id: 808,
        source_type: SourceType::Pronom,
        name: "Text Configuration file",
        extensions: &["ini"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
