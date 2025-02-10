use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2240: FileType = FileType {
    file_format: &FileFormat {
        id: 2_240,
        source_type: SourceType::Pronom,
        name: "Corel Photo House Image",
        extensions: &["cps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
