use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_531: FileType = FileType {
    file_format: &FileFormat {
        id: 531,
        source_type: SourceType::Pronom,
        name: "TeX Binary File",
        extensions: &["dvi"],
        media_types: &["application/x-dvi"],
        signatures: &[],
        related_formats: &[],
    },
};
