use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_885: FileType = FileType {
    file_format: &FileFormat {
        id: 885,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint for Macintosh",
        extensions: &["ppt"],
        media_types: &["application/vnd.ms-powerpoint"],
        signatures: &[],
        related_formats: &[],
    },
};
