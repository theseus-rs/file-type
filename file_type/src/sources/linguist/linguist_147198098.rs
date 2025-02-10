use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_147198098: FileType = FileType {
    file_format: &FileFormat {
        id: 147_198_098,
        source_type: SourceType::Linguist,
        name: "Adobe Font Metrics",
        extensions: &["afm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
