use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_180: FileType = FileType {
    file_format: &FileFormat {
        id: 180,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Toolbar",
        extensions: &["xlb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
