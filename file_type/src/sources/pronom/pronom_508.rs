use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_508: FileType = FileType {
    file_format: &FileFormat {
        id: 508,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Database",
        extensions: &["bdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
