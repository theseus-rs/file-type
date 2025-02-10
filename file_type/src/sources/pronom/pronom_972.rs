use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_972: FileType = FileType {
    file_format: &FileFormat {
        id: 972,
        source_type: SourceType::Pronom,
        name: "Microsoft Office Binder Wizard for Windows",
        extensions: &["obz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
