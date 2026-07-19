use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_601118790: FileType = FileType {
    file_format: &FileFormat {
        id: 601_118_790,
        source_type: SourceType::Linguist,
        name: "RAScript",
        extensions: &["rascript"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
