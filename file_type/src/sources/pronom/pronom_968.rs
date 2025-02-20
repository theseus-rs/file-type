use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_968: FileType = FileType {
    file_format: &FileFormat {
        id: 968,
        source_type: SourceType::Pronom,
        name: "Microsoft Office Binder Template for Windows",
        extensions: &["obt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
