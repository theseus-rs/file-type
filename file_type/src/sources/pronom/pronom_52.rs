use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_52: FileType = FileType {
    file_format: &FileFormat {
        id: 52,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Backup",
        extensions: &["xlk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
