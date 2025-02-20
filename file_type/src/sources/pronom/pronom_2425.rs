use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2425: FileType = FileType {
    file_format: &FileFormat {
        id: 2_425,
        source_type: SourceType::Pronom,
        name: "Stata .do Command File",
        extensions: &["do"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
