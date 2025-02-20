use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_214: FileType = FileType {
    file_format: &FileFormat {
        id: 214,
        source_type: SourceType::Pronom,
        name: "Digital Video",
        extensions: &["dv"],
        media_types: &["video/dv"],
        signatures: &[],
        related_formats: &[],
    },
};
