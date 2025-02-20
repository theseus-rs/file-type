use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2111: FileType = FileType {
    file_format: &FileFormat {
        id: 2_111,
        source_type: SourceType::Pronom,
        name: "602Text Document",
        extensions: &["wpd", "wpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
