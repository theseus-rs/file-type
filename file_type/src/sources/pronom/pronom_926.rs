use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_926: FileType = FileType {
    file_format: &FileFormat {
        id: 926,
        source_type: SourceType::Pronom,
        name: "Mathematica Notebook",
        extensions: &["nb"],
        media_types: &["application/mathematica"],
        signatures: &[],
        related_formats: &[],
    },
};
