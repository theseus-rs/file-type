use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_967: FileType = FileType {
    file_format: &FileFormat {
        id: 967,
        source_type: SourceType::Pronom,
        name: "Microsoft Office Binder File for Windows",
        extensions: &["obd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
