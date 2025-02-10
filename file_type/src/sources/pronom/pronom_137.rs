use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_137: FileType = FileType {
    file_format: &FileFormat {
        id: 137,
        source_type: SourceType::Pronom,
        name: "Microsoft Print File",
        extensions: &["prn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
