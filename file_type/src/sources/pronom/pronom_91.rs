use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_91: FileType = FileType {
    file_format: &FileFormat {
        id: 91,
        source_type: SourceType::Pronom,
        name: "Ventura Publisher Vector Graphics",
        extensions: &["gem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
