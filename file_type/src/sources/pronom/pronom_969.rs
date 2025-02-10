use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_969: FileType = FileType {
    file_format: &FileFormat {
        id: 969,
        source_type: SourceType::Pronom,
        name: "Microsoft Office Binder Wizard for Windows",
        extensions: &["obz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
