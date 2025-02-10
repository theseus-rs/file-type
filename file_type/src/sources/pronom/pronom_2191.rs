use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2191: FileType = FileType {
    file_format: &FileFormat {
        id: 2_191,
        source_type: SourceType::Pronom,
        name: "OmniPage Document",
        extensions: &["opd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
