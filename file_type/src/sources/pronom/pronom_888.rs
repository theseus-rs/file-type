use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_888: FileType = FileType {
    file_format: &FileFormat {
        id: 888,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint for Macintosh",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
