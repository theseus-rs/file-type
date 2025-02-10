use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2041: FileType = FileType {
    file_format: &FileFormat {
        id: 2_041,
        source_type: SourceType::Pronom,
        name: "Sibelius Sound Set Definition",
        extensions: &["set"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
