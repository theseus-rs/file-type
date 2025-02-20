use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2158: FileType = FileType {
    file_format: &FileFormat {
        id: 2_158,
        source_type: SourceType::Pronom,
        name: "BDOC",
        extensions: &["bdoc"],
        media_types: &["application/vnd.bdoc-1.0"],
        signatures: &[],
        related_formats: &[],
    },
};
