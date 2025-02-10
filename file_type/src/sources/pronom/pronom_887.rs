use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_887: FileType = FileType {
    file_format: &FileFormat {
        id: 887,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint for Macintosh",
        extensions: &["ppt"],
        media_types: &["application/vnd.ms-powerpoint"],
        signatures: &[],
        related_formats: &[],
    },
};
