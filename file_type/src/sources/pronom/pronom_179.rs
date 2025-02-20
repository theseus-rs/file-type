use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_179: FileType = FileType {
    file_format: &FileFormat {
        id: 179,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Add-In",
        extensions: &["xla", "xll"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
