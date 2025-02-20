use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_136: FileType = FileType {
    file_format: &FileFormat {
        id: 136,
        source_type: SourceType::Pronom,
        name: "Freelance File",
        extensions: &["pre"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
