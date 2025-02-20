use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_970: FileType = FileType {
    file_format: &FileFormat {
        id: 970,
        source_type: SourceType::Pronom,
        name: "Microsoft Office Binder File for Windows",
        extensions: &["obd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
