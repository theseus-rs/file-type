use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_856: FileType = FileType {
    file_format: &FileFormat {
        id: 856,
        source_type: SourceType::Pronom,
        name: "form*Z Project File",
        extensions: &["fmz"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
