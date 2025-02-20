use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2446: FileType = FileType {
    file_format: &FileFormat {
        id: 2_446,
        source_type: SourceType::Pronom,
        name: "Pascal Source Code",
        extensions: &["pas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
