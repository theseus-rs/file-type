use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_886: FileType = FileType {
    file_format: &FileFormat {
        id: 886,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint for Macintosh",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
