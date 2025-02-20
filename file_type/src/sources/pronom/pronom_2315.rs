use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2315: FileType = FileType {
    file_format: &FileFormat {
        id: 2_315,
        source_type: SourceType::Pronom,
        name: "Harvard Graphics Presentation",
        extensions: &["pr4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
