use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_290: FileType = FileType {
    file_format: &FileFormat {
        id: 290,
        source_type: SourceType::Pronom,
        name: "SDSC Image Tool X Window Dump Format",
        extensions: &["xwd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
