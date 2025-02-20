use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_455: FileType = FileType {
    file_format: &FileFormat {
        id: 455,
        source_type: SourceType::Pronom,
        name: "ACBM Graphics",
        extensions: &["acb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
