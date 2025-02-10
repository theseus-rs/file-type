use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2482: FileType = FileType {
    file_format: &FileFormat {
        id: 2_482,
        source_type: SourceType::Pronom,
        name: "cdrLabel Label File",
        extensions: &["clb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
