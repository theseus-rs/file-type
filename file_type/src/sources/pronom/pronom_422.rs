use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_422: FileType = FileType {
    file_format: &FileFormat {
        id: 422,
        source_type: SourceType::Pronom,
        name: "Real Video",
        extensions: &["rv"],
        media_types: &["video/vnd.rn-realvideo"],
        signatures: &[],
        related_formats: &[],
    },
};
