use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
