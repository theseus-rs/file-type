use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_219: FileType = FileType {
    file_format: &FileFormat {
        id: 219,
        source_type: SourceType::Pronom,
        name: "Ventura Publisher",
        extensions: &["gen"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
