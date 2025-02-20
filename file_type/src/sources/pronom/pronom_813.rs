use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_813: FileType = FileType {
    file_format: &FileFormat {
        id: 813,
        source_type: SourceType::Pronom,
        name: "License file",
        extensions: &["lic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
