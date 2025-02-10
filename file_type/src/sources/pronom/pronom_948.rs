use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_948: FileType = FileType {
    file_format: &FileFormat {
        id: 948,
        source_type: SourceType::Pronom,
        name: "Microsoft Works Database for Windows",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
